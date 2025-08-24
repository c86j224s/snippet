cmake_minimum_required(VERSION 3.25)  # C++23 지원을 위한 최신 CMake 요구

project(coroutine-study 
    VERSION 1.0.0
    LANGUAGES CXX
    DESCRIPTION "C++ Coroutine Study Project"
)

# C++23 표준 설정
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# 컴파일러별 최적화 및 경고 설정
if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
    add_compile_options(-Wall -Wextra -fcoroutines)
    # GCC 13+ 에서 std::print 지원
    if(CMAKE_CXX_COMPILER_VERSION VERSION_GREATER_EQUAL 13.0)
        message(STATUS "GCC 13+ detected, std::print should be available")
    else()
        message(WARNING "GCC version ${CMAKE_CXX_COMPILER_VERSION} may not support std::print")
    endif()
elseif(CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
    add_compile_options(-Wall -Wextra -stdlib=libc++)
    add_link_options(-stdlib=libc++)
endif()

# 디버그 빌드에서 추가 정보
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    add_compile_options(-g -O0)
    add_compile_definitions(DEBUG)
endif()

# 릴리스 빌드 최적화
if(CMAKE_BUILD_TYPE STREQUAL "Release")
    add_compile_options(-O3 -DNDEBUG)
endif()

# 스레딩 라이브러리 찾기 (coroutine에서 std::thread 사용)
find_package(Threads REQUIRED)

# 메인 실행 파일
add_executable(coroutine-study
    coroutine-study/coroutine-study.cpp
)

# 헤더 파일들을 포함 디렉토리로 추가
target_include_directories(coroutine-study PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/coroutine-study
)

# 필요한 라이브러리 링크
target_link_libraries(coroutine-study PRIVATE
    Threads::Threads
)

# 컴파일러별 특수 설정
if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
    # GCC에서 coroutine 지원을 위한 추가 플래그
    target_compile_options(coroutine-study PRIVATE -fcoroutines)
endif()

# 설치 설정
install(TARGETS coroutine-study
    RUNTIME DESTINATION bin
)

# 헤더 파일들 설치 (라이브러리로 사용할 경우)
install(FILES
    coroutine-study/generator.hpp
    coroutine-study/basic_coroutine.hpp
    coroutine-study/custom_awaiter.hpp
    coroutine-study/integrated_coroutine.hpp
    coroutine-study/async_server_chain.hpp
    coroutine-study/thread1.hpp
    coroutine-study/frame_inspection.hpp
    DESTINATION include/coroutine-study
)

# 테스팅 활성화
enable_testing()

# Google Test 찾기 (선택사항)
find_package(GTest QUIET)

if(GTest_FOUND)
    message(STATUS "Google Test found, building with GTest")
    
    # Google Test 기반 테스트 실행 파일
    add_executable(coroutine-study-gtest
        tests/test_generator.cpp
    )
    
    target_include_directories(coroutine-study-gtest PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}/coroutine-study
    )
    
    target_link_libraries(coroutine-study-gtest PRIVATE
        GTest::GTest
        GTest::Main
        Threads::Threads
    )
    
    # GTest 테스트 등록
    include(GoogleTest)
    gtest_discover_tests(coroutine-study-gtest)
    
else()
    message(STATUS "Google Test not found, building simple test executable")
    
    # 간단한 테스트 실행 파일 (Microsoft Test Framework 없이)
    add_executable(coroutine-study-test
        tests/simple_test.cpp
    )
    
    target_include_directories(coroutine-study-test PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}/coroutine-study
    )
    
    target_link_libraries(coroutine-study-test PRIVATE
        Threads::Threads
    )
    
    # 간단한 테스트 등록
    add_test(NAME generator_test COMMAND coroutine-study-test)
endif()

# 빌드 정보 출력
message(STATUS "Build type: ${CMAKE_BUILD_TYPE}")
message(STATUS "C++ Compiler: ${CMAKE_CXX_COMPILER_ID} ${CMAKE_CXX_COMPILER_VERSION}")
message(STATUS "C++ Standard: ${CMAKE_CXX_STANDARD}")