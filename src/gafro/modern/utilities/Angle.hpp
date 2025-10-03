// SPDX-FileCopyrightText: GAFRO Extended Implementation
//
// SPDX-License-Identifier: MPL-2.0

#pragma once

#include <cmath>
#include <type_traits>

namespace gafro::modern::utilities
{
    /**
     * @brief Type-safe angle representation with tau convention
     * 
     * This provides type-safe angle handling using the tau (τ = 2π) convention
     * for more natural geometric calculations.
     */
    class Angle {
    private:
        double radians_;  // Stored in radians using tau convention

    public:
        // Mathematical constants using tau convention
        static constexpr double TAU = 6.283185307179586;  // τ = 2π
        static constexpr double PI = TAU / 2.0;           // π = τ/2

        constexpr Angle() : radians_(0.0) {}
        constexpr explicit Angle(double radians) : radians_(radians) {}

        // Factory methods
        static constexpr Angle from_radians(double radians) {
            return Angle(radians);
        }

        static constexpr Angle from_degrees(double degrees) {
            return Angle(degrees * TAU / 360.0);
        }

        static constexpr Angle from_turns(double turns) {
            return Angle(turns * TAU);
        }

        // Accessors
        constexpr double radians() const { return radians_; }
        constexpr double degrees() const { return radians_ * 360.0 / TAU; }
        constexpr double turns() const { return radians_ / TAU; }

        // Arithmetic operations
        constexpr Angle operator+(const Angle& other) const {
            return Angle(radians_ + other.radians_);
        }

        constexpr Angle operator-(const Angle& other) const {
            return Angle(radians_ - other.radians_);
        }

        constexpr Angle operator-() const {
            return Angle(-radians_);
        }

        constexpr Angle operator+() const {
            return *this;
        }

        // Scalar multiplication/division
        constexpr Angle operator*(double scalar) const {
            return Angle(radians_ * scalar);
        }

        constexpr Angle operator/(double scalar) const {
            return Angle(radians_ / scalar);
        }

        // Comparison operators
        constexpr bool operator==(const Angle& other) const {
            return radians_ == other.radians_;
        }

        constexpr bool operator!=(const Angle& other) const {
            return radians_ != other.radians_;
        }

        constexpr bool operator<(const Angle& other) const {
            return radians_ < other.radians_;
        }

        constexpr bool operator<=(const Angle& other) const {
            return radians_ <= other.radians_;
        }

        constexpr bool operator>(const Angle& other) const {
            return radians_ > other.radians_;
        }

        constexpr bool operator>=(const Angle& other) const {
            return radians_ >= other.radians_;
        }

        // Normalize angle to [0, τ) range
        constexpr Angle normalized() const {
            double normalized = std::fmod(radians_, TAU);
            if (normalized < 0.0) {
                normalized += TAU;
            }
            return Angle(normalized);
        }

        // Trigonometric functions
        double sin() const { return std::sin(radians_); }
        double cos() const { return std::cos(radians_); }
        double tan() const { return std::tan(radians_); }

        // Common angle constants
        static constexpr Angle zero() { return Angle(0.0); }
        static constexpr Angle quarter_turn() { return Angle(TAU / 4.0); }  // 90°
        static constexpr Angle half_turn() { return Angle(TAU / 2.0); }     // 180°
        static constexpr Angle full_turn() { return Angle(TAU); }           // 360°
    };

    // Scalar multiplication from the left
    constexpr Angle operator*(double scalar, const Angle& angle) {
        return angle * scalar;
    }

    // User-defined literals for angles
    namespace literals {
        constexpr Angle operator""_rad(long double value) {
            return Angle::from_radians(static_cast<double>(value));
        }

        constexpr Angle operator""_rad(unsigned long long value) {
            return Angle::from_radians(static_cast<double>(value));
        }

        constexpr Angle operator""_deg(long double value) {
            return Angle::from_degrees(static_cast<double>(value));
        }

        constexpr Angle operator""_deg(unsigned long long value) {
            return Angle::from_degrees(static_cast<double>(value));
        }

        constexpr Angle operator""_turn(long double value) {
            return Angle::from_turns(static_cast<double>(value));
        }

        constexpr Angle operator""_turn(unsigned long long value) {
            return Angle::from_turns(static_cast<double>(value));
        }
    }

    // Trigonometric functions that work with Angle
    inline double sin(const Angle& angle) { return angle.sin(); }
    inline double cos(const Angle& angle) { return angle.cos(); }
    inline double tan(const Angle& angle) { return angle.tan(); }

} // namespace gafro::modern::utilities
