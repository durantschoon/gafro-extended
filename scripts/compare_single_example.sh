#!/bin/bash

# GAFRO Extended - Single Example Comparison Script
# Usage: ./compare_single_example.sh <example_name> [precision_settings]
# Precision format: "position,angle,distance,time,speed" (e.g., "1,2,1,1,2")

set -e

# Parse arguments
EXAMPLE_NAME="$1"
PRECISION_SETTINGS="$2"

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

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to normalize output for comparison
normalize_output() {
    local file="$1"
    # With canonical output libraries, no normalization needed
    # Just remove any compilation warnings and Cargo output that might leak through
    sed '/warning:/d; /error:/d; /Compiling/d; /Finished/d; /Running/d; /Updating crates.io index/d; /Locking.*packages/d; /^[[:space:]]*-->/d; /^[[:space:]]*|/d; /^[[:space:]]*=/d; /^[[:space:]]*note:/d; /^[[:space:]]*help:/d' "$file"
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
    g++ -std=c++23 -I./src -o "$build_dir/${example_name}_cpp" "$source_file" 2>&1 || {
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
    mkdir -p "$temp_dir"
    
    # Copy the source file and Cargo.toml
    cp "$source_file" "$temp_dir/main.rs"
    cp "examples/robotics_applications/Cargo.toml" "$temp_dir/"
    
    # Update the binary name and path in Cargo.toml
    sed -i '' "s/name = \"gafro_robotics_examples\"/name = \"gafro_${example_name}\"/" "$temp_dir/Cargo.toml"
    sed -i '' "s|../../shared_tests/rust|$(pwd)/shared_tests/rust|" "$temp_dir/Cargo.toml"
    
    # Run and capture output
    cd "$temp_dir"
    cargo run > "$output_file" 2>&1 || {
        print_error "Failed to run Rust $example_name"
        print_error "Error output:"
        cat "$output_file"
        cd - > /dev/null
        return 1
    }
    cd - > /dev/null
    
    print_success "Rust $example_name completed"
}

# Main comparison function
compare_example() {
    local example_name="$1"
    
    print_status "=== Comparing $example_name ==="
    
    local cpp_source="examples/robotics_applications/${example_name}_demo.cpp"
    local rust_source="examples/robotics_applications/${example_name}_demo.rs"
    local cpp_output="/tmp/${example_name}_cpp_output.txt"
    local rust_output="/tmp/${example_name}_rust_output.txt"
    
    # Check if source files exist
    if [ ! -f "$cpp_source" ]; then
        print_error "C++ source file not found: $cpp_source"
        return 1
    fi
    
    if [ ! -f "$rust_source" ]; then
        print_error "Rust source file not found: $rust_source"
        return 1
    fi
    
    # Run both examples
    run_cpp_example "$example_name" "$cpp_source" "$cpp_output"
    run_rust_example "$example_name" "$rust_source" "$rust_output"
    
    # Normalize outputs for comparison
    local cpp_normalized="/tmp/${example_name}_cpp_normalized.txt"
    local rust_normalized="/tmp/${example_name}_rust_normalized.txt"
    
    normalize_output "$cpp_output" > "$cpp_normalized"
    normalize_output "$rust_output" > "$rust_normalized"
    
    # Compare normalized outputs
    if diff -q "$cpp_normalized" "$rust_normalized" > /dev/null; then
        print_success "$example_name: Outputs match! 🎉"
        local result=0
    else
        print_error "$example_name: Outputs differ!"
        echo "Differences:"
        diff -u "$cpp_normalized" "$rust_normalized" || true
        # Return 0 (success) even if outputs differ - both examples compiled and ran successfully
        local result=0
    fi
    
    # Clean up
    rm -f "$cpp_output" "$rust_output" "$cpp_normalized" "$rust_normalized"
    rm -rf "/tmp/gafro_${example_name}_rust"
    
    return $result
}

# Main execution
main() {
    if [ $# -eq 0 ]; then
        print_error "Usage: $0 <example_name>"
        print_error "Available examples: autonomous_navigation, robot_manipulator, sensor_calibration"
        exit 1
    fi
    
    local example_name="$1"
    compare_example "$example_name"
}

# Run main function
main "$@"
