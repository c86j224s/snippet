# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Development Commands

This project supports both Windows (Visual Studio) and Linux (CMake + Ninja) builds:

### Windows Build (Visual Studio)
- Build: Open `coroutine-study.sln` in Visual Studio and build the solution (Ctrl+Shift+B)
- Run main application: Build and run the `coroutine-study` project (F5 or Ctrl+F5)
- Run tests: Build and run the `coroutine-study-test` project using Visual Studio Test Explorer

Alternative command line builds using MSBuild:
```bash
# Build entire solution
msbuild coroutine-study.sln /p:Configuration=Debug /p:Platform=x64

# Build specific project
msbuild coroutine-study\coroutine-study.vcxproj /p:Configuration=Debug /p:Platform=x64
msbuild coroutine-study-test\coroutine-study-test.vcxproj /p:Configuration=Debug /p:Platform=x64
```

### Linux Build (CMake + Ninja)
**Prerequisites:** GCC 13+/Clang 16+, CMake 3.25+, Ninja, optional Google Test

```bash
# Quick build using automated script
chmod +x build.sh
./build.sh                    # Debug build
./build.sh -r -t             # Release build with tests
./build.sh --help            # Show all options

# Manual build
mkdir build && cd build
cmake -G Ninja -DCMAKE_BUILD_TYPE=Debug ..
ninja
ninja test                   # Run tests
```

**Requirements:**
- C++23 standard support
- Coroutines support (`-fcoroutines` flag for GCC)
- Threading library (pthread on Linux)
- Google Test (optional, will use simple tests if not available)

## Project Architecture

This is a **C++ coroutine study project** using C++23 standard features to demonstrate various coroutine patterns and implementations. The project consists of two main components:

### Main Application (`coroutine-study`)
- **Entry Point**: `coroutine-study.cpp` - Main function that can run different coroutine demonstrations
- **Modular Examples**: Each coroutine concept is implemented in separate header files as self-contained examples

### Test Project (`coroutine-study-test`)
- **Unit Tests**: Uses Microsoft Visual Studio CppUnitTest framework
- **Test Coverage**: Currently tests the generator functionality
- **Precompiled Headers**: Uses `pch.h` for faster compilation

### Core Coroutine Modules

1. **`basic_coroutine.hpp`** - Fundamental coroutine concepts
   - Basic promise_type implementation
   - SimpleCoroutine class showing suspend/resume mechanics
   - Demonstrates co_await and co_return usage

2. **`generator.hpp`** - Generator pattern implementation
   - Template-based Generator class
   - Uses co_yield for value generation
   - Iterator-like interface with next() method

3. **`custom_awaiter.hpp`** - Custom awaiter implementations
   - ValueAwaiter for immediate values
   - DelayedValueAwaiter for asynchronous operations with timeouts
   - AwaitableValue demonstrating operator co_await

4. **`integrated_coroutine.hpp`** - Advanced integrated examples
   - AsyncLoader for simulating asynchronous data loading
   - AsyncGenerator combining generators with async operations
   - Coroutine chaining examples

5. **`async_server_chain.hpp`** - Complex async server simulation
   - SampleServer class with request/response queues
   - RequestManager for handling async requests with coroutines
   - Task template for managing coroutine execution
   - Awaiter template for handling async responses
   - Chain of coroutine calls demonstrating real-world patterns

6. **`thread1.hpp`** - Threading-related coroutine examples (referenced but not analyzed)

7. **`frame_inspection.hpp`** - Coroutine frame debugging utilities (referenced but not analyzed)

## Key Design Patterns

- **Promise Type Pattern**: Each coroutine type implements a promise_type nested struct
- **RAII Handle Management**: Coroutine handles are properly destroyed in destructors
- **Template-Based Design**: Heavy use of C++ templates for generic coroutine types
- **Concept Requirements**: Modern C++ concepts used for type constraints (TReq, TRes)
- **Thread Safety**: Mutex-protected queues for multi-threaded async operations

## Development Notes

- Uses C++23 features extensively (`std::println`, coroutines, concepts)
- Korean comments throughout the codebase for educational purposes
- Each module includes a test function demonstrating usage
- Main function can selectively enable/disable different demonstrations
- Currently running `async_server_chain::test()` by default