# GAFRO JSON Test Loader - C++ Implementation

This directory contains the C++ implementation of the JSON test loader for GAFRO Extended, enabling execution of cross-language test specifications.

## ⚠️ Important: Phase 1 Implementation

**This is currently a Phase 1 proof of concept implementation that uses pattern matching rather than executing real GAFRO code.**

### What it does

- ✅ Loads JSON test specifications
- ✅ Parses test cases and expected results
- ✅ Uses pattern matching to simulate GAFRO operations
- ✅ Compares results with expected outputs
- ✅ Provides comprehensive test reporting

### What it does NOT do

- ❌ Execute actual GAFRO C++ code
- ❌ Compile or run generated code
- ❌ Validate real GAFRO behavior

### Phase 2 Goal

Future implementation will generate actual C++ code from JSON specifications, compile it with GAFRO, and execute it to validate real behavior.

## Files

- `json_loader.hpp` - Header file with class definitions and interfaces
- `json_loader.cpp` - Implementation of the JSON test loader functionality
- `test_runner.cpp` - Command-line test runner executable
- `example_usage.cpp` - Example program demonstrating usage
- `CMakeLists.txt` - CMake build configuration

## Dependencies

- **nlohmann/json** - JSON parsing and manipulation
- **GAFRO** - Geometric algebra library
- **Eigen3** - Linear algebra library (GAFRO dependency)
- **C++17** - Modern C++ standard

## Building

### Prerequisites

```bash
# Install dependencies (Ubuntu/Debian)
sudo apt-get install libeigen3-dev nlohmann-json3-dev

# Install dependencies (macOS with Homebrew)
brew install eigen nlohmann-json
```

### Build Instructions

```bash
# From the project root
mkdir build
cd build
cmake ..
make

# Or build just the test infrastructure
cd shared_tests/cpp
mkdir build
cd build
cmake ..
make
```

## Usage

### Command Line Test Runner

The `test_runner` executable provides a command-line interface for running JSON test specifications:

```bash
# Run all tests in a file
./test_runner scalar_tests.json

# Run with verbose output
./test_runner -v scalar_tests.json

# Run only tests with specific tag
./test_runner -t basic scalar_tests.json

# Run only tests in specific category
./test_runner -c scalar_creation scalar_tests.json

# Show detailed statistics
./test_runner -s scalar_tests.json

# Combine options
./test_runner -v -t basic -s vector_tests.json
```

### Programmatic Usage

```cpp
#include "json_loader.hpp"
using namespace gafro_test;

// Load test suite from file
auto test_suite = TestSuite::loadFromFile("scalar_tests.json");

// Get test cases by tag
auto basic_tests = test_suite->getTestCasesByTag("basic");

// Execute a single test
TestExecutionContext context;
context.setVerbose(true);
auto result = context.executeTestCase(basic_tests[0]);

// Execute all tests in a category
auto category = test_suite->getCategory("scalar_creation");
auto results = context.executeCategory(*category);

// Get execution statistics
auto stats = context.getExecutionStats();
```

## API Reference

### TestCase

Represents a single test case with inputs, expected outputs, and execution code.

```cpp
struct TestCase {
    std::string test_name;
    std::string description;
    std::string category;
    json inputs;
    json expected_outputs;
    double tolerance = 1e-10;
    std::string cpp_test_code;
    std::vector<std::string> cpp_includes;
    std::vector<std::string> tags;
};
```

### TestSuite

Represents a complete test suite with multiple categories of tests.

```cpp
struct TestSuite {
    std::string test_suite_name;
    std::string version;
    std::string description;
    std::map<std::string, TestCategory> test_categories;
    
    static std::unique_ptr<TestSuite> loadFromFile(const std::string& filepath);
    std::vector<TestCase> getAllTestCases() const;
    TestCategory* getCategory(const std::string& category_name);
    std::vector<TestCase> getTestCasesByTag(const std::string& tag) const;
    Statistics getStatistics() const;
};
```

### TestExecutionContext

