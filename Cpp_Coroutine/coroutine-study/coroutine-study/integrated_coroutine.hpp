#pragma once

#include <coroutine>
#include <thread>
#include <chrono>
#include <optional>
#include <format>
#include <print>

// === 1. �񵿱� �� �δ� Awaiter (co_await��) ===
template<typename T>
struct AsyncLoader {
    T data;
    int load_time_ms;

    bool await_ready() const noexcept {
        std::println("[AsyncLoader] await_ready ȣ��, ������ �ε� �ʿ�: {}", data);
        return false;  // �׻� �񵿱� �ε�
    }

    void await_suspend(std::coroutine_handle<> handle) const {
        std::println("[AsyncLoader] await_suspend ȣ��, {}ms �� �ε� �Ϸ� ����", load_time_ms);
        std::thread([handle, delay = load_time_ms]() {
                std::this_thread::sleep_for(std::chrono::milliseconds(delay));

                std::println("[AsyncLoader] �ε� �Ϸ�, �ڷ�ƾ �簳");

                handle.resume();
            }).detach();
    }

    T await_resume() const noexcept {
        std::println("[AsyncLoader] await_resume ȣ��, �ε��� ������ ��ȯ: {}", data);
        return data;
    }
};

// === 2. Generator + Async ���� �ڷ�ƾ Ÿ�� ===
template<typename T>
struct AsyncGenerator {
    struct promise_type {
        std::optional<T> current_value;  // co_yield�� ������ �� ����

        AsyncGenerator get_return_object() {
            return AsyncGenerator{ std::coroutine_handle<promise_type>::from_promise(*this) };
        }

        std::suspend_always initial_suspend() {
            std::println("[Promise] �ڷ�ƾ ���� - �ʱ� �Ͻ����� ����");

            return {};
        }

        std::suspend_always final_suspend() noexcept {
            std::println("[Promise] �ڷ�ƾ �Ϸ� - ���� �Ͻ����� ����");

            return {};
        }

        // === co_yield ó�� ===
        std::suspend_always yield_value(T value) {
            std::println("[Promise] co_yield ó�� - �� ����: {}", value);

            current_value = value;  // Promise�� �� ����
            return {};  // �Ͻ������Ͽ� �ܺο��� ���� ������ �� �ְ� ��
        }

        // === co_return ó�� ===
        void return_void() {
            std::println("[Promise] co_return ó�� - ���� �Ϸ�");

            current_value.reset();  // �� �̻� �� ������ ǥ��
        }

        void unhandled_exception() {
            std::println("[Promise] ���� �߻�, �ڷ�ƾ ����");

            throw;  // ���� �߻� �� �ڷ�ƾ ����
        }
    };

    std::coroutine_handle<promise_type> handle;

    AsyncGenerator(std::coroutine_handle<promise_type> h) : handle(h) {
        std::println("[Generator] AsyncGenerator ����, handle �ּ�: {}", handle.address());
    }

    ~AsyncGenerator() {
        std::println("[Generator] AsyncGenerator �Ҹ�, handle �ּ�: {}", handle.address());
        if (handle) {
            handle.destroy();
        }
    }

    // === Iterator ��Ÿ�� �������̽� ===
    struct iterator {
        std::coroutine_handle<promise_type> coro_handle;

        iterator(std::coroutine_handle<promise_type> h) : coro_handle(h) {}

        iterator& operator++() {
            std::println("[Iterator] ���� �� ��û");

            coro_handle.resume();  // �ڷ�ƾ �簳�Ͽ� ���� �� ����

            // co_yield���� ������ ���
            while (coro_handle && !coro_handle.done() && !coro_handle.promise().current_value.has_value()) {
                // TODO: c86j224s, thread�� ��� �Ǵµ�... �̰� �³�??
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
            std::println("[Generator] begin ȣ��, ù ��° �� ���� ����");

            handle.resume();  // ù ��° co_yield���� ����

            // co_yield���� ������ ���
            while (!handle.done() && !handle.promise().current_value.has_value()) {
                // TODO: c86j224s, thread�� ��� �Ǵµ�... �̰� �³�??
                std::this_thread::yield();
            }
        }
        return iterator{ handle };
    }

