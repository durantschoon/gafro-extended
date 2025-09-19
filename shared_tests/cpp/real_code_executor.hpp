#pragma once

#include <string>
#include <vector>
#include <map>
#include <memory>
#include <filesystem>
#include <nlohmann/json.hpp>
#include "gafro_test_config.hpp"

namespace gafro_test {

/**
 * Real Code Executor for GAFRO C++ tests
 * 
 * This class generates actual C++ source files from JSON test specifications,
 * compiles them with the GAFRO library, executes them, and captures results.
 * 
 * Phase 2 Implementation: Real GAFRO code execution
 */
class RealCodeExecutor {
public:
    RealCodeExecutor();
    ~RealCodeExecutor();

    /**
     * Execute a test case with real GAFRO code
     * @param test_code The C++ test code from JSON
     * @param inputs Input parameters for the test
     * @param includes List of include files needed
     * @return JSON result with actual GAFRO execution results
     */
    nlohmann::json executeTest(const std::string& test_code, 
                              const nlohmann::json& inputs,
                              const std::vector<std::string>& includes);

    /**
     * Set the working directory for temporary files
     */
    void setWorkingDirectory(const std::string& dir);
    
    /**
     * Enable/disable verbose output
     */
    void setVerbose(bool verbose);

    /**
     * Clean up temporary files
     */
    void cleanup();

private:
    std::string working_dir_;
    std::vector<std::string> temp_files_;
    bool verbose_ = false;
    GafroTestConfig config_;

    /**
     * Generate C++ source file from test code
     */
    std::string generateSourceFile(const std::string& test_code, 
                                  const nlohmann::json& inputs,
                                  const std::vector<std::string>& includes);

    /**
     * Compile the generated source file
     */
    bool compileSourceFile(const std::string& source_file, const std::string& executable);

    /**
     * Execute the compiled program and capture output
     */
    nlohmann::json executeProgram(const std::string& executable);

    /**
     * Generate result extraction code for different GAFRO types
     */
    std::string generateResultExtraction(const std::string& test_code);

    /**
     * Parse test code to determine GAFRO type and operations
     */
    struct TestCodeInfo {
        std::string type;           // "Vector", "Multivector", "Scalar", etc.
        std::string variable_name;  // Main variable name
        std::vector<std::string> operations; // Operations performed
        bool has_arithmetic;        // Contains arithmetic operations
    };
    
    TestCodeInfo parseTestCode(const std::string& test_code);
};

} // namespace gafro_test
