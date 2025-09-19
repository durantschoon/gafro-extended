# Shared Tests for GAFRO Extended

This directory contains cross-language test specifications for GAFRO Extended, enabling identical test execution in both C++ and Rust implementations.

## Directory Structure

```
shared_tests/
├── json/                          # JSON test specifications
│   ├── test_schema.json          # JSON schema for test specifications
│   ├── gafro_algebra_tests.json  # Main test suite configuration
│   └── algebra/                  # Algebra-specific tests
│       ├── scalar_tests.json
│       ├── vector_tests.json
│       ├── multivector_tests.json
│       └── cross_language_consistency.json
├── cpp/                          # C++ test infrastructure
│   ├── json_loader.hpp
│   ├── json_loader.cpp
│   └── test_runner.cpp
└── rust/                         # Rust test infrastructure
    ├── src/
    │   ├── json_loader.rs
    │   └── test_runner.rs
    └── Cargo.toml
```

## JSON Test Specification Format

Each test case follows this structure:

```json
{
  "test_name": "unique_test_name",
  "description": "Human-readable description",
  "category": "test_category",
  "inputs": {
    "param1": "value1",
    "param2": "value2"
  },
  "expected_outputs": {
    "result1": "expected_value1",
    "result2": "expected_value2"
  },
  "tolerance": 1e-10,
  "language_specific": {
    "cpp": {
      "test_code": "C++ code snippet",
      "includes": ["header1.hpp", "header2.hpp"]
    },
    "rust": {
      "test_code": "Rust code snippet",
      "includes": ["module1", "module2"]
    }
  },
  "tags": ["tag1", "tag2"]
}
```

## Test Categories

### Scalar Tests
- Basic scalar creation and initialization
- Scalar arithmetic operations (addition, multiplication, etc.)
- Scalar properties and comparisons

### Vector Tests
- Vector creation from parameters, other vectors, and multivectors
- Vector arithmetic operations
- Vector properties and transformations

### Multivector Tests
- Multivector creation and initialization
- Multivector operations (addition, multiplication, geometric product)
- Multivector properties (norm, dual, inverse)
- Advanced operations (reverse, normalization)

### Cross-Language Consistency Tests
- Identical result validation between C++ and Rust
- Performance benchmark consistency
- Numerical precision validation

## Usage

### Running Tests

#### C++
```bash
cd shared_tests/cpp
make
./test_runner ../json/algebra/scalar_tests.json
```

#### Rust
```bash
cd shared_tests/rust
cargo test
cargo run -- --test-file ../json/algebra/scalar_tests.json
```

### Adding New Tests

1. **Create JSON test specification** in appropriate category file
2. **Update test schema** if new fields are needed
3. **Implement test execution** in both C++ and Rust loaders
4. **Validate cross-language consistency**

## Validation

The test framework ensures:
- **Identical Results**: Same numerical outputs within tolerance
- **Performance Consistency**: Similar execution times
- **API Consistency**: Equivalent functionality across languages
- **Type Safety**: Compile-time grade checking in both languages

## Dependencies

### C++
- nlohmann/json for JSON parsing
- Catch2 for test framework
- GAFRO library

### Rust
- serde_json for JSON parsing
- built-in test framework
- GAFRO Rust crate

## Contributing

When adding new tests:
1. Follow the JSON schema specification
2. Include both C++ and Rust language-specific code
3. Add appropriate tags for categorization
4. Ensure cross-language consistency
5. Update documentation as needed
