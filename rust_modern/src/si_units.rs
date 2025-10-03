// SPDX-FileCopyrightText: GAFRO Extended Implementation
//
// SPDX-License-Identifier: MPL-2.0

//! SI Unit System for GAFRO Extended
//!
//! This provides compile-time unit checking and conversion for marine robotics
//! applications using Rust's type system and const generics.
//!
//! Mathematical Convention: Uses τ (tau = 2π) instead of π for all angular calculations.

use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use serde::{Deserialize, Serialize};

/// Mathematical constants using tau convention
pub const TAU: f64 = 6.283185307179586; // 2π
pub const PI: f64 = 3.141592653589793;  // π = τ/2

/// Unit dimension representation using const generics
///
/// Dimensions are encoded as [Mass, Length, Time, Current, Temperature, Amount, Luminosity]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dimension<
    const MASS: i8,
    const LENGTH: i8,
    const TIME: i8,
    const CURRENT: i8,
    const TEMPERATURE: i8,
    const AMOUNT: i8,
    const LUMINOSITY: i8,
>;

// Type aliases for base dimensions
pub type Dimensionless = Dimension<0, 0, 0, 0, 0, 0, 0>;
pub type MassDim = Dimension<1, 0, 0, 0, 0, 0, 0>;
pub type LengthDim = Dimension<0, 1, 0, 0, 0, 0, 0>;
pub type TimeDim = Dimension<0, 0, 1, 0, 0, 0, 0>;
pub type CurrentDim = Dimension<0, 0, 0, 1, 0, 0, 0>;
pub type TemperatureDim = Dimension<0, 0, 0, 0, 1, 0, 0>;

// Derived dimensions
pub type VelocityDim = Dimension<0, 1, -1, 0, 0, 0, 0>;     // m/s
pub type AccelerationDim = Dimension<0, 1, -2, 0, 0, 0, 0>; // m/s²
pub type ForceDim = Dimension<1, 1, -2, 0, 0, 0, 0>;        // kg⋅m/s²
pub type EnergyDim = Dimension<1, 2, -2, 0, 0, 0, 0>;       // kg⋅m²/s²
pub type PowerDim = Dimension<1, 2, -3, 0, 0, 0, 0>;        // kg⋅m²/s³
pub type AngularVelocityDim = Dimension<0, 0, -1, 0, 0, 0, 0>; // rad/s (dimensionless/time)

/// Quantity struct with compile-time unit checking
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Quantity<
    T,
    const MASS: i8,
    const LENGTH: i8,
    const TIME: i8,
    const CURRENT: i8,
    const TEMPERATURE: i8,
    const AMOUNT: i8,
    const LUMINOSITY: i8,
> {
    value: T,
    _dimension: PhantomData<Dimension<MASS, LENGTH, TIME, CURRENT, TEMPERATURE, AMOUNT, LUMINOSITY>>,
}

impl<T, const M: i8, const L: i8, const Ti: i8, const C: i8, const Te: i8, const A: i8, const Lu: i8>
    Quantity<T, M, L, Ti, C, Te, A, Lu>
{
    /// Create a new quantity with the given value
    pub const fn new(value: T) -> Self {
        Self {
            value,
            _dimension: PhantomData,
        }
    }

    /// Get the value of this quantity
    pub const fn value(&self) -> &T {
        &self.value
    }

    /// Get the mutable value of this quantity
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Consume this quantity and return its value
    pub fn into_value(self) -> T {
        self.value
    }

    /// Check if this quantity is dimensionless
    pub const fn is_dimensionless() -> bool {
        M == 0 && L == 0 && Ti == 0 && C == 0 && Te == 0 && A == 0 && Lu == 0
    }
}

// Implement From<T> for dimensionless quantities
impl<T> From<T> for Quantity<T, 0, 0, 0, 0, 0, 0, 0> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

// Arithmetic operations for same dimensions
impl<T, const M: i8, const L: i8, const Ti: i8, const C: i8, const Te: i8, const A: i8, const Lu: i8>
    Add for Quantity<T, M, L, Ti, C, Te, A, Lu>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}

impl<T, const M: i8, const L: i8, const Ti: i8, const C: i8, const Te: i8, const A: i8, const Lu: i8>
    Sub for Quantity<T, M, L, Ti, C, Te, A, Lu>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.value - rhs.value)
    }
}

// Scalar multiplication and division
impl<T, S, const M: i8, const L: i8, const Ti: i8, const C: i8, const Te: i8, const A: i8, const Lu: i8>
    Mul<S> for Quantity<T, M, L, Ti, C, Te, A, Lu>
