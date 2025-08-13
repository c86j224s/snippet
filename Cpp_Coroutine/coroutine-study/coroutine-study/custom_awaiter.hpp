#pragma once

#include <coroutine>
#include <thread>
#include <chrono>
#include <format>
#include <print>

// 즉시 간단한 값을 반환하는 awaiter
template<typename T>
struct ValueAwaiter {
    T value;

    ValueAwaiter(T v) : value(v) {}

    bool await_ready() const noexcept {
        std::println("(ValueAwaiter::await_ready) value: {}", value);
        return true; // 즉시 준비됨
    }

    void await_suspend(std::coroutine_handle<>) const noexcept {
        std::println("(ValueAwaiter::await_suspend) no suspension needed for value: {}", value);
        // 이 경우에는 아무런 작업도 하지 않음
    }
    
    T await_resume() const noexcept {
        std::println("(ValueAwaiter::await_resume) returning value: {}", value);
        return value; // 값 반환
    }
};

// 비동기 지연 awaiter
template<typename T>
struct DelayedValueAwaiter {
    T value;
    int delay_ms;

    DelayedValueAwaiter(T v, int d) : value(v), delay_ms(d) {}

    bool await_ready() const noexcept {
        std::println("(DelayedValueAwaiter::await_ready) value: {}, delay: {} ms", value, delay_ms);
        return false; // 항상 준비되지 않음 - 지연 필요
    }
    void await_suspend(std::coroutine_handle<> h) const {
        std::println("(DelayedValueAwaiter::await_suspend) suspending coroutine for {} ms", delay_ms);
        std::thread([h, this]() {
            std::this_thread::sleep_for(std::chrono::milliseconds(delay_ms));
            std::println("(DelayedValueAwaiter::await_suspend) resuming coroutine after {} ms", delay_ms);
            h.resume();
        }).detach();
    }
    T await_resume() const noexcept {
        std::println("(await_resume) returning delayed value: {}", value);
        return value; // 값 반환
    }
};

// co_await을 제공하는 awaitable 객체
template<typename T>
struct AwaitableValue {
    T value;

    AwaitableValue(T v) : value(v) {}

    // 즉시 값을 반환하는 awaiter
    ValueAwaiter<T> operator co_await() const {
        std::println("(AwaitableValue::operator co_await) value: {}", value);
        return ValueAwaiter<T>(value);
    }

    // 지연된 값을 반환하는 awaiter
    DelayedValueAwaiter<T> delay(int delay_ms) const {
        std::println("(AwaitableValue::delay) value: {}, delay: {} ms", value, delay_ms);
        return DelayedValueAwaiter<T>(value, delay_ms);
    }
};

// 코루틴 타입(객체) 정의
struct AsyncTask {
    struct promise_type {
        AsyncTask get_return_object() {
            return AsyncTask{std::coroutine_handle<promise_type>::from_promise(*this)};
        }
        std::suspend_never initial_suspend() noexcept {
            std::println("(AsyncTask::initial_suspend) called");
            return {};
        }
        std::suspend_always final_suspend() noexcept {
            std::println("(AsyncTask::final_suspend) called");
            return {};
        }
        void return_void() {
            std::println("(AsyncTask::return_void) called");
        }
        void unhandled_exception() {
            std::println("(AsyncTask::unhandled_exception) called");
            throw; // 예외를 다시 던짐
        }
    };

    std::coroutine_handle<promise_type> handle;

    AsyncTask(std::coroutine_handle<promise_type> h) : handle(h) {
        std::println("(AsyncTask constructor) handle address: {}", handle.address());
    }
    ~AsyncTask() {
        std::println("(AsyncTask destructor) handle address: {}", handle.address());
        if (handle) {
            handle.destroy();
        }
    }
    void wait() {
        while (handle && !handle.done()) {
            std::println("(AsyncTask::wait) waiting...");
            std::this_thread::sleep_for(std::chrono::milliseconds(100)); // 잠시 대기
        }
    }
};

AsyncTask value_awaiter_example() {
    std::println("(value_awaiter_example) starting coroutine");

    // ValueAwaiter 바로 사용
    auto result1 = co_await ValueAwaiter<int>(42);
    std::println("(value_awaiter_example) received value: {}", result1);

    // AwaitableValue 사용(string)
    auto awaitable = AwaitableValue<std::string>("Hello, World!");
    auto result2 = co_await awaitable;
    std::println("(value_awaiter_example) received awaitable value: {}", result2);

    co_return;
}

AsyncTask delayed_awaiter_example() {
    std::println("(delayed_awaiter_example) starting coroutine");

    // DelayedValueAwaiter 사용
    auto result1 = co_await DelayedValueAwaiter<int>(100, 2000);
    std::println("(delayed_awaiter_example) received delayed value: {}", result1);

    // DelayedValueAwaiter를 사용(string)
    auto result2 = co_await AwaitableValue<std::string>("Delayed Hello").delay(3000);
    std::println("(delayed_awaiter_example) received delayed string value: {}", result2);

    co_return;
}

void test_custom_awaiter() {
    std::println("Starting value awaiter example...");
    auto task1 = value_awaiter_example();
    task1.wait();
    std::println("Value awaiter example completed.");

    std::println("Starting delayed awaiter example...");
    auto task2 = delayed_awaiter_example();
    task2.wait();
    std::println("Delayed awaiter example completed.");
}