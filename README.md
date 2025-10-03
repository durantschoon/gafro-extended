# Geometric Algebra For RObotics

![Github actions status](https://github.com/idiap/gafro/actions/workflows/build.yaml/badge.svg)
![Gitlab status](https://gitlab.com/gafro/gafro/badges/main/pipeline.svg?ignore_skipped=true&key_text=Gitlab%20build&key_width=80)

This library provides a geometric algebra tools targeted towards robotics applications. It includes various computations for the kinematics and dynamics of serial manipulators as well as optimal control.  

Please visit <https://gitlab.com/gafro> in order to find the entire *gafro* software stack.

## Installation

```sh
 mkdir build && cd build
 cmake ..
 make
 sudo make install
```

## Development Setup

This project supports both VS Code and Emacs (including Spacemacs) for optimal development experience across all platforms, with full support for both C++ and Rust development.

### VS Code Configuration

Cross-platform VS Code configuration with:

- **Cross-Platform Support**: Mac, Linux, and Windows configurations
- **C++23 Standard**: Full C++23 support with modern features
- **CMake Integration**: Automatic include path detection via CMake
- **IntelliSense**: Platform-specific IntelliSense modes for accurate code completion

#### VS Code Prerequisites

1. **Install VS Code Extensions**:
   - C/C++ Extension Pack
   - rust-analyzer (Rust language server)
   - CMake Tools
   - Just (optional, for command runner)

2. **Build the project** to generate `compile_commands.json`:

   ```bash
   mkdir build && cd build
   cmake ..
   make
   ```

3. **VS Code will automatically**:
   - Detect the correct platform configuration
   - Use CMake-generated include paths
   - Provide accurate IntelliSense for all headers

#### VS Code Platform-Specific Features

- **Mac**: Uses clang++ with macOS frameworks
- **Linux**: Uses g++ with Linux IntelliSense  
- **Windows**: Uses MSVC with Windows SDK

The `.vscode/c_cpp_properties.json` file contains platform-specific configurations that automatically adapt to your development environment.

### Emacs Configuration

**For Spacemacs users**: See [SPACEMACS_SETUP.md](SPACEMACS_SETUP.md) for detailed configuration instructions.

**For vanilla Emacs users**: The `.dir-locals.el` file provides project-specific settings that work with any Emacs configuration.

#### Emacs Features

- **LSP Integration**: Language Server Protocol with clangd and rust-analyzer
- **Auto-completion**: Intelligent C++ and Rust completion
- **Syntax Checking**: Real-time error detection for both languages
- **Project Management**: Project navigation and file management
- **Git Integration**: Version control integration
- **Code Formatting**: clang-format and rustfmt integration

#### Setup Steps

1. **Build the project** to generate `compile_commands.json`:

   ```bash
   mkdir build && cd build
   cmake ..
   make
   ```

2. **Follow the appropriate setup guide**:
   - **Spacemacs**: See [SPACEMACS_SETUP.md](SPACEMACS_SETUP.md)
   - **Vanilla Emacs**: The `.dir-locals.el` file provides automatic configuration

## Usage

gafro can be used either standalone or within a catkin workspace. In both cases it can be used in a CMakeLists.txt as follows:

 find_package(gafro REQUIRED)

 target_link_libraries(target gafro::gafro)

## Testing

This repository includes cross-language JSON test specifications for validating GAFRO implementations. Tests can be run using both C++ and Rust test runners.

### 🧮 **Mathematical Convention**

**This extended version uses τ (tau = 2π) instead of π (pi)** for all mathematical constants. This choice is more natural for geometric calculations and rotations:

- **τ = 2π ≈ 6.28318**
- Full rotation = τ, half rotation = τ/2
- Many geometric algebra formulas become simpler with tau
- All test values and examples use tau consistently

### 🚀 **Quick Start (Recommended)**

For the best developer experience, we recommend using modern command runners:

#### **Just (The Modern Make) - RECOMMENDED**

```bash
# Install just: cargo install just (or brew install just)
just help                    # Show all available commands
just build                   # Build both test runners
just test-all                # Run all tests (both languages)
just test-cpp-real           # Run C++ tests with real GAFRO execution
just test-rust               # Run Rust tests with pattern matching
just test-scalars            # Run scalar tests only
just validate                # Cross-language validation
just clean                   # Clean build artifacts
just dev-setup               # Setup development environment
```

### 📋 **Manual Commands**

If you prefer to run commands manually:

### Running C++ Tests

```bash
# Build the C++ test runner
cd shared_tests/cpp
mkdir build && cd build
cmake ..
make

# Run tests with pattern matching (Phase 1)
./test_runner ../json/algebra/scalar_tests.json
./test_runner ../json/algebra/vector_tests.json
./test_runner ../json/algebra/multivector_tests.json

# Run tests with real GAFRO code execution (Phase 2)
./test_runner -r ../json/algebra/scalar_tests.json
./test_runner -r ../json/algebra/vector_tests.json
./test_runner -r ../json/algebra/multivector_tests.json

# Run with verbose output and real code execution
./test_runner -r -v ../json/algebra/scalar_tests.json

# Run specific category
./test_runner -c scalar_creation ../json/algebra/scalar_tests.json
```

### Running Rust Tests

```bash
# Build and run Rust test runner
cd shared_tests/rust
cargo run -- ../json/algebra/scalar_tests.json
cargo run -- ../json/algebra/vector_tests.json
cargo run -- ../json/algebra/multivector_tests.json

# Run with verbose output
cargo run -- ../json/algebra/scalar_tests.json -- -v

# Run specific category
cargo run -- ../json/algebra/scalar_tests.json -- -c scalar_creation

# Run tests with specific tag
cargo run -- ../json/algebra/scalar_tests.json -- -t basic
```

### Test Results

Current test success rates:

#### Phase 1: Pattern Matching

- **Scalar tests**: 100% (4/4 tests passing) in both C++ and Rust
- **Vector tests**: 80% (4/5 tests passing) in C++, 60% (3/5 tests passing) in Rust
- **Multivector tests**: 43% (3/7 tests passing) in C++, pattern matching in progress for Rust

#### Phase 2: Real Code Execution (C++ only)

- **Scalar tests**: 100% (4/4 tests passing) with real GAFRO execution
- **Vector tests**: 100% (5/5 tests passing) with real GAFRO execution
- **Cross-platform support**: Windows/Linux/macOS configuration system

### Features

- **Cross-Language Testing**: JSON test specifications work with both C++ and Rust
- **Real Code Execution**: C++ tests compile and execute actual GAFRO library code
- **Cross-Platform**: Automatic path detection for Windows, Linux, and macOS
- **Dependency Detection**: Automatic detection of Eigen3, nlohmann/json, and GAFRO paths
- **Verbose Output**: Detailed configuration and execution information

For more detailed information about the test infrastructure, see [shared_tests/README.md](shared_tests/README.md).

## Background

You can find the accompanying [arxiv article](http://arxiv.org/abs/2212.07237) and more information on our [website](https://geometric-algebra.tobiloew.ch/).

## How to cite

If you use *gafro* in your research, please cite the

 @article{loewGeometricAlgebraOptimal2023,
   title = {Geometric {{Algebra}} for {{Optimal Control}} with {{Applications}} in {{Manipulation Tasks}}},
   author = {L\"ow, Tobias and Calinon, Sylvain},
   date = {2023},
   journal = {IEEE Transactions on Robotics},
   doi = {10.1109/TRO.2023.3277282}
 }
