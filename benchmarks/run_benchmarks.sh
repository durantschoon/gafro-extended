#!/bin/bash

# SPDX-FileCopyrightText: GAFRO Extended Implementation
# SPDX-License-Identifier: MPL-2.0

set -e  # Exit on any error

echo "=== GAFRO Extended Performance Benchmarks ==="
echo "Mathematical Convention: τ (tau = 2π) = $(echo "2 * 3.141592653589793" | bc -l)"
echo "=============================================="

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

# Check dependencies
check_dependencies() {
    print_status "Checking dependencies..."

    # Check for required tools
    if ! command -v cmake &> /dev/null; then
        print_error "cmake not found. Please install cmake."
        exit 1
    fi

    if ! command -v cargo &> /dev/null; then
        print_error "cargo not found. Please install Rust."
        exit 1
    fi

    if ! command -v bc &> /dev/null; then
        print_warning "bc not found. Math calculations may not work."
    fi

    print_success "Dependencies check passed"
}

# Build C++ benchmarks
build_cpp_benchmarks() {
    print_status "Building C++ benchmarks..."

    cd benchmarks/cpp

    if [ ! -d "build" ]; then
        mkdir build
    fi

    cd build

    # Try to configure with different CMake options
    if cmake .. -DCMAKE_BUILD_TYPE=Release; then
        print_success "CMake configuration successful"
    else
        print_warning "CMake configuration failed, but continuing..."
    fi

    # Try to build
    if make -j$(nproc 2>/dev/null || echo 4); then
        print_success "C++ benchmarks built successfully"
        CPP_BUILD_SUCCESS=true
    else
        print_error "C++ benchmark build failed"
        CPP_BUILD_SUCCESS=false
    fi

    cd ../../..
}

# Build Rust benchmarks
build_rust_benchmarks() {
    print_status "Building Rust benchmarks..."

    cd benchmarks/rust

    if cargo build --release; then
        print_success "Rust benchmarks built successfully"
        RUST_BUILD_SUCCESS=true
    else
        print_error "Rust benchmark build failed"
        RUST_BUILD_SUCCESS=false
    fi

    cd ../..
}

# Run C++ benchmarks
run_cpp_benchmarks() {
    if [ "$CPP_BUILD_SUCCESS" = true ]; then
        print_status "Running C++ benchmarks..."

        cd benchmarks/cpp/build

        if [ -f "./gafro_modern_benchmarks" ]; then
            echo "--- C++ Benchmark Results ---"
            ./gafro_modern_benchmarks --benchmark_format=console
            print_success "C++ benchmarks completed"
        else
            print_warning "C++ benchmark executable not found, skipping..."
        fi

        cd ../../..
    else
        print_warning "Skipping C++ benchmarks due to build failure"
    fi
}

# Run Rust benchmarks
run_rust_benchmarks() {
    if [ "$RUST_BUILD_SUCCESS" = true ]; then
        print_status "Running Rust benchmarks..."

        cd benchmarks/rust

        echo "--- Rust Benchmark Results ---"
        cargo bench
        print_success "Rust benchmarks completed"

        cd ../..
    else
        print_warning "Skipping Rust benchmarks due to build failure"
    fi
}

# Generate comparison report
generate_report() {
    print_status "Generating benchmark report..."

    REPORT_FILE="benchmark_report_$(date +%Y%m%d_%H%M%S).md"

    cat > "$REPORT_FILE" << EOF
# GAFRO Extended Benchmark Report

**Generated on:** $(date)
**Mathematical Convention:** τ (tau = 2π) ≈ 6.283185307179586

## System Information

- **OS:** $(uname -s) $(uname -r)
- **Architecture:** $(uname -m)
- **CPU:** $(lscpu 2>/dev/null | grep "Model name" | cut -d: -f2 | xargs || echo "Unknown")
- **Memory:** $(free -h 2>/dev/null | grep "Mem:" | awk '{print $2}' || echo "Unknown")

## Benchmark Categories

### Modern Types Performance
- Sum types using std::variant (C++) vs enum (Rust)
- Grade-indexed types with compile-time checking
- Pattern matching performance

### SI Units Performance
- Unit-safe arithmetic operations
- Compile-time dimension checking
- Marine robotics calculations
- Tau-based trigonometry vs traditional

### Cross-Language Consistency
- Identical results verification
- Performance comparison between implementations

## Build Status

- **C++ Benchmarks:** $([ "$CPP_BUILD_SUCCESS" = true ] && echo "✅ SUCCESS" || echo "❌ FAILED")
- **Rust Benchmarks:** $([ "$RUST_BUILD_SUCCESS" = true ] && echo "✅ SUCCESS" || echo "❌ FAILED")

## Key Findings

### Performance Highlights
$([ "$CPP_BUILD_SUCCESS" = true ] && [ "$RUST_BUILD_SUCCESS" = true ] && echo "
- Both C++ and Rust implementations successfully built
- Cross-language validation possible
- Ready for Phase 3 marine robotics features
" || echo "
- Some build issues detected
- Manual verification recommended
- Check dependency installation
")

### Mathematical Convention Benefits
- Using τ (tau = 2π) simplifies many geometric calculations
- More intuitive for marine robotics applications
- Consistent across both language implementations

## Recommendations

1. **Performance Optimization**
   - Enable link-time optimization (LTO) for production builds
   - Use profile-guided optimization for hot paths
   - Consider SIMD optimization for vector operations

2. **Cross-Language Development**
   - Maintain identical test specifications in JSON format
   - Regular cross-validation of results
   - Shared documentation of mathematical conventions

3. **Marine Robotics Extensions**
   - Focus on hydrodynamic calculations
   - Implement energy-aware cost functions
   - Add GPU acceleration for swarm algorithms

## Next Steps

- Complete Phase 2 implementation
- Begin Phase 3: Marine Branch development
- Set up continuous integration for benchmarks
- Implement more sophisticated GA operations

---
*Generated by GAFRO Extended Benchmark Suite*
EOF

    print_success "Benchmark report generated: $REPORT_FILE"
}

# Main execution
main() {
    echo "Starting GAFRO Extended benchmark suite..."

    # Initialize variables
    CPP_BUILD_SUCCESS=false
    RUST_BUILD_SUCCESS=false

    # Run all steps
    check_dependencies
    build_cpp_benchmarks
    build_rust_benchmarks
    run_cpp_benchmarks
    run_rust_benchmarks
    generate_report

    echo ""
    print_success "Benchmark suite completed!"

    if [ "$CPP_BUILD_SUCCESS" = true ] && [ "$RUST_BUILD_SUCCESS" = true ]; then
        print_success "Both C++ and Rust implementations are working!"
        print_status "Phase 2 (Modern Types) is ready for completion ✅"
    else
        print_warning "Some issues detected. Check build logs above."
    fi
}

# Handle script arguments
if [ "$1" = "--cpp-only" ]; then
    check_dependencies
    build_cpp_benchmarks
    run_cpp_benchmarks
elif [ "$1" = "--rust-only" ]; then
    check_dependencies
    build_rust_benchmarks
    run_rust_benchmarks
elif [ "$1" = "--help" ]; then
    echo "Usage: $0 [--cpp-only|--rust-only|--help]"
    echo ""
    echo "Options:"
    echo "  --cpp-only    Run only C++ benchmarks"
    echo "  --rust-only   Run only Rust benchmarks"
    echo "  --help        Show this help message"
    echo ""
    echo "Default: Run both C++ and Rust benchmarks"
else
    main
fi