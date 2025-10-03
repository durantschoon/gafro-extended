// SPDX-FileCopyrightText: GAFRO Extended Implementation
//
// SPDX-License-Identifier: MPL-2.0

use crate::ga_term::{GATerm, Grade, Scalar, BladeTerm, Index};
use crate::grade_indexed::GradeIndexed;

/// Pattern matching utilities using Rust's match expressions
///
/// This provides functional-style pattern matching for GATerm enums,
/// allowing safe and ergonomic handling of different grades.

/// Pattern matching on GATerm with different handlers for each grade
pub fn match_gaterm<T, R, SF, VF, BF, TF, MF>(
    term: &GATerm<T>,
    scalar_handler: SF,
    vector_handler: VF,
    bivector_handler: BF,
    trivector_handler: TF,
    multivector_handler: MF,
) -> R
where
    SF: FnOnce(&Scalar<T>) -> R,
    VF: FnOnce(&Vec<(Index, T)>) -> R,
    BF: FnOnce(&Vec<(Index, Index, T)>) -> R,
    TF: FnOnce(&Vec<(Index, Index, Index, T)>) -> R,
    MF: FnOnce(&Vec<BladeTerm<T>>) -> R,
{
    match term {
        GATerm::Scalar(scalar) => scalar_handler(scalar),
        GATerm::Vector(vector) => vector_handler(vector),
        GATerm::Bivector(bivector) => bivector_handler(bivector),
        GATerm::Trivector(trivector) => trivector_handler(trivector),
        GATerm::Multivector(multivector) => multivector_handler(multivector),
    }
}

/// Simplified visitor pattern for GATerm
pub trait GATermVisitor<T, R> {
    fn visit_scalar(&self, scalar: &Scalar<T>) -> R;
    fn visit_vector(&self, vector: &Vec<(Index, T)>) -> R;
    fn visit_bivector(&self, bivector: &Vec<(Index, Index, T)>) -> R;
    fn visit_trivector(&self, trivector: &Vec<(Index, Index, Index, T)>) -> R;
    fn visit_multivector(&self, multivector: &Vec<BladeTerm<T>>) -> R;
}

/// Apply visitor to GATerm
pub fn visit_gaterm<T, R, V: GATermVisitor<T, R>>(term: &GATerm<T>, visitor: &V) -> R {
    match term {
        GATerm::Scalar(scalar) => visitor.visit_scalar(scalar),
        GATerm::Vector(vector) => visitor.visit_vector(vector),
        GATerm::Bivector(bivector) => visitor.visit_bivector(bivector),
        GATerm::Trivector(trivector) => visitor.visit_trivector(trivector),
        GATerm::Multivector(multivector) => visitor.visit_multivector(multivector),
    }
}

/// Type-safe operations using pattern matching
pub mod operations {
    use super::*;

    /// Addition of two GA terms (same grade only)
    pub fn add<T>(lhs: &GATerm<T>, rhs: &GATerm<T>) -> Option<GATerm<T>>
    where
        T: Clone + std::ops::Add<Output = T> + Default,
    {
        // Check if both terms have the same grade
        if lhs.grade() != rhs.grade() {
            return None; // Cannot add different grades
        }

        match (lhs, rhs) {
            (GATerm::Scalar(s1), GATerm::Scalar(s2)) => {
                Some(GATerm::scalar(s1.value.clone() + s2.value.clone()))
            }
            (GATerm::Vector(v1), GATerm::Vector(v2)) => {
                let mut result = v1.clone();
                for (idx, coeff) in v2.iter() {
                    if let Some((_, existing_coeff)) = result.iter_mut().find(|(i, _)| i == idx) {
                        *existing_coeff = existing_coeff.clone() + coeff.clone();
                    } else {
                        result.push((*idx, coeff.clone()));
                    }
                }
                Some(GATerm::vector(result))
            }
            (GATerm::Bivector(b1), GATerm::Bivector(b2)) => {
                let mut result = b1.clone();
                for (i1, i2, coeff) in b2.iter() {
                    if let Some((_, _, existing_coeff)) = result
                        .iter_mut()
                        .find(|(j1, j2, _)| j1 == i1 && j2 == i2)
                    {
                        *existing_coeff = existing_coeff.clone() + coeff.clone();
                    } else {
                        result.push((*i1, *i2, coeff.clone()));
                    }
                }
                Some(GATerm::bivector(result))
            }
            (GATerm::Trivector(t1), GATerm::Trivector(t2)) => {
                let mut result = t1.clone();
                for (i1, i2, i3, coeff) in t2.iter() {
                    if let Some((_, _, _, existing_coeff)) = result
                        .iter_mut()
                        .find(|(j1, j2, j3, _)| j1 == i1 && j2 == i2 && j3 == i3)
                    {
                        *existing_coeff = existing_coeff.clone() + coeff.clone();
                    } else {
                        result.push((*i1, *i2, *i3, coeff.clone()));
                    }
                }
                Some(GATerm::trivector(result))
            }
            (GATerm::Multivector(m1), GATerm::Multivector(m2)) => {
                let mut result = m1.clone();
                for term in m2.iter() {
                    if let Some(existing_term) = result
                        .iter_mut()
                        .find(|t| t.indices == term.indices)
                    {
                        existing_term.coefficient = existing_term.coefficient.clone() + term.coefficient.clone();
                    } else {
                        result.push(term.clone());
                    }
                }
                Some(GATerm::multivector(result))
            }
            _ => None,
        }
    }

