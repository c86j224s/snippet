#pragma once

#include <atomic>
#include <coroutine>
#include <functional>
#include <memory>
#include <map>
#include <optional>
#include <queue>
#include <shared_mutex>
#include <string>
#include <thread>
#include <format>
#include <print>

namespace async_server_chain {

    template<typename T>
    concept TReq = requires(T a) {
        { a.tid } -> std::convertible_to<int>;
        { a.command } -> std::convertible_to<std::string>;
        { a.body } -> std::convertible_to<std::string>;
    };

    template<typename T>
    concept TRes = requires(T a) {
        { a.tid } -> std::convertible_to<int>;
        { a.success } -> std::convertible_to<bool>;
        { a.body } -> std::convertible_to<std::string>;
    };

    struct Request {
        int tid;
        std::string command;
        std::string body;
    };

    struct Response {
        int tid;
        bool success;
        std::string body;
    };

    template<typename Req, typename Res>
        requires TReq<Req> && TRes<Res>
    class SampleServer : std::enable_shared_from_this<SampleServer<Req, Res>> {
        std::shared_mutex mutex_;
        std::queue<Req> request_queue_;
        std::queue<Res> response_queue_;

        void process_requests() {
            while (true) {

                // 요청이 들어올 때까지 대기
                Req req;
                while (true) {
                    {
                        std::shared_lock lock(mutex_);

                        if (request_queue_.empty()) {
                            // 요청이 없으면 잠시 대기
                            std::this_thread::yield();
                            continue;
                        }
                    }

                    // 요청이 왔으므로 가져오기
                    std::unique_lock lock(mutex_);
                    req = request_queue_.front();
                    request_queue_.pop();
                    break;
                }

                std::println("[SampleServer] 요청 처리: {}", req.command);

                // 응답 생성 (시뮬레이션)
                Res res{ req.tid, true, "Response to " + req.command };

                std::unique_lock lock(mutex_);
                response_queue_.push(res);
            }
        }

    public:
        void run() {
            // 요청 처리 스레드 시작
            std::thread([this]() {
                std::println("[SampleServer] 서버 시작");

                process_requests();

                std::println("[SampleServer] 서버 종료");
            }).detach();
        }

        // 요청 발신
        void add_request(const Req& req) {
            {
                std::unique_lock lock(mutex_);
                request_queue_.push(req);
            }

            std::println("[SampleServer] 요청 추가: tid={}, command={}, body={}", 
                req.tid, req.command, req.body);
        }

        // 응답 수신
        Res get_response() {
            Res res;

            while (true) {
                {
                    std::shared_lock lock(mutex_);

                    if (response_queue_.empty()) {
                        std::this_thread::yield();
                        continue;
                    }
                }

                std::unique_lock lock(mutex_);
                res = response_queue_.front();
                response_queue_.pop();
                break;
            }

            std::println("[SampleServer] 응답 반환: tid={}, success={}, body={}", 
                res.tid, res.success, res.body);
            return res;
        }
    };

    template<typename Res>
        requires TRes<Res>
    struct Awaiter {
        std::coroutine_handle<> handle_;
        std::optional<Res> response_;
        Awaiter() : handle_(nullptr) {}
        bool await_ready() const noexcept {
            return false; // 항상 대기 상태
        }
        void await_suspend(std::coroutine_handle<> handle) const {
            const_cast<Awaiter*>(this)->handle_ = handle;
        }
        Res await_resume() const noexcept {
            //if (!response_) {
            //    throw std::runtime_error("No response received");
            //}
            return std::move(response_.value());
        }
        void resume(Res res) {
            response_ = std::move(res);
            if (handle_) {
                handle_.resume();
            }
        }
    };

    // TODO: 체이닝 어떻게 하지?
    // TODO: 동시에 여러개 join?

    struct Task {
        struct promise_type {
            Task get_return_object() {
                return Task{ std::coroutine_handle<promise_type>::from_promise(*this) };
            }
            std::suspend_never initial_suspend() noexcept { return {}; }
            std::suspend_always final_suspend() noexcept { return {}; }
            void return_void() {}
            void unhandled_exception() {}
        };
        std::coroutine_handle<promise_type> handle;
        Task(std::coroutine_handle<promise_type> h) : handle(h) {}
        ~Task() { if (handle) handle.destroy(); }
    };;

