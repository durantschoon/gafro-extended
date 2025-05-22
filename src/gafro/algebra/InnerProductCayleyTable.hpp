// SPDX-FileCopyrightText: Idiap Research Institute <contact@idiap.ch>
//
// SPDX-FileContributor: Tobias Loew <tobias.loew@idiap.ch
//
// SPDX-License-Identifier: MPL-2.0

#pragma once

#include <gafro/algebra/Algebra.hpp>

namespace gafro
{

    template <class Metric>
    template <class T, int i1, int i2>
    class Algebra<Metric>::InnerProduct
    {
      public:
        template <int b1, int b2>
        struct Inner
        {
            template <int i, int j, int lhs, int rhs, int sign_>
            constexpr static std::pair<int, int> next()
            {
                if constexpr (j == Metric::dim - 1 && i < Metric::dim - 1)
                {
                    return recurse<i + 1, 0, lhs, rhs, sign_>();
                }
                else if constexpr (j == Metric::dim - 1 && i == Metric::dim - 1)
                {
                    if constexpr ((lhs & rhs) != 0 || (Algebra<Metric>::BladeBitmap::template getGrade<lhs ^ rhs>() !=
                                                       Algebra<Metric>::BladeBitmap::template getGrade<b1, b2>()))
                    {
                        return std::make_pair(lhs ^ rhs, 0);
                    }

                    return std::make_pair(lhs ^ rhs, sign_);
                }
                else
                {
                    return recurse<i, j + 1, lhs, rhs, sign_>();
                }
            }

            template <int i, int j, int lhs, int rhs, int sign_>
            constexpr static std::pair<int, int> recurse()
            {
                if (b1 == 0 || b2 == 0)
                {
                    return std::make_pair(0.0, 0.0);
                }

                if constexpr ((Metric::template get<i, j>() != 0.0) && (((b1 & (1 << i)) != 0) && ((b2 & (1 << j)) != 0)))
                {
                    constexpr int s = sign_ * Metric::template get<i, j>() *
                                      math::pown<math::positive<Algebra<Metric>::BladeBitmap::template getRightShifts<(1 << i), lhs>()>() +
                                                 math::positive<Algebra<Metric>::BladeBitmap::template getLeftShifts<(1 << j), rhs>()>()>();

                    constexpr int lhs2 = lhs ^ (1 << i);
                    constexpr int rhs2 = rhs ^ (1 << j);

                    return next<i, j, lhs2, rhs2, s>();
                }

                return next<i, j, lhs, rhs, sign_>();
            }

            constexpr static std::pair<int, double> result = recurse<0, 0, b1, b2, 1>();

            constexpr static int value()
            {
                return result.first;
            }

            constexpr static double sign()
            {
                return result.second;
            }

            constexpr static bool valid()
            {
                return sign() != 0.0;
            }

            using Type = std::conditional_t<valid(),                                                       //
                                            typename Algebra<Metric>::template Multivector<T, (value())>,  //
                                            typename Algebra<Metric>::template Multivector<T>>;
        };

        using Type = typename Inner<i1, i2>::Type;
        constexpr static std::array<double, Type::size> signs = [] {
            if constexpr (Inner<i1, i2>::valid())
            {
                return std::array<double, 1>({ Inner<i1, i2>::sign() });
            }
            else
            {
                return std::array<double, 0>();
            }
        }();
    };

}  // namespace gafro