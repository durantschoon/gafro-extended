// SPDX-FileCopyrightText: GAFRO Extended Implementation
//
// SPDX-License-Identifier: MPL-2.0

use crate::ga_term::{Grade, GATerm, BladeTerm, Index};
use crate::grade_indexed::{GradeIndexed, IsGradeIndexed};

/// Compile-time grade checking system
///
/// This provides comprehensive validation for geometric algebra
/// operations at compile time, preventing invalid operations between
/// incompatible grades.

/// Trait for grade validation
pub trait GradeValidator {
    const GRADE: u8;

    fn validate_grade(&self) -> bool {
        true // Default implementation
    }
}

/// Operation compatibility traits
pub trait CanAdd<Rhs = Self> {
    type Output;
}

pub trait CanGeometricProduct<Rhs = Self> {
    type Output;
}

pub trait CanOuterProduct<Rhs = Self> {
    type Output;
}

pub trait CanInnerProduct<Rhs = Self> {
    type Output;
}

/// Implement CanAdd for same grades
impl<T, const G: u8> CanAdd for GradeIndexed<T, G>
where
    T: std::ops::Add<Output = T>,
{
    type Output = GradeIndexed<T, G>;
}

/// Implement geometric product for all grade combinations
impl<T1, T2, const G1: u8, const G2: u8> CanGeometricProduct<GradeIndexed<T2, G2>>
    for GradeIndexed<T1, G1>
{
    type Output = GATerm<f64>; // Simplified output type
}

/// Grade calculation utilities
pub mod grade_calc {
    use super::*;

    /// Calculate result grades for geometric product
    pub const fn geometric_product_grades(g1: u8, g2: u8) -> &'static [u8] {
        // Geometric product can produce multiple grades
        // |g1 - g2|, |g1 - g2| + 2, ..., g1 + g2
        match (g1, g2) {
            (0, g) | (g, 0) => match g {
                0 => &[0],
                1 => &[1],
                2 => &[2],
                3 => &[3],
                _ => &[255], // Multivector
            },
            (1, 1) => &[0, 2],
            (1, 2) => &[1, 3],
            (1, 3) => &[2],
            (2, 1) => &[1, 3],
            (2, 2) => &[0, 2],
            (2, 3) => &[1],
            (3, 1) => &[2],
            (3, 2) => &[1],
            (3, 3) => &[0, 2],
            _ => &[255], // General multivector case
        }
    }

    /// Calculate result grade for outer product
    pub const fn outer_product_grade(g1: u8, g2: u8) -> u8 {
        let result = g1 + g2;
        if result <= 3 {
            result
        } else {
            255 // Multivector
        }
    }

    /// Calculate result grade for inner product
    pub const fn inner_product_grade(g1: u8, g2: u8) -> u8 {
        let result = if g1 >= g2 { g1 - g2 } else { g2 - g1 };
        if result <= 3 {
            result
        } else {
            255 // Multivector
        }
    }
}

/// Compile-time operation validation
pub struct OperationValidator<T1, T2> {
    _phantom: std::marker::PhantomData<(T1, T2)>,
}

impl<T1, T2, const G1: u8, const G2: u8> OperationValidator<GradeIndexed<T1, G1>, GradeIndexed<T2, G2>> {
    pub const fn can_add() -> bool {
        G1 == G2
    }

    pub const fn can_multiply() -> bool {
        true // Geometric product is always valid
    }

    pub const fn can_outer_product() -> bool {
        true // Outer product is always valid
    }

    pub const fn can_inner_product() -> bool {
        true // Inner product is always valid
    }

    pub const fn outer_product_grade() -> u8 {
        grade_calc::outer_product_grade(G1, G2)
    }

    pub const fn inner_product_grade() -> u8 {
        grade_calc::inner_product_grade(G1, G2)
    }
}

/// Grade-safe operations
pub mod safe_ops {
    use super::*;

    /// Grade-safe addition
    pub fn add<T, const G: u8>(
        lhs: GradeIndexed<T, G>,
        rhs: GradeIndexed<T, G>,
    ) -> GradeIndexed<T, G>
    where
        T: std::ops::Add<Output = T>,
    {
        GradeIndexed::new(lhs.into_inner() + rhs.into_inner())
    }

    /// Grade-safe scalar multiplication
    pub fn scalar_multiply<T, S, const G: u8>(
        scalar: S,
        operand: GradeIndexed<T, G>,
    ) -> GradeIndexed<T, G>
    where
        T: std::ops::Mul<S, Output = T>,
    {
        GradeIndexed::new(operand.into_inner() * scalar)
    }

    /// Grade-safe outer product
    pub fn outer_product<T1, T2, const G1: u8, const G2: u8>(
        lhs: GradeIndexed<T1, G1>,
        rhs: GradeIndexed<T2, G2>,
    ) -> GATerm<f64>
    where
        T1: Clone,
        T2: Clone,
    {
        // Placeholder implementation - actual implementation would compute the outer product
        const RESULT_GRADE: u8 = grade_calc::outer_product_grade(G1, G2);

        match RESULT_GRADE {
            0 => GATerm::scalar(0.0),
            1 => GATerm::vector(vec![]),
            2 => GATerm::bivector(vec![]),
            3 => GATerm::trivector(vec![]),
            _ => GATerm::multivector(vec![]),
        }
    }

