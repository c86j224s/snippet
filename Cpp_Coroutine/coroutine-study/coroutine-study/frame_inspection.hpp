#pragma once

#include <iostream>
#include <coroutine>
#include <print>
#include <format>

struct InspectableCoroutine {
    struct promise_type {
        int promise_data = 42;

        InspectableCoroutine get_return_object() {
            std::println("(get_return_object) promise address: {}", reinterpret_cast<size_t>(this));
            std::println("(get_return_object) promise data: {}", promise_data);
            return InspectableCoroutine{ std::coroutine_handle<promise_type>::from_promise(*this) };
        }

        std::suspend_never initial_suspend() noexcept {
            std::println("(initial_suspend) called, suspend_never, promise_data[{}]", promise_data);
            return {};
        }

        std::suspend_always final_suspend() noexcept {
            std::println("(final_suspend) called, suspend_always, promise_data[{}]", promise_data);
            return {};
        }

        void return_void() {
            std::println("(return_void) called, promise_data[{}]", promise_data);
        }

        //void return_value(int value) {
        //    std::println("(return_value) called with value {}, promise_data[{}]", value, promise_data);
        //}

        void unhandled_exception() {
            std::println("(unhandled_exception) called, promise_data[{}]", promise_data);
            throw;
        }
    };

    std::coroutine_handle<promise_type> handle;

    InspectableCoroutine(std::coroutine_handle<promise_type> h) : handle(h) {
        std::println("(InspectableCoroutine constructor) handle address: {}", handle.address());
    }

    ~InspectableCoroutine() {
        std::println("(~InspectableCoroutine destructor) handle address: {}", handle.address());
        if (handle) {
            handle.destroy();
        }
    }

    void resume() {
        if (handle && !handle.done()) {
            handle.resume();
            std::println("(resume) handle address: {}, promise_data[{}]", handle.address(), handle.promise().promise_data);
        } else {
            std::println("(resume) coroutine already done.");
        }
    }

    void inspect_frame() {
        if (handle) {
            std::println("(inspect_frame) handle address: {}", handle.address());
            std::println("(inspect_frame) handle done: {}", handle.done());
            std::println("(inspect_frame) promise address: {}", reinterpret_cast<size_t>(std::addressof(handle.promise())));
            std::println("(inspect_frame) promise data: {}", handle.promise().promise_data);
        }
    }
};

InspectableCoroutine frame_demo(int param1, const std::string& param2) {
    int local_var1 = param1 * 2;
    std::string local_var2 = param2 + "_modified";

    std::println("(frame_demo) parameters: param1[{}({})], param2[{}({})]", param1, reinterpret_cast<size_t>(&param1), param2, reinterpret_cast<size_t>(&param2));
    std::println("(frame_demo) local variables: local_var1[{}({})], local_var2[{}({})]", local_var1, reinterpret_cast<size_t>(&local_var1), local_var2, reinterpret_cast<size_t>(&local_var2));

    std::println("(frame_demo) first suspend");
    co_await std::suspend_always{};

    // 값복사가 아닌 참조인 param2는 유지되지 않음
    std::println("(frame_demo) after first suspend, parameters: param1[{}({})], param2[{}({})]", param1, reinterpret_cast<size_t>(&param1), param2, reinterpret_cast<size_t>(&param2));
    std::println("(frame_demo) after first suspend, local variables: local_var1[{}({})], local_var2[{}({})]", local_var1, reinterpret_cast<size_t>(&local_var1), local_var2, reinterpret_cast<size_t>(&local_var2));

    local_var1 += 100;
    local_var2 += "_again";

    std::println("(frame_demo) second suspend");
    co_await std::suspend_always{};

    std::println("(frame_demo) after second suspend, parameters: param1[{}({})], param2[{}({})]", param1, reinterpret_cast<size_t>(&param1), param2, reinterpret_cast<size_t>(&param2));
    std::println("(frame_demo) after second suspend, local variables: local_var1[{}({})], local_var2[{}({})]", local_var1, reinterpret_cast<size_t>(&local_var1), local_var2, reinterpret_cast<size_t>(&local_var2));

    co_return;
}

void test_frame_inspection() {
    std::println("========== test_frame_inspection");

    std::println("=== Coroutine Creation ===");
    auto coro = frame_demo(10, "test_string");
    
    std::println("=== first inspection ===");
    coro.inspect_frame();
    std::println("=== Resuming Coroutine ===");
    coro.resume();

    std::println("=== second inspection ===");
    coro.inspect_frame();
    std::println("=== Resuming Coroutine Again ===");
    coro.resume();

    std::println("=== Final Inspection ===");
    coro.inspect_frame();
    std::println("=== Resuming Coroutine Finally ===");
    coro.resume();

    std::println("=== Frame Inspection Test Completed ===");
}