Handles test execution and result collection.

```cpp
class TestExecutionContext {
public:
    TestResult executeTestCase(const TestCase& test_case);
    std::vector<TestResult> executeCategory(const TestCategory& category);
    std::vector<TestResult> executeTestSuite(const TestSuite& test_suite);
    void setVerbose(bool verbose);
    void setTestExecutor(std::function<json(const TestCase&)> executor);
    ExecutionStats getExecutionStats() const;
};
```

## Test Execution

### Default Test Executor

The default test executor parses C++ code strings and executes them. Currently supports:

- **Scalar operations**: `Scalar<double> scalar(value);`
- **Vector operations**: `Vector<double> vector(x, y, z);`
- **Basic arithmetic**: Addition, multiplication, etc.

### Custom Test Executor

You can provide a custom test executor function:

```cpp
TestExecutionContext context;
context.setTestExecutor([](const TestCase& test_case) -> json {
    // Custom execution logic
    json result;
    // ... execute test_case.cpp_test_code ...
    return result;
});
```

## Test Results

### TestResult Structure

```cpp
struct TestResult {
    std::string test_name;
    bool passed = false;
    std::string error_message;
    double execution_time_ms = 0.0;
    json actual_outputs;
    json expected_outputs;
    double tolerance = 1e-10;
};
```

### Output Comparison

Tests are considered passed if:

- Actual outputs match expected outputs within tolerance
- For numeric values: `|actual - expected| <= tolerance`
- For objects: All fields match within tolerance
- For arrays: All elements match within tolerance

## Examples

### Running Tests

```bash
# Run scalar tests
./test_runner ../json/algebra/scalar_tests.json

# Run vector tests with verbose output
./test_runner -v ../json/algebra/vector_tests.json

# Run only basic tests
./test_runner -t basic ../json/algebra/scalar_tests.json

# Run creation tests from vector category
./test_runner -c vector_creation ../json/algebra/vector_tests.json
```

### Programmatic Example

See `example_usage.cpp` for a complete example of:

- Loading test suites
- Filtering tests by tag/category
- Executing individual tests
- Getting statistics
- Creating custom test cases

## Integration with CMake

The CMakeLists.txt provides several test targets:

```bash
# Run all JSON tests
ctest

# Run specific test categories
ctest -R scalar_tests
ctest -R vector_tests
ctest -R multivector_tests

# Run with verbose output
ctest --verbose
```

## Extending the Test Executor

To add support for new GAFRO operations:

1. **Update `executeCppCode` method** in `json_loader.cpp`
2. **Add pattern matching** for new code patterns
3. **Implement result extraction** for new operation types
4. **Add corresponding JSON test cases**

Example for adding motor support:

```cpp
// In executeCppCode method
else if (code.find("Motor<double>") != std::string::npos) {
    // Handle motor tests
    std::regex motor_regex(R"(Motor<double>\s*motor\s*\(\s*([0-9.]+)\s*,\s*([0-9.]+)\s*,\s*([0-9.]+)\s*,\s*([0-9.]+)\s*\))");
    std::smatch match;
    if (std::regex_search(code, match, motor_regex)) {
        result["scalar"] = std::stod(match[1].str());
        result["e23"] = std::stod(match[2].str());
        result["e13"] = std::stod(match[3].str());
        result["e12"] = std::stod(match[4].str());
    }
}
```

## Troubleshooting

### Common Issues

1. **JSON parse errors**: Check JSON syntax in test files
2. **Missing dependencies**: Ensure nlohmann-json and Eigen3 are installed
3. **Test execution failures**: Verify C++ code syntax in test cases
4. **Tolerance issues**: Adjust tolerance values for floating-point comparisons

### Debug Mode

Enable verbose output to see detailed test execution:

```cpp
TestExecutionContext context;
context.setVerbose(true);
```

### Performance

For large test suites, consider:

- Running tests in parallel (future enhancement)
- Using custom test executors for better performance
- Filtering tests by tag/category to run subsets
