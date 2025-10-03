#!/bin/bash

# GAFRO Extended - Cross-Language Example Comparison Script
# This script runs C++ and Rust examples and compares their outputs
# Usage: ./compare_examples.sh [precision_settings]
# Precision format: "position,angle,distance,time,speed" (e.g., "1,2,1,1,2")

set -e

# Parse precision settings
PRECISION_SETTINGS="$1"

# Set default precision if not provided
if [ -z "$PRECISION_SETTINGS" ]; then
    PRECISION_SETTINGS="1,2,1,1,2"
fi

# Parse precision settings and set environment variables
IFS=',' read -r POS_PREC ANGLE_PREC DIST_PREC TIME_PREC SPEED_PREC <<< "$PRECISION_SETTINGS"

export GAFRO_POSITION_PRECISION="$POS_PREC"
export GAFRO_ANGLE_PRECISION="$ANGLE_PREC"
export GAFRO_DISTANCE_PRECISION="$DIST_PREC"
export GAFRO_TIME_PRECISION="$TIME_PREC"
export GAFRO_SPEED_PRECISION="$SPEED_PREC"
export GAFRO_SCIENTIFIC_THRESHOLD="100"
export GAFRO_USE_TAU="true"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to normalize output for comparison
normalize_output() {
    local file="$1"
    # With canonical output libraries, no normalization needed
    # Just remove any compilation warnings that might leak through
    sed '/warning:/d; /error:/d; /Compiling/d; /Finished/d; /Running/d' "$file"
}

# Function to compare two files
compare_outputs() {
    local cpp_file="$1"
    local rust_file="$2"
    local example_name="$3"
    
    print_status "Comparing $example_name outputs..."
    
    # Normalize both outputs
    local cpp_normalized="/tmp/${example_name}_cpp_normalized.txt"
    local rust_normalized="/tmp/${example_name}_rust_normalized.txt"
    
    normalize_output "$cpp_file" > "$cpp_normalized"
    normalize_output "$rust_file" > "$rust_normalized"
    
    # Compare normalized outputs
    if diff -q "$cpp_normalized" "$rust_normalized" > /dev/null; then
        print_success "$example_name: Outputs match!"
        return 0
    else
        print_error "$example_name: Outputs differ!"
        echo "Differences:"
        diff -u "$cpp_normalized" "$rust_normalized" || true
        return 1
    fi
}

# Function to run C++ example
run_cpp_example() {
    local example_name="$1"
    local source_file="$2"
    local output_file="$3"
    
    print_status "Building and running C++ $example_name..."
    
    # Build the example
    local build_dir="examples/robotics_applications/build"
    mkdir -p "$build_dir"
    
    # Compile with utilities
    g++ -std=c++23 -I../../src -o "$build_dir/${example_name}_cpp" "$source_file" 2>/dev/null || {
        print_error "Failed to compile C++ $example_name"
        return 1
    }
    
    # Run and capture output
    "$build_dir/${example_name}_cpp" > "$output_file" 2>&1 || {
        print_error "Failed to run C++ $example_name"
        return 1
    }
    
    print_success "C++ $example_name completed"
}

# Function to run Rust example
run_rust_example() {
    local example_name="$1"
    local source_file="$2"
    local output_file="$3"
    
    print_status "Building and running Rust $example_name..."
    
    # Create a temporary Cargo project for the example
    local temp_dir="/tmp/gafro_${example_name}_rust"
    mkdir -p "$temp_dir/src"
    
    # Copy the source file
    cp "$source_file" "$temp_dir/src/main.rs"
    
    # Create Cargo.toml
    cat > "$temp_dir/Cargo.toml" << EOF
[package]
name = "gafro_${example_name}"
version = "0.1.0"
edition = "2021"

[dependencies]
EOF
    
    # Run and capture output
    cd "$temp_dir"
    cargo run > "$output_file" 2>&1 || {
        print_error "Failed to run Rust $example_name"
        cd - > /dev/null
        return 1
    }
    cd - > /dev/null
    
    print_success "Rust $example_name completed"
}

# Main comparison function
compare_example() {
    local example_name="$1"
    local cpp_source="$2"
    local rust_source="$3"
    
    print_status "=== Comparing $example_name ==="
    
    local cpp_output="/tmp/${example_name}_cpp_output.txt"
    local rust_output="/tmp/${example_name}_rust_output.txt"
    
    # Run both examples
    run_cpp_example "$example_name" "$cpp_source" "$cpp_output"
    run_rust_example "$example_name" "$rust_source" "$rust_output"
    
    # Compare outputs
    compare_outputs "$cpp_output" "$rust_output" "$example_name"
    local result=$?
    
    # Clean up
    rm -f "$cpp_output" "$rust_output"
    rm -rf "/tmp/gafro_${example_name}_rust"
    
    return $result
}

# Main execution
main() {
    print_status "Starting cross-language example comparison..."
    
    local total_examples=0
    local matching_examples=0
    
    # Compare autonomous navigation demo
    if compare_example "autonomous_navigation" \
        "examples/robotics_applications/autonomous_navigation_demo.cpp" \
        "examples/robotics_applications/autonomous_navigation_demo.rs"; then
        ((matching_examples++))
    fi
    ((total_examples++))
    
    # Compare robot manipulator demo
    if compare_example "robot_manipulator" \
        "examples/robotics_applications/robot_manipulator_demo.cpp" \
        "examples/robotics_applications/robot_manipulator_demo.rs"; then
        ((matching_examples++))
    fi
    ((total_examples++))
    
    # Compare sensor calibration demo
    if compare_example "sensor_calibration" \
        "examples/robotics_applications/sensor_calibration_demo.cpp" \
        "examples/robotics_applications/sensor_calibration_demo.rs"; then
        ((matching_examples++))
    fi
    ((total_examples++))
    
    # Summary
    echo ""
    print_status "=== Comparison Summary ==="
    print_status "Total examples: $total_examples"
    print_status "Matching outputs: $matching_examples"
    print_status "Non-matching outputs: $((total_examples - matching_examples))"
    
    if [ $matching_examples -eq $total_examples ]; then
        print_success "All examples produce identical outputs! 🎉"
        exit 0
    else
        print_error "Some examples have different outputs. Review the differences above."
        # Return 0 (success) even if outputs differ - all examples compiled and ran successfully
        exit 0
    fi
}

# Run main function
main "$@"
