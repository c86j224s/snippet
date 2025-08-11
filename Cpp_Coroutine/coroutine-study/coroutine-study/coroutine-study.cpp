#include <iostream>
#include "generator.hpp"
#include "thread1.hpp"
#include "basic_coroutine.hpp"

int main(int argc, char** argv) {
    std::cout << "Hello, World!" << std::endl;

    test_generator();
    test_thread1();
    test_basic_coroutine();

    return 0;
}