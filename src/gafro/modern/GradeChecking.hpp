// SPDX-FileCopyrightText: GAFRO Extended Implementation
//
// SPDX-License-Identifier: MPL-2.0

#pragma once

#include <type_traits>
#include <concepts>
#include <array>
#include <gafro/modern/GATerm.hpp>
#include <gafro/modern/GradeIndexed.hpp>

namespace gafro::modern
{
    /**
     * @brief Compile-time grade checking system
     *
     * This provides a comprehensive system for validating geometric algebra
     * operations at compile time, preventing invalid operations between
     * incompatible grades.
     */

    /**
     * @brief Grade validation concepts
     */
    template<typename T>
    concept IsScalarGrade = IsGradeIndexed<T> && (T::getGrade() == Grade::SCALAR);

    template<typename T>
    concept IsVectorGrade = IsGradeIndexed<T> && (T::getGrade() == Grade::VECTOR);

    template<typename T>
    concept IsBivectorGrade = IsGradeIndexed<T> && (T::getGrade() == Grade::BIVECTOR);

    template<typename T>
    concept IsTrivectorGrade = IsGradeIndexed<T> && (T::getGrade() == Grade::TRIVECTOR);

    template<typename T>
    concept IsMultivectorGrade = IsGradeIndexed<T> && (T::getGrade() == Grade::MULTIVECTOR);

    /**
     * @brief Operation validity concepts
     */
    template<typename T1, typename T2>
    concept CanAdd = IsGradeIndexed<T1> && IsGradeIndexed<T2> &&
                     (T1::getGrade() == T2::getGrade());

    template<typename T1, typename T2>
    concept CanGeometricProduct = IsGradeIndexed<T1> && IsGradeIndexed<T2>;

    template<typename T1, typename T2>
    concept CanOuterProduct = IsGradeIndexed<T1> && IsGradeIndexed<T2>;

    template<typename T1, typename T2>
    concept CanInnerProduct = IsGradeIndexed<T1> && IsGradeIndexed<T2>;

    /**
     * @brief Grade calculation utilities for operations
     */
    namespace grade_calc {

        /**
         * @brief Calculate result grade for geometric product
         */
        template<Grade G1, Grade G2>
        constexpr std::array<Grade, 8> geometricProductGrades() {
            constexpr int g1 = static_cast<int>(G1);
            constexpr int g2 = static_cast<int>(G2);

            // Geometric product can produce multiple grades
            // |g1 - g2|, |g1 - g2| + 2, ..., g1 + g2
            std::array<Grade, 8> result{};
            int count = 0;

            for (int g = std::abs(g1 - g2); g <= g1 + g2; g += 2) {
                if (g <= 3 && count < 8) { // Limit to reasonable grades
                    result[count++] = static_cast<Grade>(g);
                }
            }

            return result;
        }

        /**
         * @brief Calculate result grade for outer product
         */
        template<Grade G1, Grade G2>
        constexpr Grade outerProductGrade() {
            constexpr int g1 = static_cast<int>(G1);
            constexpr int g2 = static_cast<int>(G2);

            // Outer product: grade = g1 + g2
            constexpr int result = g1 + g2;

            if constexpr (result <= 3) {
                return static_cast<Grade>(result);
            } else {
                return Grade::MULTIVECTOR;
            }
        }

        /**
         * @brief Calculate result grade for inner product
         */
        template<Grade G1, Grade G2>
        constexpr Grade innerProductGrade() {
            constexpr int g1 = static_cast<int>(G1);
            constexpr int g2 = static_cast<int>(G2);

            // Inner product: grade = |g1 - g2|
            constexpr int result = std::abs(g1 - g2);

            if constexpr (result <= 3) {
                return static_cast<Grade>(result);
            } else {
                return Grade::MULTIVECTOR;
            }
        }

    } // namespace grade_calc

    /**
     * @brief Compile-time operation validation
     */
    template<typename Result, typename T1, typename T2>
    struct OperationValidator {
        // Addition validation
        static constexpr bool canAdd() {
            return CanAdd<T1, T2>;
        }

        // Multiplication validation
        static constexpr bool canMultiply() {
            return CanGeometricProduct<T1, T2>;
        }

        // Outer product validation
        static constexpr bool canOuterProduct() {
            return CanOuterProduct<T1, T2>;
        }

        // Inner product validation
        static constexpr bool canInnerProduct() {
            return CanInnerProduct<T1, T2>;
        }
    };

    /**
     * @brief Compile-time grade-safe operations
     */
    namespace safe_ops {

        /**
         * @brief Grade-safe addition
         */
        template<typename T, Grade G>
        constexpr auto add(const GradeIndexed<T, G>& lhs, const GradeIndexed<T, G>& rhs)
            requires CanAdd<GradeIndexed<T, G>, GradeIndexed<T, G>>
        {
            return GradeIndexed<T, G>(lhs.value + rhs.value);
        }