where
    T: Mul<S, Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: S) -> Self::Output {
        Self::new(self.value * rhs)
    }
}

impl<T, S, const M: i8, const L: i8, const Ti: i8, const C: i8, const Te: i8, const A: i8, const Lu: i8>
    Div<S> for Quantity<T, M, L, Ti, C, Te, A, Lu>
where
    T: Div<S, Output = T>,
{
    type Output = Self;

    fn div(self, rhs: S) -> Self::Output {
        Self::new(self.value / rhs)
    }
}

// Quantity multiplication (dimension addition)
impl<
    T1, T2,
    const M1: i8, const L1: i8, const Ti1: i8, const C1: i8, const Te1: i8, const A1: i8, const Lu1: i8,
    const M2: i8, const L2: i8, const Ti2: i8, const C2: i8, const Te2: i8, const A2: i8, const Lu2: i8,
> Mul<Quantity<T2, M2, L2, Ti2, C2, Te2, A2, Lu2>>
    for Quantity<T1, M1, L1, Ti1, C1, Te1, A1, Lu1>
where
    T1: Mul<T2>,
{
    type Output = Quantity<
        <T1 as Mul<T2>>::Output,
        { M1 + M2 },
        { L1 + L2 },
        { Ti1 + Ti2 },
        { C1 + C2 },
        { Te1 + Te2 },
        { A1 + A2 },
        { Lu1 + Lu2 },
    >;

    fn mul(self, rhs: Quantity<T2, M2, L2, Ti2, C2, Te2, A2, Lu2>) -> Self::Output {
        Quantity::new(self.value * rhs.value)
    }
}

// Quantity division (dimension subtraction)
impl<
    T1, T2,
    const M1: i8, const L1: i8, const Ti1: i8, const C1: i8, const Te1: i8, const A1: i8, const Lu1: i8,
    const M2: i8, const L2: i8, const Ti2: i8, const C2: i8, const Te2: i8, const A2: i8, const Lu2: i8,
> Div<Quantity<T2, M2, L2, Ti2, C2, Te2, A2, Lu2>>
    for Quantity<T1, M1, L1, Ti1, C1, Te1, A1, Lu1>
where
    T1: Div<T2>,
{
    type Output = Quantity<
        <T1 as Div<T2>>::Output,
        { M1 - M2 },
        { L1 - L2 },
        { Ti1 - Ti2 },
        { C1 - C2 },
        { Te1 - Te2 },
        { A1 - A2 },
        { Lu1 - Lu2 },
    >;

    fn div(self, rhs: Quantity<T2, M2, L2, Ti2, C2, Te2, A2, Lu2>) -> Self::Output {
        Quantity::new(self.value / rhs.value)
    }
}

// Comparison operations
impl<T, const M: i8, const L: i8, const Ti: i8, const C: i8, const Te: i8, const A: i8, const Lu: i8>
    PartialOrd for Quantity<T, M, L, Ti, C, Te, A, Lu>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

// Unary operations
impl<T, const M: i8, const L: i8, const Ti: i8, const C: i8, const Te: i8, const A: i8, const Lu: i8>
    Neg for Quantity<T, M, L, Ti, C, Te, A, Lu>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.value)
    }
}

/// Type aliases for common quantities
pub type DimensionlessQ<T = f64> = Quantity<T, 0, 0, 0, 0, 0, 0, 0>;
pub type Mass<T = f64> = Quantity<T, 1, 0, 0, 0, 0, 0, 0>;
pub type Length<T = f64> = Quantity<T, 0, 1, 0, 0, 0, 0, 0>;
pub type Time<T = f64> = Quantity<T, 0, 0, 1, 0, 0, 0, 0>;
pub type Velocity<T = f64> = Quantity<T, 0, 1, -1, 0, 0, 0, 0>;
pub type Acceleration<T = f64> = Quantity<T, 0, 1, -2, 0, 0, 0, 0>;
pub type Force<T = f64> = Quantity<T, 1, 1, -2, 0, 0, 0, 0>;
pub type Energy<T = f64> = Quantity<T, 1, 2, -2, 0, 0, 0, 0>;
pub type Power<T = f64> = Quantity<T, 1, 2, -3, 0, 0, 0, 0>;
pub type AngularVelocity<T = f64> = Quantity<T, 0, 0, -1, 0, 0, 0, 0>;

/// Unit construction functions
pub mod units {
    use super::*;

    // Length units
    pub fn meters<T>(value: T) -> Length<T> {
        Length::new(value)
    }

