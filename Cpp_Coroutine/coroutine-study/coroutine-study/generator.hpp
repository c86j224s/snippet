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
            std::cout << std::format("Initial suspend called. {}", now_value) << std::endl;
            return {};
        }
        std::suspend_always final_suspend() noexcept {
            std::cout << std::format("Final suspend called. {}", now_value) << std::endl;
            return {};
        }
        Generator get_return_object() {
            std::cout << std::format("Get return object called. {}", now_value) << std::endl;

            return Generator{ std::coroutine_handle<promise_type>::from_promise(*this) };
        }
        void unhandled_exception() {
            std::cerr << "Unhandled exception in coroutine." << std::endl;
            throw;
        }
        std::suspend_always yield_value(T value) noexcept {
            std::cout << std::format("Yield value called. {}", value) << std::endl;

            now_value = value;
            return {};
        }
        void return_void() noexcept {
            std::cout << std::format("Return void called. {}", now_value) << std::endl;;
        }
    };

    using handle_type = std::coroutine_handle<promise_type>;
    handle_type coro_handle;

    Generator(handle_type h) : coro_handle(h) {
        std::cout << std::format("Generator created with handle: {}", coro_handle.address()) << std::endl;
    }
    ~Generator() {
        std::cout << std::format("Generator destroyed with handle: {}", coro_handle.address()) << std::endl;

        if (coro_handle) {
            coro_handle.destroy();
        }
    }

    std::optional<T> next() {
        if (!coro_handle || coro_handle.done()) {
            std::cout << "on next, Coroutine is done or not initialized." << std::endl;

            return std::nullopt;
        }

        coro_handle.resume();
        if (coro_handle.done()) {
            std::cout << "on next, Coroutine is done after resume." << std::endl;

            return std::nullopt;
        }

        std::cout << "on next, Coroutine resumed successfully." << std::endl;

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
        std::cout << *value << std::endl;
    }

    std::println("==========");
}
