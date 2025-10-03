// SPDX-FileCopyrightText: GAFRO Extended Implementation
//
// SPDX-License-Identifier: MPL-2.0

use std::ops::{Add, Sub, Mul, Div, Neg};

/// Type-safe angle representation with tau convention
/// 
/// This provides type-safe angle handling using the tau (τ = 2π) convention
/// for more natural geometric calculations.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle {
    radians: f64,  // Stored in radians using tau convention
}

impl Angle {
    /// Mathematical constants using tau convention
    pub const TAU: f64 = 6.283185307179586;  // τ = 2π
    pub const PI: f64 = Self::TAU / 2.0;     // π = τ/2

    /// Create a new angle from radians
    pub const fn from_radians(radians: f64) -> Self {
        Self { radians }
    }

    /// Create a new angle from degrees
    pub const fn from_degrees(degrees: f64) -> Self {
        Self {
            radians: degrees * Self::TAU / 360.0,
        }
    }

    /// Create a new angle from turns (full rotations)
    pub const fn from_turns(turns: f64) -> Self {
        Self {
            radians: turns * Self::TAU,
        }
    }

    /// Get the angle in radians
    pub fn radians(self) -> f64 {
        self.radians
    }

    /// Get the angle in degrees
    pub fn degrees(self) -> f64 {
        self.radians * 360.0 / Self::TAU
    }

    /// Get the angle in turns (full rotations)
    pub fn turns(self) -> f64 {
        self.radians / Self::TAU
    }

    /// Normalize angle to [0, τ) range
    pub fn normalized(self) -> Self {
        let normalized = self.radians % Self::TAU;
        let normalized = if normalized < 0.0 {
            normalized + Self::TAU
        } else {
            normalized
        };
        Self { radians: normalized }
    }

    /// Trigonometric functions
    pub fn sin(self) -> f64 {
        self.radians.sin()
    }

    pub fn cos(self) -> f64 {
        self.radians.cos()
    }

    pub fn tan(self) -> f64 {
        self.radians.tan()
    }

    /// Common angle constants
    pub const fn zero() -> Self {
        Self { radians: 0.0 }
    }

    pub const fn quarter_turn() -> Self {
        Self { radians: Self::TAU / 4.0 }  // 90°
    }

    pub const fn half_turn() -> Self {
        Self { radians: Self::TAU / 2.0 }  // 180°
    }

    pub const fn full_turn() -> Self {
        Self { radians: Self::TAU }        // 360°
    }
}

// Arithmetic operations
impl Add<Angle> for Angle {
    type Output = Angle;

    fn add(self, other: Angle) -> Self::Output {
        Angle::from_radians(self.radians + other.radians)
    }
}

impl Sub<Angle> for Angle {
    type Output = Angle;

    fn sub(self, other: Angle) -> Self::Output {
        Angle::from_radians(self.radians - other.radians)
    }
}

impl Neg for Angle {
    type Output = Angle;

    fn neg(self) -> Self::Output {
        Angle::from_radians(-self.radians)
    }
}

// Scalar multiplication/division
impl Mul<f64> for Angle {
    type Output = Angle;

    fn mul(self, scalar: f64) -> Self::Output {
        Angle::from_radians(self.radians * scalar)
    }
}

impl Div<f64> for Angle {
    type Output = Angle;

    fn div(self, scalar: f64) -> Self::Output {
        Angle::from_radians(self.radians / scalar)
    }
}

// Scalar multiplication from the left
impl Mul<Angle> for f64 {
    type Output = Angle;

    fn mul(self, angle: Angle) -> Self::Output {
        angle * self
    }
}

// Trigonometric functions that work with Angle
pub fn sin(angle: Angle) -> f64 {
    angle.sin()
}

pub fn cos(angle: Angle) -> f64 {
    angle.cos()
}

pub fn tan(angle: Angle) -> f64 {
    angle.tan()
}

// Convenience constructors
impl Angle {
    pub fn rad(value: f64) -> Self {
        Self::from_radians(value)
    }

    pub fn deg(value: f64) -> Self {
        Self::from_degrees(value)
    }

    pub fn turn(value: f64) -> Self {
        Self::from_turns(value)
    }
}
