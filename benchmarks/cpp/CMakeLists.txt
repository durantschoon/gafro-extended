cmake_minimum_required(VERSION 3.16)
project(gafro_modern_benchmarks)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find required packages
find_package(Eigen3 REQUIRED)

# Try to find Google Benchmark
find_package(benchmark QUIET)

if(NOT benchmark_FOUND)
    message(STATUS "Google Benchmark not found, downloading...")

    include(FetchContent)
    FetchContent_Declare(
        benchmark
        GIT_REPOSITORY https://github.com/google/benchmark.git
        GIT_TAG v1.8.3
    )

    set(BENCHMARK_ENABLE_TESTING OFF CACHE BOOL "" FORCE)
    set(BENCHMARK_ENABLE_INSTALL OFF CACHE BOOL "" FORCE)

    FetchContent_MakeAvailable(benchmark)
endif()

# Include directories
include_directories(../../src)
include_directories(${EIGEN3_INCLUDE_DIR})

# Benchmark executable
add_executable(gafro_modern_benchmarks
    modern_types_benchmark.cpp
    si_units_benchmark.cpp
    pattern_matching_benchmark.cpp
    grade_checking_benchmark.cpp
)

target_link_libraries(gafro_modern_benchmarks
    benchmark::benchmark
    benchmark::benchmark_main
    Eigen3::Eigen
)

# Compiler-specific optimizations
target_compile_options(gafro_modern_benchmarks PRIVATE
    $<$<CXX_COMPILER_ID:GNU>:-O3 -march=native -DNDEBUG>
    $<$<CXX_COMPILER_ID:Clang>:-O3 -march=native -DNDEBUG>
    $<$<CXX_COMPILER_ID:MSVC>:/O2 /DNDEBUG>
)

# Enable LTO for release builds
set_property(TARGET gafro_modern_benchmarks PROPERTY INTERPROCEDURAL_OPTIMIZATION TRUE)