// SPDX-FileCopyrightText: GAFRO Extended Implementation
//
// SPDX-License-Identifier: MPL-2.0

use std::ops::{Add, Sub, Mul, Div, Neg};

/// Compile-time dimensional analysis for physical quantities
/// 
/// This provides type-safe physical quantities with compile-time unit checking.
/// Dimensions are specified as const generics for Mass, Length, and Time.
/// 
/// # Type Parameters
/// * `M` - Mass dimension (0 = dimensionless, 1 = kg, etc.)
/// * `L` - Length dimension (0 = dimensionless, 1 = m, etc.)
/// * `T` - Time dimension (0 = dimensionless, 1 = s, etc.)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SIQuantity<const M: i32, const L: i32, const T: i32> {
    value: f64,
}

impl<const M: i32, const L: i32, const T: i32> SIQuantity<M, L, T> {
    /// Create a new SIQuantity with the given value
    pub const fn new(value: f64) -> Self {
        Self { value }
    }

    /// Get the raw value (use with caution)
    pub fn value(self) -> f64 {
        self.value
    }

    /// Get the mass dimension
    pub const fn mass_dim() -> i32 {
        M
    }

    /// Get the length dimension
    pub const fn length_dim() -> i32 {
        L
    }

    /// Get the time dimension
    pub const fn time_dim() -> i32 {
        T
    }
}

// Note: For this demo, we'll use simple multiplication/division without const arithmetic
// In a real implementation, you'd use a more sophisticated approach

impl<const M: i32, const L: i32, const T: i32> Add<SIQuantity<M, L, T>> for SIQuantity<M, L, T> {
    type Output = SIQuantity<M, L, T>;

    fn add(self, other: SIQuantity<M, L, T>) -> Self::Output {
        SIQuantity::new(self.value + other.value)
    }
}

impl<const M: i32, const L: i32, const T: i32> Sub<SIQuantity<M, L, T>> for SIQuantity<M, L, T> {
    type Output = SIQuantity<M, L, T>;

    fn sub(self, other: SIQuantity<M, L, T>) -> Self::Output {
        SIQuantity::new(self.value - other.value)
    }
}

impl<const M: i32, const L: i32, const T: i32> Neg for SIQuantity<M, L, T> {
    type Output = SIQuantity<M, L, T>;

    fn neg(self) -> Self::Output {
        SIQuantity::new(-self.value)
    }
}

// Scalar operations
impl<const M: i32, const L: i32, const T: i32> Mul<f64> for SIQuantity<M, L, T> {
    type Output = SIQuantity<M, L, T>;

    fn mul(self, scalar: f64) -> Self::Output {
        SIQuantity::new(self.value * scalar)
    }
}

impl<const M: i32, const L: i32, const T: i32> Div<f64> for SIQuantity<M, L, T> {
    type Output = SIQuantity<M, L, T>;

    fn div(self, scalar: f64) -> Self::Output {
        SIQuantity::new(self.value / scalar)
    }
}

// Scalar multiplication from the left
impl<const M: i32, const L: i32, const T: i32> Mul<SIQuantity<M, L, T>> for f64 {
    type Output = SIQuantity<M, L, T>;

    fn mul(self, quantity: SIQuantity<M, L, T>) -> Self::Output {
        quantity * self
    }
}

// Common physical quantity type aliases
pub type Dimensionless = SIQuantity<0, 0, 0>;
pub type Mass = SIQuantity<1, 0, 0>;           // kg
pub type Length = SIQuantity<0, 1, 0>;         // m
pub type Time = SIQuantity<0, 0, 1>;           // s
pub type Velocity = SIQuantity<0, 1, -1>;      // m/s
pub type Acceleration = SIQuantity<0, 1, -2>;  // m/s²
pub type Force = SIQuantity<1, 1, -2>;         // N (kg⋅m/s²)
pub type Energy = SIQuantity<1, 2, -2>;        // J (kg⋅m²/s²)
pub type Power = SIQuantity<1, 2, -3>;         // W (kg⋅m²/s³)
pub type Pressure = SIQuantity<1, -1, -2>;     // Pa (kg/m⋅s²)
pub type Torque = SIQuantity<1, 2, -2>;        // N⋅m (same as Energy dimensionally)

// Convenience constructors
impl Mass {
    pub fn kg(value: f64) -> Self {
        Self::new(value)
    }
}

impl Length {
    pub fn m(value: f64) -> Self {
        Self::new(value)
    }
}

impl Time {
    pub fn s(value: f64) -> Self {
        Self::new(value)
    }
}

impl Velocity {
    pub fn mps(value: f64) -> Self {
        Self::new(value)
    }
}

impl Force {
    pub fn n(value: f64) -> Self {
        Self::new(value)
    }
}
