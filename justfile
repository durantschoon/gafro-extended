# GAFRO Extended - Modern Command Runner
# Install with: cargo install just (or brew install just)

# Default recipe - show available commands
default:
    @just --list

# Build C++ test runner
build-cpp:
    @echo "🔨 Building C++ test runner..."
    cd shared_tests/cpp && mkdir -p build && cd build && cmake .. && make

# Build Rust test runner  
build-rust:
    @echo "🦀 Building Rust test runner..."
    cd shared_tests/rust && cargo build

# Build both test runners
build: build-cpp build-rust

# Run C++ tests with pattern matching (Phase 1)
test-cpp:
    @echo "🧪 Running C++ tests (Pattern Matching - Phase 1)..."
    cd shared_tests/cpp/build && ./test_runner ../../json/algebra/scalar_tests.json

# Run C++ tests with real GAFRO execution (Phase 2)
test-cpp-real:
    @echo "🚀 Running C++ tests (Real GAFRO Execution - Phase 2)..."
    cd shared_tests/cpp/build && ./test_runner -r ../../json/algebra/scalar_tests.json

# Run C++ tests with verbose output
test-cpp-verbose:
    @echo "🔍 Running C++ tests with verbose output..."
    cd shared_tests/cpp/build && ./test_runner -r -v ../../json/algebra/scalar_tests.json

# Run Rust tests
test-rust:
    @echo "🦀 Running Rust tests (Pattern Matching - Phase 1)..."
    cd shared_tests/rust && cargo run -- ../json/algebra/scalar_tests.json

# Run Rust tests with verbose output
test-rust-verbose:
    @echo "🔍 Running Rust tests with verbose output..."
    cd shared_tests/rust && cargo run -- --verbose ../json/algebra/scalar_tests.json

# Run all tests (both languages)
test-all: test-cpp-real test-rust

# Run specific test categories
test-scalars:
    @echo "📊 Running scalar tests..."
    cd shared_tests/cpp/build && ./test_runner -r -c scalar_creation ../../json/algebra/scalar_tests.json
    cd shared_tests/rust && cargo run -- --category scalar_creation ../json/algebra/scalar_tests.json

test-vectors:
    @echo "📐 Running vector tests..."
    cd shared_tests/cpp/build && ./test_runner -r -c vector_creation ../../json/algebra/vector_tests.json
    cd shared_tests/rust && cargo run -- --category vector_creation ../json/algebra/vector_tests.json

test-multivectors:
    @echo "🔢 Running multivector tests..."
    cd shared_tests/cpp/build && ./test_runner -r -c multivector_creation ../../json/algebra/multivector_tests.json
    cd shared_tests/rust && cargo run -- --category multivector_creation ../json/algebra/multivector_tests.json

# Cross-language validation
validate:
    @echo "🔄 Running cross-language validation..."
    @echo "C++ Results:"
    cd shared_tests/cpp/build && ./test_runner -r ../../json/algebra/scalar_tests.json | tail -5
    @echo "\nRust Results:"
    cd shared_tests/rust && cargo run -- ../json/algebra/scalar_tests.json | tail -5

# Compare C++ and Rust example outputs
compare-examples:
    @echo "🔄 Comparing C++ and Rust example outputs..."
    ./scripts/compare_examples.sh

# Run individual example comparisons
compare-autonomous:
    @echo "🤖 Comparing autonomous navigation demo..."
    ./scripts/compare_single_example.sh autonomous_navigation

compare-manipulator:
    @echo "🦾 Comparing robot manipulator demo..."
    ./scripts/compare_single_example.sh robot_manipulator

compare-sensor:
    @echo "📡 Comparing sensor calibration demo..."
    ./scripts/compare_single_example.sh sensor_calibration

# Development helpers
dev-setup:
    @echo "🛠️  Setting up development environment..."
    @echo "Installing dependencies..."
    @echo "  - C++: Ensure you have g++, cmake, and nlohmann/json"
    @echo "  - Rust: Ensure you have cargo and serde_json"
    @echo "  - Just: Install with 'cargo install just' or 'brew install just'"
    @echo "Building test runners..."
    just build

# Clean build artifacts
clean:
    @echo "🧹 Cleaning build artifacts..."
    rm -rf shared_tests/cpp/build
    cd shared_tests/rust && cargo clean
    rm -rf examples/cpp/build
    rm -rf examples/robotics_applications/build
    cd examples/rust && cargo clean
    cd rust_modern && cargo clean

# Clean all build artifacts including main build directory
clean-all:
    @echo "🧹 Cleaning ALL build artifacts..."
    rm -rf build
    rm -rf shared_tests/cpp/build
    cd shared_tests/rust && cargo clean
    rm -rf examples/cpp/build
    rm -rf examples/robotics_applications/build
    cd examples/rust && cargo clean
    cd rust_modern && cargo clean
    rm -rf .gafro-idris2/build
    find . -name "*.o" -delete
    find . -name "*.so" -delete
    find . -name "*.dylib" -delete
    find . -name "*.exe" -delete
    find . -name "*.dll" -delete

# Show help
help:
    @echo "🎯 GAFRO Extended - Available Commands:"
    @echo ""
    @echo "📦 Build Commands:"
    @echo "  just build-cpp     - Build C++ test runner"
    @echo "  just build-rust    - Build Rust test runner"
    @echo "  just build         - Build both test runners"
    @echo ""
    @echo "🧪 Test Commands:"
    @echo "  just test-cpp      - Run C++ tests (pattern matching)"
    @echo "  just test-cpp-real - Run C++ tests (real GAFRO execution)"
    @echo "  just test-rust     - Run Rust tests (pattern matching)"
    @echo "  just test-all      - Run all tests"
    @echo ""
    @echo "📊 Specific Tests:"
    @echo "  just test-scalars     - Run scalar tests"
    @echo "  just test-vectors     - Run vector tests"
    @echo "  just test-multivectors - Run multivector tests"
    @echo ""
    @echo "🔧 Development:"
    @echo "  just dev-setup     - Setup development environment"
    @echo "  just clean         - Clean build artifacts"
    @echo "  just clean-all     - Clean ALL build artifacts (including main build/)"
    @echo "  just validate      - Cross-language validation"
    @echo ""
    @echo "🔄 Example Comparison:"
    @echo "  just compare-examples    - Compare all C++ and Rust examples"
    @echo "  just compare-autonomous  - Compare autonomous navigation demo"
    @echo "  just compare-manipulator - Compare robot manipulator demo"
    @echo "  just compare-sensor      - Compare sensor calibration demo"
    @echo ""
    @echo "💡 Tip: Run 'just' to see all available commands"