    /// Scalar multiplication
    pub fn scalar_multiply<T, S>(scalar: S, term: &GATerm<T>) -> GATerm<T>
    where
        T: Clone + std::ops::Mul<S, Output = T>,
        S: Clone,
    {
        match term {
            GATerm::Scalar(s) => GATerm::scalar(s.value.clone() * scalar),
            GATerm::Vector(v) => {
                let result: Vec<(Index, T)> = v
                    .iter()
                    .map(|(idx, coeff)| (*idx, coeff.clone() * scalar.clone()))
                    .collect();
                GATerm::vector(result)
            }
            GATerm::Bivector(b) => {
                let result: Vec<(Index, Index, T)> = b
                    .iter()
                    .map(|(i1, i2, coeff)| (*i1, *i2, coeff.clone() * scalar.clone()))
                    .collect();
                GATerm::bivector(result)
            }
            GATerm::Trivector(t) => {
                let result: Vec<(Index, Index, Index, T)> = t
                    .iter()
                    .map(|(i1, i2, i3, coeff)| (*i1, *i2, *i3, coeff.clone() * scalar.clone()))
                    .collect();
                GATerm::trivector(result)
            }
            GATerm::Multivector(m) => {
                let result: Vec<BladeTerm<T>> = m
                    .iter()
                    .map(|term| BladeTerm::new(
                        term.indices.clone(),
                        term.coefficient.clone() * scalar.clone()
                    ))
                    .collect();
                GATerm::multivector(result)
            }
        }
    }

    /// Get norm of a GA term
    pub fn norm<T>(term: &GATerm<T>) -> T
    where
        T: Clone + std::ops::Add<Output = T> + std::ops::Mul<Output = T> + From<f64>,
        f64: From<T>,
    {
        match term {
            GATerm::Scalar(s) => {
                let val: f64 = s.value.clone().into();
                T::from(val.abs())
            }
            GATerm::Vector(v) => {
                let sum: T = v
                    .iter()
                    .map(|(_, coeff)| coeff.clone() * coeff.clone())
                    .fold(T::from(0.0), |acc, x| acc + x);
                let sum_f64: f64 = sum.into();
                T::from(sum_f64.sqrt())
            }
            GATerm::Bivector(b) => {
                let sum: T = b
                    .iter()
                    .map(|(_, _, coeff)| coeff.clone() * coeff.clone())
                    .fold(T::from(0.0), |acc, x| acc + x);
                let sum_f64: f64 = sum.into();
                T::from(sum_f64.sqrt())
            }
            GATerm::Trivector(t) => {
                let sum: T = t
                    .iter()
                    .map(|(_, _, _, coeff)| coeff.clone() * coeff.clone())
                    .fold(T::from(0.0), |acc, x| acc + x);
                let sum_f64: f64 = sum.into();
                T::from(sum_f64.sqrt())
            }
            GATerm::Multivector(m) => {
                let sum: T = m
                    .iter()
                    .map(|term| term.coefficient.clone() * term.coefficient.clone())
                    .fold(T::from(0.0), |acc, x| acc + x);
                let sum_f64: f64 = sum.into();
                T::from(sum_f64.sqrt())
            }
        }
    }

