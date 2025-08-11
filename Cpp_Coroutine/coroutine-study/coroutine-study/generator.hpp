#pragma once

#include <iostream>
#include <coroutine>
#include <optional>
#include <format>
#include <print>

template<typename T>
struct Generator {
    struct promise_type {
        T  now_value;
        std::suspend_always initial_suspend() noexcept {
            std::println("Initial suspend called. {}", now_value);
            return {};
        }
        std::suspend_always final_suspend() noexcept {
            std::println("Final suspend called. {}", now_value);
            return {};
        }
        Generator get_return_object() {
            std::println("Get return object called. {}", now_value);
            return Generator{ std::coroutine_handle<promise_type>::from_promise(*this) };
        }
        void unhandled_exception() {
            std::println("Unhandled exception in coroutine.");
            throw;
        }
        std::suspend_always yield_value(T value) noexcept {
            std::println("Yield value called. {}", value);
            now_value = value;
            return {};
        }
        void return_void() noexcept {
            std::println("Return void called. {}", now_value);
        }
    };

    using handle_type = std::coroutine_handle<promise_type>;
    handle_type coro_handle;

    Generator(handle_type h) : coro_handle(h) {
        std::println("Generator created with handle: {}", coro_handle.address());
    }
    ~Generator() {
        std::println("Generator destroyed with handle: {}", coro_handle.address());
        if (coro_handle) {
            coro_handle.destroy();
        }
    }

    std::optional<T> next() {
        if (!coro_handle || coro_handle.done()) {
            std::println("on next, Coroutine is done or not initialized.");
            return std::nullopt;
        }
        coro_handle.resume();
        if (coro_handle.done()) {
            std::println("on next, Coroutine is done after resume.");
            return std::nullopt;
        }
        std::println("on next, Coroutine resumed successfully.");
        return coro_handle.promise().now_value;
    }
};

Generator<int> generate_numbers(int start, int end) {
    for (int i = start; i < end; ++i) {
        co_yield i;
    }
    co_return;
}

void test_generator() {
    std::println("========== test_generator");
    auto gen = generate_numbers(0, 10);
    while (auto value = gen.next()) {
        std::println("{}", *value);
    }
    std::println("==========");
}
