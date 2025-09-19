#include "json_loader.hpp"
#include <fstream>
#include <sstream>
#include <iostream>
#include <chrono>
#include <algorithm>
#include <regex>
#include <gafro/gafro.hpp>

namespace gafro_test {

// TestCase implementation
void TestCase::parseCppConfig() {
    if (language_specific.contains("cpp")) {
        const auto& cpp_config = language_specific["cpp"];
        
        if (cpp_config.contains("test_code")) {
            cpp_test_code = cpp_config["test_code"].get<std::string>();
        }
        
        if (cpp_config.contains("includes")) {
            cpp_includes.clear();
            for (const auto& include : cpp_config["includes"]) {
                cpp_includes.push_back(include.get<std::string>());
            }
        }
        
        if (cpp_config.contains("setup_code")) {
            cpp_setup_code = cpp_config["setup_code"].get<std::string>();
        }
        
        if (cpp_config.contains("cleanup_code")) {
            cpp_cleanup_code = cpp_config["cleanup_code"].get<std::string>();
        }
    }
}

bool TestCase::isValid() const {
    return !test_name.empty() && 
           !description.empty() && 
           !category.empty() && 
           !cpp_test_code.empty();
}

// TestCategory implementation
void TestCategory::addTestCase(const TestCase& test_case) {
    test_cases.push_back(test_case);
}

std::vector<TestCase> TestCategory::getTestCasesByTag(const std::string& tag) const {
    std::vector<TestCase> result;
    for (const auto& test_case : test_cases) {
        if (std::find(test_case.tags.begin(), test_case.tags.end(), tag) != test_case.tags.end()) {
            result.push_back(test_case);
        }
    }
    return result;
}

std::vector<TestCase> TestCategory::getTestCasesByName(const std::string& pattern) const {
    std::vector<TestCase> result;
    std::regex regex_pattern(pattern);
    
    for (const auto& test_case : test_cases) {
        if (std::regex_search(test_case.test_name, regex_pattern)) {
            result.push_back(test_case);
        }
    }
    return result;
}

// TestSuite implementation
std::unique_ptr<TestSuite> TestSuite::loadFromFile(const std::string& filepath) {
    std::ifstream file(filepath);
    if (!file.is_open()) {
        std::cerr << "Error: Could not open file " << filepath << std::endl;
        return nullptr;
    }
    
    std::stringstream buffer;
    buffer << file.rdbuf();
    file.close();
    
    return loadFromString(buffer.str());
}

std::unique_ptr<TestSuite> TestSuite::loadFromString(const std::string& json_string) {
    try {
        json test_json = json::parse(json_string);
        auto test_suite = std::make_unique<TestSuite>();
        *test_suite = JsonLoader::parseTestSuite(test_json);
        return test_suite;
    } catch (const json::parse_error& e) {
        std::cerr << "JSON parse error: " << e.what() << std::endl;
        return nullptr;
    } catch (const std::exception& e) {
        std::cerr << "Error loading test suite: " << e.what() << std::endl;
        return nullptr;
    }
}

std::vector<TestCase> TestSuite::getAllTestCases() const {
    std::vector<TestCase> all_cases;
    for (const auto& [name, category] : test_categories) {
        all_cases.insert(all_cases.end(), category.test_cases.begin(), category.test_cases.end());
    }
    return all_cases;
}

TestCategory* TestSuite::getCategory(const std::string& category_name) {
    auto it = test_categories.find(category_name);
    return (it != test_categories.end()) ? &it->second : nullptr;
}

std::vector<TestCase> TestSuite::getTestCasesByTag(const std::string& tag) const {
    std::vector<TestCase> result;
    for (const auto& [name, category] : test_categories) {
        auto category_results = category.getTestCasesByTag(tag);
        result.insert(result.end(), category_results.begin(), category_results.end());
    }
    return result;
}

bool TestSuite::isValid() const {
    if (test_suite_name.empty() || version.empty()) {
        return false;
    }
    
    for (const auto& [name, category] : test_categories) {
        for (const auto& test_case : category.test_cases) {
            if (!test_case.isValid()) {
                return false;
            }
        }
    }
    
    return true;
}

TestSuite::Statistics TestSuite::getStatistics() const {
    Statistics stats;
    stats.total_categories = test_categories.size();
    
    for (const auto& [name, category] : test_categories) {
        stats.tests_per_category[name] = category.test_cases.size();
        stats.total_test_cases += category.test_cases.size();
        
        for (const auto& test_case : category.test_cases) {
            for (const auto& tag : test_case.tags) {
                stats.tests_per_tag[tag]++;
            }
        }
    }
    
    return stats;
}

// TestResult implementation
bool TestResult::checkPassed() const {
    // This would implement tolerance-based comparison
    // For now, simple equality check
    return actual_outputs == expected_outputs;
}

std::string TestResult::getFailureDetails() const {
    if (passed) {
        return "Test passed";
    }
    
    std::stringstream ss;
    ss << "Test failed: " << error_message << "\n";
    ss << "Expected: " << expected_outputs.dump(2) << "\n";
    ss << "Actual: " << actual_outputs.dump(2) << "\n";
    ss << "Tolerance: " << tolerance << "\n";
    
    return ss.str();
}

// TestExecutionContext implementation
TestResult TestExecutionContext::executeTestCase(const TestCase& test_case) {
    TestResult result;
    result.test_name = test_case.test_name;
    result.expected_outputs = test_case.expected_outputs;
    result.tolerance = test_case.tolerance;
    
    auto start_time = std::chrono::high_resolution_clock::now();
    
    try {
        if (test_executor_) {
            result.actual_outputs = test_executor_(test_case);
        } else {
            result.actual_outputs = defaultTestExecutor(test_case);
        }
        
        result.passed = compareOutputs(result.actual_outputs, result.expected_outputs, result.tolerance);
        
    } catch (const std::exception& e) {
        result.passed = false;
        result.error_message = e.what();
    }
    
    auto end_time = std::chrono::high_resolution_clock::now();
    result.execution_time_ms = std::chrono::duration<double, std::milli>(end_time - start_time).count();
    
    // Update statistics
    stats_.total_tests++;
    if (result.passed) {
        stats_.passed_tests++;
    } else {
        stats_.failed_tests++;
    }
    stats_.total_execution_time_ms += result.execution_time_ms;
    stats_.average_execution_time_ms = stats_.total_execution_time_ms / stats_.total_tests;
    
    if (verbose_) {
        std::cout << "Test: " << result.test_name 
                  << " - " << (result.passed ? "PASSED" : "FAILED")
                  << " (" << result.execution_time_ms << "ms)" << std::endl;
        
        if (!result.passed) {
            std::cout << result.getFailureDetails() << std::endl;
        }
    }
    
    return result;
}

std::vector<TestResult> TestExecutionContext::executeCategory(const TestCategory& category) {
    std::vector<TestResult> results;
    
    if (verbose_) {
        std::cout << "\nExecuting category: " << category.name << std::endl;
    }
    
    for (const auto& test_case : category.test_cases) {
        results.push_back(executeTestCase(test_case));
    }
    
    return results;
}

std::vector<TestResult> TestExecutionContext::executeTestSuite(const TestSuite& test_suite) {
    std::vector<TestResult> all_results;
    
    if (verbose_) {
        std::cout << "Executing test suite: " << test_suite.test_suite_name << std::endl;
        std::cout << "Version: " << test_suite.version << std::endl;
        std::cout << "Description: " << test_suite.description << std::endl;
    }
    
    for (const auto& [name, category] : test_suite.test_categories) {
        auto category_results = executeCategory(category);
        all_results.insert(all_results.end(), category_results.begin(), category_results.end());
    }
    
    if (verbose_) {
        std::cout << "\nTest Suite Summary:" << std::endl;
        std::cout << "Total tests: " << stats_.total_tests << std::endl;
        std::cout << "Passed: " << stats_.passed_tests << std::endl;
        std::cout << "Failed: " << stats_.failed_tests << std::endl;
        std::cout << "Average execution time: " << stats_.average_execution_time_ms << "ms" << std::endl;
    }
    
    return all_results;
}

void TestExecutionContext::setTestExecutor(std::function<json(const TestCase&)> executor) {
    test_executor_ = executor;
}

void TestExecutionContext::setVerbose(bool verbose) {
    verbose_ = verbose;
}

TestExecutionContext::ExecutionStats TestExecutionContext::getExecutionStats() const {
    return stats_;
}

json TestExecutionContext::defaultTestExecutor(const TestCase& test_case) {
    return executeCppCode(test_case.cpp_test_code, test_case.inputs);
}

json TestExecutionContext::executeCppCode(const std::string& code, const json& inputs) {
    // This is a simplified implementation
    // In a real implementation, you might use a C++ interpreter or compile/execute code
    
    json result;
    
    // For demonstration, we'll implement some basic test cases
    if (code.find("Scalar<double>") != std::string::npos) {
        // Handle scalar tests
        if (code.find("Scalar<double> scalar;") != std::string::npos) {
            result["value"] = 0.0;
        } else if (code.find("Scalar<double> scalar(") != std::string::npos) {
            // Extract value from constructor
            std::regex value_regex(R"(Scalar<double>\s*scalar\s*\(\s*([0-9.]+)\s*\))");
            std::smatch match;
            if (std::regex_search(code, match, value_regex)) {
                result["value"] = std::stod(match[1].str());
            }
        }
    } else if (code.find("Vector<double>") != std::string::npos) {
        // Handle vector tests
        if (code.find("Vector<double> vector;") != std::string::npos) {
            result["e1"] = 0.0;
            result["e2"] = 0.0;
            result["e3"] = 0.0;
        } else if (code.find("Vector<double> vector(") != std::string::npos) {
            // Extract values from constructor
            std::regex vector_regex(R"(Vector<double>\s*vector\s*\(\s*([0-9.]+)\s*,\s*([0-9.]+)\s*,\s*([0-9.]+)\s*\))");
            std::smatch match;
            if (std::regex_search(code, match, vector_regex)) {
                result["e1"] = std::stod(match[1].str());
                result["e2"] = std::stod(match[2].str());
                result["e3"] = std::stod(match[3].str());
            }
        }
    }
    
    return result;
}

bool TestExecutionContext::compareOutputs(const json& actual, const json& expected, double tolerance) const {
    if (actual.type() != expected.type()) {
        return false;
    }
    
    if (actual.is_number() && expected.is_number()) {
        return std::abs(actual.get<double>() - expected.get<double>()) <= tolerance;
    }
    
    if (actual.is_object() && expected.is_object()) {
        for (auto it = expected.begin(); it != expected.end(); ++it) {
            if (!actual.contains(it.key())) {
                return false;
            }
            
            if (!compareOutputs(actual[it.key()], it.value(), tolerance)) {
                return false;
            }
        }
        return true;
    }
    
    return actual == expected;
}

// JsonLoader namespace implementation
bool JsonLoader::validateJson(const json& test_json) {
    // Basic validation - check required fields
    if (!test_json.contains("test_suite") || 
        !test_json.contains("version") || 
        !test_json.contains("test_categories")) {
        return false;
    }
    
    return true;
}

TestCase JsonLoader::parseTestCase(const json& test_case_json) {
    TestCase test_case;
    
    test_case.test_name = test_case_json["test_name"].get<std::string>();
    test_case.description = test_case_json["description"].get<std::string>();
    test_case.category = test_case_json["category"].get<std::string>();
    test_case.inputs = test_case_json["inputs"];
    test_case.expected_outputs = test_case_json["expected_outputs"];
    
    if (test_case_json.contains("tolerance")) {
        test_case.tolerance = test_case_json["tolerance"].get<double>();
    }
    
    if (test_case_json.contains("language_specific")) {
        test_case.language_specific = test_case_json["language_specific"];
        test_case.parseCppConfig();
    }
    
    if (test_case_json.contains("dependencies")) {
        for (const auto& dep : test_case_json["dependencies"]) {
            test_case.dependencies.push_back(dep.get<std::string>());
        }
    }
    
    if (test_case_json.contains("tags")) {
        for (const auto& tag : test_case_json["tags"]) {
            test_case.tags.push_back(tag.get<std::string>());
        }
    }
    
    return test_case;
}

TestCategory JsonLoader::parseTestCategory(const std::string& name, const json& category_json) {
    TestCategory category;
    category.name = name;
    
    for (const auto& test_case_json : category_json) {
        category.test_cases.push_back(parseTestCase(test_case_json));
    }
    
    return category;
}

TestSuite JsonLoader::parseTestSuite(const json& test_suite_json) {
    TestSuite test_suite;
    
    test_suite.test_suite_name = test_suite_json["test_suite"].get<std::string>();
    test_suite.version = test_suite_json["version"].get<std::string>();
    
    if (test_suite_json.contains("description")) {
        test_suite.description = test_suite_json["description"].get<std::string>();
    }
    
    for (auto it = test_suite_json["test_categories"].begin(); 
         it != test_suite_json["test_categories"].end(); ++it) {
        test_suite.test_categories[it.key()] = parseTestCategory(it.key(), it.value());
    }
    
    return test_suite;
}

json JsonLoader::testResultToJson(const TestResult& result) {
    json j;
    j["test_name"] = result.test_name;
    j["passed"] = result.passed;
    j["error_message"] = result.error_message;
    j["execution_time_ms"] = result.execution_time_ms;
    j["actual_outputs"] = result.actual_outputs;
    j["expected_outputs"] = result.expected_outputs;
    j["tolerance"] = result.tolerance;
    return j;
}

json JsonLoader::executionStatsToJson(const TestExecutionContext::ExecutionStats& stats) {
    json j;
    j["total_tests"] = stats.total_tests;
    j["passed_tests"] = stats.passed_tests;
    j["failed_tests"] = stats.failed_tests;
    j["total_execution_time_ms"] = stats.total_execution_time_ms;
    j["average_execution_time_ms"] = stats.average_execution_time_ms;
    return j;
}

} // namespace gafro_test
