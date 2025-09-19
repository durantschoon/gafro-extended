cmake_minimum_required(VERSION 3.16)

project(gafro_json_test_loader)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find required packages
find_package(nlohmann_json REQUIRED)
find_package(Eigen3 REQUIRED)

# Include directories
include_directories(${CMAKE_SOURCE_DIR}/../src)
include_directories(${CMAKE_CURRENT_SOURCE_DIR})

# Find GAFRO (it's an INTERFACE library)
find_package(gafro REQUIRED PATHS ${CMAKE_SOURCE_DIR}/../build)

# Create the JSON test loader library
add_library(gafro_json_test_loader
    json_loader.cpp
    real_code_executor.cpp
    gafro_test_config.cpp
)

target_link_libraries(gafro_json_test_loader
    PRIVATE
        nlohmann_json::nlohmann_json
        gafro::gafro
)

target_include_directories(gafro_json_test_loader
    PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}
)

# Create the test runner executable
add_executable(test_runner
    test_runner.cpp
)

# Create the example usage executable
add_executable(example_usage
    example_usage.cpp
)

target_link_libraries(test_runner
    PRIVATE
        gafro_json_test_loader
        nlohmann_json::nlohmann_json
        gafro::gafro
)

target_link_libraries(example_usage
    PRIVATE
        gafro_json_test_loader
        nlohmann_json::nlohmann_json
        gafro::gafro
)

# Compiler-specific options
if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
    target_compile_options(gafro_json_test_loader PRIVATE "-Wall" "-Wextra")
    target_compile_options(test_runner PRIVATE "-Wall" "-Wextra")
    target_compile_options(example_usage PRIVATE "-Wall" "-Wextra")
endif()

if(CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
    target_compile_options(gafro_json_test_loader PRIVATE "-Wall" "-Wextra")
    target_compile_options(test_runner PRIVATE "-Wall" "-Wextra")
    target_compile_options(example_usage PRIVATE "-Wall" "-Wextra")
endif()

# Add test targets
enable_testing()

# Test target for scalar tests
add_test(NAME scalar_tests
    COMMAND test_runner ${CMAKE_SOURCE_DIR}/shared_tests/json/algebra/scalar_tests.json
    WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
)

# Test target for vector tests
add_test(NAME vector_tests
    COMMAND test_runner ${CMAKE_SOURCE_DIR}/shared_tests/json/algebra/vector_tests.json
    WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
)

# Test target for multivector tests
add_test(NAME multivector_tests
    COMMAND test_runner ${CMAKE_SOURCE_DIR}/shared_tests/json/algebra/multivector_tests.json
    WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
)

# Test target for cross-language consistency
add_test(NAME cross_language_consistency
    COMMAND test_runner ${CMAKE_SOURCE_DIR}/shared_tests/json/algebra/cross_language_consistency.json
    WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
)

# Verbose test targets
add_test(NAME scalar_tests_verbose
    COMMAND test_runner -v ${CMAKE_SOURCE_DIR}/shared_tests/json/algebra/scalar_tests.json
    WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
)

add_test(NAME vector_tests_verbose
    COMMAND test_runner -v ${CMAKE_SOURCE_DIR}/shared_tests/json/algebra/vector_tests.json
    WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
)

# Tag-based test targets
add_test(NAME basic_tests
    COMMAND test_runner -t basic ${CMAKE_SOURCE_DIR}/shared_tests/json/algebra/scalar_tests.json
    WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
)

add_test(NAME creation_tests
    COMMAND test_runner -t creation ${CMAKE_SOURCE_DIR}/shared_tests/json/algebra/vector_tests.json
    WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
)

# Category-based test targets
add_test(NAME scalar_creation_tests
    COMMAND test_runner -c scalar_creation ${CMAKE_SOURCE_DIR}/shared_tests/json/algebra/scalar_tests.json
    WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
)

add_test(NAME vector_creation_tests
    COMMAND test_runner -c vector_creation ${CMAKE_SOURCE_DIR}/shared_tests/json/algebra/vector_tests.json
    WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
)

# Install targets
install(TARGETS gafro_json_test_loader test_runner
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
    RUNTIME DESTINATION bin
)

install(FILES json_loader.hpp real_code_executor.hpp gafro_test_config.hpp
    DESTINATION include/gafro_test
)
