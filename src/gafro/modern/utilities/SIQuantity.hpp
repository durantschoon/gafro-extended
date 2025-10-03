// SPDX-FileCopyrightText: GAFRO Extended Implementation
//
// SPDX-License-Identifier: MPL-2.0

#pragma once

#include <type_traits>
#include <concepts>

namespace gafro::modern::utilities
{
    /**
     * @brief Compile-time dimensional analysis for physical quantities
     * 
     * This provides type-safe physical quantities with compile-time unit checking.
     * Dimensions are specified as template parameters for Mass, Length, and Time.
     * 
     * @tparam M Mass dimension (0 = dimensionless, 1 = kg, etc.)
     * @tparam L Length dimension (0 = dimensionless, 1 = m, etc.)
     * @tparam T Time dimension (0 = dimensionless, 1 = s, etc.)
     */
    template<int M, int L, int T>
    struct SIQuantity {
        double value;
        
        static constexpr int mass_dim = M;
        static constexpr int length_dim = L;
        static constexpr int time_dim = T;

        constexpr SIQuantity() : value(0.0) {}
        constexpr SIQuantity(double v) : value(v) {}

        // Arithmetic operations with dimensional analysis
        template<int M2, int L2, int T2>
        constexpr auto operator*(const SIQuantity<M2, L2, T2>& other) const {
            return SIQuantity<M + M2, L + L2, T + T2>(value * other.value);
        }

        template<int M2, int L2, int T2>
        constexpr auto operator/(const SIQuantity<M2, L2, T2>& other) const {
            return SIQuantity<M - M2, L - L2, T - T2>(value / other.value);
        }

        constexpr SIQuantity operator+(const SIQuantity& other) const {
            return SIQuantity(value + other.value);
        }

        constexpr SIQuantity operator-(const SIQuantity& other) const {
            return SIQuantity(value - other.value);
        }

        constexpr SIQuantity operator-() const {
            return SIQuantity(-value);
        }

        constexpr SIQuantity operator+() const {
            return *this;
        }

        // Scalar operations
        constexpr SIQuantity operator*(double scalar) const {
            return SIQuantity(value * scalar);
        }

        constexpr SIQuantity operator/(double scalar) const {
            return SIQuantity(value / scalar);
        }

        // Comparison operators
        constexpr bool operator==(const SIQuantity& other) const {
            return value == other.value;
        }

        constexpr bool operator!=(const SIQuantity& other) const {
            return value != other.value;
        }

        constexpr bool operator<(const SIQuantity& other) const {
            return value < other.value;
        }

        constexpr bool operator<=(const SIQuantity& other) const {
            return value <= other.value;
        }

        constexpr bool operator>(const SIQuantity& other) const {
            return value > other.value;
        }

        constexpr bool operator>=(const SIQuantity& other) const {
            return value >= other.value;
        }

        // Access the raw value (use with caution)
        constexpr double get_value() const { return value; }
    };

    // Scalar multiplication from the left
    template<int M, int L, int T>
    constexpr SIQuantity<M, L, T> operator*(double scalar, const SIQuantity<M, L, T>& quantity) {
        return quantity * scalar;
    }

    // Common physical quantity type aliases
    using Dimensionless = SIQuantity<0, 0, 0>;
    using Mass = SIQuantity<1, 0, 0>;           // kg
    using Length = SIQuantity<0, 1, 0>;         // m
    using Time = SIQuantity<0, 0, 1>;           // s
    using Velocity = SIQuantity<0, 1, -1>;      // m/s
    using Acceleration = SIQuantity<0, 1, -2>;  // m/s²
    using Force = SIQuantity<1, 1, -2>;         // N (kg⋅m/s²)
    using Energy = SIQuantity<1, 2, -2>;        // J (kg⋅m²/s²)
    using Power = SIQuantity<1, 2, -3>;         // W (kg⋅m²/s³)
    using Pressure = SIQuantity<1, -1, -2>;     // Pa (kg/m⋅s²)
    using Torque = SIQuantity<1, 2, -2>;        // N⋅m (same as Energy dimensionally)

    // User-defined literals for common units
    namespace literals {
        constexpr Mass operator""_kg(long double value) {
            return Mass(static_cast<double>(value));
        }

        constexpr Mass operator""_kg(unsigned long long value) {
            return Mass(static_cast<double>(value));
        }

        constexpr Length operator""_m(long double value) {
            return Length(static_cast<double>(value));
        }

        constexpr Length operator""_m(unsigned long long value) {
            return Length(static_cast<double>(value));
        }

        constexpr Time operator""_s(long double value) {
            return Time(static_cast<double>(value));
        }

        constexpr Time operator""_s(unsigned long long value) {
            return Time(static_cast<double>(value));
        }

        constexpr Velocity operator""_mps(long double value) {
            return Velocity(static_cast<double>(value));
        }

        constexpr Velocity operator""_mps(unsigned long long value) {
            return Velocity(static_cast<double>(value));
        }

        constexpr Force operator""_N(long double value) {
            return Force(static_cast<double>(value));
        }

        constexpr Force operator""_N(unsigned long long value) {
            return Force(static_cast<double>(value));
        }
    }

} // namespace gafro::modern::utilities