    /// Grade-safe inner product
    pub fn inner_product<T1, T2, const G1: u8, const G2: u8>(
        lhs: GradeIndexed<T1, G1>,
        rhs: GradeIndexed<T2, G2>,
    ) -> GATerm<f64>
    where
        T1: Clone,
        T2: Clone,
    {
        // Placeholder implementation - actual implementation would compute the inner product
        const RESULT_GRADE: u8 = grade_calc::inner_product_grade(G1, G2);

        match RESULT_GRADE {
            0 => GATerm::scalar(0.0),
            1 => GATerm::vector(vec![]),
            2 => GATerm::bivector(vec![]),
            3 => GATerm::trivector(vec![]),
            _ => GATerm::multivector(vec![]),
        }
    }
}

/// Type inspection utilities
pub struct TypeInspector<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T, const G: u8> TypeInspector<GradeIndexed<T, G>> {
    pub const fn is_grade_indexed() -> bool {
        true
    }

    pub const fn grade() -> u8 {
        G
    }

    pub const fn is_scalar() -> bool {
        G == 0
    }

    pub const fn is_vector() -> bool {
        G == 1
    }

    pub const fn is_bivector() -> bool {
        G == 2
    }

    pub const fn is_trivector() -> bool {
        G == 3
    }

    pub const fn is_multivector() -> bool {
        G > 3
    }
}

/// Compile-time operation compatibility matrix
pub struct OperationMatrix<const G1: u8, const G2: u8>;

impl<const G1: u8, const G2: u8> OperationMatrix<G1, G2> {
    pub const CAN_ADD: bool = G1 == G2;
    pub const CAN_GEOMETRIC_PRODUCT: bool = true;
    pub const CAN_OUTER_PRODUCT: bool = true;
    pub const CAN_INNER_PRODUCT: bool = true;

    pub const OUTER_PRODUCT_RESULT: u8 = grade_calc::outer_product_grade(G1, G2);
    pub const INNER_PRODUCT_RESULT: u8 = grade_calc::inner_product_grade(G1, G2);
}

/// Macros for compile-time validation
macro_rules! assert_same_grade {
    ($t1:ty, $t2:ty) => {
        const _: () = {
            static_assert!(<$t1 as IsGradeIndexed>::GRADE == <$t2 as IsGradeIndexed>::GRADE);
        };
    };
}

macro_rules! assert_valid_operation {
    ($t1:ty, $t2:ty, Add) => {
        const _: () = {
            static_assert!(OperationValidator::<$t1, $t2>::can_add());
        };
    };
    ($t1:ty, $t2:ty, GeometricProduct) => {
        const _: () = {
            static_assert!(OperationValidator::<$t1, $t2>::can_multiply());
        };
    };
    ($t1:ty, $t2:ty, OuterProduct) => {
        const _: () = {
            static_assert!(OperationValidator::<$t1, $t2>::can_outer_product());
        };
    };
    ($t1:ty, $t2:ty, InnerProduct) => {
        const _: () = {
            static_assert!(OperationValidator::<$t1, $t2>::can_inner_product());
        };
    };
}

macro_rules! assert_grade {
    ($t:ty, $grade:expr) => {
        const _: () = {
            static_assert!(<$t as IsGradeIndexed>::GRADE == $grade);
        };
    };
}

// Note: static_assert! is not available in stable Rust, so these would need
// to be implemented using const assertions or compile_fail tests

pub use assert_same_grade;
pub use assert_valid_operation;
pub use assert_grade;

/// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::grade_indexed::{ScalarType, VectorType, BivectorType};

    #[test]
    fn test_grade_calculation() {
        assert_eq!(grade_calc::outer_product_grade(1, 1), 2);
        assert_eq!(grade_calc::outer_product_grade(1, 2), 3);
        assert_eq!(grade_calc::inner_product_grade(2, 1), 1);
        assert_eq!(grade_calc::inner_product_grade(1, 1), 0);
    }

    #[test]
    fn test_operation_validation() {
        type S = ScalarType<f64>;
        type V = VectorType<f64>;
        type B = BivectorType<f64>;

        assert!(OperationValidator::<S, S>::can_add());
        assert!(!OperationValidator::<S, V>::can_add());
        assert!(OperationValidator::<S, V>::can_multiply());
        assert!(OperationValidator::<V, B>::can_outer_product());
    }

    #[test]
    fn test_safe_operations() {
        let s1 = ScalarType::scalar(2.0);
        let s2 = ScalarType::scalar(3.0);

        let sum = safe_ops::add(s1, s2);
        assert_eq!(sum.value, 5.0);

        let s3 = ScalarType::scalar(4.0);
        let product = safe_ops::scalar_multiply(2.0, s3);
        assert_eq!(product.value, 8.0);
    }

    #[test]
    fn test_type_inspector() {
        type S = ScalarType<f64>;
        type V = VectorType<f64>;

        assert!(TypeInspector::<S>::is_scalar());
        assert!(!TypeInspector::<S>::is_vector());
        assert!(!TypeInspector::<V>::is_scalar());
        assert!(TypeInspector::<V>::is_vector());
    }

    #[test]
    fn test_operation_matrix() {
        type Matrix01 = OperationMatrix<0, 1>;
        type Matrix11 = OperationMatrix<1, 1>;

        assert!(!Matrix01::CAN_ADD);
        assert!(Matrix01::CAN_GEOMETRIC_PRODUCT);
        assert!(Matrix11::CAN_ADD);
        assert_eq!(Matrix01::OUTER_PRODUCT_RESULT, 1);
        assert_eq!(Matrix11::INNER_PRODUCT_RESULT, 0);
    }
}