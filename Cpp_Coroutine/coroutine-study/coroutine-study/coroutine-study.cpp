#include <iostream>
#include "generator.hpp"
#include "thread1.hpp"

int main(int argc, char** argv) {
    std::cout << "Hello, World!" << std::endl;

    test_generator();
    test_thread1();

    return 0;
}