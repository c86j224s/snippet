#pragma once

#include <iostream>
#include <coroutine>
#include <format>
#include <print>
#include <chrono>
#include <thread>

struct resume_on_new_thread {
    bool await_ready() const noexcept {
        std::println("on await ready");
        return false;
    }

    void await_suspend(std::coroutine_handle<> handle) const noexcept {
        std::println("on await suspend");

        std::thread([handle] {
            std::this_thread::sleep_for(std::chrono::seconds(1));
            handle.resume();
            }).detach();
    }

    int await_resume() const noexcept {
        std::println("on await resume");
        return 42;
    }
};

template<typename T>
struct Task {
    struct promise_type;
    using handle_type = std::coroutine_handle<promise_type>;
    handle_type coro_handle;

    struct promise_type {
        T res;
        Task get_return_object() { 
            std::println("on get return object, {}", res);
            return Task{ handle_type::from_promise(*this) };
        }
        std::suspend_never initial_suspend() noexcept { 
            std::println("on initial suspend, {}", res);
            return {}; 
        }
        std::suspend_always final_suspend() noexcept { 
            std::println("on final suspend, {}", res);
            return {}; 
        }
        void unhandled_exception() { 
            std::cerr << "Unhandled exception in coroutine." << std::endl;
            throw; 
        }
        void return_value(T value) { 
            std::println("on return value, {}", value);
            res = value; 
        }
    };

    T get() {
        std::println("on get, {}", coro_handle.address());

        while (!coro_handle.done()) {
            std::println("Coroutine is not done, waiting...");
            std::this_thread::sleep_for(std::chrono::milliseconds(200));
        }

        std::println("Coroutine is done.");
        return coro_handle.promise().res;
    }

    ~Task() {
        std::println("on Task destructor, {}", coro_handle.address());
        if (coro_handle) {
            coro_handle.destroy();
        }
    }
};

Task<int> create_task() {
    std::println("begin on create_task");

    auto result = co_await resume_on_new_thread();

    std::println("resumed on create_task");

    co_return result + 10;
}

void test_thread1() {
    std::println("========== test_thread1");

    auto my_task = create_task();

    auto final_result = my_task.get();

    std::println("Final result: {}", final_result);

    std::println("==========");
}