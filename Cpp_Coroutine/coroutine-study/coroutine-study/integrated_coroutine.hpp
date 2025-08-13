#pragma once

#include <coroutine>
#include <thread>
#include <chrono>
#include <optional>
#include <format>
#include <print>

// === 1. 비동기 값 로더 Awaiter (co_await용) ===
template<typename T>
struct AsyncLoader {
    T data;
    int load_time_ms;

    bool await_ready() const noexcept {
        std::println("[AsyncLoader] await_ready 호출, 데이터 로딩 필요: {}", data);
        return false;  // 항상 비동기 로딩
    }

    void await_suspend(std::coroutine_handle<> handle) const {
        std::println("[AsyncLoader] await_suspend 호출, {}ms 후 로딩 완료 예정", load_time_ms);
        std::thread([handle, delay = load_time_ms]() {
                std::this_thread::sleep_for(std::chrono::milliseconds(delay));

                std::println("[AsyncLoader] 로딩 완료, 코루틴 재개");

                handle.resume();
            }).detach();
    }

    T await_resume() const noexcept {
        std::println("[AsyncLoader] await_resume 호출, 로딩된 데이터 반환: {}", data);
        return data;
    }
};

// === 2. Generator + Async 통합 코루틴 타입 ===
template<typename T>
struct AsyncGenerator {
    struct promise_type {
        std::optional<T> current_value;  // co_yield로 생성된 값 저장

        AsyncGenerator get_return_object() {
            return AsyncGenerator{ std::coroutine_handle<promise_type>::from_promise(*this) };
        }

        std::suspend_always initial_suspend() {
            std::println("[Promise] 코루틴 시작 - 초기 일시정지 상태");

            return {};
        }

        std::suspend_always final_suspend() noexcept {
            std::println("[Promise] 코루틴 완료 - 최종 일시정지 상태");

            return {};
        }

        // === co_yield 처리 ===
        std::suspend_always yield_value(T value) {
            std::println("[Promise] co_yield 처리 - 값 저장: {}", value);

            current_value = value;  // Promise에 값 저장
            return {};  // 일시정지하여 외부에서 값을 가져갈 수 있게 함
        }

        // === co_return 처리 ===
        void return_void() {
            std::println("[Promise] co_return 처리 - 생성 완료");

            current_value.reset();  // 더 이상 값 없음을 표시
        }

        void unhandled_exception() {
            std::println("[Promise] 예외 발생, 코루틴 종료");

            throw;  // 예외 발생 시 코루틴 종료
        }
    };

    std::coroutine_handle<promise_type> handle;

    AsyncGenerator(std::coroutine_handle<promise_type> h) : handle(h) {
        std::println("[Generator] AsyncGenerator 생성, handle 주소: {}", handle.address());
    }

    ~AsyncGenerator() {
        std::println("[Generator] AsyncGenerator 소멸, handle 주소: {}", handle.address());
        if (handle) {
            handle.destroy();
        }
    }

    // === Iterator 스타일 인터페이스 ===
    struct iterator {
        std::coroutine_handle<promise_type> coro_handle;

        iterator(std::coroutine_handle<promise_type> h) : coro_handle(h) {}

        iterator& operator++() {
            std::println("[Iterator] 다음 값 요청");

            coro_handle.resume();  // 코루틴 재개하여 다음 값 생성

            // co_yield까지 완전히 대기
            while (coro_handle && !coro_handle.done() && !coro_handle.promise().current_value.has_value()) {
                // TODO: c86j224s, thread를 잡게 되는데... 이게 맞나??
                std::this_thread::yield();
            }

            return *this;
        }

        T operator*() const {
            return coro_handle.promise().current_value.value();
        }

        bool operator!=(const iterator& other) const {
            return coro_handle != other.coro_handle;
        }

        bool is_done() const {
            //return !coro_handle || coro_handle.done() || !coro_handle.promise().current_value.has_value();
            return !coro_handle || coro_handle.done();
        }
    };

