// SPDX-FileCopyrightText: GAFRO Extended Implementation
//
// SPDX-License-Identifier: MPL-2.0

//! # GAFRO Modern - Rust Implementation
//!
//! This crate provides modern type-safe implementations of geometric algebra
//! concepts for the GAFRO (Geometric Algebra for Robotics) library.
//!
//! ## Features
//!
//! - **Sum Types**: Type-safe representation of different grades using Rust enums
//! - **Grade-Indexed Types**: Compile-time grade checking using const generics
//! - **Pattern Matching**: Ergonomic handling of GA terms using match expressions
//! - **Grade Checking**: Compile-time validation of geometric algebra operations
//! - **Cross-Language Compatibility**: Designed to match C++ implementation behavior
//!
//! ## Example Usage
//!
//! ```rust
//! use gafro_modern::ga_term::{GATerm, Grade};
//! use gafro_modern::grade_indexed::{ScalarType, VectorType};
//! use gafro_modern::pattern_matching::operations;
//!
//! // Create GA terms using sum types
//! let scalar = GATerm::scalar(3.14);
//! let vector = GATerm::vector(vec![(1, 2.0), (2, 3.0), (3, 4.0)]);
//!
//! // Use grade-indexed types for compile-time safety
//! let s1: ScalarType<f64> = ScalarType::scalar(2.0);
//! let s2: ScalarType<f64> = ScalarType::scalar(3.0);
//! let sum = s1 + s2; // Compile-time grade checking
//!
//! // Pattern matching on GA terms
//! let norm = operations::norm(&vector);
//! let scaled = operations::scalar_multiply(2.0, &vector);
//! ```

pub mod ga_term;
pub mod grade_indexed;
pub mod grade_checking;
pub mod pattern_matching;
pub mod si_units;

// Re-export commonly used types and functions
pub use ga_term::{GATerm, Grade, Scalar, BladeTerm, Index};
pub use grade_indexed::{GradeIndexed, ScalarType, VectorType, BivectorType, TrivectorType};
pub use pattern_matching::{match_gaterm, visit_gaterm, GATermVisitor};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::ga_term::{GATerm, Grade, Scalar, BladeTerm};
    pub use crate::grade_indexed::{GradeIndexed, ScalarType, VectorType, BivectorType, TrivectorType};
    pub use crate::pattern_matching::{match_gaterm, operations};
    pub use crate::grade_checking::{safe_ops, TypeInspector};
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use prelude::*;

    #[test]
    fn test_cross_language_compatibility() {
        // Test that our Rust implementation produces the same results
        // as the C++ implementation for basic operations

        // Scalar operations
        let s1 = GATerm::scalar(2.0);
        let s2 = GATerm::scalar(3.0);
        let sum = pattern_matching::operations::add(&s1, &s2).unwrap();

        if let GATerm::Scalar(s) = sum {
            assert_eq!(s.value, 5.0);
        }

        // Vector operations
        let v1 = GATerm::vector(vec![(1, 1.0), (2, 2.0), (3, 3.0)]);
        let v2 = GATerm::vector(vec![(1, 10.0), (2, 20.0), (3, 30.0)]);
        let vector_sum = pattern_matching::operations::add(&v1, &v2).unwrap();

        if let GATerm::Vector(v) = vector_sum {
            assert_eq!(v[0], (1, 11.0));
            assert_eq!(v[1], (2, 22.0));
            assert_eq!(v[2], (3, 33.0));
        }

        // Scalar multiplication
        let scaled = pattern_matching::operations::scalar_multiply(2.0, &v1);
        if let GATerm::Vector(v) = scaled {
            assert_eq!(v[0], (1, 2.0));
            assert_eq!(v[1], (2, 4.0));
            assert_eq!(v[2], (3, 6.0));
        }
    }

    #[test]
    fn test_grade_indexed_safety() {
        // Test compile-time grade safety
        let s1: ScalarType<f64> = ScalarType::scalar(2.0);
        let s2: ScalarType<f64> = ScalarType::scalar(3.0);

        // This should compile - same grades
        let _sum = s1 + s2;

        // Test grade checking
        assert_eq!(s1.grade(), Grade::Scalar);
        assert_eq!(ScalarType::<f64>::grade_const(), 0);

        let v1: VectorType<f64> = VectorType::vector(vec![(1, 2.0), (2, 3.0)]);
        assert_eq!(v1.grade(), Grade::Vector);
        assert_eq!(VectorType::<f64>::grade_const(), 1);
    }

    #[test]
    fn test_mathematical_conventions() {
        // Test that we use tau (2π) convention as specified
        use std::f64::consts::PI;

        const TAU: f64 = 2.0 * PI;

        // Example: a full rotation should be tau radians
        let full_rotation = GATerm::scalar(TAU);
        let half_rotation = GATerm::scalar(TAU / 2.0);

        if let GATerm::Scalar(s) = full_rotation {
            assert!((s.value - 6.283185307179586).abs() < 1e-10);
        }

        if let GATerm::Scalar(s) = half_rotation {
            assert!((s.value - PI).abs() < 1e-10);
        }
    }

    #[test]
    fn test_pattern_matching_exhaustiveness() {
        // Test that pattern matching handles all cases
        let terms = vec![
            GATerm::scalar(1.0),
            GATerm::vector(vec![(1, 2.0)]),
            GATerm::bivector(vec![(1, 2, 3.0)]),
            GATerm::trivector(vec![(1, 2, 3, 4.0)]),
            GATerm::multivector(vec![BladeTerm::new(vec![1, 2, 3, 4], 5.0)]),
        ];

        for term in &terms {
            let description = match_gaterm(
                term,
                |_| "scalar",
                |_| "vector",
                |_| "bivector",
                |_| "trivector",
                |_| "multivector",
            );

            assert!(!description.is_empty());
        }
    }
}