    pub fn centimeters<T>(value: T) -> Length<T>
    where
        T: Mul<f64, Output = T>,
    {
        Length::new(value * 0.01)
    }

    pub fn millimeters<T>(value: T) -> Length<T>
    where
        T: Mul<f64, Output = T>,
    {
        Length::new(value * 0.001)
    }

    pub fn kilometers<T>(value: T) -> Length<T>
    where
        T: Mul<f64, Output = T>,
    {
        Length::new(value * 1000.0)
    }

    // Time units
    pub fn seconds<T>(value: T) -> Time<T> {
        Time::new(value)
    }

    pub fn milliseconds<T>(value: T) -> Time<T>
    where
        T: Mul<f64, Output = T>,
    {
        Time::new(value * 0.001)
    }

    pub fn minutes<T>(value: T) -> Time<T>
    where
        T: Mul<f64, Output = T>,
    {
        Time::new(value * 60.0)
    }

    pub fn hours<T>(value: T) -> Time<T>
    where
        T: Mul<f64, Output = T>,
    {
        Time::new(value * 3600.0)
    }

    // Mass units
    pub fn kilograms<T>(value: T) -> Mass<T> {
        Mass::new(value)
    }

    pub fn grams<T>(value: T) -> Mass<T>
    where
        T: Mul<f64, Output = T>,
    {
        Mass::new(value * 0.001)
    }

    pub fn tons<T>(value: T) -> Mass<T>
    where
        T: Mul<f64, Output = T>,
    {
        Mass::new(value * 1000.0)
    }

    // Velocity units
    pub fn meters_per_second<T>(value: T) -> Velocity<T> {
        Velocity::new(value)
    }

    pub fn kilometers_per_hour<T>(value: T) -> Velocity<T>
    where
        T: Div<f64, Output = T>,
    {
        Velocity::new(value / 3.6)
    }

    pub fn knots<T>(value: T) -> Velocity<T>
    where
        T: Mul<f64, Output = T>,
    {
        Velocity::new(value * 0.514444)
    }

    // Force units
    pub fn newtons<T>(value: T) -> Force<T> {
        Force::new(value)
    }

    pub fn kilonewtons<T>(value: T) -> Force<T>
    where
        T: Mul<f64, Output = T>,
    {
        Force::new(value * 1000.0)
    }

    // Energy units
    pub fn joules<T>(value: T) -> Energy<T> {
        Energy::new(value)
    }

    pub fn kilojoules<T>(value: T) -> Energy<T>
    where
        T: Mul<f64, Output = T>,
    {
        Energy::new(value * 1000.0)
    }

    pub fn watt_hours<T>(value: T) -> Energy<T>
    where
        T: Mul<f64, Output = T>,
    {
        Energy::new(value * 3600.0)
    }

    pub fn kilowatt_hours<T>(value: T) -> Energy<T>
    where
        T: Mul<f64, Output = T>,
    {
        Energy::new(value * 3600000.0)
    }

    // Power units
    pub fn watts<T>(value: T) -> Power<T> {
        Power::new(value)
    }

    pub fn kilowatts<T>(value: T) -> Power<T>
    where
        T: Mul<f64, Output = T>,
    {
        Power::new(value * 1000.0)
    }

    pub fn horsepower<T>(value: T) -> Power<T>
    where
        T: Mul<f64, Output = T>,
    {
        Power::new(value * 745.7)
    }

    // Angular units (using tau convention)
    pub fn radians<T>(value: T) -> DimensionlessQ<T> {
        DimensionlessQ::new(value)
    }

    pub fn degrees<T>(value: T) -> DimensionlessQ<T>
    where
        T: Mul<f64, Output = T>,
    {
        DimensionlessQ::new(value * TAU / 360.0)
    }

    pub fn turns<T>(value: T) -> DimensionlessQ<T>
    where
        T: Mul<f64, Output = T>,
    {
        DimensionlessQ::new(value * TAU)
    }

    // Angular velocity units
    pub fn radians_per_second<T>(value: T) -> AngularVelocity<T> {
        AngularVelocity::new(value)
    }

    pub fn rpm<T>(value: T) -> AngularVelocity<T>
    where
        T: Mul<f64, Output = T>,
    {
        AngularVelocity::new(value * TAU / 60.0)
    }
}

/// Mathematical functions with units
pub mod math {
    use super::*;

    /// Trigonometric functions (dimensionless input)
    pub fn sin<T>(angle: DimensionlessQ<T>) -> T
    where
        T: Into<f64>,
        f64: Into<T>,
    {
        let angle_f64: f64 = angle.into_value().into();
        angle_f64.sin().into()
    }

