# GAFRO Extended Examples

This directory contains comprehensive examples demonstrating GAFRO Extended's type-safe robotics programming capabilities. All examples are implemented in both C++ and Rust to showcase cross-language consistency and the canonical output system.

## 🎯 Overview

The examples demonstrate **Phase 2 Modern Types** for robotics applications, focusing on:

- **Type-safe coordinate frames** (world, robot, sensor)
- **SI unit system** with compile-time dimensional analysis
- **Tau convention** for intuitive angle calculations
- **Canonical output formatting** for consistent cross-language results

## 📁 Directory Structure

```
examples/
├── README.md                           # This file
├── robotics_applications/              # Main robotics examples
│   ├── autonomous_navigation_demo.cpp  # C++ autonomous navigation
│   ├── autonomous_navigation_demo.rs   # Rust autonomous navigation
│   ├── robot_manipulator_demo.cpp      # C++ robot manipulator
│   ├── robot_manipulator_demo.rs       # Rust robot manipulator
│   ├── sensor_calibration_demo.cpp     # C++ sensor calibration
│   ├── sensor_calibration_demo.rs      # Rust sensor calibration
│   └── Cargo.toml                      # Rust dependencies
├── cpp/                                # Additional C++ examples
└── rust/                               # Additional Rust examples
```

## 🚀 Quick Start

### Prerequisites

- **C++23** compatible compiler (g++, clang++, or MSVC)
- **Rust** 1.70+ with Cargo
- **CMake** 3.20+ (for C++ examples)
- **Just** command runner (optional, for convenience)

### Building and Running Examples

#### Individual Examples

**C++ Examples:**

```bash
# Navigate to examples directory
cd examples/robotics_applications

# Compile and run C++ example
g++ -std=c++23 -I../../src -o autonomous_navigation_cpp autonomous_navigation_demo.cpp
./autonomous_navigation_cpp
```

**Rust Examples:**

```bash
# Navigate to examples directory
cd examples/robotics_applications

# Run Rust example (uses Cargo.toml)
cargo run --bin main
```

#### Using Just Commands

From the project root:

```bash
# Compare all examples
just compare-examples

# Compare specific example
just compare-autonomous
just compare-manipulator
just compare-sensor

# Clean build artifacts
just clean
just clean-all
```

## 📚 Example Descriptions

### 1. Autonomous Navigation Demo

**Files:** `autonomous_navigation_demo.cpp`, `autonomous_navigation_demo.rs`

**Demonstrates:**

- Type-safe coordinate frame operations (world, robot, sensor)
- SI unit system with dimensional analysis
- Tau convention for angle calculations
- Type-safe path planning and obstacle avoidance
- Sensor fusion with compile-time verification

**Key Features:**

- Prevents mixing coordinate frames at compile time
- Ensures dimensional correctness in calculations
- Uses intuitive τ (tau) instead of π for angles
- Demonstrates real-world robotics programming patterns

### 2. Robot Manipulator Demo

**Files:** `robot_manipulator_demo.cpp`, `robot_manipulator_demo.rs`

**Demonstrates:**

- Forward and inverse kinematics
- Type-safe joint angle calculations
- Workspace analysis with dimensional verification
- End-effector pose calculations

**Key Features:**

- Compile-time verification of kinematic equations
- Type-safe joint space to Cartesian space transformations
- Dimensional analysis for position and orientation

### 3. Sensor Calibration Demo

**Files:** `sensor_calibration_demo.cpp`, `sensor_calibration_demo.rs`

**Demonstrates:**

- IMU accelerometer calibration
- LIDAR range calibration
- Camera intrinsic calibration
- Multi-sensor synchronization
- Temperature compensation

**Key Features:**

- Type-safe calibration matrix operations
- Frame-specific sensor readings
- Dimensional verification of calibration parameters
- Timestamp synchronization with type safety

## 🔧 Canonical Output System

All examples use the **Canonical Output Library** to ensure identical formatting between C++ and Rust implementations.

### Features

