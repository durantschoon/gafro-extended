#include "json_loader.hpp"
#include <iostream>
#include <memory>

using namespace gafro_test;

int main() {
    std::cout << "=== GAFRO JSON Test Loader Example ===\n\n";
    
    // Example 1: Load a test suite from file
    std::cout << "1. Loading test suite from file...\n";
    auto test_suite = TestSuite::loadFromFile("../../json/algebra/scalar_tests.json");
    
    if (!test_suite) {
        std::cerr << "Failed to load test suite\n";
        return 1;
    }
    
    std::cout << "Loaded test suite: " << test_suite->test_suite_name << "\n";
    std::cout << "Version: " << test_suite->version << "\n";
    std::cout << "Description: " << test_suite->description << "\n\n";
    
    // Example 2: Get test suite statistics
    std::cout << "2. Test suite statistics:\n";
    auto stats = test_suite->getStatistics();
    std::cout << "  Total categories: " << stats.total_categories << "\n";
    std::cout << "  Total test cases: " << stats.total_test_cases << "\n";
    
    std::cout << "  Tests per category:\n";
    for (const auto& [name, count] : stats.tests_per_category) {
        std::cout << "    " << name << ": " << count << " tests\n";
    }
    
    std::cout << "  Tests per tag:\n";
    for (const auto& [tag, count] : stats.tests_per_tag) {
        std::cout << "    " << tag << ": " << count << " tests\n";
    }
    std::cout << "\n";
    
    // Example 3: Get test cases by tag
    std::cout << "3. Getting test cases by tag 'basic':\n";
    auto basic_tests = test_suite->getTestCasesByTag("basic");
    std::cout << "  Found " << basic_tests.size() << " tests with tag 'basic'\n";
    
    for (const auto& test_case : basic_tests) {
        std::cout << "    - " << test_case.test_name << ": " << test_case.description << "\n";
    }
    std::cout << "\n";
    
    // Example 4: Get test cases by category
    std::cout << "4. Getting test cases from category 'scalar_creation':\n";
    auto category = test_suite->getCategory("scalar_creation");
    if (category) {
        std::cout << "  Category '" << category->name << "' has " << category->test_cases.size() << " tests\n";
        
        for (const auto& test_case : category->test_cases) {
            std::cout << "    - " << test_case.test_name << "\n";
            std::cout << "      Description: " << test_case.description << "\n";
            std::cout << "      C++ Code: " << test_case.cpp_test_code << "\n";
            std::cout << "      Expected Output: " << test_case.expected_outputs.dump(2) << "\n";
        }
    } else {
        std::cout << "  Category 'scalar_creation' not found\n";
    }
    std::cout << "\n";
    
    // Example 5: Execute a single test case
    std::cout << "5. Executing a single test case:\n";
    if (!basic_tests.empty()) {
        TestExecutionContext context;
        context.setVerbose(true);
        
        auto result = context.executeTestCase(basic_tests[0]);
        
        std::cout << "  Test: " << result.test_name << "\n";
        std::cout << "  Result: " << (result.passed ? "PASSED" : "FAILED") << "\n";
        std::cout << "  Execution time: " << result.execution_time_ms << "ms\n";
        
        if (!result.passed) {
            std::cout << "  Error: " << result.error_message << "\n";
        }
    }
    std::cout << "\n";
    
    // Example 6: Execute all tests in a category
    std::cout << "6. Executing all tests in 'scalar_creation' category:\n";
    if (category) {
        TestExecutionContext context;
        context.setVerbose(false);  // Less verbose for multiple tests
        
        auto results = context.executeCategory(*category);
        
        int passed = 0, failed = 0;
        for (const auto& result : results) {
            if (result.passed) {
                passed++;
            } else {
                failed++;
            }
        }
        
        std::cout << "  Results: " << passed << " passed, " << failed << " failed\n";
        
        auto exec_stats = context.getExecutionStats();
        std::cout << "  Total execution time: " << exec_stats.total_execution_time_ms << "ms\n";
        std::cout << "  Average execution time: " << exec_stats.average_execution_time_ms << "ms\n";
    }
    std::cout << "\n";
    
    // Example 7: Create a test case programmatically
    std::cout << "7. Creating a test case programmatically:\n";
    TestCase custom_test;
    custom_test.test_name = "custom_scalar_test";
    custom_test.description = "A custom test case created programmatically";
    custom_test.category = "custom";
    custom_test.inputs = json::object();
    custom_test.expected_outputs = {{"value", 42.0}};
    custom_test.tolerance = 1e-6;
    custom_test.cpp_test_code = "Scalar<double> scalar(42.0);";
    custom_test.tags = {"custom", "example"};
    
    std::cout << "  Created test: " << custom_test.test_name << "\n";
    std::cout << "  Description: " << custom_test.description << "\n";
    std::cout << "  C++ Code: " << custom_test.cpp_test_code << "\n";
    std::cout << "  Expected: " << custom_test.expected_outputs.dump() << "\n";
    
    // Execute the custom test
    TestExecutionContext context;
    auto result = context.executeTestCase(custom_test);
    std::cout << "  Result: " << (result.passed ? "PASSED" : "FAILED") << "\n";
    
    std::cout << "\n=== Example Complete ===\n";
    
    return 0;
}
