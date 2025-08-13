#include <iostream>
#include "generator.hpp"
#include "thread1.hpp"
#include "basic_coroutine.hpp"
#include "frame_inspection.hpp"
#include "custom_awaiter.hpp"
#include "integrated_coroutine.hpp"

int main(int argc, char** argv) {
    std::cout << "Hello, World!" << std::endl;

    //test_generator();
    //test_thread1();
    //test_basic_coroutine();
    //test_frame_inspection();
    //test_custom_awaiter();
    test_integrated_coroutine();

    return 0;
}