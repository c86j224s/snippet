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
        int tid = 0;
        std::string command;
        std::string body;
    };

    struct Response {
        int tid = 0;
        bool success = false;
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

    template<typename T>
    struct Awaiter {
        std::coroutine_handle<> handle_;
        std::optional<T> result_;
        Awaiter() : handle_(nullptr) {}
        bool await_ready() const noexcept {
            return false; // 항상 대기 상태
        }
        void await_suspend(std::coroutine_handle<> handle) const {
            const_cast<Awaiter*>(this)->handle_ = handle;
        }
        T await_resume() const noexcept {
            //if (!response_) {
            //    throw std::runtime_error("No response received");
            //}
            return std::move(result_.value());
        }
        void resume(T v) {
            result_ = std::move(v);
            if (handle_) {
                handle_.resume();
            }
        }
    };

    template<>
    struct Awaiter<void> {
        std::coroutine_handle<> handle_;

        Awaiter() : handle_(nullptr) {}
        bool await_ready() const noexcept {
            return false; // 항상 대기 상태
        }
        void await_suspend(std::coroutine_handle<> handle) const {
            const_cast<Awaiter*>(this)->handle_ = handle;
        }
        void await_resume() const noexcept {
            // do nothing
        }
        void resume() {
            if (handle_) {
                handle_.resume();
            }
        }
    };

    // TODO: 체이닝 어떻게 하지?
    // TODO: 동시에 여러개 join?

    template<typename T>
    struct Task {
        struct promise_type {
            std::coroutine_handle<> handle_;
            T result_;
            std::exception_ptr exception_;

            Task get_return_object() {
                return Task{ std::coroutine_handle<promise_type>::from_promise(*this) };
            }
            std::suspend_never initial_suspend() noexcept { return {}; }
            std::suspend_always final_suspend() noexcept { 
                if (handle_) {
                    handle_.resume();
                }
                return {};
            }
            void return_value(T value) {
                result_ = std::move(value);
            }
            void unhandled_exception() {
                exception_ = std::current_exception();
            }
        };

        std::coroutine_handle<promise_type> handle_;
        Task(std::coroutine_handle<promise_type> h) : handle_(h) {}
        ~Task() { if (handle_) handle_.destroy(); }
        T get() {
            if (handle_.promise().exception_) {
                std::rethrow_exception(handle_.promise().exception_);
            }
            return std::move(handle_.promise().result_);
        }

        bool await_ready() const noexcept {
            return handle_.done();
        }
        void await_suspend(std::coroutine_handle<> awaiting_handle) const {
            handle_.promise().handle_ = awaiting_handle;
        }
        T await_resume() {
            if (handle_.promise().exception_) {
                std::rethrow_exception(handle_.promise().exception_);
            }
            return std::move(handle_.promise().result_);
        }
    };

    template<>
    struct Task<void> {
        struct promise_type {
            std::coroutine_handle<> handle_;
            std::exception_ptr exception_;
            Task get_return_object() {
                return Task{ std::coroutine_handle<promise_type>::from_promise(*this) };
            }
            std::suspend_never initial_suspend() noexcept { return {}; }
            std::suspend_always final_suspend() noexcept { 
                if (handle_) {
                    handle_.resume();
                }
                return {};
            }
            void return_void() {}
            void unhandled_exception() {
                exception_ = std::current_exception();
            }
        };

        std::coroutine_handle<promise_type> handle_;

        Task(std::coroutine_handle<promise_type> h) : handle_(h) {}
        ~Task() { if (handle_) handle_.destroy(); }

        void get() {
            if (handle_.promise().exception_) {
                std::rethrow_exception(handle_.promise().exception_);
            }
        }

        bool await_ready() const noexcept {
            return handle_.done();
        }
        void await_suspend(std::coroutine_handle<> awaiting_handle) const {
            handle_.promise().handle_ = awaiting_handle;
        }
        void await_resume() {
            if (handle_.promise().exception_) {
                std::rethrow_exception(handle_.promise().exception_);
            }
        }
    };

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

        std::shared_ptr<Awaiter<Res>> async_request(Req req) {
            std::println("[RequestManager] 비동기 요청 시작: {}", req.command);
            // 요청 ID 생성
            auto awaiter = std::make_shared<Awaiter<Res>>();
            int id = add_awaiter(awaiter);

            // 요청을 SampleServer에 추가
            if (server_ == nullptr) {
                throw std::runtime_error("서버가 연결되지 않았습니다.");
            }

            req.tid = id;
            server_->add_request(req);

            return awaiter;
        }
    };

    Task<void> simple_test(std::shared_ptr<RequestManager<Request, Response>> manager) {
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

    Task<Response> get_user_info(std::shared_ptr<RequestManager<Request, Response>> manager, int user_id) {
        std::println("[get_user_info] 사용자 정보 요청 시작: user_id={}", user_id);

        // GET /user_info 요청
        Request req{ 2, "GET /user_info", std::to_string(user_id) };
        auto awaiter = manager->async_request(req);
        std::println("[get_user_info] 사용자 정보 응답 대기 요청: tid={}, command={}, body={}", 
            req.tid, req.command, req.body);
        Response res = co_await *awaiter;
        std::println("[get_user_info] 사용자 정보 응답 수신: tid={}, success={}, body={}", 
            res.tid, res.success, res.body);
        std::println("[get_user_info] 사용자 정보 요청 완료");

        // GET /user_profile 요청
        Request profile_req{ 3, "GET /user_profile", "" };
        auto profile_awaiter = manager->async_request(profile_req);
        std::println("[get_user_info] 사용자 프로필 응답 대기 요청: tid={}, command={}, body={}",
            profile_req.tid, profile_req.command, profile_req.body);
        Response profile_res = co_await *profile_awaiter;
        std::println("[get_user_info] 사용자 프로필 응답 수신: tid={}, success={}, body={}",
            profile_res.tid, profile_res.success, profile_res.body);

        Response ret{ res.tid, res.success && profile_res.success, 
            std::format("User Info: {}, User Profile: {}", res.body, profile_res.body) };

        co_return ret;
    }

    Task<void> chain_test(std::shared_ptr<RequestManager<Request, Response>> manager) {
        std::println("[chain_test] 체인 테스트 시작");

        // GET /auth 요청
        Request auth_req{ 1, "GET /auth", "" };
        auto auth_awaiter = manager->async_request(auth_req);
        std::println("[chain_test] 인증 응답 대기 요청: tid={}, command={}, body={}", 
            auth_req.tid, auth_req.command, auth_req.body);
        Response auth_res = co_await *auth_awaiter;
        std::println("[chain_test] 인증 응답 수신: tid={}, success={}, body={}", 
            auth_res.tid, auth_res.success, auth_res.body);

        // simple_test 함수 호출
        std::println("[chain_test] simple_test 호출");
        auto simple_task = simple_test(manager);
        // 코루틴을 시작하고 완료될 때까지 대기
        co_await simple_task;
        std::println("[chain_test] simple_test 완료");

        // get_user_info 함수 호출
        std::println("[chain_test] get_user_info 호출");
        auto user_info_task = get_user_info(manager, 42);
        // 코루틴을 시작하고 완료될 때까지 대기
        Response user_info_res = co_await user_info_task;
        std::println("[chain_test] 사용자 정보 응답 수신: tid={}, success={}, body={}", 
            user_info_res.tid, user_info_res.success, user_info_res.body);

        // GET /role 요청
        Request role_req{ 4, "GET /role", std::to_string(user_info_res.tid) };
        auto role_awaiter = manager->async_request(role_req);
        std::println("[chain_test] 역할 응답 대기 요청: tid={}, command={}, body={}", 
            role_req.tid, role_req.command, role_req.body);
        Response role_res = co_await *role_awaiter;
        std::println("[chain_test] 역할 응답 수신: tid={}, success={}, body={}", 
            role_res.tid, role_res.success, role_res.body);

        std::println("[chain_test] 체인 테스트 완료");

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

        //auto task = simple_test(manager);
        auto task = chain_test(manager);
        task.get();

        std::this_thread::sleep_for(std::chrono::seconds(10)); // 서버 종료 대기

        std::println("[async_server_chain] 테스트 완료");
    }
}