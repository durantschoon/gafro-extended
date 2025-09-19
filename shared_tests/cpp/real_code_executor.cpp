#include "real_code_executor.hpp"
#include <fstream>
#include <sstream>
#include <iostream>
#include <regex>
#include <cstdlib>
#include <filesystem>
#include <chrono>
#include <thread>

namespace gafro_test {

RealCodeExecutor::RealCodeExecutor() {
    // Set cross-platform temporary directory
#ifdef _WIN32
    working_dir_ = std::string(std::getenv("TEMP")) + "\\gafro_tests";
#else
    working_dir_ = "/tmp/gafro_tests";
#endif
    
    // Create working directory
    std::filesystem::create_directories(working_dir_);
}

RealCodeExecutor::~RealCodeExecutor() {
    cleanup();
}

void RealCodeExecutor::setWorkingDirectory(const std::string& dir) {
    working_dir_ = dir;
    std::filesystem::create_directories(working_dir_);
}


void RealCodeExecutor::setVerbose(bool verbose) {
    verbose_ = verbose;
    
    // Print configuration for debugging when verbose is enabled
    if (verbose_) {
        config_.printConfiguration();
    }
}

void RealCodeExecutor::cleanup() {
    for (const auto& file : temp_files_) {
        try {
            std::filesystem::remove(file);
        } catch (const std::exception& e) {
            // Ignore cleanup errors
        }
    }
    temp_files_.clear();
}

nlohmann::json RealCodeExecutor::executeTest(const std::string& test_code, 
                                            const nlohmann::json& inputs,
                                            const std::vector<std::string>& includes) {
    try {
        // Generate source file
        std::string source_file = generateSourceFile(test_code, inputs, includes);
        temp_files_.push_back(source_file);
        
        // Generate executable name
        std::string executable = source_file.substr(0, source_file.find_last_of('.')) + "_exec";
        temp_files_.push_back(executable);
        
        // Compile source file
        if (!compileSourceFile(source_file, executable)) {
            return nlohmann::json::object(); // Return empty on compilation failure
        }
        
        // Execute program and capture results
        return executeProgram(executable);
        
    } catch (const std::exception& e) {
        std::cerr << "Error in executeTest: " << e.what() << std::endl;
        return nlohmann::json::object();
    }
}

std::string RealCodeExecutor::generateSourceFile(const std::string& test_code, 
                                                const nlohmann::json& inputs,
                                                const std::vector<std::string>& includes) {
    // Generate unique filename
    auto now = std::chrono::high_resolution_clock::now();
    auto timestamp = std::chrono::duration_cast<std::chrono::microseconds>(now.time_since_epoch()).count();
    std::string filename = working_dir_ + "/test_" + std::to_string(timestamp) + ".cpp";
    
    std::ofstream file(filename);
    if (!file.is_open()) {
        throw std::runtime_error("Failed to create source file: " + filename);
    }
    
    // Parse test code to understand what we're working with
    TestCodeInfo info = parseTestCode(test_code);
    
    // Write includes
    file << "#include <iostream>\n";
    file << "#include <nlohmann/json.hpp>\n";
    file << "#include <gafro/gafro.hpp>\n";
    
    for (const auto& include : includes) {
        if (include != "<gafro/gafro.hpp>") { // Already included
            file << "#include " << include << "\n";
        }
    }
    
    file << "\nusing json = nlohmann::json;\n";
    file << "using namespace gafro;\n\n";
    
    // Write main function
    file << "int main() {\n";
    file << "    json json_result;\n";
    file << "    \n";
    
    // Write input variables with unique names to avoid conflicts
    for (auto& [key, value] : inputs.items()) {
        if (value.is_number()) {
            file << "    double input_" << key << " = " << value.get<double>() << ";\n";
        } else if (value.is_array()) {
            file << "    std::vector<double> input_" << key << " = {";
            bool first = true;
            for (const auto& item : value) {
                if (!first) file << ", ";
                file << item.get<double>();
                first = false;
            }
            file << "};\n";
        }
    }
    file << "\n";
    
    // Write the actual test code
    file << "    // Test code execution\n";
    file << "    " << test_code << "\n";
    file << "\n";
    
    // Generate result extraction code
    std::string result_extraction = generateResultExtraction(test_code);
    file << result_extraction;
    
    // Debug: print the generated code
    if (verbose_) {
        std::cout << "Generated source file: " << filename << std::endl;
        std::cout << "Test code: " << test_code << std::endl;
        std::cout << "Result extraction: " << result_extraction << std::endl;
    }
    
    file << "    \n";
    file << "    // Output results as JSON\n";
    file << "    std::cout << json_result.dump() << std::endl;\n";
    file << "    \n";
    file << "    return 0;\n";
    file << "}\n";
    
    file.close();
    return filename;
}

std::string RealCodeExecutor::generateResultExtraction(const std::string& test_code) {
    TestCodeInfo info = parseTestCode(test_code);
    std::ostringstream extraction;
    
    if (info.type == "Vector") {
        // Extract vector components
        extraction << "    // Extract vector components\n";
        extraction << "    json_result[\"e1\"] = " << info.variable_name << ".template get<blades::e1>();\n";
        extraction << "    json_result[\"e2\"] = " << info.variable_name << ".template get<blades::e2>();\n";
        extraction << "    json_result[\"e3\"] = " << info.variable_name << ".template get<blades::e3>();\n";
    }
    else if (info.type == "Multivector") {
        // Extract multivector components
        extraction << "    // Extract multivector components\n";
        extraction << "    json_result[\"e0\"] = " << info.variable_name << ".template get<blades::e0>();\n";
        extraction << "    json_result[\"e1\"] = " << info.variable_name << ".template get<blades::e1>();\n";
        extraction << "    json_result[\"e2\"] = " << info.variable_name << ".template get<blades::e2>();\n";
        extraction << "    json_result[\"e3\"] = " << info.variable_name << ".template get<blades::e3>();\n";
        extraction << "    json_result[\"ei\"] = " << info.variable_name << ".template get<blades::ei>();\n";
    }
    else if (info.type == "Scalar") {
        // Extract scalar value
        extraction << "    // Extract scalar value\n";
        extraction << "    json_result[\"value\"] = " << info.variable_name << ".template get<blades::scalar>();\n";
    }
    else if (info.type == "Point") {
        // Extract point components
        extraction << "    // Extract point components\n";
        extraction << "    json_result[\"e1\"] = " << info.variable_name << ".template get<blades::e1>();\n";
        extraction << "    json_result[\"e2\"] = " << info.variable_name << ".template get<blades::e2>();\n";
        extraction << "    json_result[\"e3\"] = " << info.variable_name << ".template get<blades::e3>();\n";
        extraction << "    json_result[\"e0\"] = " << info.variable_name << ".template get<blades::e0>();\n";
        extraction << "    json_result[\"ei\"] = " << info.variable_name << ".template get<blades::ei>();\n";
    }
    else {
        // Generic multivector extraction
        extraction << "    // Generic multivector extraction\n";
        extraction << "    json_result[\"e0\"] = " << info.variable_name << ".template get<blades::e0>();\n";
        extraction << "    json_result[\"e1\"] = " << info.variable_name << ".template get<blades::e1>();\n";
        extraction << "    json_result[\"e2\"] = " << info.variable_name << ".template get<blades::e2>();\n";
        extraction << "    json_result[\"e3\"] = " << info.variable_name << ".template get<blades::e3>();\n";
        extraction << "    json_result[\"ei\"] = " << info.variable_name << ".template get<blades::ei>();\n";
    }
    
    // Add special cases for operations
    if (info.has_arithmetic) {
        if (test_code.find("norm()") != std::string::npos) {
            extraction << "    json_result[\"norm\"] = " << info.variable_name << ".norm();\n";
        }
        if (test_code.find("size") != std::string::npos) {
            extraction << "    json_result[\"size\"] = " << info.variable_name << ".size();\n";
        }
        // For arithmetic operations, extract the result variable
        if (test_code.find("auto result =") != std::string::npos) {
            if (info.type == "Vector") {
                extraction << "    json_result[\"e1\"] = result.template get<blades::e1>();\n";
                extraction << "    json_result[\"e2\"] = result.template get<blades::e2>();\n";
                extraction << "    json_result[\"e3\"] = result.template get<blades::e3>();\n";
            } else if (info.type == "Scalar") {
                extraction << "    json_result[\"result\"] = result.template get<blades::scalar>();\n";
            } else {
                extraction << "    json_result[\"result\"] = result.template get<blades::scalar>();\n";
            }
        }
    }
    
    return extraction.str();
}

RealCodeExecutor::TestCodeInfo RealCodeExecutor::parseTestCode(const std::string& test_code) {
    TestCodeInfo info;
    
    // Determine type
    if (test_code.find("Vector<double>") != std::string::npos) {
        info.type = "Vector";
    } else if (test_code.find("Multivector<double") != std::string::npos) {
        info.type = "Multivector";
    } else if (test_code.find("Scalar<double>") != std::string::npos) {
        info.type = "Scalar";
    } else if (test_code.find("Point<double>") != std::string::npos) {
        info.type = "Point";
    } else {
        info.type = "Multivector"; // Default
    }
    
    // Extract variable name - handle both declarations and assignments
    std::regex var_regex(R"((\w+)\s*[=;(])");
    std::smatch match;
    if (std::regex_search(test_code, match, var_regex)) {
        info.variable_name = match[1].str();
    } else {
        info.variable_name = "result"; // Default
    }
    
    // Check for operations
    info.has_arithmetic = (test_code.find("+") != std::string::npos || 
                          test_code.find("*") != std::string::npos ||
                          test_code.find("-") != std::string::npos ||
                          test_code.find("/") != std::string::npos);
    
    return info;
}

bool RealCodeExecutor::compileSourceFile(const std::string& source_file, const std::string& executable) {
    if (!config_.isValid()) {
        std::cerr << "Configuration error: " << config_.getErrorMessage() << std::endl;
        return false;
    }
    
    std::ostringstream cmd;
    
    // Add compiler
    cmd << config_.getCompiler() << " ";
    
    // Add compilation flags
    for (const auto& flag : config_.getCompilationFlags()) {
        cmd << " " << flag;
    }
    
    // Add GAFRO include path
    cmd << " -I" << config_.getGafroIncludePath();
    
    // Add GAFRO build path for package config
    cmd << " -I" << config_.getGafroBuildPath() << "/src";
    
    // Add additional include paths
    for (const auto& path : config_.getAdditionalIncludePaths()) {
        cmd << " -I" << path;
    }
    
    // Add additional library paths
    for (const auto& path : config_.getAdditionalLibraryPaths()) {
        cmd << " -L" << path;
    }
    
    // Add source file and output
    cmd << " " << source_file;
    cmd << " -o " << executable;
    cmd << " 2>&1";
    
    if (verbose_) {
        std::cout << "Compilation command: " << cmd.str() << std::endl;
    }
    
    int result = std::system(cmd.str().c_str());
    return (result == 0);
}

nlohmann::json RealCodeExecutor::executeProgram(const std::string& executable) {
    std::ostringstream cmd;
    cmd << executable << " 2>&1";
    
    FILE* pipe = popen(cmd.str().c_str(), "r");
    if (!pipe) {
        return nlohmann::json::object();
    }
    
    std::string output;
    char buffer[128];
    while (fgets(buffer, sizeof(buffer), pipe) != nullptr) {
        output += buffer;
    }
    pclose(pipe);
    
    try {
        return nlohmann::json::parse(output);
    } catch (const std::exception& e) {
        std::cerr << "Failed to parse JSON output: " << output << std::endl;
        return nlohmann::json::object();
    }
}

} // namespace gafro_test
