#pragma once

#include <iostream>
#include <coroutine>
#include <format>
#include <print>

// �ڷ�ƾ Ÿ�� �Ǵ� �ڷ�ƾ ��ü
// - ����ڰ� �����ϴ� Ŭ����
// - �ڷ�ƾ�� ���ΰ� �����ϴ� ���� ����
struct SimpleCoroutine {
    // ��� Ÿ�� Ȥ�� ��� ��ü
    // - �ڷ�ƾ�� ���� ��å�� �����ϴ� ��ø Ŭ����
    // - �����Ϸ��� �ڷ�ƾ ��ȯ �� ����ϴ� �������̽�
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

    // �ڷ�ƾ �ڵ�
    // - ���� �ڷ�ƾ�� ���¿� ������ �����ϴ� ������ ��ü
    // - promise_type�� ���� �ڷ�ƾ�� ���¸� ����
    // - �ڷ�ƾ�� ������ �� promise_type�� �ν��Ͻ��� �����ϰ�, �̸� ���� �ڷ�ƾ�� ����
    // - �ڷ�ƾ�� �Ϸ�Ǹ� �ڵ��� ���� ���¸� Ȯ���ϰ�, �ʿ�� �ı�
    // - �ڷ�ƾ �ڵ��� �ڷ�ƾ�� ���� ���¸� ��Ÿ����, resume(), destroy() ���� �޼��带 ����
    // - �޸��� �ڷ�ƾ �������� ����Ű�� ������ ����
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

// �ڷ�ƾ �Լ�
SimpleCoroutine my_coroutine() {
    std::println("[Coroutine] �ڷ�ƾ �Լ� ����");
    std::println("[Coroutine] ù ��° �۾� ����");

    // === co_await: �Ͻ����� ���� ===
    
    co_await std::suspend_always{}; // ���⼭ ���� �ߴ�, ���� ����

    std::println("[Coroutine] �� ��° �۾� ���� (�簳 ��)");

    co_await std::suspend_always{}; // �� �ٸ� �ߴ���    

    std::println("[Coroutine] �� ��° �۾� ���� (�� ��° �簳 ��)");

    // === co_return: �ڷ�ƾ ���� ===
    co_return; // return_void() ȣ���
}

void test_basic_coroutine() {
    std::println("========== test_basic_coroutine");

    std::println("=== �ڷ�ƾ ���� ===");
    auto coro = my_coroutine(); // Promise ��ü ����, get_return_object() ȣ��

    std::println("\n=== ù ��° �簳 ===");
    coro.resume(); // ù ��° co_await���� ����

    std::println("\n=== �� ��° �簳 ===");
    coro.resume(); // �� ��° co_await���� ����

    std::println("\n=== �� ��° �簳 (�Ϸ�) ===");
    coro.resume(); // co_return���� ����, final_suspend() ȣ��

    std::println("\n=== �ڷ�ƾ ���� Ȯ�� ===");
    std::println("�ڷ�ƾ �Ϸ� ����: {}", (coro.is_done() ? "�Ϸ�" : "���� ��"));

    std::println("==========");
}