    pub fn cos<T>(angle: DimensionlessQ<T>) -> T
    where
        T: Into<f64>,
        f64: Into<T>,
    {
        let angle_f64: f64 = angle.into_value().into();
        angle_f64.cos().into()
    }

    pub fn tan<T>(angle: DimensionlessQ<T>) -> T
    where
        T: Into<f64>,
        f64: Into<T>,
    {
        let angle_f64: f64 = angle.into_value().into();
        angle_f64.tan().into()
    }

    /// Square root (requires even dimension powers - simplified version)
    pub fn sqrt<T>(quantity: Quantity<T, 0, 2, 0, 0, 0, 0, 0>) -> Length<T>
    where
        T: Into<f64>,
        f64: Into<T>,
    {
        let value_f64: f64 = quantity.into_value().into();
        Length::new(value_f64.sqrt().into())
    }

    /// Absolute value
    pub fn abs<T, const M: i8, const L: i8, const Ti: i8, const C: i8, const Te: i8, const A: i8, const Lu: i8>(
        quantity: Quantity<T, M, L, Ti, C, Te, A, Lu>,
    ) -> Quantity<T, M, L, Ti, C, Te, A, Lu>
    where
        T: Into<f64>,
        f64: Into<T>,
    {
        let value_f64: f64 = quantity.into_value().into();
        Quantity::new(value_f64.abs().into())
    }
}

/// Conversion utilities
pub mod convert {
    use super::*;

    /// Convert degrees to radians using tau convention
    pub fn degrees_to_radians<T>(degrees: T) -> DimensionlessQ<T>
    where
        T: Mul<f64, Output = T>,
    {
        DimensionlessQ::new(degrees * TAU / 360.0)
    }

    /// Convert radians to degrees using tau convention
    pub fn radians_to_degrees<T>(radians: DimensionlessQ<T>) -> T
    where
        T: Mul<f64, Output = T>,
    {
        radians.into_value() * 360.0 / TAU
    }

    /// Convert knots to m/s
    pub fn knots_to_mps<T>(knots: T) -> Velocity<T>
    where
        T: Mul<f64, Output = T>,
    {
        Velocity::new(knots * 0.514444)
    }

    /// Convert m/s to knots
    pub fn mps_to_knots<T>(velocity: Velocity<T>) -> T
    where
        T: Div<f64, Output = T>,
    {
        velocity.into_value() / 0.514444
    }
}

/// Marine robotics specific quantities and constants
pub mod marine {
    use super::*;

    /// Water density at standard conditions (kg/m³)
    pub fn water_density<T>() -> Quantity<T, 1, -3, 0, 0, 0, 0, 0>
    where
        T: From<f64>,
    {
        Quantity::new(T::from(1025.0))
    }

    /// Standard gravity (m/s²)
    pub fn gravity<T>() -> Acceleration<T>
    where
        T: From<f64>,
    {
        Acceleration::new(T::from(9.81))
    }

    /// Atmospheric pressure at sea level (Pa)
    pub fn atmospheric_pressure<T>() -> Quantity<T, 1, -1, -2, 0, 0, 0, 0>
    where
        T: From<f64>,
    {
        Quantity::new(T::from(101325.0))
    }

    /// Calculate buoyancy force
    pub fn buoyancy_force<T>(volume: Quantity<T, 0, 3, 0, 0, 0, 0, 0>) -> Force<T>
    where
        T: Mul<T, Output = T> + From<f64>,
    {
        water_density::<T>() * gravity::<T>() * volume
    }

    /// Calculate hydrostatic pressure at depth
    pub fn pressure_at_depth<T>(depth: Length<T>) -> Quantity<T, 1, -1, -2, 0, 0, 0, 0>
    where
        T: Add<T, Output = T> + Mul<T, Output = T> + From<f64>,
    {
        atmospheric_pressure::<T>() + (water_density::<T>() * gravity::<T>() * depth)
    }
}

/// Extension trait for numeric types to add unit methods
pub trait UnitExt<T> {
    // Length
    fn meters(self) -> Length<T>;
    fn centimeters(self) -> Length<T>;
    fn millimeters(self) -> Length<T>;
    fn kilometers(self) -> Length<T>;

    // Time
    fn seconds(self) -> Time<T>;
    fn milliseconds(self) -> Time<T>;
    fn minutes(self) -> Time<T>;
    fn hours(self) -> Time<T>;

    // Mass
    fn kilograms(self) -> Mass<T>;
    fn grams(self) -> Mass<T>;
    fn tons(self) -> Mass<T>;

