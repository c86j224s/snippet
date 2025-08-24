#!/bin/bash

# C++ Coroutine Study Project Build Script
# CMake + Ninja 기반 빌드 스크립트

set -e  # 오류 발생 시 스크립트 중단

# 색상 정의
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 함수 정의
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 도움말 표시
show_help() {
    echo "C++ Coroutine Study Project Build Script"
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -h, --help     Show this help message"
    echo "  -c, --clean    Clean build directory before building"
    echo "  -r, --release  Build in Release mode (default: Debug)"
    echo "  -t, --test     Run tests after building"
    echo "  -i, --install  Install after building"
    echo "  --gtest        Force build with Google Test"
    echo ""
    echo "Examples:"
    echo "  $0                    # Debug build"
    echo "  $0 -r                 # Release build"
    echo "  $0 -c -r -t           # Clean, Release build with tests"
    echo "  $0 --gtest -t         # Build with Google Test and run tests"
}

# 기본값 설정
BUILD_TYPE="Debug"
CLEAN_BUILD=false
RUN_TESTS=false
INSTALL_AFTER_BUILD=false
FORCE_GTEST=false

# 명령줄 인수 파싱
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_help
            exit 0
            ;;
        -c|--clean)
            CLEAN_BUILD=true
            shift
            ;;
        -r|--release)
            BUILD_TYPE="Release"
            shift
            ;;
        -t|--test)
            RUN_TESTS=true
            shift
            ;;
        -i|--install)
            INSTALL_AFTER_BUILD=true
            shift
            ;;
        --gtest)
            FORCE_GTEST=true
            shift
            ;;
        *)
            print_error "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

print_info "C++ Coroutine Study Project - CMake + Ninja Build"
print_info "Build Type: $BUILD_TYPE"

# 필수 도구 확인
check_tool() {
    if ! command -v $1 &> /dev/null; then
        print_error "$1이 설치되어 있지 않습니다."
        print_info "다음 명령으로 설치하세요:"
        case $1 in
            cmake)
                print_info "  Ubuntu/Debian: sudo apt install cmake"
                print_info "  CentOS/RHEL: sudo yum install cmake"
                print_info "  Fedora: sudo dnf install cmake"
                print_info "  Arch Linux: sudo pacman -S cmake"
                ;;
            ninja)
                print_info "  Ubuntu/Debian: sudo apt install ninja-build"
                print_info "  CentOS/RHEL: sudo yum install ninja-build"
                print_info "  Fedora: sudo dnf install ninja-build"
                print_info "  Arch Linux: sudo pacman -S ninja"
                ;;
        esac
        exit 1
    fi
}

print_info "필수 도구 확인 중..."
check_tool cmake
check_tool ninja

# 컴파일러 확인
if command -v g++ &> /dev/null; then
    GCC_VERSION=$(g++ --version | head -n1 | grep -oP '\d+\.\d+' | head -1)
    print_info "GCC 버전: $GCC_VERSION"
    if (( $(echo "$GCC_VERSION >= 13.0" | bc -l) )); then
        print_success "GCC 13+ 감지됨, std::print 지원 가능"
    else
        print_warning "GCC 버전이 13 미만입니다. std::print 지원이 제한될 수 있습니다."
    fi
elif command -v clang++ &> /dev/null; then
    CLANG_VERSION=$(clang++ --version | head -n1 | grep -oP '\d+\.\d+' | head -1)
    print_info "Clang 버전: $CLANG_VERSION"
else
    print_error "C++ 컴파일러를 찾을 수 없습니다."
    print_info "다음 명령으로 설치하세요:"
    print_info "  Ubuntu/Debian: sudo apt install build-essential"
    print_info "  CentOS/RHEL: sudo yum groupinstall 'Development Tools'"
    print_info "  Fedora: sudo dnf groupinstall 'Development Tools'"
    print_info "  Arch Linux: sudo pacman -S base-devel"
    exit 1
fi

# Google Test 확인 (선택사항)
if $FORCE_GTEST || pkg-config --exists gtest; then
    print_info "Google Test를 사용합니다."
    GTEST_AVAILABLE=true
else
    print_warning "Google Test가 설치되어 있지 않습니다. 간단한 테스트를 사용합니다."
    print_info "Google Test 설치 방법:"
    print_info "  Ubuntu/Debian: sudo apt install libgtest-dev"
    print_info "  CentOS/RHEL: sudo yum install gtest-devel"
    print_info "  Fedora: sudo dnf install gtest-devel"
    print_info "  Arch Linux: sudo pacman -S gtest"
    GTEST_AVAILABLE=false
fi

# 빌드 디렉토리 설정
BUILD_DIR="build"

if $CLEAN_BUILD && [ -d "$BUILD_DIR" ]; then
    print_info "기존 빌드 디렉토리 정리 중..."
    rm -rf "$BUILD_DIR"
fi

# 빌드 디렉토리 생성
mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

# CMake 구성
print_info "CMake 구성 중..."
CMAKE_ARGS="-G Ninja -DCMAKE_BUILD_TYPE=$BUILD_TYPE"

if $FORCE_GTEST; then
    CMAKE_ARGS="$CMAKE_ARGS -DGTEST_ROOT=/usr"
fi

cmake .. $CMAKE_ARGS

if [ $? -ne 0 ]; then
    print_error "CMake 구성에 실패했습니다."
    exit 1
fi

# 빌드 실행
print_info "빌드 중... (Ninja 사용)"
ninja

if [ $? -ne 0 ]; then
    print_error "빌드에 실패했습니다."
    exit 1
fi

print_success "빌드 완료!"

# 빌드된 실행 파일 정보 표시
print_info "빌드된 실행 파일:"
if [ -f "coroutine-study" ]; then
    print_info "  메인 프로그램: $(pwd)/coroutine-study"
fi
if [ -f "coroutine-study-gtest" ]; then
    print_info "  Google Test: $(pwd)/coroutine-study-gtest"
fi
if [ -f "coroutine-study-test" ]; then
    print_info "  간단한 테스트: $(pwd)/coroutine-study-test"
fi

# 테스트 실행
if $RUN_TESTS; then
    print_info "테스트 실행 중..."
    if ninja test; then
        print_success "모든 테스트가 통과했습니다!"
    else
        print_error "일부 테스트가 실패했습니다."
        exit 1
    fi
fi

# 설치
if $INSTALL_AFTER_BUILD; then
    print_info "설치 중..."
    if ninja install; then
        print_success "설치 완료!"
    else
        print_error "설치에 실패했습니다."
        exit 1
    fi
fi

print_success "모든 작업이 완료되었습니다!"
print_info "메인 프로그램을 실행하려면: ./build/coroutine-study"