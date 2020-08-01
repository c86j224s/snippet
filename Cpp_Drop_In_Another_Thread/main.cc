#include <iostream>
#include <map>
#include <unordered_map>
#include <variant>
#include <optional>
#include <string>
#include <mutex>
#include <chrono>
#include <numeric>
#include <functional>
#include <atomic>
#include <memory>
#include <thread>


struct Obj {
    char m_mem[512 * 1024 * 1024] = { 0, };
    int m_i = 0;

    Obj(int i) : m_i(i) {}
    ~Obj() {}

    int I() const { return m_i; }
};

struct SparseObj {
    std::unordered_map<int, std::vector<int>> m_mem;
    int m_i = 0;

    SparseObj(int i) : m_i(i) {
        for (auto i = 0; i < 1'000'000; i++) {
            m_mem.emplace(i, std::vector<int> {i, });
        }
    }
    ~SparseObj() {}

    int I() const { return m_i; }
};

template<typename T>
class DropQueue {
    std::mutex m_mtx;
    std::vector<T> m_ptrs;
    std::atomic<bool> m_dirty = false;

public:
    DropQueue() {
        m_ptrs.reserve(100'000);
    }
    ~DropQueue() {}

    void Push(T&& ptr) {
        std::lock_guard<std::mutex> lg{ m_mtx };
        m_ptrs.emplace_back(std::forward<T>(ptr));

        m_dirty = true;
    }

    void Consume() {
        if (!m_dirty) return;

        std::lock_guard<std::mutex> lg{ m_mtx };
        m_ptrs.clear();

        m_dirty = false;
    }

    bool Empty() {
        return !m_dirty;
    }
};


double check_duration(std::function<void(void)> f, bool should_print) {
    auto start = std::chrono::steady_clock::now();

    f();

    std::chrono::duration<double> dur = std::chrono::steady_clock::now() - start;

    if (should_print)
        std::cout << "execution duration : " << dur.count() << " secs" << std::endl;

    return dur.count();
}

template<typename T>
double check_duration2(std::function<void(std::unique_ptr<T>)> f, bool should_print, std::unique_ptr<T> arg) {
    auto start = std::chrono::steady_clock::now();

    f(std::move(arg));

    std::chrono::duration<double> dur = std::chrono::steady_clock::now() - start;

    if (should_print)
        std::cout << "execution duration : " << dur.count() << " secs" << std::endl;

    return dur.count();
}

template<typename T, int N>
void check_main() {
    check_duration([] {
        auto result = 0;

        for (auto i = 0; i < N; i++) {
            auto obj = std::make_unique<T>(i);

            result += obj->I();
        }

        std::cout << result << std::endl;
    }, true);

    check_duration([] {
        auto result = 0;

        for (auto i = 0; i < N; i++) {
            auto obj = std::make_unique<T>(i);

            result += obj->I();

            std::thread([](std::unique_ptr<T> p) {}, std::move(obj)).detach();
        }

        std::cout << result << std::endl;
    }, true);

    check_duration([] {
        auto result = 0;
        auto drop_queue = std::make_shared<DropQueue<std::unique_ptr<T>>>();
        auto fin = false;

        std::thread t([&fin, drop_queue]() mutable {
            while (!fin || !drop_queue->Empty()) {
                drop_queue->Consume();

                std::this_thread::yield();
            }
        });

        for (auto i = 0; i < N; i++) {
            auto obj = std::make_unique<T>(i);

            result += obj->I();

            drop_queue->Push(std::move(obj));
        }

        std::cout << result << std::endl;

        fin = true;
        t.join();
    }, true);
}

template<typename T, int N>
void check_main2() {
    {
        std::vector<double> durs;
        auto result = 0;
        for (auto i = 0; i < N; i++) {
            auto obj = std::make_unique<T>(i);

            auto dur = check_duration2<T>([&result](auto obj) mutable {
                result += obj->I();
            }, false, std::move(obj));

            durs.emplace_back(dur);

        }
        std::cout << result << std::endl;
        std::cout << "execution duration : " << std::accumulate(durs.begin(), durs.end(), 0.0) / durs.size() << " secs" << std::endl;
    }

    {
        std::vector<double> durs;
        auto result = 0;
        for (auto i = 0; i < N; i++) {
            auto obj = std::make_unique<T>(i);

            auto dur = check_duration2<T>([&result](auto obj) mutable {
                result += obj->I();

                std::thread([](std::unique_ptr<T> p) {}, std::move(obj)).detach();
            }, false, std::move(obj));

            durs.emplace_back(dur);
        }
        std::cout << result << std::endl;
        std::cout << "execution duration : " << std::accumulate(durs.begin(), durs.end(), 0.0) / durs.size() << " secs" << std::endl;
    }

    {
        std::vector<double> durs;
        auto result = 0;
        auto drop_queue = std::make_shared<DropQueue<std::unique_ptr<T>>>();
        auto fin = false;

        std::thread t([&fin, drop_queue]() mutable {
            while (!fin || !drop_queue->Empty()) {
                drop_queue->Consume();

                std::this_thread::yield();
            }
        });

        for (auto i = 0; i < N; i++) {
            auto obj = std::make_unique<T>(i);
    
            auto dur = check_duration2<T>([drop_queue, result](auto obj) mutable {
                result += obj->I();

                drop_queue->Push(std::move(obj));
            }, false, std::move(obj));

            durs.emplace_back(dur);
        }

        fin = true;
        t.join();

        std::cout << result << std::endl;
        std::cout << "execution duration : " << std::accumulate(durs.begin(), durs.end(), 0.0) / durs.size() << " secs" << std::endl;
    }

}


int main(int argc, char** argv)
{
    std::cout << "========== check method 1 ==========" << std::endl;

    std::cout << "=== 1 ===" << std::endl;
    check_main<Obj, 1>();
    check_main<SparseObj, 1>();

    std::cout << "=== 10 ===" << std::endl;
    check_main<Obj, 10>();
    check_main<SparseObj, 10>();

    std::cout << "=== 20 ===" << std::endl;
    check_main<Obj, 20>();
    check_main<SparseObj, 20>();

    /*
    std::cout << "=== 50 ===" << std::endl;
    check_main<Obj, 50>();
    check_main<SparseObj, 50>();

    std::cout << "=== 100 ===" << std::endl;
    check_main<Obj, 100>();
    check_main<SparseObj, 100>();
    */

    std::cout << "========== check method 2 ==========" << std::endl;

    std::cout << "=== 1 ===" << std::endl;
    check_main2<Obj, 1>();
    check_main2<SparseObj, 1>();

    std::cout << "=== 10 ===" << std::endl;
    check_main2<Obj, 10>();
    check_main2<SparseObj, 10>();

    std::cout << "=== 20 ===" << std::endl;
    check_main2<Obj, 20>();
    check_main2<SparseObj, 20>();

    /*
    std::cout << "=== 50 ===" << std::endl;
    check_main2<Obj, 50>();
    check_main2<SparseObj, 50>();

    std::cout << "=== 100 ===" << std::endl;
    check_main2<Obj, 100>();
    check_main2<SparseObj, 100>();
    */

    return 0;
}