    /// Convert GA term to string representation
    pub fn to_string<T>(term: &GATerm<T>) -> String
    where
        T: std::fmt::Display,
    {
        match term {
            GATerm::Scalar(s) => format!("Scalar({})", s.value),
            GATerm::Vector(v) => {
                let components: Vec<String> = v
                    .iter()
                    .map(|(idx, coeff)| format!("e{}:{}", idx, coeff))
                    .collect();
                format!("Vector({})", components.join(", "))
            }
            GATerm::Bivector(b) => {
                let components: Vec<String> = b
                    .iter()
                    .map(|(i1, i2, coeff)| format!("e{}e{}:{}", i1, i2, coeff))
                    .collect();
                format!("Bivector({})", components.join(", "))
            }
            GATerm::Trivector(t) => {
                let components: Vec<String> = t
                    .iter()
                    .map(|(i1, i2, i3, coeff)| format!("e{}e{}e{}:{}", i1, i2, i3, coeff))
                    .collect();
                format!("Trivector({})", components.join(", "))
            }
            GATerm::Multivector(m) => {
                let components: Vec<String> = m
                    .iter()
                    .map(|term| {
                        let indices: Vec<String> = term.indices.iter().map(|i| format!("e{}", i)).collect();
                        format!("{}:{}", indices.join(""), term.coefficient)
                    })
                    .collect();
                format!("Multivector({})", components.join(", "))
            }
        }
    }
}

/// Functional-style combinators for pattern matching
pub mod combinators {
    use super::*;

    /// Map over GA term preserving structure
    pub fn map<T, U, F>(term: &GATerm<T>, f: F) -> GATerm<U>
    where
        F: Fn(&T) -> U + Clone,
        T: Clone,
    {
        match term {
            GATerm::Scalar(s) => GATerm::scalar(f(&s.value)),
            GATerm::Vector(v) => {
                let result: Vec<(Index, U)> = v
                    .iter()
                    .map(|(idx, coeff)| (*idx, f(coeff)))
                    .collect();
                GATerm::vector(result)
            }
            GATerm::Bivector(b) => {
                let result: Vec<(Index, Index, U)> = b
                    .iter()
                    .map(|(i1, i2, coeff)| (*i1, *i2, f(coeff)))
                    .collect();
                GATerm::bivector(result)
            }
            GATerm::Trivector(t) => {
                let result: Vec<(Index, Index, Index, U)> = t
                    .iter()
                    .map(|(i1, i2, i3, coeff)| (*i1, *i2, *i3, f(coeff)))
                    .collect();
                GATerm::trivector(result)
            }
            GATerm::Multivector(m) => {
                let result: Vec<BladeTerm<U>> = m
                    .iter()
                    .map(|term| BladeTerm::new(term.indices.clone(), f(&term.coefficient)))
                    .collect();
                GATerm::multivector(result)
            }
        }
    }

    /// Filter components based on predicate
    pub fn filter<T, P>(term: &GATerm<T>, predicate: P) -> GATerm<T>
    where
        P: Fn(&T) -> bool,
        T: Clone,
    {
        match term {
            GATerm::Scalar(s) => {
                if predicate(&s.value) {
                    term.clone()
                } else {
                    GATerm::scalar(s.value.clone()) // Return as-is for scalars
                }
            }
            GATerm::Vector(v) => {
                let result: Vec<(Index, T)> = v
                    .iter()
                    .filter(|(_, coeff)| predicate(coeff))
                    .map(|(idx, coeff)| (*idx, coeff.clone()))
                    .collect();
                GATerm::vector(result)
            }
            GATerm::Bivector(b) => {
                let result: Vec<(Index, Index, T)> = b
                    .iter()
                    .filter(|(_, _, coeff)| predicate(coeff))
                    .map(|(i1, i2, coeff)| (*i1, *i2, coeff.clone()))
                    .collect();
                GATerm::bivector(result)
            }
            GATerm::Trivector(t) => {
                let result: Vec<(Index, Index, Index, T)> = t
                    .iter()
                    .filter(|(_, _, _, coeff)| predicate(coeff))
                    .map(|(i1, i2, i3, coeff)| (*i1, *i2, *i3, coeff.clone()))
                    .collect();
                GATerm::trivector(result)
            }
            GATerm::Multivector(m) => {
                let result: Vec<BladeTerm<T>> = m
                    .iter()
                    .filter(|term| predicate(&term.coefficient))
                    .cloned()
                    .collect();
                GATerm::multivector(result)
            }
        }
    }

