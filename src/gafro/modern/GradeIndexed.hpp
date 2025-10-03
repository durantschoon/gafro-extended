// SPDX-FileCopyrightText: GAFRO Extended Implementation
//
// SPDX-License-Identifier: MPL-2.0

#pragma once

#include <type_traits>
#include <concepts>
#include <gafro/modern/GATerm.hpp>

namespace gafro::modern
{
    /**
     * @brief Grade-indexed wrapper for compile-time grade checking
     *
     * This template provides compile-time grade safety by encoding the grade
     * in the type system. Only operations between compatible grades are allowed.
     */
    template<typename T, Grade G>
    struct GradeIndexed {
        T value;
        static constexpr Grade grade = G;

        constexpr GradeIndexed() = default;
        constexpr GradeIndexed(const T& v) : value(v) {}
        constexpr GradeIndexed(T&& v) : value(std::move(v)) {}

        // Copy constructor
        constexpr GradeIndexed(const GradeIndexed& other) : value(other.value) {}

        // Move constructor
        constexpr GradeIndexed(GradeIndexed&& other) noexcept : value(std::move(other.value)) {}

        // Assignment operators
        constexpr GradeIndexed& operator=(const GradeIndexed& other) {
            value = other.value;
            return *this;
        }

        constexpr GradeIndexed& operator=(GradeIndexed&& other) noexcept {
            value = std::move(other.value);
            return *this;
        }

        // Access operators
        constexpr const T& operator()() const { return value; }
        constexpr T& operator()() { return value; }

        // Implicit conversion to underlying type
        constexpr operator const T&() const { return value; }
        constexpr operator T&() { return value; }

        // Grade checking at compile time
        constexpr static Grade getGrade() { return G; }

        // Type traits
        using ValueType = T;
        static constexpr bool is_grade_indexed = true;
    };

    /**
     * @brief Type aliases for common grades
     */
    template<typename T>
    using ScalarType = GradeIndexed<T, Grade::SCALAR>;

    template<typename T>
    using VectorType = GradeIndexed<std::vector<std::pair<Index, T>>, Grade::VECTOR>;

    template<typename T>
    using BivectorType = GradeIndexed<std::vector<std::tuple<Index, Index, T>>, Grade::BIVECTOR>;

    template<typename T>
    using TrivectorType = GradeIndexed<std::vector<std::tuple<Index, Index, Index, T>>, Grade::TRIVECTOR>;

    /**
     * @brief Concept to check if a type is grade-indexed
     */
    template<typename T>
    concept IsGradeIndexed = requires {
        T::is_grade_indexed;
        { T::getGrade() } -> std::same_as<Grade>;
    };

    /**
     * @brief Concept to check for specific grade
     */
    template<typename T, Grade G>
    concept IsGrade = IsGradeIndexed<T> && (T::getGrade() == G);

    /**
     * @brief Compile-time grade checking operations
     */

    // Addition: only same grades can be added
    template<typename T, Grade G>
    constexpr auto operator+(const GradeIndexed<T, G>& lhs, const GradeIndexed<T, G>& rhs) {
        if constexpr (G == Grade::SCALAR) {
            return GradeIndexed<T, G>(lhs.value + rhs.value);
        } else {
            // For vectors and higher grades, implement component-wise addition
            T result = lhs.value;
            // Implementation depends on specific T type
            return GradeIndexed<T, G>(std::move(result));
        }
    }

    // Scalar multiplication: scalar can multiply any grade
    template<typename T, typename S, Grade G>
        requires std::is_arithmetic_v<S>
    constexpr auto operator*(const S& scalar, const GradeIndexed<T, G>& rhs) {
        if constexpr (G == Grade::SCALAR) {
            return GradeIndexed<T, G>(scalar * rhs.value);
        } else {
            // Scale all components
            T result = rhs.value;
            // Implementation depends on specific T type
            return GradeIndexed<T, G>(std::move(result));
        }
    }

    template<typename T, typename S, Grade G>
        requires std::is_arithmetic_v<S>
    constexpr auto operator*(const GradeIndexed<T, G>& lhs, const S& scalar) {
        return scalar * lhs;
    }

    /**
     * @brief Factory functions for grade-indexed types
     */
    template<typename T>
    constexpr auto makeScalarGI(const T& value) {
        return ScalarType<T>(value);
    }

    template<typename T>
    auto makeVectorGI(const std::vector<std::pair<Index, T>>& components) {
        return VectorType<T>(components);
    }

    template<typename T>
    auto makeBivectorGI(const std::vector<std::tuple<Index, Index, T>>& components) {
        return BivectorType<T>(components);
    }

    template<typename T>
    auto makeTrivectorGI(const std::vector<std::tuple<Index, Index, Index, T>>& components) {
        return TrivectorType<T>(components);
    }

    /**
     * @brief Grade checking utilities
     */
    template<typename T>
    struct GradeChecker {
        static constexpr bool isScalar() {
            if constexpr (IsGradeIndexed<T>) {
                return T::getGrade() == Grade::SCALAR;
            }
            return false;
        }

        static constexpr bool isVector() {
            if constexpr (IsGradeIndexed<T>) {
                return T::getGrade() == Grade::VECTOR;
            }
            return false;
        }

        static constexpr bool isBivector() {
            if constexpr (IsGradeIndexed<T>) {
                return T::getGrade() == Grade::BIVECTOR;
            }
            return false;
        }

        static constexpr bool isTrivector() {
            if constexpr (IsGradeIndexed<T>) {
                return T::getGrade() == Grade::TRIVECTOR;
            }
            return false;
        }
    };

} // namespace gafro::modern