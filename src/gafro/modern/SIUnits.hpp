// SPDX-FileCopyrightText: GAFRO Extended Implementation
//
// SPDX-License-Identifier: MPL-2.0

#pragma once

#include <type_traits>
#include <concepts>
#include <ratio>
#include <string>
#include <cmath>

namespace gafro::modern::units
{
    /**
     * @brief SI Unit System for GAFRO Extended
     *
     * This provides compile-time unit checking and conversion for marine robotics
     * applications. All units are checked at compile time to prevent unit errors.
     *
     * Mathematical Convention: Uses τ (tau = 2π) instead of π for all angular calculations.
     */

    // Mathematical constants using tau convention
    constexpr double TAU = 6.283185307179586; // τ = 2π (full rotation)

    /**
     * @brief Unit dimension representation using compile-time rationals
     */
    template<int Mass, int Length, int Time, int Current, int Temperature, int Amount, int Luminosity>
    struct Dimension {
        static constexpr int mass = Mass;
        static constexpr int length = Length;
        static constexpr int time = Time;
        static constexpr int current = Current;
        static constexpr int temperature = Temperature;
        static constexpr int amount = Amount;
        static constexpr int luminosity = Luminosity;

        // Helper for dimension arithmetic
        template<int M2, int L2, int T2, int C2, int Te2, int A2, int Lu2>
        using Multiply = Dimension<Mass + M2, Length + L2, Time + T2, Current + C2, Temperature + Te2, Amount + A2, Luminosity + Lu2>;

        template<int M2, int L2, int T2, int C2, int Te2, int A2, int Lu2>
        using Divide = Dimension<Mass - M2, Length - L2, Time - T2, Current - C2, Temperature - Te2, Amount - A2, Luminosity - Lu2>;
    };

    // Base dimensions
    using Dimensionless = Dimension<0, 0, 0, 0, 0, 0, 0>;
    using MassDim = Dimension<1, 0, 0, 0, 0, 0, 0>;
    using LengthDim = Dimension<0, 1, 0, 0, 0, 0, 0>;
    using TimeDim = Dimension<0, 0, 1, 0, 0, 0, 0>;
    using CurrentDim = Dimension<0, 0, 0, 1, 0, 0, 0>;
    using TemperatureDim = Dimension<0, 0, 0, 0, 1, 0, 0>;

    // Derived dimensions
    using VelocityDim = LengthDim::Divide<0, 0, 1, 0, 0, 0, 0>;      // m/s
    using AccelerationDim = VelocityDim::Divide<0, 0, 1, 0, 0, 0, 0>; // m/s²
    using ForceDim = MassDim::Multiply<0, 1, -2, 0, 0, 0, 0>;        // kg⋅m/s²
    using EnergyDim = ForceDim::Multiply<0, 1, 0, 0, 0, 0, 0>;       // kg⋅m²/s²
    using PowerDim = EnergyDim::Divide<0, 0, 1, 0, 0, 0, 0>;         // kg⋅m²/s³
    using AngularVelocityDim = Dimensionless::Divide<0, 0, 1, 0, 0, 0, 0>; // rad/s (dimensionless)

    /**
     * @brief Quantity class template with compile-time unit checking
     */
    template<typename ValueType, typename Dim>
    class Quantity {
    public:
        using value_type = ValueType;
        using dimension = Dim;

        constexpr Quantity() : value_(ValueType{}) {}
        constexpr explicit Quantity(const ValueType& value) : value_(value) {}

        // Copy and move semantics
        constexpr Quantity(const Quantity& other) = default;
        constexpr Quantity(Quantity&& other) = default;
        constexpr Quantity& operator=(const Quantity& other) = default;
        constexpr Quantity& operator=(Quantity&& other) = default;

        // Access
        constexpr const ValueType& value() const { return value_; }
        constexpr ValueType& value() { return value_; }

        // Implicit conversion to value type (for dimensionless quantities)
        constexpr operator ValueType() const requires std::same_as<Dim, Dimensionless> {
            return value_;
        }

        // Arithmetic operations (same dimension)
        constexpr Quantity operator+(const Quantity& other) const {
            return Quantity(value_ + other.value_);
        }

        constexpr Quantity operator-(const Quantity& other) const {
            return Quantity(value_ - other.value_);
        }

        constexpr Quantity& operator+=(const Quantity& other) {
            value_ += other.value_;
            return *this;
        }

        constexpr Quantity& operator-=(const Quantity& other) {
            value_ -= other.value_;
            return *this;
        }

        // Scalar multiplication/division
        constexpr Quantity operator*(const ValueType& scalar) const {
            return Quantity(value_ * scalar);
        }

        constexpr Quantity operator/(const ValueType& scalar) const {
            return Quantity(value_ / scalar);
        }

        constexpr Quantity& operator*=(const ValueType& scalar) {
            value_ *= scalar;
            return *this;
        }

        constexpr Quantity& operator/=(const ValueType& scalar) {
            value_ /= scalar;
            return *this;
        }

