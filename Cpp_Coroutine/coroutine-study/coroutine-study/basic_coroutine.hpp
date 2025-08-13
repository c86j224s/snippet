#pragma once

#include <iostream>
#include <coroutine>
#include <format>
#include <print>

// 코루틴 타입 또는 코루틴 객체
// - 사용자가 정의하는 클래스
// - 코루틴을 감싸고 제어하는 래퍼 역할
struct SimpleCoroutine {
    // 약속 타입 혹은 약속 객체
    // - 코루틴의 동작 정책을 정의하는 중첩 클래스
    // - 컴파일러가 코루틴 변환 시 사용하는 인터페이스
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

    // 코루틴 핸들
    // - 실제 코루틴의 상태와 실행을 제어하는 저수준 객체
    // - promise_type을 통해 코루틴의 상태를 관리
    // - 코루틴이 생성될 때 promise_type의 인스턴스를 생성하고, 이를 통해 코루틴을 제어
    // - 코루틴이 완료되면 핸들을 통해 상태를 확인하고, 필요시 파괴
    // - 코루틴 핸들은 코루틴의 실행 상태를 나타내며, resume(), destroy() 등의 메서드를 제공
    // - 메모리의 코루틴 프레임을 가리키는 포인터 역할
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

// 코루틴 함수
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