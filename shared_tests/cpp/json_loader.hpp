#pragma once

#include <string>
#include <vector>
#include <map>
#include <memory>
#include <functional>
#include <nlohmann/json.hpp>

namespace gafro_test {

using json = nlohmann::json;

/**
 * @brief Represents a single test case from JSON specification
 */
struct TestCase {
    std::string test_name;
    std::string description;
    std::string category;
    json inputs;
    json expected_outputs;
    double tolerance = 1e-10;
    json language_specific;
    std::vector<std::string> dependencies;
    std::vector<std::string> tags;
    
    // C++ specific configuration
    std::string cpp_test_code;
    std::vector<std::string> cpp_includes;
    std::string cpp_setup_code;
    std::string cpp_cleanup_code;
    
    /**
     * @brief Parse language-specific configuration for C++
     */
    void parseCppConfig();
    
    /**
     * @brief Validate that the test case has required fields
     */
    bool isValid() const;
};

/**
 * @brief Represents a test category containing multiple test cases
 */
struct TestCategory {
    std::string name;
    std::vector<TestCase> test_cases;
    
    /**
     * @brief Add a test case to this category
     */
    void addTestCase(const TestCase& test_case);
    
    /**
     * @brief Get test cases by tag
     */
    std::vector<TestCase> getTestCasesByTag(const std::string& tag) const;
    
    /**
     * @brief Get test cases by name pattern
     */
    std::vector<TestCase> getTestCasesByName(const std::string& pattern) const;
};

/**
 * @brief Represents a complete test suite
 */
struct TestSuite {
    std::string test_suite_name;
    std::string version;
    std::string description;
    std::map<std::string, TestCategory> test_categories;
    
    /**
     * @brief Load test suite from JSON file
     */
    static std::unique_ptr<TestSuite> loadFromFile(const std::string& filepath);
    
    /**
     * @brief Load test suite from JSON string
     */
    static std::unique_ptr<TestSuite> loadFromString(const std::string& json_string);
    
    /**
     * @brief Get all test cases across all categories
     */
    std::vector<TestCase> getAllTestCases() const;
    
    /**
     * @brief Get test cases by category name
     */
    TestCategory* getCategory(const std::string& category_name);
    
    /**
     * @brief Get test cases by tag across all categories
     */
    std::vector<TestCase> getTestCasesByTag(const std::string& tag) const;
    
    /**
     * @brief Validate the test suite structure
     */
    bool isValid() const;
    
    /**
     * @brief Get statistics about the test suite
     */
    struct Statistics {
        size_t total_test_cases = 0;
        size_t total_categories = 0;
        std::map<std::string, size_t> tests_per_category;
        std::map<std::string, size_t> tests_per_tag;
    };
    Statistics getStatistics() const;
};

/**
 * @brief Test execution result
 */
struct TestResult {
    std::string test_name;
    bool passed = false;
    std::string error_message;
    double execution_time_ms = 0.0;
    json actual_outputs;
    json expected_outputs;
    double tolerance = 1e-10;
    
    /**
     * @brief Check if the test passed based on tolerance
     */
    bool checkPassed() const;
    
    /**
     * @brief Get detailed failure information
     */
    std::string getFailureDetails() const;
};

/**
 * @brief Test execution context
 */
class TestExecutionContext {
public:
    /**
     * @brief Execute a single test case
     */
    TestResult executeTestCase(const TestCase& test_case);
    
    /**
     * @brief Execute all test cases in a category
     */
    std::vector<TestResult> executeCategory(const TestCategory& category);
    
    /**
     * @brief Execute all test cases in a test suite
     */
    std::vector<TestResult> executeTestSuite(const TestSuite& test_suite);
    
    /**
     * @brief Set custom test execution function
     */
    void setTestExecutor(std::function<json(const TestCase&)> executor);
    
    /**
     * @brief Enable/disable verbose output
     */
    void setVerbose(bool verbose);
    
    /**
     * @brief Get execution statistics
     */
    struct ExecutionStats {
        size_t total_tests = 0;
        size_t passed_tests = 0;
        size_t failed_tests = 0;
        double total_execution_time_ms = 0.0;
        double average_execution_time_ms = 0.0;
    };
    ExecutionStats getExecutionStats() const;

private:
    std::function<json(const TestCase&)> test_executor_;
    bool verbose_ = false;
    ExecutionStats stats_;
    
    /**
     * @brief Default test executor that evaluates C++ code
     */
    json defaultTestExecutor(const TestCase& test_case);
    
    /**
     * @brief Execute C++ code string and return results
     */
    json executeCppCode(const std::string& code, const json& inputs);
    
    /**
     * @brief Execute scalar operations
     */
    json executeScalarOperations(const std::string& code, const json& inputs);
    
    /**
     * @brief Execute vector operations
     */
    json executeVectorOperations(const std::string& code, const json& inputs);
    
    /**
     * @brief Execute multivector operations
     */
    json executeMultivectorOperations(const std::string& code, const json& inputs);
    
    /**
     * @brief Execute point operations
     */
    json executePointOperations(const std::string& code, const json& inputs);
    
    /**
     * @brief Execute basic operations (fallback)
     */
    json executeBasicOperations(const std::string& code, const json& inputs);
    
    /**
     * @brief Compare actual and expected outputs with tolerance
     */
    bool compareOutputs(const json& actual, const json& expected, double tolerance) const;
};

/**
 * @brief JSON test loader utility functions
 */
namespace JsonLoader {
    /**
     * @brief Validate JSON against test schema
     */
    bool validateJson(const json& test_json);
    
    /**
     * @brief Load and parse test case from JSON
     */
    TestCase parseTestCase(const json& test_case_json);
    
    /**
     * @brief Load and parse test category from JSON
     */
    TestCategory parseTestCategory(const std::string& name, const json& category_json);
    
    /**
     * @brief Load and parse test suite from JSON
     */
    TestSuite parseTestSuite(const json& test_suite_json);
    
    /**
     * @brief Convert test result to JSON
     */
    json testResultToJson(const TestResult& result);
    
    /**
     * @brief Convert execution stats to JSON
     */
    json executionStatsToJson(const TestExecutionContext::ExecutionStats& stats);
}

} // namespace gafro_test