        // Comparison operators
        constexpr bool operator==(const Quantity& other) const {
            return value_ == other.value_;
        }

        constexpr bool operator!=(const Quantity& other) const {
            return value_ != other.value_;
        }

        constexpr bool operator<(const Quantity& other) const {
            return value_ < other.value_;
        }

        constexpr bool operator<=(const Quantity& other) const {
            return value_ <= other.value_;
        }

        constexpr bool operator>(const Quantity& other) const {
            return value_ > other.value_;
        }

        constexpr bool operator>=(const Quantity& other) const {
            return value_ >= other.value_;
        }

        // Unary operators
        constexpr Quantity operator-() const {
            return Quantity(-value_);
        }

        constexpr Quantity operator+() const {
            return *this;
        }

    private:
        ValueType value_;
    };

    // Multiplication of quantities (dimension addition)
    template<typename T1, typename T2, typename D1, typename D2>
    constexpr auto operator*(const Quantity<T1, D1>& lhs, const Quantity<T2, D2>& rhs) {
        using ResultDim = typename D1::template Multiply<D2::mass, D2::length, D2::time, D2::current, D2::temperature, D2::amount, D2::luminosity>;
        using ResultType = decltype(lhs.value() * rhs.value());
        return Quantity<ResultType, ResultDim>(lhs.value() * rhs.value());
    }

    // Division of quantities (dimension subtraction)
    template<typename T1, typename T2, typename D1, typename D2>
    constexpr auto operator/(const Quantity<T1, D1>& lhs, const Quantity<T2, D2>& rhs) {
        using ResultDim = typename D1::template Divide<D2::mass, D2::length, D2::time, D2::current, D2::temperature, D2::amount, D2::luminosity>;
        using ResultType = decltype(lhs.value() / rhs.value());
        return Quantity<ResultType, ResultDim>(lhs.value() / rhs.value());
    }

    // Scalar * quantity
    template<typename S, typename T, typename D>
        requires std::is_arithmetic_v<S>
    constexpr auto operator*(const S& scalar, const Quantity<T, D>& quantity) {
        return quantity * scalar;
    }

    /**
     * @brief Type aliases for common quantities
     */
    using Dimensionless_t = Quantity<double, Dimensionless>;
    using Mass = Quantity<double, MassDim>;
    using Length = Quantity<double, LengthDim>;
    using Time = Quantity<double, TimeDim>;
    using Velocity = Quantity<double, VelocityDim>;
    using Acceleration = Quantity<double, AccelerationDim>;
    using Force = Quantity<double, ForceDim>;
    using Energy = Quantity<double, EnergyDim>;
    using Power = Quantity<double, PowerDim>;
    using AngularVelocity = Quantity<double, AngularVelocityDim>;

    /**
     * @brief Unit literals for marine robotics
     */
    namespace literals {

        // Length units
        constexpr Length operator""_m(long double value) {
            return Length(static_cast<double>(value));
        }

        constexpr Length operator""_m(unsigned long long value) {
            return Length(static_cast<double>(value));
        }

        constexpr Length operator""_cm(long double value) {
            return Length(static_cast<double>(value) * 0.01);
        }

        constexpr Length operator""_mm(long double value) {
            return Length(static_cast<double>(value) * 0.001);
        }

        constexpr Length operator""_km(long double value) {
            return Length(static_cast<double>(value) * 1000.0);
        }

        // Time units
        constexpr Time operator""_s(long double value) {
            return Time(static_cast<double>(value));
        }

        constexpr Time operator""_s(unsigned long long value) {
            return Time(static_cast<double>(value));
        }

        constexpr Time operator""_ms(long double value) {
            return Time(static_cast<double>(value) * 0.001);
        }

        constexpr Time operator""_min(long double value) {
            return Time(static_cast<double>(value) * 60.0);
        }

        constexpr Time operator""_h(long double value) {
            return Time(static_cast<double>(value) * 3600.0);
        }

        // Mass units
        constexpr Mass operator""_kg(long double value) {
            return Mass(static_cast<double>(value));
        }

        constexpr Mass operator""_g(long double value) {
            return Mass(static_cast<double>(value) * 0.001);
        }

        constexpr Mass operator""_ton(long double value) {
            return Mass(static_cast<double>(value) * 1000.0);
        }

        // Velocity units
        constexpr Velocity operator""_mps(long double value) {
            return Velocity(static_cast<double>(value));
        }

        constexpr Velocity operator""_kmph(long double value) {
            return Velocity(static_cast<double>(value) / 3.6);
        }

        constexpr Velocity operator""_knots(long double value) {
            return Velocity(static_cast<double>(value) * 0.514444);
        }

        // Force units
        constexpr Force operator""_N(long double value) {
            return Force(static_cast<double>(value));
        }

        constexpr Force operator""_kN(long double value) {
            return Force(static_cast<double>(value) * 1000.0);
        }

        // Energy units
        constexpr Energy operator""_J(long double value) {
            return Energy(static_cast<double>(value));
        }

        constexpr Energy operator""_kJ(long double value) {
            return Energy(static_cast<double>(value) * 1000.0);
        }