- **Configurable precision** for different data types
- **Scientific notation control** with automatic thresholds
- **Tau convention support** built-in
- **Cross-language consistency** guaranteed
- **No post-processing needed** - output is canonical from the start

### Usage

**C++:**

```cpp
#include <gafro/modern/utilities.hpp>

using namespace gafro::modern::utilities;

// Configure output precision
output::set_precision(1, 0, 1, 1, 2);  // position, angle, distance, time, speed
output::set_tau_convention(true);

// Use canonical formatting
output::print_position("Current position", x, y, z, "world");
output::print_angle("Heading", degrees);
output::print_distance("Distance", value, "m");
```

**Rust:**

```rust
use gafro_test_runner::canonical_output::CanonicalOutput;

// Create and configure output
let mut output = CanonicalOutput::new();
output.set_precision(1, 0, 1, 1, 2);
output.set_tau_convention(true);

// Use canonical formatting
output.print_position("Current position", x, y, z, Some("world"));
output.print_angle("Heading", degrees);
output.print_distance("Distance", value, "m");
```

## 🧪 Cross-Language Validation

The project includes automated comparison tools to ensure C++ and Rust examples produce identical output.

### Comparison Commands

```bash
# Compare all examples
just compare-examples

# Compare individual examples
just compare-autonomous    # Autonomous navigation
just compare-manipulator   # Robot manipulator
just compare-sensor        # Sensor calibration
```

### What Gets Compared

- **Output formatting** (positions, angles, distances, etc.)
- **Numerical precision** (configurable per data type)
- **Scientific notation** (automatic threshold-based)
- **Tau convention** (consistent angle representation)
- **Error messages** (compilation warnings filtered out)

### Expected Results

When examples are properly synchronized:

```
✅ autonomous_navigation: Outputs match! 🎉
✅ robot_manipulator: Outputs match! 🎉
✅ sensor_calibration: Outputs match! 🎉
```

## 🛠️ Development Workflow

### Adding New Examples

1. **Create C++ version** in appropriate directory
2. **Create Rust version** with identical functionality
3. **Use canonical output library** for all formatting
4. **Add to comparison scripts** in `scripts/compare_examples.sh`
5. **Test with comparison tools** to ensure identical output

### Modifying Existing Examples

1. **Update both C++ and Rust versions** simultaneously
2. **Maintain canonical output usage** for consistency
3. **Run comparison tests** to verify synchronization
4. **Update documentation** if functionality changes

### Debugging Output Differences

1. **Run individual comparison** to see specific differences
2. **Check canonical output configuration** (precision, tau convention)
3. **Verify method signatures** match between languages
4. **Filter compilation warnings** if they appear in output

## 📖 Learning Path

### For Beginners

1. **Start with autonomous navigation** - most comprehensive example
2. **Understand coordinate frames** - fundamental to robotics
3. **Learn SI unit system** - prevents dimensional errors
4. **Explore tau convention** - intuitive angle representation

### For Advanced Users

1. **Study canonical output system** - ensures consistency
2. **Examine cross-language validation** - automated testing
3. **Modify examples** - add new features while maintaining consistency
4. **Contribute new examples** - expand the demonstration suite

## 🔍 Troubleshooting

### Common Issues

**Compilation Errors:**

- Ensure C++23 standard is enabled
- Check include paths are correct
- Verify Rust dependencies are available

**Output Differences:**

- Check canonical output configuration
- Verify precision settings match
- Ensure tau convention is consistently enabled

**Comparison Failures:**

- Run individual examples to isolate issues
- Check for compilation warnings in output
- Verify both versions use canonical output library

### Getting Help

- Check the main project README for general setup
- Review the canonical output library documentation
- Examine working examples for patterns
- Run comparison tools to identify specific differences

## 🎯 Next Steps

After exploring these examples:

1. **Read the main project documentation** for deeper understanding
2. **Explore the utilities libraries** for reusable components
3. **Study the comparison system** for validation approaches
4. **Contribute improvements** or new examples
5. **Apply patterns** to your own robotics projects

The examples demonstrate that **type safety doesn't mean complexity** - it means **correctness, maintainability, and confidence** in your robotics code.
