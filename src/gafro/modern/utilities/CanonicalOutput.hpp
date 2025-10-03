// SPDX-FileCopyrightText: GAFRO Extended Implementation
//
// SPDX-License-Identifier: MPL-2.0

#pragma once

#include <iostream>
#include <iomanip>
#include <sstream>
#include <string>
#include <vector>
#include <tuple>
#include <type_traits>
#include <concepts>

// Concept for types that can be printed as positions
template<typename T>
concept PositionLike = requires(const T& pos) {
    { pos.x } -> std::convertible_to<double>;
    { pos.y } -> std::convertible_to<double>;
    { pos.z } -> std::convertible_to<double>;
    { T::frame_name() } -> std::convertible_to<const char*>;
};

namespace gafro::modern::utilities
{
    /**
     * @brief Canonical Output Library for GAFRO Extended
     * 
     * This library provides consistent, configurable output formatting
     * for both C++ and Rust implementations to ensure identical output.
     */
    
    class CanonicalOutput {
    public:
        // Configuration for output precision and formatting
        struct Config {
            int position_precision = 1;      // Decimal places for positions
            int angle_precision = 0;         // Decimal places for angles
            int distance_precision = 1;      // Decimal places for distances
            int time_precision = 1;          // Decimal places for time
            int speed_precision = 2;         // Decimal places for speed
            int scientific_threshold = 100;  // Use scientific notation above this
            bool use_tau_convention = true;  // Use τ instead of π
        };
        
        static Config& config() {
            static Config cfg;
            return cfg;
        }
        
        // Position formatting
        template<typename T>
        static std::string position(T x, T y, T z) {
            auto& cfg = config();
            std::ostringstream oss;
            oss << "(" 
                << std::fixed << std::setprecision(cfg.position_precision) << x << ", "
                << std::fixed << std::setprecision(cfg.position_precision) << y << ", "
                << std::fixed << std::setprecision(cfg.position_precision) << z << ")";
            return oss.str();
        }
        
        // Distance formatting
        template<typename T>
        static std::string distance(T value, const std::string& unit = "m") {
            auto& cfg = config();
            std::ostringstream oss;
            if (std::abs(value) >= cfg.scientific_threshold) {
                oss << std::scientific << std::setprecision(cfg.distance_precision) << value << " " << unit;
            } else {
                oss << std::fixed << std::setprecision(cfg.distance_precision) << value << " " << unit;
            }
            return oss.str();
        }
        
        // Angle formatting
        template<typename T>
        static std::string angle_degrees(T degrees) {
            auto& cfg = config();
            std::ostringstream oss;
            oss << std::fixed << std::setprecision(cfg.angle_precision) << degrees << "°";
            return oss.str();
        }
        
        template<typename T>
        static std::string angle_tau(T tau_fraction) {
            auto& cfg = config();
            std::ostringstream oss;
            oss << std::fixed << std::setprecision(cfg.angle_precision) << tau_fraction << "τ";
            return oss.str();
        }
        
        template<typename T>
        static std::string angle_combined(T degrees, T tau_fraction) {
            return angle_degrees(degrees) + " (" + angle_tau(tau_fraction) + ")";
        }
        
        // Time formatting
        template<typename T>
        static std::string time(T value, const std::string& unit = "s") {
            auto& cfg = config();
            std::ostringstream oss;
            oss << std::fixed << std::setprecision(cfg.time_precision) << value << " " << unit;
            return oss.str();
        }
        
        // Speed formatting
        template<typename T>
        static std::string speed(T value, const std::string& unit = "m/s") {
            auto& cfg = config();
            std::ostringstream oss;
            oss << std::fixed << std::setprecision(cfg.speed_precision) << value << " " << unit;
            return oss.str();
        }
        
        // Scientific notation formatting
        template<typename T>
        static std::string scientific(T value, int precision = 1) {
            std::ostringstream oss;
            oss << std::scientific << std::setprecision(precision) << value;
            return oss.str();
        }
        
        // Section headers
        static std::string section_header(const std::string& title) {
            return "\n" + title + "\n" + std::string(title.length(), '=');
        }
        
        // Checkmarks and status
        static std::string checkmark() { return "✓"; }
        static std::string crossmark() { return "🚫"; }
        static std::string success() { return "✅"; }
        static std::string error() { return "❌"; }
        
        // List formatting
        template<typename T>
        static std::string list_item(int index, const std::string& content) {
            return "  " + std::to_string(index) + ". " + content;
        }
        
        // Tau convention constants
        static constexpr double TAU = 6.283185307179586; // 2π
        
        // Tau conversion utilities
        template<typename T>
        static T degrees_to_tau(T degrees) {
            return degrees * TAU / 360.0;
        }
        
        template<typename T>
        static T tau_to_degrees(T tau_fraction) {
            return tau_fraction * 360.0 / TAU;
        }
        
        // Mathematical constant formatting
        static std::string tau_constant() {
            auto& cfg = config();
            std::ostringstream oss;
            oss << "τ (tau = 2π) = " << std::fixed << std::setprecision(5) << TAU;
            return oss.str();
        }
        
        // Print utilities that ensure consistent formatting
        static void print_position(const std::string& label, double x, double y, double z, const std::string& frame = "") {
            std::cout << checkmark() << " " << label << ": " << position(x, y, z);
            if (!frame.empty()) {
                std::cout << " [" << frame << " frame]";
            }
            std::cout << std::endl;
        }
        
        // Print a position-like object directly
        template<PositionLike T>
        static void print_position_like(const std::string& label, const T& pos) {
            print_position(label, pos.x, pos.y, pos.z, T::frame_name());
        }
        
        static void print_distance(const std::string& label, double value, const std::string& unit = "m") {
            std::cout << checkmark() << " " << label << ": " << distance(value, unit) << std::endl;
        }
        
        static void print_angle(const std::string& label, double degrees) {
            auto& cfg = config();
            if (cfg.use_tau_convention) {
                double tau_fraction = degrees_to_tau(degrees);
                std::cout << checkmark() << " " << label << ": " << angle_combined(degrees, tau_fraction) << std::endl;
            } else {
                std::cout << checkmark() << " " << label << ": " << angle_degrees(degrees) << std::endl;
            }
        }
        
        static void print_speed(const std::string& label, double value) {
            std::cout << checkmark() << " " << label << ": " << speed(value) << std::endl;
        }
        
        static void print_time(const std::string& label, double value) {
            std::cout << checkmark() << " " << label << ": " << time(value) << std::endl;
        }
        
        static void print_success(const std::string& message) {
            std::cout << success() << " " << message << std::endl;
        }
        
        static void print_error(const std::string& message) {
            std::cout << error() << " " << message << std::endl;
        }
        
        static void print_warning(const std::string& message) {
            std::cout << crossmark() << " " << message << std::endl;
        }
        
        // Configuration utilities
        static void set_precision(int position = 1, int angle = 0, int distance = 1, int time = 1, int speed = 2) {
            auto& cfg = config();
            cfg.position_precision = position;
            cfg.angle_precision = angle;
            cfg.distance_precision = distance;
            cfg.time_precision = time;
            cfg.speed_precision = speed;
        }
        
        static void set_scientific_threshold(double threshold) {
            config().scientific_threshold = threshold;
        }
        
        static void set_tau_convention(bool use_tau) {
            config().use_tau_convention = use_tau;
        }
    };
    
    // Convenience alias for shorter usage
    using output = CanonicalOutput;
    
} // namespace gafro::modern::utilities
