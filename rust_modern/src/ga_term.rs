// SPDX-FileCopyrightText: GAFRO Extended Implementation
//
// SPDX-License-Identifier: MPL-2.0

use std::marker::PhantomData;
use serde::{Deserialize, Serialize};

/// Type alias for blade indices
pub type Index = i32;

/// Grade enumeration for compile-time grade tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Grade {
    Scalar = 0,
    Vector = 1,
    Bivector = 2,
    Trivector = 3,
    Multivector = -1, // General case
}

/// Scalar wrapper for type safety
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scalar<T> {
    pub value: T,
}

impl<T> Scalar<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn grade() -> Grade {
        Grade::Scalar
    }
}

impl<T> From<T> for Scalar<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add for Scalar<T> {
    type Output = Scalar<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Scalar::new(self.value + rhs.value)
    }
}

impl<T: std::ops::Mul<Output = T>> std::ops::Mul for Scalar<T> {
    type Output = Scalar<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Scalar::new(self.value * rhs.value)
    }
}

/// Blade term representation for general multivectors
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BladeTerm<T> {
    pub indices: Vec<Index>,
    pub coefficient: T,
}

impl<T> BladeTerm<T> {
    pub fn new(indices: Vec<Index>, coefficient: T) -> Self {
        Self { indices, coefficient }
    }

    pub fn grade(&self) -> Grade {
        match self.indices.len() {
            0 => Grade::Scalar,
            1 => Grade::Vector,
            2 => Grade::Bivector,
            3 => Grade::Trivector,
            _ => Grade::Multivector,
        }
    }
}

/// Sum type representing different grades of geometric algebra terms
///
/// This uses Rust enums to provide type-safe sum types for geometric algebra
/// elements with different grades.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GATerm<T> {
    Scalar(Scalar<T>),                                        // 0-vector (scalar)
    Vector(Vec<(Index, T)>),                                  // 1-vector
    Bivector(Vec<(Index, Index, T)>),                         // 2-vector (bivector)
    Trivector(Vec<(Index, Index, Index, T)>),                 // 3-vector (trivector)
    Multivector(Vec<BladeTerm<T>>),                          // General multivector
}

impl<T> GATerm<T> {
    /// Get the grade of this GA term
    pub fn grade(&self) -> Grade {
        match self {
            GATerm::Scalar(_) => Grade::Scalar,
            GATerm::Vector(_) => Grade::Vector,
            GATerm::Bivector(_) => Grade::Bivector,
            GATerm::Trivector(_) => Grade::Trivector,
            GATerm::Multivector(_) => Grade::Multivector,
        }
    }

    /// Check if this term has a specific grade
    pub fn has_grade(&self, grade: Grade) -> bool {
        self.grade() == grade
    }
}

/// Factory functions for creating GA terms
impl<T> GATerm<T> {
    pub fn scalar(value: T) -> Self {
        GATerm::Scalar(Scalar::new(value))
    }

    pub fn vector(components: Vec<(Index, T)>) -> Self {
        GATerm::Vector(components)
    }

    pub fn bivector(components: Vec<(Index, Index, T)>) -> Self {
        GATerm::Bivector(components)
    }

    pub fn trivector(components: Vec<(Index, Index, Index, T)>) -> Self {
        GATerm::Trivector(components)
    }

    pub fn multivector(terms: Vec<BladeTerm<T>>) -> Self {
        GATerm::Multivector(terms)
    }
}

/// Trait for types that have a definite grade
pub trait HasGrade {
    fn grade() -> Grade;
}

/// Implementation for scalar types
impl<T> HasGrade for Scalar<T> {
    fn grade() -> Grade {
        Grade::Scalar
    }
}

/// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_creation() {
        let scalar = Scalar::new(3.14);
        assert_eq!(scalar.value, 3.14);
        assert_eq!(Scalar::<f64>::grade(), Grade::Scalar);
    }

    #[test]
    fn test_scalar_arithmetic() {
        let s1 = Scalar::new(2.0);
        let s2 = Scalar::new(3.0);

        let sum = s1 + s2;
        assert_eq!(sum.value, 5.0);

        let s3 = Scalar::new(2.0);
        let s4 = Scalar::new(3.0);
        let product = s3 * s4;
        assert_eq!(product.value, 6.0);
    }

    #[test]
    fn test_gaterm_grades() {
        let scalar = GATerm::scalar(1.0);
        assert_eq!(scalar.grade(), Grade::Scalar);

        let vector = GATerm::vector(vec![(1, 2.0), (2, 3.0)]);
        assert_eq!(vector.grade(), Grade::Vector);

        let bivector = GATerm::bivector(vec![(1, 2, 4.0)]);
        assert_eq!(bivector.grade(), Grade::Bivector);

        let trivector = GATerm::trivector(vec![(1, 2, 3, 5.0)]);
        assert_eq!(trivector.grade(), Grade::Trivector);
    }

    #[test]
    fn test_blade_term() {
        let term = BladeTerm::new(vec![1, 2], 3.0);
        assert_eq!(term.grade(), Grade::Bivector);
        assert_eq!(term.coefficient, 3.0);
        assert_eq!(term.indices, vec![1, 2]);
    }
}