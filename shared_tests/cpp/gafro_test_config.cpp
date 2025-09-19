#include "gafro_test_config.hpp"
#include <algorithm>
#include <cstdlib>
#include <sstream>

#ifdef _WIN32
    #include <windows.h>
    #include <shlobj.h>
#elif defined(__APPLE__)
    #include <mach-o/dyld.h>
#elif defined(__linux__)
    #include <unistd.h>
    #include <linux/limits.h>
#endif

namespace gafro_test {

GafroTestConfig::GafroTestConfig() 
    : cpp_standard_("c++20")
    , valid_(false)
    , error_message_("Configuration not initialized") {
    
    // Detect platform and setup accordingly
    Platform platform = detectPlatform();
    if (platform == Platform::Unknown) {
        error_message_ = "Unknown platform detected";
        return;
    }
    
    // Find GAFRO paths
    if (!findGafroPaths()) {
        return;
    }
    
    // Find dependency paths
    findDependencyPaths();
    
    // Setup platform-specific settings
    setupPlatformSpecific();
    
    // Validate configuration
    if (gafro_include_path_.empty() || gafro_build_path_.empty()) {
        error_message_ = "Could not find GAFRO paths";
        return;
    }
    
    valid_ = true;
    error_message_ = "Configuration successful";
}

GafroTestConfig::Platform GafroTestConfig::detectPlatform() {
#ifdef _WIN32
    return Platform::Windows;
#elif defined(__APPLE__)
    return Platform::macOS;
#elif defined(__linux__)
    return Platform::Linux;
#else
    return Platform::Unknown;
#endif
}

bool GafroTestConfig::findGafroPaths() {
    // Get the directory containing the current executable
    std::string exe_dir = getExecutableDirectory();
    if (exe_dir.empty()) {
        error_message_ = "Could not determine executable directory";
        return false;
    }
    
    // Try to find GAFRO relative to the executable
    // The executable is typically in: <project>/shared_tests/cpp/build/
    // GAFRO source is typically in: <project>/src/gafro/
    // GAFRO build is typically in: <project>/build/
    
    std::vector<std::string> possible_gafro_src_paths = {
        exe_dir + "/../../src",                    // shared_tests/cpp/build/../../src
        exe_dir + "/../../../src",                 // shared_tests/cpp/build/../../../src
        exe_dir + "/../../../../src",              // shared_tests/cpp/build/../../../../src
        exe_dir + "/../../../../../../src",        // shared_tests/cpp/build/../../../../../../src
        "../src",                                  // Relative to current working directory
        "../../src",                               // Relative to current working directory
        "../../../src",                            // Relative to current working directory
        "../../../../src",                         // Relative to current working directory
    };
    
    std::vector<std::string> possible_gafro_build_paths = {
        exe_dir + "/../../build",                  // shared_tests/cpp/build/../../build
        exe_dir + "/../../../build",               // shared_tests/cpp/build/../../../build
        exe_dir + "/../../../../build",            // shared_tests/cpp/build/../../../../build
        exe_dir + "/../../../../../../build",      // shared_tests/cpp/build/../../../../../../build
        "../build",                                // Relative to current working directory
        "../../build",                             // Relative to current working directory
        "../../../build",                          // Relative to current working directory
        "../../../../build",                       // Relative to current working directory
    };
    
    // Find GAFRO source path
    for (const auto& path : possible_gafro_src_paths) {
        std::string gafro_hpp = path + "/gafro/gafro.hpp";
        if (pathExists(gafro_hpp)) {
            gafro_include_path_ = path;
            break;
        }
    }
    
    // Find GAFRO build path
    for (const auto& path : possible_gafro_build_paths) {
        std::string package_config = path + "/src/gafro/gafro_package_config.hpp";
        if (pathExists(package_config)) {
            gafro_build_path_ = path;
            break;
        }
    }
    
    return !gafro_include_path_.empty() && !gafro_build_path_.empty();
}

void GafroTestConfig::findDependencyPaths() {
    Platform platform = detectPlatform();
    
    if (platform == Platform::macOS) {
        // macOS with Homebrew
        additional_include_paths_ = {
            "/opt/homebrew/include",
            "/usr/local/include",
            "/opt/homebrew/Cellar/eigen/3.4.0_1/include/eigen3",
            "/usr/local/Cellar/eigen/3.4.0_1/include/eigen3"
        };
        additional_library_paths_ = {
            "/opt/homebrew/lib",
            "/usr/local/lib"
        };
    } else if (platform == Platform::Linux) {
        // Linux with common package managers
        additional_include_paths_ = {
            "/usr/include",
            "/usr/local/include",
            "/usr/include/eigen3",
            "/usr/local/include/eigen3"
        };
        additional_library_paths_ = {
            "/usr/lib",
            "/usr/local/lib",
            "/usr/lib/x86_64-linux-gnu",
            "/usr/lib64"
        };
    } else if (platform == Platform::Windows) {
        // Windows with vcpkg or manual installation
        additional_include_paths_ = {
            "C:/vcpkg/installed/x64-windows/include",
            "C:/vcpkg/installed/x86-windows/include",
            "C:/Program Files/Eigen3/include",
            "C:/Program Files (x86)/Eigen3/include"
        };
        additional_library_paths_ = {
            "C:/vcpkg/installed/x64-windows/lib",
            "C:/vcpkg/installed/x86-windows/lib",
            "C:/Program Files/Eigen3/lib",
            "C:/Program Files (x86)/Eigen3/lib"
        };
    }
    
    // Add GAFRO build path for package config
    if (!gafro_build_path_.empty()) {
        additional_include_paths_.push_back(gafro_build_path_ + "/src");
    }
}

void GafroTestConfig::setupPlatformSpecific() {
    Platform platform = detectPlatform();
    
    if (platform == Platform::Windows) {
        compiler_ = "cl.exe";
        compilation_flags_ = {
            "/std:" + cpp_standard_,
            "/EHsc",
            "/W3"
        };
    } else {
        // Unix-like systems (Linux, macOS)
        compiler_ = "g++";
        compilation_flags_ = {
            "-std=" + cpp_standard_,
            "-Wall",
            "-Wextra"
        };
    }
}

std::string GafroTestConfig::findInCommonLocations(const std::string& name, const std::vector<std::string>& extensions) {
    std::vector<std::string> search_paths = additional_include_paths_;
    search_paths.insert(search_paths.end(), additional_library_paths_.begin(), additional_library_paths_.end());
    
    for (const auto& path : search_paths) {
        std::string full_path = path + "/" + name;
        if (pathExists(full_path)) {
            return full_path;
        }
        
        // Try with extensions
        for (const auto& ext : extensions) {
            std::string full_path_with_ext = full_path + ext;
            if (pathExists(full_path_with_ext)) {
                return full_path_with_ext;
            }
        }
    }
    
    return "";
}

std::string GafroTestConfig::getExecutableDirectory() {
#ifdef _WIN32
    char path[MAX_PATH];
    if (GetModuleFileNameA(NULL, path, MAX_PATH) != 0) {
        std::string full_path(path);
        size_t last_slash = full_path.find_last_of("\\/");
        if (last_slash != std::string::npos) {
            return full_path.substr(0, last_slash);
        }
    }
#elif defined(__APPLE__)
    char path[1024];
    uint32_t size = sizeof(path);
    if (_NSGetExecutablePath(path, &size) == 0) {
        std::string full_path(path);
        size_t last_slash = full_path.find_last_of("/");
        if (last_slash != std::string::npos) {
            return full_path.substr(0, last_slash);
        }
    }
#elif defined(__linux__)
    char path[PATH_MAX];
    ssize_t len = readlink("/proc/self/exe", path, sizeof(path) - 1);
    if (len != -1) {
        path[len] = '\0';
        std::string full_path(path);
        size_t last_slash = full_path.find_last_of("/");
        if (last_slash != std::string::npos) {
            return full_path.substr(0, last_slash);
        }
    }
#endif
    
    return "";
}

bool GafroTestConfig::pathExists(const std::string& path) {
    return std::filesystem::exists(path);
}

void GafroTestConfig::printConfiguration() const {
    std::cout << "=== GAFRO Test Configuration ===" << std::endl;
    std::cout << "Valid: " << (valid_ ? "Yes" : "No") << std::endl;
    std::cout << "Error: " << error_message_ << std::endl;
    std::cout << "GAFRO Include Path: " << gafro_include_path_ << std::endl;
    std::cout << "GAFRO Build Path: " << gafro_build_path_ << std::endl;
    std::cout << "Compiler: " << compiler_ << std::endl;
    std::cout << "C++ Standard: " << cpp_standard_ << std::endl;
    
    std::cout << "Additional Include Paths:" << std::endl;
    for (const auto& path : additional_include_paths_) {
        std::cout << "  - " << path << std::endl;
    }
    
    std::cout << "Additional Library Paths:" << std::endl;
    for (const auto& path : additional_library_paths_) {
        std::cout << "  - " << path << std::endl;
    }
    
    std::cout << "Compilation Flags:" << std::endl;
    for (const auto& flag : compilation_flags_) {
        std::cout << "  - " << flag << std::endl;
    }
    std::cout << "================================" << std::endl;
}

} // namespace gafro_test
