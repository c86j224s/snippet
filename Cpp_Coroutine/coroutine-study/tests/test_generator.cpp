#include <gtest/gtest.h>
#include "generator.hpp"

// Generator 기본 기능 테스트
TEST(GeneratorTest, BasicGeneration) {
    auto gen = generate_numbers(0, 5);
    
    // 0부터 4까지 생성되는지 확인
    int expected = 0;
    while (auto value = gen.next()) {
        EXPECT_EQ(value.value(), expected);
        expected++;
    }
    
    // 총 5개가 생성되었는지 확인
    EXPECT_EQ(expected, 5);
}

// 빈 범위 테스트
TEST(GeneratorTest, EmptyRange) {
    auto gen = generate_numbers(5, 5);  // 시작과 끝이 같음
    
    // 아무것도 생성되지 않아야 함
    EXPECT_FALSE(gen.next().has_value());
}

// 역방향 범위 테스트
TEST(GeneratorTest, ReverseRange) {
    auto gen = generate_numbers(5, 0);  // 시작이 끝보다 큼
    
    // 아무것도 생성되지 않아야 함
    EXPECT_FALSE(gen.next().has_value());
}

// 단일 값 테스트
TEST(GeneratorTest, SingleValue) {
    auto gen = generate_numbers(10, 11);
    
    auto first = gen.next();
    ASSERT_TRUE(first.has_value());
    EXPECT_EQ(first.value(), 10);
    
    // 두 번째 호출에서는 값이 없어야 함
    EXPECT_FALSE(gen.next().has_value());
}

// 큰 범위 테스트
TEST(GeneratorTest, LargeRange) {
    auto gen = generate_numbers(0, 1000);
    
    int count = 0;
    int expected = 0;
    while (auto value = gen.next()) {
        EXPECT_EQ(value.value(), expected);
        expected++;
        count++;
    }
    
    EXPECT_EQ(count, 1000);
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}