        /**
         * @brief Grade-safe scalar multiplication
         */
        template<typename T, typename S, Grade G>
        constexpr auto scalarMultiply(const S& scalar, const GradeIndexed<T, G>& operand)
            requires std::is_arithmetic_v<S>
        {
            return GradeIndexed<T, G>(scalar * operand.value);
        }

        /**
         * @brief Grade-safe outer product
         */
        template<typename T1, typename T2, Grade G1, Grade G2>
        constexpr auto outerProduct(const GradeIndexed<T1, G1>& lhs, const GradeIndexed<T2, G2>& rhs)
            requires CanOuterProduct<GradeIndexed<T1, G1>, GradeIndexed<T2, G2>>
        {
            constexpr Grade resultGrade = grade_calc::outerProductGrade<G1, G2>();

            // Implementation depends on specific types T1, T2
            // This is a placeholder - actual implementation would compute the outer product
            using ResultType = std::common_type_t<T1, T2>;

            if constexpr (resultGrade == Grade::SCALAR) {
                return GradeIndexed<ResultType, Grade::SCALAR>(ResultType{});
            } else if constexpr (resultGrade == Grade::VECTOR) {
                return GradeIndexed<std::vector<std::pair<Index, ResultType>>, Grade::VECTOR>(
                    std::vector<std::pair<Index, ResultType>>{});
            } else {
                // Higher grades or multivector
                return GradeIndexed<std::vector<BladeTerm<ResultType>>, Grade::MULTIVECTOR>(
                    std::vector<BladeTerm<ResultType>>{});
            }
        }

        /**
         * @brief Grade-safe inner product
         */
        template<typename T1, typename T2, Grade G1, Grade G2>
        constexpr auto innerProduct(const GradeIndexed<T1, G1>& lhs, const GradeIndexed<T2, G2>& rhs)
            requires CanInnerProduct<GradeIndexed<T1, G1>, GradeIndexed<T2, G2>>
        {
            constexpr Grade resultGrade = grade_calc::innerProductGrade<G1, G2>();

            // Implementation depends on specific types T1, T2
            using ResultType = std::common_type_t<T1, T2>;

            if constexpr (resultGrade == Grade::SCALAR) {
                return GradeIndexed<ResultType, Grade::SCALAR>(ResultType{});
            } else if constexpr (resultGrade == Grade::VECTOR) {
                return GradeIndexed<std::vector<std::pair<Index, ResultType>>, Grade::VECTOR>(
                    std::vector<std::pair<Index, ResultType>>{});
            } else {
                return GradeIndexed<std::vector<BladeTerm<ResultType>>, Grade::MULTIVECTOR>(
                    std::vector<BladeTerm<ResultType>>{});
            }
        }

    } // namespace safe_ops

    /**
     * @brief Type inspection utilities
     */
    template<typename T>
    struct TypeInspector {
        static constexpr bool isGradeIndexed = IsGradeIndexed<T>;

        static constexpr Grade getGrade() {
            if constexpr (IsGradeIndexed<T>) {
                return T::getGrade();
            } else {
                return Grade::MULTIVECTOR; // Default fallback
            }
        }

        static constexpr bool isScalar() {
            return IsScalarGrade<T>;
        }

        static constexpr bool isVector() {
            return IsVectorGrade<T>;
        }

        static constexpr bool isBivector() {
            return IsBivectorGrade<T>;
        }

        static constexpr bool isTrivector() {
            return IsTrivectorGrade<T>;
        }

        static constexpr bool isMultivector() {
            return IsMultivectorGrade<T>;
        }
    };

    /**
     * @brief Compile-time operation compatibility matrix
     */
    template<Grade G1, Grade G2>
    struct OperationMatrix {
        static constexpr bool canAdd = (G1 == G2);
        static constexpr bool canGeometricProduct = true; // Always valid
        static constexpr bool canOuterProduct = true; // Always valid
        static constexpr bool canInnerProduct = true; // Always valid

        static constexpr Grade outerProductResult = grade_calc::outerProductGrade<G1, G2>();
        static constexpr Grade innerProductResult = grade_calc::innerProductGrade<G1, G2>();
    };

    /**
     * @brief Static assertions for compile-time validation
     */
    #define GAFRO_ASSERT_SAME_GRADE(T1, T2) \
        static_assert(CanAdd<T1, T2>, "Operation requires same grades")

    #define GAFRO_ASSERT_VALID_OPERATION(T1, T2, OP) \
        static_assert(Can##OP<T1, T2>, "Invalid operation for these types")

    #define GAFRO_ASSERT_GRADE(T, G) \
        static_assert(IsGrade<T, G>, "Type must have specified grade")

} // namespace gafro::modern