    /// Fold over GA term components
    pub fn fold<T, Acc, F>(term: &GATerm<T>, initial: Acc, f: F) -> Acc
    where
        F: Fn(Acc, &T) -> Acc,
    {
        match term {
            GATerm::Scalar(s) => f(initial, &s.value),
            GATerm::Vector(v) => v.iter().fold(initial, |acc, (_, coeff)| f(acc, coeff)),
            GATerm::Bivector(b) => b.iter().fold(initial, |acc, (_, _, coeff)| f(acc, coeff)),
            GATerm::Trivector(t) => t.iter().fold(initial, |acc, (_, _, _, coeff)| f(acc, coeff)),
            GATerm::Multivector(m) => m.iter().fold(initial, |acc, term| f(acc, &term.coefficient)),
        }
    }
}

/// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use super::operations::*;

    #[test]
    fn test_pattern_matching() {
        let scalar = GATerm::scalar(3.14);
        let vector = GATerm::vector(vec![(1, 2.0), (2, 3.0)]);

        let scalar_result = match_gaterm(
            &scalar,
            |s| format!("Got scalar: {}", s.value),
            |_| "Not a scalar".to_string(),
            |_| "Not a scalar".to_string(),
            |_| "Not a scalar".to_string(),
            |_| "Not a scalar".to_string(),
        );

        assert_eq!(scalar_result, "Got scalar: 3.14");

        let vector_result = match_gaterm(
            &vector,
            |_| "Not a vector".to_string(),
            |v| format!("Got vector with {} components", v.len()),
            |_| "Not a vector".to_string(),
            |_| "Not a vector".to_string(),
            |_| "Not a vector".to_string(),
        );

        assert_eq!(vector_result, "Got vector with 2 components");
    }

    #[test]
    fn test_addition() {
        let s1 = GATerm::scalar(2.0);
        let s2 = GATerm::scalar(3.0);
        let sum = add(&s1, &s2).unwrap();

        if let GATerm::Scalar(s) = sum {
            assert_eq!(s.value, 5.0);
        } else {
            panic!("Expected scalar result");
        }

        let v1 = GATerm::vector(vec![(1, 2.0), (2, 3.0)]);
        let v2 = GATerm::vector(vec![(1, 1.0), (3, 4.0)]);
        let vector_sum = add(&v1, &v2).unwrap();

        if let GATerm::Vector(v) = vector_sum {
            assert_eq!(v.len(), 3);
            // Should have combined like terms
            assert!(v.iter().any(|(idx, coeff)| *idx == 1 && *coeff == 3.0));
        } else {
            panic!("Expected vector result");
        }
    }

    #[test]
    fn test_scalar_multiplication() {
        let vector = GATerm::vector(vec![(1, 2.0), (2, 3.0)]);
        let scaled = scalar_multiply(2.0, &vector);

        if let GATerm::Vector(v) = scaled {
            assert_eq!(v[0].1, 4.0);
            assert_eq!(v[1].1, 6.0);
        } else {
            panic!("Expected vector result");
        }
    }

    #[test]
    fn test_norm() {
        let vector = GATerm::vector(vec![(1, 3.0), (2, 4.0)]);
        let n = norm(&vector);
        assert!((n - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_to_string() {
        let scalar = GATerm::scalar(3.14);
        assert_eq!(to_string(&scalar), "Scalar(3.14)");

        let vector = GATerm::vector(vec![(1, 2.0), (2, 3.0)]);
        assert_eq!(to_string(&vector), "Vector(e1:2, e2:3)");
    }

    #[test]
    fn test_combinators() {
        let vector = GATerm::vector(vec![(1, 2.0), (2, 3.0), (3, 4.0)]);

        // Test map
        let doubled = combinators::map(&vector, |x| x * 2.0);
        if let GATerm::Vector(v) = doubled {
            assert_eq!(v[0].1, 4.0);
            assert_eq!(v[1].1, 6.0);
            assert_eq!(v[2].1, 8.0);
        }

        // Test filter
        let filtered = combinators::filter(&vector, |x| *x > 2.5);
        if let GATerm::Vector(v) = filtered {
            assert_eq!(v.len(), 2); // Should filter out 2.0
        }

        // Test fold
        let sum = combinators::fold(&vector, 0.0, |acc, x| acc + x);
        assert_eq!(sum, 9.0);
    }
}