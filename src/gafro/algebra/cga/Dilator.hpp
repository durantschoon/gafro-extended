/*
    Copyright (c) 2022 Idiap Research Institute, http://www.idiap.ch/
    Written by Tobias Löw <https://tobiloew.ch>

    This file is part of gafro.

    gafro is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License version 3 as
    published by the Free Software Foundation.

    gafro is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with gafro. If not, see <http://www.gnu.org/licenses/>.
*/

#pragma once

#include <gafro/algebra/cga/Blades.hpp>
#include <gafro/algebra/cga/Versor.hpp>

namespace gafro
{
    template <class T>
    class Dilator : public Versor<Dilator<T>, T, blades::scalar, blades::e0i>
    {
      public:
        using Base = Versor<Dilator<T>, T, blades::scalar, blades::e0i>;

        using Parameters = typename Base::Parameters;

        using Base::Base;

        using Generator = Multivector<T, blades::e0i>;

        Dilator();

        Dilator(const T &dilation);

        virtual ~Dilator();

        Generator logarithm() const;

      protected:
      private:
      public:
        static Dilator exp(const Generator &generator);
    };

}  // namespace gafro

#include <gafro/algebra/cga/Dilator.hxx>