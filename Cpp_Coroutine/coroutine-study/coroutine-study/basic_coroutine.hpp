#pragma once

#include <iostream>
#include <coroutine>
#include <format>
#include <print>

struct SimpleCoroutine {
    struct promise_type {
        SimpleCoroutine get_return_object() {
            std::println("Creating coroutine return object.");

            return SimpleCoroutine{ std::coroutine_handle<promise_type>::from_promise(*this) };
        }

        std::suspend_never initial_suspend() noexcept {
            std::println("Coroutine initial suspend.");

            return {};
        }
        std::suspend_always final_suspend() noexcept {
            std::println("Coroutine final suspend.");

            return {};
        }
        void unhandled_exception() {
            std::println("Unhandled exception in coroutine.");

            throw;
        }

        void return_void() {
            std::println("Coroutine returning void.");
        }
    };

    using handle_type = std::coroutine_handle<promise_type>;
    handle_type coro_handle;

    SimpleCoroutine(handle_type h) : coro_handle(h) {}
    
    ~SimpleCoroutine() {
        std::println("Destroying coroutine handle.");

        if (coro_handle) {
            coro_handle.destroy();
        }
    }
    void resume() {
        std::println("Resuming coroutine.");

        if (coro_handle  && !coro_handle.done()) {
            std::println("Coroutine is not done, resuming...");

            coro_handle.resume();
        }
    }

    bool is_done() const {
        std::println("Checking if coroutine is done.");

        return coro_handle.done();
    }
};

SimpleCoroutine my_coroutine() {
    std::println("[Coroutine] 코루틴 함수 시작");
    std::println("[Coroutine] 첫 번째 작업 수행");

    // === co_await: 일시정지 지점 ===
    
    co_await std::suspend_always{}; // 여기서 실행 중단, 상태 보존

    std::println("[Coroutine] 두 번째 작업 수행 (재개 후)");

    co_await std::suspend_always{}; // 또 다른 중단점    

    std::println("[Coroutine] 세 번째 작업 수행 (두 번째 재개 후)");

    // === co_return: 코루틴 종료 ===
    co_return; // return_void() 호출됨
}

void test_basic_coroutine() {
    std::println("========== test_basic_coroutine");

    std::println("=== 코루틴 생성 ===");
    auto coro = my_coroutine(); // Promise 객체 생성, get_return_object() 호출

    std::println("\n=== 첫 번째 재개 ===");
    coro.resume(); // 첫 번째 co_await까지 실행

    std::println("\n=== 두 번째 재개 ===");
    coro.resume(); // 두 번째 co_await까지 실행

    std::println("\n=== 세 번째 재개 (완료) ===");
    coro.resume(); // co_return까지 실행, final_suspend() 호출

    std::println("\n=== 코루틴 상태 확인 ===");
    std::println("코루틴 완료 여부: {}", (coro.is_done() ? "완료" : "진행 중"));

    std::println("==========");
}