// SPDX-FileCopyrightText: GAFRO Extended Implementation
//
// SPDX-License-Identifier: MPL-2.0

#pragma once

#include <variant>
#include <functional>
#include <type_traits>
#include <gafro/modern/GATerm.hpp>
#include <gafro/modern/GradeIndexed.hpp>

namespace gafro::modern
{
    /**
     * @brief Pattern matching utilities using std::visit
     *
     * This provides functional-style pattern matching for GATerm variants,
     * allowing safe and ergonomic handling of different grades.
     */

    /**
     * @brief Visitor for pattern matching on GA terms
     */
    template<typename F>
    struct GATermVisitor {
        F func;

        template<typename T>
        constexpr auto operator()(const T& value) -> decltype(func(value)) {
            return func(value);
        }
    };

    /**
     * @brief Create a visitor from a lambda/function
     */
    template<typename F>
    constexpr auto makeVisitor(F&& func) {
        return GATermVisitor<std::decay_t<F>>{std::forward<F>(func)};
    }

    /**
     * @brief Pattern matching on GATerm with different handlers for each grade
     */
    template<typename T, typename ScalarF, typename VectorF, typename BivectorF, typename TrivectorF, typename MultivectorF>
    auto match(const GATerm<T>& term, ScalarF&& scalarHandler, VectorF&& vectorHandler,
               BivectorF&& bivectorHandler, TrivectorF&& trivectorHandler, MultivectorF&& multivectorHandler) {
        return std::visit([&](const auto& value) -> decltype(auto) {
            using ValueType = std::decay_t<decltype(value)>;

            if constexpr (std::is_same_v<ValueType, Scalar<T>>) {
                return scalarHandler(value);
            }
            else if constexpr (std::is_same_v<ValueType, std::vector<std::pair<Index, T>>>) {
                return vectorHandler(value);
            }
            else if constexpr (std::is_same_v<ValueType, std::vector<std::tuple<Index, Index, T>>>) {
                return bivectorHandler(value);
            }
            else if constexpr (std::is_same_v<ValueType, std::vector<std::tuple<Index, Index, Index, T>>>) {
                return trivectorHandler(value);
            }
            else {
                return multivectorHandler(value);
            }
        }, term);
    }

    /**
     * @brief Simplified pattern matching with fallback for unhandled cases
     */
    template<typename T, typename F>
    auto visitGATerm(const GATerm<T>& term, F&& visitor) {
        return std::visit(std::forward<F>(visitor), term);
    }

    /**
     * @brief Type-safe operations using pattern matching
     */
    namespace operations {

        /**
         * @brief Addition of two GA terms (same grade only)
         */
        template<typename T>
        auto add(const GATerm<T>& lhs, const GATerm<T>& rhs) -> std::optional<GATerm<T>> {
            // Check if both terms have the same grade
            if (getGrade(lhs) != getGrade(rhs)) {
                return std::nullopt; // Cannot add different grades
            }

            return std::visit([](const auto& left, const auto& right) -> std::optional<GATerm<T>> {
                using LeftType = std::decay_t<decltype(left)>;
                using RightType = std::decay_t<decltype(right)>;

                if constexpr (std::is_same_v<LeftType, RightType>) {
                    if constexpr (std::is_same_v<LeftType, Scalar<T>>) {
                        return makeScalar(left.value + right.value);
                    }
                    else if constexpr (std::is_same_v<LeftType, std::vector<std::pair<Index, T>>>) {
                        // Vector addition - combine like terms
                        std::vector<std::pair<Index, T>> result = left;
                        for (const auto& [idx, coeff] : right) {
                            bool found = false;
                            for (auto& [ridx, rcoeff] : result) {
                                if (ridx == idx) {
                                    rcoeff += coeff;
                                    found = true;
                                    break;
                                }
                            }
                            if (!found) {
                                result.emplace_back(idx, coeff);
                            }
                        }
                        return makeVector(result);
                    }
                    // Similar implementations for bivector, trivector, and multivector...
                    else {
                        return std::nullopt; // Not implemented for this type
                    }
                }
                else {
                    return std::nullopt; // Different types
                }
            }, lhs, rhs);
        }