    template<typename Req, typename Res>
        requires TReq<Req> && TRes<Res>
    class RequestManager : std::enable_shared_from_this<RequestManager<Req, Res>> {
        std::atomic_int tid_{ 0 };
        std::shared_mutex mutex_;
        std::map<int, std::shared_ptr<Awaiter<Res>>> awaiters_;

        std::shared_ptr<SampleServer<Req, Res>> server_;

        int add_awaiter(std::shared_ptr<Awaiter<Res>> awaiter) {
            std::unique_lock lock(mutex_);
            int id = ++tid_;
            awaiters_[id] = std::move(awaiter);
            return id;
        }

        std::shared_ptr<Awaiter<Res>> pop_awaiter(int id) {
            std::unique_lock lock(mutex_);
            auto it = awaiters_.find(id);
            if (it != awaiters_.end()) {
                auto awaiter = std::move(it->second);
                awaiters_.erase(it);
                return awaiter;
            }

            std::println("[RequestManager] Awaiter not found for id: {}", id);
            return nullptr;
        }

    public:
        void connect_to_server(std::shared_ptr<SampleServer<Req, Res>> server) {
            server_ = std::move(server);

            // 서버 응답 처리 쓰레드 시작
            std::thread([this]() {
                std::println("[RequestManager] 서버 연결됨, 응답 처리 시작");

                process_responses();

                std::println("[RequestManager] 서버 응답 처리 종료");
            }).detach();
        }

        void process_responses() {
            if (!server_) {
                std::println("[RequestManager] 서버가 연결되지 않았습니다.");
                return;
            }

            while (true) {
                auto res = server_->get_response();
                int id = res.tid;
                auto awaiter = pop_awaiter(id);
                if (awaiter) {
                    awaiter->resume(res);
                }
            }
        }

        std::shared_ptr<Awaiter<Res>> async_request(const Req& req) {
            std::println("[RequestManager] 비동기 요청 시작: {}", req.command);
            // 요청 ID 생성
            auto awaiter = std::make_shared<Awaiter<Res>>();
            int id = add_awaiter(awaiter);

            // 요청을 SampleServer에 추가
            if (server_ == nullptr) {
                throw std::runtime_error("서버가 연결되지 않았습니다.");
            }

            server_->add_request(req);

            return awaiter;
        }
    };

    Task simple_test(std::shared_ptr<RequestManager<Request, Response>> manager) {
        // 비동기 요청 예시
        Request req{ 1, "GET /data", "" };
        auto awaiter = manager->async_request(req);
        // 응답 대기 및 처리
        std::println("[simple_test] 응답 대기 요청: tid={}, command={}, body={}", 
            req.tid, req.command, req.body);
        Response res = co_await *awaiter;
        std::println("[simple_test] 응답 수신: tid={}, success={}, body={}", 
            res.tid, res.success, res.body);

        // 추가 요청 예시
        Request req2{ 2, "POST /data", "Sample Data" };
        auto awaiter2 = manager->async_request(req2);
        std::println("[simple_test] 두 번째 응답 대기 요청: tid={}, command={}, body={}", 
            req2.tid, req2.command, req2.body);
        Response res2 = co_await *awaiter2;
        std::println("[simple_test] 두 번째 응답 수신: tid={}, success={}, body={}", 
            res2.tid, res2.success, res2.body);

        std::println("[simple_test] 테스트 완료");

        co_return;
    }

    void test() {
        std::println("[async_server_chain] 테스트 시작");

        // SampleServer 인스턴스 생성 및 실행
        auto server = std::make_shared<SampleServer<Request, Response>>();
        server->run();

        // RequestManager 인스턴스 생성 및 서버 연결
        auto manager = std::make_shared<RequestManager<Request, Response>>();
        manager->connect_to_server(server);

        auto task = simple_test(manager);

        //task.handle.resume(); // 코루틴 시작(이미 시작되어 있음)
        //while (!task.handle.done()) {
        //    std::this_thread::sleep_for(std::chrono::milliseconds(100)); // 대기
        //}

        std::this_thread::sleep_for(std::chrono::seconds(10)); // 마지막 응답 출력 대기

        std::println("[async_server_chain] 테스트 완료");
    }
}