        constexpr Energy operator""_Wh(long double value) {
            return Energy(static_cast<double>(value) * 3600.0);
        }

        constexpr Energy operator""_kWh(long double value) {
            return Energy(static_cast<double>(value) * 3600000.0);
        }

        // Power units
        constexpr Power operator""_W(long double value) {
            return Power(static_cast<double>(value));
        }

        constexpr Power operator""_kW(long double value) {
            return Power(static_cast<double>(value) * 1000.0);
        }

        constexpr Power operator""_hp(long double value) {
            return Power(static_cast<double>(value) * 745.7);
        }

        // Angular units (using tau convention)
        constexpr Dimensionless_t operator""_rad(long double value) {
            return Dimensionless_t(static_cast<double>(value));
        }

        constexpr Dimensionless_t operator""_deg(long double value) {
            return Dimensionless_t(static_cast<double>(value) * TAU / 360.0);
        }

        constexpr Dimensionless_t operator""_turn(long double value) {
            return Dimensionless_t(static_cast<double>(value) * TAU);
        }

        // Angular velocity units
        constexpr AngularVelocity operator""_radps(long double value) {
            return AngularVelocity(static_cast<double>(value));
        }

        constexpr AngularVelocity operator""_rpm(long double value) {
            return AngularVelocity(static_cast<double>(value) * TAU / 60.0);
        }

    } // namespace literals

    /**
     * @brief Mathematical functions with units
     */
    namespace math {

        // Trigonometric functions (dimensionless input)
        template<typename T>
        constexpr T sin(const Quantity<T, Dimensionless>& angle) {
            return std::sin(angle.value());
        }

        template<typename T>
        constexpr T cos(const Quantity<T, Dimensionless>& angle) {
            return std::cos(angle.value());
        }

        template<typename T>
        constexpr T tan(const Quantity<T, Dimensionless>& angle) {
            return std::tan(angle.value());
        }

        // Square root (dimension must be even powers)
        template<typename T, int M, int L, int Ti, int C, int Te, int A, int Lu>
            requires ((M % 2 == 0) && (L % 2 == 0) && (Ti % 2 == 0) &&
                     (C % 2 == 0) && (Te % 2 == 0) && (A % 2 == 0) && (Lu % 2 == 0))
        constexpr auto sqrt(const Quantity<T, Dimension<M, L, Ti, C, Te, A, Lu>>& quantity) {
            using ResultDim = Dimension<M/2, L/2, Ti/2, C/2, Te/2, A/2, Lu/2>;
            return Quantity<T, ResultDim>(std::sqrt(quantity.value()));
        }

        // Power functions
        template<typename T, typename D, int N>
        constexpr auto pow(const Quantity<T, D>& quantity) {
            using ResultDim = Dimension<D::mass * N, D::length * N, D::time * N,
                                      D::current * N, D::temperature * N, D::amount * N, D::luminosity * N>;
            return Quantity<T, ResultDim>(std::pow(quantity.value(), N));
        }

        // Absolute value
        template<typename T, typename D>
        constexpr auto abs(const Quantity<T, D>& quantity) {
            return Quantity<T, D>(std::abs(quantity.value()));
        }

    } // namespace math

    /**
     * @brief Conversion utilities
     */
    namespace convert {

        // Convert degrees to radians using tau convention
        constexpr Dimensionless_t degrees_to_radians(double degrees) {
            return Dimensionless_t(degrees * TAU / 360.0);
        }

        // Convert radians to degrees using tau convention
        constexpr double radians_to_degrees(const Dimensionless_t& radians) {
            return radians.value() * 360.0 / TAU;
        }

        // Convert knots to m/s
        constexpr Velocity knots_to_mps(double knots) {
            return Velocity(knots * 0.514444);
        }

        // Convert m/s to knots
        constexpr double mps_to_knots(const Velocity& velocity) {
            return velocity.value() / 0.514444;
        }

    } // namespace convert

    /**
     * @brief Marine robotics specific quantities
     */
    namespace marine {

        // Water density at standard conditions
        constexpr auto WATER_DENSITY = 1025.0_kg / (1.0_m * 1.0_m * 1.0_m); // kg/m³

        // Standard gravity
        constexpr auto GRAVITY = 9.81_mps / 1.0_s; // m/s²

        // Atmospheric pressure at sea level
        constexpr auto ATMOSPHERIC_PRESSURE = 101325.0_N / (1.0_m * 1.0_m); // Pa

        // Pressure per meter of depth
        constexpr auto PRESSURE_PER_METER = WATER_DENSITY * GRAVITY; // Pa/m

        // Calculate buoyancy force
        template<typename VolumeType>
        constexpr auto buoyancy_force(const VolumeType& volume) {
            return WATER_DENSITY * GRAVITY * volume;
        }

        // Calculate hydrostatic pressure at depth
        constexpr auto pressure_at_depth(const Length& depth) {
            return ATMOSPHERIC_PRESSURE + PRESSURE_PER_METER * depth;
        }

    } // namespace marine

} // namespace gafro::modern::units