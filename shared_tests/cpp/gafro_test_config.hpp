#pragma once

#include <string>
#include <vector>
#include <filesystem>
#include <iostream>

namespace gafro_test {

/**
 * Cross-platform configuration for GAFRO test execution
 * 
 * This class automatically detects GAFRO library paths, dependency paths,
 * and compiler settings for Windows, Linux, and macOS.
 */
class GafroTestConfig {
public:
    GafroTestConfig();
    
    /**
     * Get the GAFRO source include path
     */
    const std::string& getGafroIncludePath() const { return gafro_include_path_; }
    
    /**
     * Get the GAFRO build path (for package config)
     */
    const std::string& getGafroBuildPath() const { return gafro_build_path_; }
    
    /**
     * Get additional include paths for dependencies
     */
    const std::vector<std::string>& getAdditionalIncludePaths() const { return additional_include_paths_; }
    
    /**
     * Get additional library paths
     */
    const std::vector<std::string>& getAdditionalLibraryPaths() const { return additional_library_paths_; }
    
    /**
     * Get the C++ standard flag
     */
    const std::string& getCppStandard() const { return cpp_standard_; }
    
    /**
     * Get the compiler executable
     */
    const std::string& getCompiler() const { return compiler_; }
    
    /**
     * Get platform-specific compilation flags
     */
    const std::vector<std::string>& getCompilationFlags() const { return compilation_flags_; }
    
    /**
     * Check if configuration is valid
     */
    bool isValid() const { return valid_; }
    
    /**
     * Get error message if configuration failed
     */
    const std::string& getErrorMessage() const { return error_message_; }
    
    /**
     * Print configuration for debugging
     */
    void printConfiguration() const;

private:
    std::string gafro_include_path_;
    std::string gafro_build_path_;
    std::vector<std::string> additional_include_paths_;
    std::vector<std::string> additional_library_paths_;
    std::string cpp_standard_;
    std::string compiler_;
    std::vector<std::string> compilation_flags_;
    bool valid_;
    std::string error_message_;
    
    /**
     * Detect the current platform
     */
    enum class Platform { Windows, Linux, macOS, Unknown };
    Platform detectPlatform();
    
    /**
     * Find GAFRO paths relative to the current executable
     */
    bool findGafroPaths();
    
    /**
     * Find dependency paths (Eigen3, nlohmann/json, etc.)
     */
    void findDependencyPaths();
    
    /**
     * Setup platform-specific compiler and flags
     */
    void setupPlatformSpecific();
    
    /**
     * Find a file or directory in common locations
     */
    std::string findInCommonLocations(const std::string& name, const std::vector<std::string>& extensions = {});
    
    /**
     * Get the directory containing the current executable
     */
    std::string getExecutableDirectory();
    
    /**
     * Check if a path exists
     */
    bool pathExists(const std::string& path);
};

} // namespace gafro_test
