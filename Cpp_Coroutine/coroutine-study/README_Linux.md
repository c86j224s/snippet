# C++ Coroutine Study Project - Linux Build Guide

이 프로젝트는 C++23 코루틴 기능을 학습하기 위한 교육용 프로젝트입니다. 다양한 코루틴 패턴과 구현을 데모로 제공합니다.

## 🚀 빠른 시작

### 1. 필수 요구사항

#### 컴파일러
- **GCC 13+** (추천) 또는 **Clang 16+**
- C++23 표준 지원
- 코루틴 지원 (`-fcoroutines` 플래그)

#### 빌드 도구
- **CMake 3.25+**
- **Ninja** (빌드 백엔드)

#### 선택사항
- **Google Test** (테스트용, 없어도 간단한 테스트로 대체)

### 2. 의존성 설치

#### Ubuntu/Debian
```bash
# 필수 의존성
sudo apt update
sudo apt install build-essential cmake ninja-build

# GCC 13 (Ubuntu 22.04+)
sudo apt install gcc-13 g++-13
sudo update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-13 100
sudo update-alternatives --install /usr/bin/g++ g++ /usr/bin/g++-13 100

# Google Test (선택사항)
sudo apt install libgtest-dev
```

#### CentOS/RHEL/AlmaLinux
```bash
# EPEL 저장소 활성화 (CentOS/RHEL)
sudo dnf install epel-release

# 필수 의존성
sudo dnf groupinstall "Development Tools"
sudo dnf install cmake ninja-build

# GCC 13 (추가 저장소 필요할 수 있음)
# 또는 Clang 사용
sudo dnf install clang

# Google Test (선택사항)
sudo dnf install gtest-devel
```

#### Fedora
```bash
# 필수 의존성
sudo dnf groupinstall "Development Tools"
sudo dnf install cmake ninja-build

# 최신 GCC는 기본 제공
sudo dnf install gcc-c++

# Google Test (선택사항)
sudo dnf install gtest-devel
```

#### Arch Linux
```bash
# 필수 의존성
sudo pacman -S base-devel cmake ninja

# GCC는 base-devel에 포함됨

# Google Test (선택사항)
sudo pacman -S gtest
```

### 3. 빌드 및 실행

#### 자동 빌드 스크립트 사용 (추천)
```bash
# 스크립트에 실행 권한 부여
chmod +x build.sh

# Debug 빌드
./build.sh

# Release 빌드 + 테스트 실행
./build.sh --release --test

# 전체 옵션 확인
./build.sh --help
```

#### 수동 빌드
```bash
# 1. 빌드 디렉토리 생성
mkdir build && cd build

# 2. CMake 구성 (Debug)
cmake -G Ninja -DCMAKE_BUILD_TYPE=Debug ..

# 또는 Release
cmake -G Ninja -DCMAKE_BUILD_TYPE=Release ..

# 3. 빌드
ninja

# 4. 테스트 실행 (선택사항)
ninja test

# 또는 직접 실행
./coroutine-study-test  # 간단한 테스트
# 또는
./coroutine-study-gtest  # Google Test (설치된 경우)
```

### 4. 프로그램 실행

```bash
# 빌드 디렉토리에서
./coroutine-study

# 또는 프로젝트 루트에서
./build/coroutine-study
```

## 📁 프로젝트 구조

```
coroutine-study/
├── CMakeLists.txt              # CMake 빌드 설정
├── build.sh                    # 자동 빌드 스크립트
├── README_Linux.md            # Linux 빌드 가이드
├── CLAUDE.md                  # AI 개발 도구용 가이드
│
├── coroutine-study/           # 메인 소스 코드
│   ├── coroutine-study.cpp    # 메인 함수
│   ├── basic_coroutine.hpp    # 기본 코루틴 예제
│   ├── generator.hpp          # 제너레이터 패턴
│   ├── custom_awaiter.hpp     # 커스텀 awaiter 구현
│   ├── integrated_coroutine.hpp # 통합 코루틴 예제
│   ├── async_server_chain.hpp # 비동기 서버 체인
│   ├── thread1.hpp           # 스레딩 예제
│   └── frame_inspection.hpp   # 프레임 검사 도구
│
└── tests/                     # 테스트 파일
    ├── test_generator.cpp     # Google Test 기반 테스트
    └── simple_test.cpp        # 간단한 테스트
```

## 🧪 테스트

### Google Test 사용 (설치된 경우)
```bash
# 빌드와 함께 테스트 실행
./build.sh --test

# 또는 수동으로
cd build
./coroutine-study-gtest
```

### 간단한 테스트 (Google Test 없는 경우)
```bash
cd build
./coroutine-study-test
```

### CMake 테스트 러너 사용
```bash
cd build
ninja test
# 또는
ctest
```

## 🔧 문제 해결

### 1. 컴파일러 오류

#### `std::print` not found
- GCC 13+ 또는 최신 Clang 사용 필요
- 또는 `std::println`을 `std::cout`로 임시 대체

#### 코루틴 지원 오류
```bash
# GCC의 경우 명시적으로 플래그 추가
export CXXFLAGS="-fcoroutines"
cmake -G Ninja -DCMAKE_BUILD_TYPE=Debug ..
```

### 2. CMake 오류

#### CMake 버전이 너무 낮음
```bash
# Ubuntu/Debian에서 최신 CMake 설치
sudo apt remove cmake
sudo apt install python3-pip
pip3 install cmake

# 또는 Kitware 공식 저장소 사용
wget -O - https://apt.kitware.com/keys/kitware-archive-latest.asc | sudo apt-key add -
sudo apt-add-repository 'deb https://apt.kitware.com/ubuntu/ focal main'
sudo apt update
sudo apt install cmake
```

### 3. Ninja 오류

#### Ninja를 찾을 수 없음
```bash
# Make로 대체 빌드
cmake -DCMAKE_BUILD_TYPE=Debug ..
make -j$(nproc)
```

## 🎯 학습 내용

이 프로젝트는 다음 코루틴 개념들을 다룹니다:

1. **기본 코루틴** (`basic_coroutine.hpp`)
   - `promise_type` 구현
   - `co_await`, `co_return` 사용법
   - 코루틴 핸들 관리

2. **제너레이터** (`generator.hpp`)
   - `co_yield` 사용법
   - Iterator 패턴과 코루틴 결합
   - 값 생성 순서 제어

3. **커스텀 Awaiter** (`custom_awaiter.hpp`)
   - `await_ready`, `await_suspend`, `await_resume` 구현
   - 동기/비동기 awaiter 패턴
   - 타임아웃과 지연 처리

4. **통합 코루틴** (`integrated_coroutine.hpp`)
   - 복합적인 코루틴 패턴
   - 코루틴 체이닝
   - 비동기 데이터 처리

5. **비동기 서버 체인** (`async_server_chain.hpp`)
   - 실제 서버 시뮬레이션
   - 멀티스레드 환경에서 코루틴
   - 요청/응답 패턴

## 📚 참고 자료

- [C++20/23 Coroutines](https://en.cppreference.com/w/cpp/language/coroutines)
- [GCC Coroutine Support](https://gcc.gnu.org/wiki/cxx-coroutines)
- [CMake Documentation](https://cmake.org/documentation/)
- [Ninja Build System](https://ninja-build.org/)

## 🤝 기여하기

1. 이슈나 개선사항이 있으면 GitHub Issues에 등록
2. Pull Request 환영
3. 한국어 주석과 영어 주석 모두 환영

## 📄 라이센스

이 프로젝트는 교육 목적으로 만들어졌습니다. 자유롭게 학습과 참고용으로 사용하세요.