    iterator end() {
        return iterator{ nullptr };
    }
};

// === 3. ��� Ű���带 ����ϴ� �ڷ�ƾ �Լ� ===
AsyncGenerator<std::string> data_processor() {
    std::println("[Coroutine] ������ ó�� �ڷ�ƾ ����");

    // === co_await: �񵿱� ������ �ε� ===
    std::println("[Coroutine] ù ��° ������ �ε� ��...");
    std::string data1 = co_await AsyncLoader<std::string>{"Database_Record_1", 200};

    // === co_yield: ó���� ������ ��ȯ ===
    std::string processed1 = "Processed: " + data1;
    std::println("[Coroutine] ù ��° ������ ó�� �Ϸ�, yield");
    co_yield processed1;  // �ܺη� �� ��ȯ�ϰ� �Ͻ�����

    // === �簳 �� �� ��° ������ ó�� ===
    std::println("[Coroutine] �� ��° ������ �ε� ��...");
    std::string data2 = co_await AsyncLoader<std::string>{"API_Response_2", 150};

    std::string processed2 = "Processed: " + data2;
    std::println("[Coroutine] �� ��° ������ ó�� �Ϸ�, yield");
    co_yield processed2;

    // === ������ ������ ó�� ===
    std::println("[Coroutine] �� ��° ������ �ε� ��...");
    std::string data3 = co_await AsyncLoader<std::string>{"Cache_Data_3", 100};

    std::string processed3 = "Final: " + data3;
    std::println("[Coroutine] ������ ������ ó�� �Ϸ�, yield");
    co_yield processed3;

    // === co_return: ��� ó�� �Ϸ� ===
    std::println("[Coroutine] ��� ������ ó�� �Ϸ�");
    co_return;  // ������ ����
}

// === 4. �ڷ�ƾ ü�̴� ���� (�ڷ�ƾ �ȿ��� �ٸ� �ڷ�ƾ ȣ��) ===
AsyncGenerator<int> number_generator() {
    std::println("[NumberGen] ���� ���� �ڷ�ƾ ����");

    for (int i = 1; i <= 3; ++i) {
        // === co_await: �񵿱� ���� ���� ===
        int number = co_await AsyncLoader<int>{i * 10, 100};

        // === co_yield: ������ ���� ��ȯ ===
        std::println("[NumberGen] ���� ���� �Ϸ�, yield: {}", number);
        co_yield number;
    }

    co_return;
}

AsyncGenerator<std::string> number_processor() {
    std::println("[NumberProcessor] ���� ó�� �ڷ�ƾ ����");

    // === �ڷ�ƾ �ȿ��� �ٸ� �ڷ�ƾ ��� ===
    auto num_gen = number_generator();

    for (auto it = num_gen.begin(); !it.is_done(); ++it) {
        int number = *it;

        // === co_await: ó�� �ð� �ùķ��̼� ===
        std::string processed = co_await AsyncLoader<std::string>{
            "Processed_" + std::to_string(number), 80
        };

        // === co_yield: ó�� ��� ��ȯ ===
        co_yield processed;
    }

    co_return;
}

void test_integrated_coroutine() {
    std::println("=== ���� �ڷ�ƾ ���� ===");

    // === 1. �⺻ ���� ���� ===
    //std::println("\n--- ������ ó�� �ڷ�ƾ ---");
    //auto processor = data_processor();
    //
    //for (auto it = processor.begin(); !it.is_done(); ++it) {
    //    std::string result = *it;
    //    std::println("[Main] ���� ���: {}", result);
    //    std::println("[Main] ���� ��� ���...\n");
    //}
    //
    //std::this_thread::sleep_for(std::chrono::seconds(3));  // ������ ��� ��� ���

    // === 2. �ڷ�ƾ ü�̴� ���� === (���� ũ���� ��)
    std::println("\n--- �ڷ�ƾ ü�̴� ---");
    auto chained_processor = number_processor();
    
    for (auto it = chained_processor.begin(); !it.is_done(); ++it) {
        std::string result = *it;
        std::println("[Main] ü�̴� ���: {}", result);
        std::println("[Main] ���� ��� ���...\n");
    }
    
    std::println("[Main] ��� ó�� �Ϸ�");
}