#include <iostream>
#include <cassert>
#include "generator.hpp"

void test_basic_generation() {
    std::cout << "Testing basic generation..." << std::endl;
    
    auto gen = generate_numbers(0, 5);
    int expected = 0;
    
    while (auto value = gen.next()) {
        assert(value.value() == expected);
        expected++;
    }
    
    assert(expected == 5);
    std::cout << "✓ Basic generation test passed" << std::endl;
}

void test_empty_range() {
    std::cout << "Testing empty range..." << std::endl;
    
    auto gen = generate_numbers(5, 5);
    assert(!gen.next().has_value());
    
    std::cout << "✓ Empty range test passed" << std::endl;
}

void test_single_value() {
    std::cout << "Testing single value..." << std::endl;
    
    auto gen = generate_numbers(10, 11);
    
    auto first = gen.next();
    assert(first.has_value());
    assert(first.value() == 10);
    
    assert(!gen.next().has_value());
    
    std::cout << "✓ Single value test passed" << std::endl;
}

void test_large_range() {
    std::cout << "Testing large range..." << std::endl;
    
    auto gen = generate_numbers(0, 1000);
    int count = 0;
    int expected = 0;
    
    while (auto value = gen.next()) {
        assert(value.value() == expected);
        expected++;
        count++;
    }
    
    assert(count == 1000);
    std::cout << "✓ Large range test passed" << std::endl;
}

int main() {
    std::cout << "Running C++ Coroutine Generator Tests..." << std::endl;
    std::cout << "========================================" << std::endl;
    
    try {
        test_basic_generation();
        test_empty_range();
        test_single_value();
        test_large_range();
        
        std::cout << "========================================" << std::endl;
        std::cout << "✅ All tests passed!" << std::endl;
        return 0;
        
    } catch (const std::exception& e) {
        std::cerr << "❌ Test failed with exception: " << e.what() << std::endl;
        return 1;
    } catch (...) {
        std::cerr << "❌ Test failed with unknown exception" << std::endl;
        return 1;
    }
}