    // Angular (tau convention)
    fn radians(self) -> DimensionlessQ<T>;
    fn degrees(self) -> DimensionlessQ<T>;
    fn turns(self) -> DimensionlessQ<T>;
}

impl UnitExt<f64> for f64 {
    fn meters(self) -> Length<f64> { units::meters(self) }
    fn centimeters(self) -> Length<f64> { units::centimeters(self) }
    fn millimeters(self) -> Length<f64> { units::millimeters(self) }
    fn kilometers(self) -> Length<f64> { units::kilometers(self) }

    fn seconds(self) -> Time<f64> { units::seconds(self) }
    fn milliseconds(self) -> Time<f64> { units::milliseconds(self) }
    fn minutes(self) -> Time<f64> { units::minutes(self) }
    fn hours(self) -> Time<f64> { units::hours(self) }

    fn kilograms(self) -> Mass<f64> { units::kilograms(self) }
    fn grams(self) -> Mass<f64> { units::grams(self) }
    fn tons(self) -> Mass<f64> { units::tons(self) }

    fn radians(self) -> DimensionlessQ<f64> { units::radians(self) }
    fn degrees(self) -> DimensionlessQ<f64> { units::degrees(self) }
    fn turns(self) -> DimensionlessQ<f64> { units::turns(self) }
}

impl UnitExt<f32> for f32 {
    fn meters(self) -> Length<f32> { units::meters(self) }
    fn centimeters(self) -> Length<f32> { units::centimeters(self) }
    fn millimeters(self) -> Length<f32> { units::millimeters(self) }
    fn kilometers(self) -> Length<f32> { units::kilometers(self) }

    fn seconds(self) -> Time<f32> { units::seconds(self) }
    fn milliseconds(self) -> Time<f32> { units::milliseconds(self) }
    fn minutes(self) -> Time<f32> { units::minutes(self) }
    fn hours(self) -> Time<f32> { units::hours(self) }

    fn kilograms(self) -> Mass<f32> { units::kilograms(self) }
    fn grams(self) -> Mass<f32> { units::grams(self) }
    fn tons(self) -> Mass<f32> { units::tons(self) }

    fn radians(self) -> DimensionlessQ<f32> { units::radians(self) }
    fn degrees(self) -> DimensionlessQ<f32> { units::degrees(self) }
    fn turns(self) -> DimensionlessQ<f32> { units::turns(self) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_units() {
        let length = units::meters(5.0);
        let time = units::seconds(2.0);
        let velocity = length / time;

        assert_eq!(*velocity.value(), 2.5);
    }

    #[test]
    fn test_unit_arithmetic() {
        let l1 = units::meters(3.0);
        let l2 = units::meters(4.0);
        let sum = l1 + l2;

        assert_eq!(*sum.value(), 7.0);

        let area = l1 * l2;
        assert_eq!(*area.value(), 12.0);
    }

    #[test]
    fn test_unit_conversions() {
        let angle_deg = units::degrees(90.0);
        let angle_rad = convert::degrees_to_radians(90.0);

        // 90 degrees should be τ/4 radians
        assert!((angle_rad.value() - TAU / 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_marine_calculations() {
        let volume = units::meters(1.0) * units::meters(1.0) * units::meters(1.0);
        let buoyancy = marine::buoyancy_force(volume);

        // Should be approximately 1025 * 9.81 = 10055.25 N
        assert!((*buoyancy.value() - 10055.25).abs() < 0.1);

        let depth = units::meters(10.0);
        let pressure = marine::pressure_at_depth(depth);

        // Should be atmospheric + 10 * 1025 * 9.81
        let expected = 101325.0 + 10.0 * 1025.0 * 9.81;
        assert!((*pressure.value() - expected).abs() < 1.0);
    }

    #[test]
    fn test_extension_trait() {
        let length = 5.0.meters();
        let time = 2.0.seconds();
        let velocity = length / time;

        assert_eq!(*velocity.value(), 2.5);

        let angle = 180.0.degrees();
        assert!((angle.value() - TAU / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_tau_convention() {
        // Full circle should be τ radians
        let full_circle = 1.0.turns();
        assert!((full_circle.value() - TAU).abs() < 1e-10);

        // Half circle should be τ/2 radians (traditional π)
        let half_circle = 0.5.turns();
        assert!((half_circle.value() - PI).abs() < 1e-10);

        // 90 degrees should be τ/4 radians
        let quarter_circle = 90.0.degrees();
        assert!((quarter_circle.value() - TAU / 4.0).abs() < 1e-10);
    }
}