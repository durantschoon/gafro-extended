// SPDX-FileCopyrightText: GAFRO Extended Implementation
//
// SPDX-License-Identifier: MPL-2.0

use std::marker::PhantomData;
use serde::{Deserialize, Serialize};
use crate::ga_term::{Grade, Index, BladeTerm};

/// Grade marker for const generics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GradeMarker<const G: u8>;

/// Grade-indexed wrapper for compile-time grade checking
///
/// This provides compile-time grade safety by encoding the grade
/// in the type system using const generics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GradeIndexed<T, const G: u8> {
    pub value: T,
    _phantom: PhantomData<GradeMarker<G>>,
}

impl<T, const G: u8> GradeIndexed<T, G> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }

    pub fn grade(&self) -> Grade {
        match G {
            0 => Grade::Scalar,
            1 => Grade::Vector,
            2 => Grade::Bivector,
            3 => Grade::Trivector,
            _ => Grade::Multivector,
        }
    }

    pub const fn grade_const() -> u8 {
        G
    }

    pub fn into_inner(self) -> T {
        self.value
    }

    pub fn as_ref(&self) -> &T {
        &self.value
    }

    pub fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T, const G: u8> From<T> for GradeIndexed<T, G> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T, const G: u8> AsRef<T> for GradeIndexed<T, G> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T, const G: u8> AsMut<T> for GradeIndexed<T, G> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

/// Type aliases for common grades
pub type ScalarType<T> = GradeIndexed<T, 0>;
pub type VectorType<T> = GradeIndexed<Vec<(Index, T)>, 1>;
pub type BivectorType<T> = GradeIndexed<Vec<(Index, Index, T)>, 2>;
pub type TrivectorType<T> = GradeIndexed<Vec<(Index, Index, Index, T)>, 3>;

/// Trait for grade-indexed types
pub trait IsGradeIndexed {
    const GRADE: u8;

    fn grade(&self) -> Grade {
        match Self::GRADE {
            0 => Grade::Scalar,
            1 => Grade::Vector,
            2 => Grade::Bivector,
            3 => Grade::Trivector,
            _ => Grade::Multivector,
        }
    }
}

impl<T, const G: u8> IsGradeIndexed for GradeIndexed<T, G> {
    const GRADE: u8 = G;
}

/// Operations for grade-indexed types

// Addition: only same grades can be added
impl<T, const G: u8> std::ops::Add for GradeIndexed<T, G>
where
    T: std::ops::Add<Output = T>,
{
    type Output = GradeIndexed<T, G>;

    fn add(self, rhs: Self) -> Self::Output {
        GradeIndexed::new(self.value + rhs.value)
    }
}

// Scalar multiplication
impl<T, S, const G: u8> std::ops::Mul<S> for GradeIndexed<T, G>
where
    T: std::ops::Mul<S, Output = T>,
    S: Copy,
{
    type Output = GradeIndexed<T, G>;

    fn mul(self, rhs: S) -> Self::Output {
        GradeIndexed::new(self.value * rhs)
    }
}

/// Factory functions for grade-indexed types
impl<T> ScalarType<T> {
    pub fn scalar(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> VectorType<T> {
    pub fn vector(components: Vec<(Index, T)>) -> Self {
        Self::new(components)
    }
}

impl<T> BivectorType<T> {
    pub fn bivector(components: Vec<(Index, Index, T)>) -> Self {
        Self::new(components)
    }
}

impl<T> TrivectorType<T> {
    pub fn trivector(components: Vec<(Index, Index, Index, T)>) -> Self {
        Self::new(components)
    }
}

/// Grade checking utilities
pub struct GradeChecker<T> {
    _phantom: PhantomData<T>,
}

impl<T> GradeChecker<T> {
    pub fn is_scalar<const G: u8>() -> bool {
        G == 0
    }

    pub fn is_vector<const G: u8>() -> bool {
        G == 1
    }

    pub fn is_bivector<const G: u8>() -> bool {
        G == 2
    }

    pub fn is_trivector<const G: u8>() -> bool {
        G == 3
    }

    pub fn is_multivector<const G: u8>() -> bool {
        G > 3
    }
}

/// Compile-time grade validation macros
macro_rules! assert_same_grade {
    ($t1:ty, $t2:ty) => {
        const _: () = {
            assert!(<$t1 as IsGradeIndexed>::GRADE == <$t2 as IsGradeIndexed>::GRADE);
        };
    };
}

macro_rules! assert_grade {
    ($t:ty, $grade:expr) => {
        const _: () = {
            assert!(<$t as IsGradeIndexed>::GRADE == $grade);
        };
    };
}

pub use assert_same_grade;
pub use assert_grade;

/// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grade_indexed_creation() {
        let scalar: ScalarType<f64> = ScalarType::scalar(3.14);
        assert_eq!(scalar.grade(), Grade::Scalar);
        assert_eq!(scalar.value, 3.14);

        let vector: VectorType<f64> = VectorType::vector(vec![(1, 2.0), (2, 3.0)]);
        assert_eq!(vector.grade(), Grade::Vector);
        assert_eq!(vector.value.len(), 2);
    }

    #[test]
    fn test_grade_indexed_arithmetic() {
        let s1: ScalarType<f64> = ScalarType::scalar(2.0);
        let s2: ScalarType<f64> = ScalarType::scalar(3.0);

        let sum = s1 + s2;
        assert_eq!(sum.value, 5.0);
        assert_eq!(sum.grade(), Grade::Scalar);

        let s3: ScalarType<f64> = ScalarType::scalar(2.0);
        let product = s3 * 3.0;
        assert_eq!(product.value, 6.0);
    }

    #[test]
    fn test_grade_checking() {
        assert!(GradeChecker::<f64>::is_scalar::<0>());
        assert!(!GradeChecker::<f64>::is_scalar::<1>());
        assert!(GradeChecker::<f64>::is_vector::<1>());
        assert!(GradeChecker::<f64>::is_bivector::<2>());
        assert!(GradeChecker::<f64>::is_trivector::<3>());
    }

    #[test]
    fn test_const_grade() {
        assert_eq!(ScalarType::<f64>::grade_const(), 0);
        assert_eq!(VectorType::<f64>::grade_const(), 1);
        assert_eq!(BivectorType::<f64>::grade_const(), 2);
        assert_eq!(TrivectorType::<f64>::grade_const(), 3);
    }
}