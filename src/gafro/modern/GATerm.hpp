// SPDX-FileCopyrightText: GAFRO Extended Implementation
//
// SPDX-License-Identifier: MPL-2.0

#pragma once

#include <variant>
#include <vector>
#include <tuple>
#include <type_traits>
#include <concepts>
#include <gafro/algebra/Multivector.hpp>

namespace gafro::modern
{
    // Forward declarations
    template<typename T>
    struct Scalar;

    template<typename T>
    struct BladeTerm;

    // Type alias for blade indices
    using Index = int;

    // Grade type for compile-time grade tracking
    enum class Grade : int {
        SCALAR = 0,
        VECTOR = 1,
        BIVECTOR = 2,
        TRIVECTOR = 3,
        MULTIVECTOR = -1  // General case
    };

    /**
     * @brief Scalar wrapper for type safety
     */
    template<typename T>
    struct Scalar {
        T value;

        constexpr Scalar() : value(T{}) {}
        constexpr Scalar(const T& v) : value(v) {}

        constexpr static Grade grade() { return Grade::SCALAR; }

        constexpr T operator()() const { return value; }
        constexpr operator T() const { return value; }

        constexpr Scalar operator+(const Scalar& other) const {
            return Scalar(value + other.value);
        }

        constexpr Scalar operator*(const Scalar& other) const {
            return Scalar(value * other.value);
        }
    };

    /**
     * @brief Blade term representation for general multivectors
     */
    template<typename T>
    struct BladeTerm {
        std::vector<Index> indices;
        T coefficient;

        BladeTerm() = default;
        BladeTerm(std::vector<Index> idx, T coeff) : indices(std::move(idx)), coefficient(coeff) {}

        Grade getGrade() const { return static_cast<Grade>(indices.size()); }
    };

    /**
     * @brief Sum type representing different grades of geometric algebra terms
     *
     * This uses std::variant to provide type-safe sum types for geometric algebra
     * elements with different grades.
     */
    template<typename T>
    using GATerm = std::variant<
        Scalar<T>,                                        // 0-vector (scalar)
        std::vector<std::pair<Index, T>>,                 // 1-vector
        std::vector<std::tuple<Index, Index, T>>,         // 2-vector (bivector)
        std::vector<std::tuple<Index, Index, Index, T>>,  // 3-vector (trivector)
        std::vector<BladeTerm<T>>                         // General multivector
    >;

    /**
     * @brief Get the grade of a GATerm
     */
    template<typename T>
    constexpr Grade getGrade(const GATerm<T>& term) {
        return std::visit([](const auto& value) -> Grade {
            using ValueType = std::decay_t<decltype(value)>;

            if constexpr (std::is_same_v<ValueType, Scalar<T>>) {
                return Grade::SCALAR;
            }
            else if constexpr (std::is_same_v<ValueType, std::vector<std::pair<Index, T>>>) {
                return Grade::VECTOR;
            }
            else if constexpr (std::is_same_v<ValueType, std::vector<std::tuple<Index, Index, T>>>) {
                return Grade::BIVECTOR;
            }
            else if constexpr (std::is_same_v<ValueType, std::vector<std::tuple<Index, Index, Index, T>>>) {
                return Grade::TRIVECTOR;
            }
            else {
                return Grade::MULTIVECTOR;
            }
        }, term);
    }

    /**
     * @brief Type predicate to check if a type is a specific grade
     */
    template<typename T, Grade G>
    concept HasGrade = requires(const T& t) {
        { getGrade(t) } -> std::same_as<Grade>;
        getGrade(t) == G;
    };

    /**
     * @brief Factory functions for creating GA terms
     */
    template<typename T>
    constexpr GATerm<T> makeScalar(const T& value) {
        return Scalar<T>(value);
    }

    template<typename T>
    GATerm<T> makeVector(const std::vector<std::pair<Index, T>>& components) {
        return components;
    }

    template<typename T>
    GATerm<T> makeBivector(const std::vector<std::tuple<Index, Index, T>>& components) {
        return components;
    }

    template<typename T>
    GATerm<T> makeTrivector(const std::vector<std::tuple<Index, Index, Index, T>>& components) {
        return components;
    }

    template<typename T>
    GATerm<T> makeMultivector(const std::vector<BladeTerm<T>>& terms) {
        return terms;
    }

} // namespace gafro::modern