        /**
         * @brief Scalar multiplication
         */
        template<typename T, typename S>
            requires std::is_arithmetic_v<S>
        auto scalarMultiply(const S& scalar, const GATerm<T>& term) -> GATerm<T> {
            return std::visit([scalar](const auto& value) -> GATerm<T> {
                using ValueType = std::decay_t<decltype(value)>;

                if constexpr (std::is_same_v<ValueType, Scalar<T>>) {
                    return makeScalar(scalar * value.value);
                }
                else if constexpr (std::is_same_v<ValueType, std::vector<std::pair<Index, T>>>) {
                    std::vector<std::pair<Index, T>> result;
                    result.reserve(value.size());
                    for (const auto& [idx, coeff] : value) {
                        result.emplace_back(idx, scalar * coeff);
                    }
                    return makeVector(result);
                }
                else if constexpr (std::is_same_v<ValueType, std::vector<std::tuple<Index, Index, T>>>) {
                    std::vector<std::tuple<Index, Index, T>> result;
                    result.reserve(value.size());
                    for (const auto& [i1, i2, coeff] : value) {
                        result.emplace_back(i1, i2, scalar * coeff);
                    }
                    return makeBivector(result);
                }
                else if constexpr (std::is_same_v<ValueType, std::vector<std::tuple<Index, Index, Index, T>>>) {
                    std::vector<std::tuple<Index, Index, Index, T>> result;
                    result.reserve(value.size());
                    for (const auto& [i1, i2, i3, coeff] : value) {
                        result.emplace_back(i1, i2, i3, scalar * coeff);
                    }
                    return makeTrivector(result);
                }
                else {
                    // General multivector
                    std::vector<BladeTerm<T>> result;
                    result.reserve(value.size());
                    for (const auto& term : value) {
                        result.emplace_back(term.indices, scalar * term.coefficient);
                    }
                    return makeMultivector(result);
                }
            }, term);
        }

        /**
         * @brief Get norm of a GA term
         */
        template<typename T>
        auto norm(const GATerm<T>& term) -> T {
            return std::visit([](const auto& value) -> T {
                using ValueType = std::decay_t<decltype(value)>;

                if constexpr (std::is_same_v<ValueType, Scalar<T>>) {
                    return std::abs(value.value);
                }
                else if constexpr (std::is_same_v<ValueType, std::vector<std::pair<Index, T>>>) {
                    T sum = T{};
                    for (const auto& [idx, coeff] : value) {
                        sum += coeff * coeff;
                    }
                    return std::sqrt(sum);
                }
                else if constexpr (std::is_same_v<ValueType, std::vector<std::tuple<Index, Index, T>>>) {
                    T sum = T{};
                    for (const auto& [i1, i2, coeff] : value) {
                        sum += coeff * coeff;
                    }
                    return std::sqrt(sum);
                }
                else if constexpr (std::is_same_v<ValueType, std::vector<std::tuple<Index, Index, Index, T>>>) {
                    T sum = T{};
                    for (const auto& [i1, i2, i3, coeff] : value) {
                        sum += coeff * coeff;
                    }
                    return std::sqrt(sum);
                }
                else {
                    // General multivector
                    T sum = T{};
                    for (const auto& term : value) {
                        sum += term.coefficient * term.coefficient;
                    }
                    return std::sqrt(sum);
                }
            }, term);
        }

        /**
         * @brief Convert GA term to string representation
         */
        template<typename T>
        std::string toString(const GATerm<T>& term) {
            return std::visit([](const auto& value) -> std::string {
                using ValueType = std::decay_t<decltype(value)>;

                if constexpr (std::is_same_v<ValueType, Scalar<T>>) {
                    return "Scalar(" + std::to_string(value.value) + ")";
                }
                else if constexpr (std::is_same_v<ValueType, std::vector<std::pair<Index, T>>>) {
                    std::string result = "Vector(";
                    for (size_t i = 0; i < value.size(); ++i) {
                        if (i > 0) result += ", ";
                        result += "e" + std::to_string(value[i].first) + ":" + std::to_string(value[i].second);
                    }
                    result += ")";
                    return result;
                }
                else if constexpr (std::is_same_v<ValueType, std::vector<std::tuple<Index, Index, T>>>) {
                    std::string result = "Bivector(";
                    for (size_t i = 0; i < value.size(); ++i) {
                        if (i > 0) result += ", ";
                        const auto& [i1, i2, coeff] = value[i];
                        result += "e" + std::to_string(i1) + "e" + std::to_string(i2) + ":" + std::to_string(coeff);
                    }
                    result += ")";
                    return result;
                }
                else if constexpr (std::is_same_v<ValueType, std::vector<std::tuple<Index, Index, Index, T>>>) {
                    std::string result = "Trivector(";
                    for (size_t i = 0; i < value.size(); ++i) {
                        if (i > 0) result += ", ";
                        const auto& [i1, i2, i3, coeff] = value[i];
                        result += "e" + std::to_string(i1) + "e" + std::to_string(i2) + "e" + std::to_string(i3) + ":" + std::to_string(coeff);
                    }
                    result += ")";
                    return result;
                }
                else {
                    return "Multivector(...)";
                }
            }, term);
        }

    } // namespace operations

} // namespace gafro::modern