    iterator begin() {
        if (handle) {
            std::println("[Generator] begin 호출, 첫 번째 값 생성 시작");

            handle.resume();  // 첫 번째 co_yield까지 실행

            // co_yield까지 완전히 대기
            while (!handle.done() && !handle.promise().current_value.has_value()) {
                // TODO: c86j224s, thread를 잡게 되는데... 이게 맞나??
                std::this_thread::yield();
            }
        }
        return iterator{ handle };
    }

    iterator end() {
        return iterator{ nullptr };
    }
};

// === 3. 모든 키워드를 사용하는 코루틴 함수 ===
AsyncGenerator<std::string> data_processor() {
    std::println("[Coroutine] 데이터 처리 코루틴 시작");

    // === co_await: 비동기 데이터 로딩 ===
    std::println("[Coroutine] 첫 번째 데이터 로딩 중...");
    std::string data1 = co_await AsyncLoader<std::string>{"Database_Record_1", 200};

    // === co_yield: 처리된 데이터 반환 ===
    std::string processed1 = "Processed: " + data1;
    std::println("[Coroutine] 첫 번째 데이터 처리 완료, yield");
    co_yield processed1;  // 외부로 값 반환하고 일시정지

    // === 재개 후 두 번째 데이터 처리 ===
    std::println("[Coroutine] 두 번째 데이터 로딩 중...");
    std::string data2 = co_await AsyncLoader<std::string>{"API_Response_2", 150};

    std::string processed2 = "Processed: " + data2;
    std::println("[Coroutine] 두 번째 데이터 처리 완료, yield");
    co_yield processed2;

    // === 마지막 데이터 처리 ===
    std::println("[Coroutine] 세 번째 데이터 로딩 중...");
    std::string data3 = co_await AsyncLoader<std::string>{"Cache_Data_3", 100};

    std::string processed3 = "Final: " + data3;
    std::println("[Coroutine] 마지막 데이터 처리 완료, yield");
    co_yield processed3;

    // === co_return: 모든 처리 완료 ===
    std::println("[Coroutine] 모든 데이터 처리 완료");
    co_return;  // 생성기 종료
}

// === 4. 코루틴 체이닝 예제 (코루틴 안에서 다른 코루틴 호출) ===
AsyncGenerator<int> number_generator() {
    std::println("[NumberGen] 숫자 생성 코루틴 시작");

    for (int i = 1; i <= 3; ++i) {
        // === co_await: 비동기 숫자 생성 ===
        int number = co_await AsyncLoader<int>{i * 10, 100};

        // === co_yield: 생성된 숫자 반환 ===
        std::println("[NumberGen] 숫자 생성 완료, yield: {}", number);
        co_yield number;
    }

    co_return;
}

AsyncGenerator<std::string> number_processor() {
    std::println("[NumberProcessor] 숫자 처리 코루틴 시작");

    // === 코루틴 안에서 다른 코루틴 사용 ===
    auto num_gen = number_generator();

    for (auto it = num_gen.begin(); !it.is_done(); ++it) {
        int number = *it;

        // === co_await: 처리 시간 시뮬레이션 ===
        std::string processed = co_await AsyncLoader<std::string>{
            "Processed_" + std::to_string(number), 80
        };

        // === co_yield: 처리 결과 반환 ===
        co_yield processed;
    }

    co_return;
}

void test_integrated_coroutine() {
    std::println("=== 통합 코루틴 예제 ===");

    // === 1. 기본 통합 예제 ===
    //std::println("\n--- 데이터 처리 코루틴 ---");
    //auto processor = data_processor();
    //
    //for (auto it = processor.begin(); !it.is_done(); ++it) {
    //    std::string result = *it;
    //    std::println("[Main] 받은 결과: {}", result);
    //    std::println("[Main] 다음 결과 대기...\n");
    //}
    //
    //std::this_thread::sleep_for(std::chrono::seconds(3));  // 마지막 결과 출력 대기

    // === 2. 코루틴 체이닝 예제 === (아직 크래시 남)
    std::println("\n--- 코루틴 체이닝 ---");
    auto chained_processor = number_processor();
    
    for (auto it = chained_processor.begin(); !it.is_done(); ++it) {
        std::string result = *it;
        std::println("[Main] 체이닝 결과: {}", result);
        std::println("[Main] 다음 결과 대기...\n");
    }
    
    std::println("[Main] 모든 처리 완료");
}