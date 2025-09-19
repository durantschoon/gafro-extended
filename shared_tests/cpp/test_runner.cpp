#include "json_loader.hpp"
#include <iostream>
#include <string>
#include <vector>
#include <filesystem>

using namespace gafro_test;

void printUsage(const char* program_name) {
    std::cout << "Usage: " << program_name << " [options] <test_file.json>\n";
    std::cout << "Options:\n";
    std::cout << "  -v, --verbose     Enable verbose output\n";
    std::cout << "  -t, --tag <tag>   Run only tests with specified tag\n";
    std::cout << "  -c, --category <name>  Run only tests in specified category\n";
    std::cout << "  -s, --stats       Show detailed statistics\n";
    std::cout << "  -r, --real-code   Enable real GAFRO code execution (Phase 2)\n";
    std::cout << "  -h, --help        Show this help message\n";
    std::cout << "\nExamples:\n";
    std::cout << "  " << program_name << " scalar_tests.json\n";
    std::cout << "  " << program_name << " -v -t basic vector_tests.json\n";
    std::cout << "  " << program_name << " -c vector_creation vector_tests.json\n";
    std::cout << "  " << program_name << " -r scalar_tests.json  # Real GAFRO execution\n";
}

void printTestSuiteInfo(const TestSuite& test_suite) {
    std::cout << "\n=== Test Suite Information ===\n";
    std::cout << "Name: " << test_suite.test_suite_name << "\n";
    std::cout << "Version: " << test_suite.version << "\n";
    std::cout << "Description: " << test_suite.description << "\n";
    
    auto stats = test_suite.getStatistics();
    std::cout << "Total Categories: " << stats.total_categories << "\n";
    std::cout << "Total Test Cases: " << stats.total_test_cases << "\n";
    
    std::cout << "\nCategories:\n";
    for (const auto& [name, count] : stats.tests_per_category) {
        std::cout << "  " << name << ": " << count << " tests\n";
    }
    
    if (!stats.tests_per_tag.empty()) {
        std::cout << "\nTags:\n";
        for (const auto& [tag, count] : stats.tests_per_tag) {
            std::cout << "  " << tag << ": " << count << " tests\n";
        }
    }
    std::cout << "==============================\n\n";
}

void printTestResults(const std::vector<TestResult>& results, bool show_stats) {
    std::cout << "\n=== Test Results ===\n";
    
    int passed = 0, failed = 0;
    double total_time = 0.0;
    
    for (const auto& result : results) {
        std::cout << "[" << (result.passed ? "PASS" : "FAIL") << "] " 
                  << result.test_name;
        
        if (show_stats) {
            std::cout << " (" << result.execution_time_ms << "ms)";
        }
        std::cout << "\n";
        
        if (result.passed) {
            passed++;
        } else {
            failed++;
            std::cout << "  Error: " << result.error_message << "\n";
        }
        
        total_time += result.execution_time_ms;
    }
    
    std::cout << "\nSummary:\n";
    std::cout << "  Passed: " << passed << "\n";
    std::cout << "  Failed: " << failed << "\n";
    std::cout << "  Total: " << (passed + failed) << "\n";
    std::cout << "  Total Time: " << total_time << "ms\n";
    
    if (passed + failed > 0) {
        std::cout << "  Average Time: " << (total_time / (passed + failed)) << "ms\n";
    }
    
    std::cout << "===================\n";
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        printUsage(argv[0]);
        return 1;
    }
    
    // Parse command line arguments
    bool verbose = false;
    bool show_stats = false;
    bool real_code_execution = false;
    std::string filter_tag;
    std::string filter_category;
    std::string test_file;
    
    for (int i = 1; i < argc; i++) {
        std::string arg = argv[i];
        
        if (arg == "-h" || arg == "--help") {
            printUsage(argv[0]);
            return 0;
        } else if (arg == "-v" || arg == "--verbose") {
            verbose = true;
        } else if (arg == "-s" || arg == "--stats") {
            show_stats = true;
        } else if (arg == "-r" || arg == "--real-code") {
            real_code_execution = true;
        } else if (arg == "-t" || arg == "--tag") {
            if (i + 1 < argc) {
                filter_tag = argv[++i];
            } else {
                std::cerr << "Error: --tag requires a tag name\n";
                return 1;
            }
        } else if (arg == "-c" || arg == "--category") {
            if (i + 1 < argc) {
                filter_category = argv[++i];
            } else {
                std::cerr << "Error: --category requires a category name\n";
                return 1;
            }
        } else if (arg[0] != '-') {
            test_file = arg;
        } else {
            std::cerr << "Error: Unknown option " << arg << "\n";
            printUsage(argv[0]);
            return 1;
        }
    }
    
    if (test_file.empty()) {
        std::cerr << "Error: No test file specified\n";
        printUsage(argv[0]);
        return 1;
    }
    
    // Check if file exists
    if (!std::filesystem::exists(test_file)) {
        std::cerr << "Error: Test file " << test_file << " does not exist\n";
        return 1;
    }
    
    // Load test suite
    std::cout << "Loading test suite from: " << test_file << "\n";
    auto test_suite = TestSuite::loadFromFile(test_file);
    
    if (!test_suite) {
        std::cerr << "Error: Failed to load test suite\n";
        return 1;
    }
    
    if (!test_suite->isValid()) {
        std::cerr << "Error: Invalid test suite\n";
        return 1;
    }
    
    // Print test suite information
    printTestSuiteInfo(*test_suite);
    
    // Set up test execution context
    TestExecutionContext context;
    context.setVerbose(verbose);
    
    // Enable real code execution if requested
    if (real_code_execution) {
        context.enableRealCodeExecution(true);
        if (verbose) {
            std::cout << "🚀 Real GAFRO code execution enabled (Phase 2)\n";
        }
    }
    
    // Execute tests based on filters
    std::vector<TestResult> results;
    
    if (!filter_category.empty()) {
        // Run specific category
        auto category = test_suite->getCategory(filter_category);
        if (!category) {
            std::cerr << "Error: Category '" << filter_category << "' not found\n";
            return 1;
        }
        
        if (!filter_tag.empty()) {
            // Filter by tag within category
            auto test_cases = category->getTestCasesByTag(filter_tag);
            for (const auto& test_case : test_cases) {
                results.push_back(context.executeTestCase(test_case));
            }
        } else {
            // Run all tests in category
            results = context.executeCategory(*category);
        }
    } else if (!filter_tag.empty()) {
        // Run all tests with specific tag
        auto test_cases = test_suite->getTestCasesByTag(filter_tag);
        for (const auto& test_case : test_cases) {
            results.push_back(context.executeTestCase(test_case));
        }
    } else {
        // Run all tests
        results = context.executeTestSuite(*test_suite);
    }
    
    // Print results
    printTestResults(results, show_stats);
    
    // Return exit code based on results
    bool all_passed = std::all_of(results.begin(), results.end(), 
                                 [](const TestResult& r) { return r.passed; });
    
    return all_passed ? 0 : 1;
}
