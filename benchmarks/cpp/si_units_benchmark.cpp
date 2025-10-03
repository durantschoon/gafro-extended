// SPDX-FileCopyrightText: GAFRO Extended Implementation
//
// SPDX-License-Identifier: MPL-2.0

#include <benchmark/benchmark.h>
#include <random>
#include <vector>
#include <chrono>

// Simplified SI Units for benchmarking (avoiding compilation issues)
namespace gafro::modern::units {

    constexpr double TAU = 6.283185307179586; // 2π

    template<int Mass, int Length, int Time>
    struct SimpleDimension {
        static constexpr int mass = Mass;
        static constexpr int length = Length;
        static constexpr int time = Time;
    };

    template<typename ValueType, typename Dim>
    class SimpleQuantity {
    public:
        using value_type = ValueType;
        using dimension = Dim;

        constexpr SimpleQuantity() : value_(ValueType{}) {}
        constexpr explicit SimpleQuantity(const ValueType& value) : value_(value) {}

        constexpr const ValueType& value() const { return value_; }

        constexpr SimpleQuantity operator+(const SimpleQuantity& other) const {
            return SimpleQuantity(value_ + other.value_);
        }

        constexpr SimpleQuantity operator*(const ValueType& scalar) const {
            return SimpleQuantity(value_ * scalar);
        }

        template<typename OtherDim>
        constexpr auto operator*(const SimpleQuantity<ValueType, OtherDim>& other) const {
            using ResultDim = SimpleDimension<Dim::mass + OtherDim::mass,
                                            Dim::length + OtherDim::length,
                                            Dim::time + OtherDim::time>;
            return SimpleQuantity<ValueType, ResultDim>(value_ * other.value());
        }

        template<typename OtherDim>
        constexpr auto operator/(const SimpleQuantity<ValueType, OtherDim>& other) const {
            using ResultDim = SimpleDimension<Dim::mass - OtherDim::mass,
                                            Dim::length - OtherDim::length,
                                            Dim::time - OtherDim::time>;
            return SimpleQuantity<ValueType, ResultDim>(value_ / other.value());
        }

    private:
        ValueType value_;
    };

    using Dimensionless = SimpleDimension<0, 0, 0>;
    using LengthDim = SimpleDimension<0, 1, 0>;
    using TimeDim = SimpleDimension<0, 0, 1>;
    using VelocityDim = SimpleDimension<0, 1, -1>;
    using MassDim = SimpleDimension<1, 0, 0>;
    using ForceDim = SimpleDimension<1, 1, -2>;

    using Length = SimpleQuantity<double, LengthDim>;
    using Time = SimpleQuantity<double, TimeDim>;
    using Velocity = SimpleQuantity<double, VelocityDim>;
    using Mass = SimpleQuantity<double, MassDim>;
    using Force = SimpleQuantity<double, ForceDim>;

    // Unit constructors
    constexpr Length meters(double value) { return Length(value); }
    constexpr Time seconds(double value) { return Time(value); }
    constexpr Mass kilograms(double value) { return Mass(value); }
    constexpr Force newtons(double value) { return Force(value); }

    // Angular functions using tau convention
    double sin_tau(double angle_in_tau_radians) {
        return std::sin(angle_in_tau_radians);
    }

    double cos_tau(double angle_in_tau_radians) {
        return std::cos(angle_in_tau_radians);
    }

    double degrees_to_tau_radians(double degrees) {
        return degrees * TAU / 360.0;
    }

} // namespace gafro::modern::units

using namespace gafro::modern::units;

// Global test data
std::mt19937 rng(42);
std::uniform_real_distribution<double> dist(0.1, 100.0);

/**
 * @brief Benchmark unit arithmetic operations
 */
static void BM_LengthAddition(benchmark::State& state) {
    const size_t count = static_cast<size_t>(state.range(0));
    std::vector<Length> lengths1, lengths2;

    for (size_t i = 0; i < count; ++i) {
        lengths1.push_back(meters(dist(rng)));
        lengths2.push_back(meters(dist(rng)));
    }

    for (auto _ : state) {
        for (size_t i = 0; i < count; ++i) {
            auto result = lengths1[i] + lengths2[i];
            benchmark::DoNotOptimize(result);
        }
    }

    state.SetItemsProcessed(state.iterations() * count);
    state.SetBytesProcessed(state.iterations() * count * sizeof(Length) * 2);
}

static void BM_VelocityCalculation(benchmark::State& state) {
    const size_t count = static_cast<size_t>(state.range(0));
    std::vector<Length> distances;
    std::vector<Time> times;

    for (size_t i = 0; i < count; ++i) {
        distances.push_back(meters(dist(rng)));
        times.push_back(seconds(dist(rng)));
    }

    for (auto _ : state) {
        for (size_t i = 0; i < count; ++i) {
            auto velocity = distances[i] / times[i];
            benchmark::DoNotOptimize(velocity);
        }
    }

    state.SetItemsProcessed(state.iterations() * count);
    state.SetBytesProcessed(state.iterations() * count * (sizeof(Length) + sizeof(Time)));
}

static void BM_ForceCalculation(benchmark::State& state) {
    const size_t count = static_cast<size_t>(state.range(0));
    std::vector<Mass> masses;
    std::vector<Length> distances;
    std::vector<Time> times;

    for (size_t i = 0; i < count; ++i) {
        masses.push_back(kilograms(dist(rng)));
        distances.push_back(meters(dist(rng)));
        times.push_back(seconds(dist(rng)));
    }

    for (auto _ : state) {
        for (size_t i = 0; i < count; ++i) {
            // F = ma = m * (d / t^2)
            auto acceleration = distances[i] / (times[i] * times[i]);
            auto force = masses[i] * acceleration;
            benchmark::DoNotOptimize(force);
        }
    }

    state.SetItemsProcessed(state.iterations() * count);
}

/**
 * @brief Benchmark unit conversions
 */
static void BM_UnitConversions(benchmark::State& state) {
    const size_t count = static_cast<size_t>(state.range(0));
    std::vector<double> values;

    for (size_t i = 0; i < count; ++i) {
        values.push_back(dist(rng));
    }

    for (auto _ : state) {
        for (size_t i = 0; i < count; ++i) {
            // Convert meters to kilometers and back
            auto length_m = meters(values[i]);
            auto length_km = Length(length_m.value() / 1000.0);
            auto back_to_m = Length(length_km.value() * 1000.0);
            benchmark::DoNotOptimize(back_to_m);
        }
    }

    state.SetItemsProcessed(state.iterations() * count);
}

/**
 * @brief Benchmark tau-based angular calculations
 */
static void BM_TauTrigonometry(benchmark::State& state) {
    const size_t count = static_cast<size_t>(state.range(0));
    std::uniform_real_distribution<double> angle_dist(0.0, 360.0);
    std::vector<double> angles_deg;

    for (size_t i = 0; i < count; ++i) {
        angles_deg.push_back(angle_dist(rng));
    }

    for (auto _ : state) {
        for (size_t i = 0; i < count; ++i) {
            // Convert degrees to tau radians and compute sin/cos
            auto tau_rad = degrees_to_tau_radians(angles_deg[i]);
            auto sin_val = sin_tau(tau_rad);
            auto cos_val = cos_tau(tau_rad);
            benchmark::DoNotOptimize(sin_val + cos_val);
        }
    }

    state.SetItemsProcessed(state.iterations() * count);
}

static void BM_TraditionalTrigonometry(benchmark::State& state) {
    const size_t count = static_cast<size_t>(state.range(0));
    std::uniform_real_distribution<double> angle_dist(0.0, 360.0);
    std::vector<double> angles_deg;

    for (size_t i = 0; i < count; ++i) {
        angles_deg.push_back(angle_dist(rng));
    }

    for (auto _ : state) {
        for (size_t i = 0; i < count; ++i) {
            // Convert degrees to traditional radians and compute sin/cos
            auto rad = angles_deg[i] * 3.141592653589793 / 180.0;
            auto sin_val = std::sin(rad);
            auto cos_val = std::cos(rad);
            benchmark::DoNotOptimize(sin_val + cos_val);
        }
    }

    state.SetItemsProcessed(state.iterations() * count);
}

/**
 * @brief Benchmark marine robotics calculations
 */
static void BM_BuoyancyCalculation(benchmark::State& state) {
    const size_t count = static_cast<size_t>(state.range(0));
    std::vector<Length> lengths, widths, heights;

    for (size_t i = 0; i < count; ++i) {
        lengths.push_back(meters(dist(rng)));
        widths.push_back(meters(dist(rng)));
        heights.push_back(meters(dist(rng)));
    }

    const auto water_density = kilograms(1025.0) / (meters(1.0) * meters(1.0) * meters(1.0));
    const auto gravity = meters(9.81) / (seconds(1.0) * seconds(1.0));

    for (auto _ : state) {
        for (size_t i = 0; i < count; ++i) {
            // Calculate buoyancy force: F = ρ * V * g
            auto volume = lengths[i] * widths[i] * heights[i];
            auto buoyancy_force = water_density * volume * gravity;
            benchmark::DoNotOptimize(buoyancy_force);
        }
    }

    state.SetItemsProcessed(state.iterations() * count);
}

static void BM_PressureAtDepth(benchmark::State& state) {
    const size_t count = static_cast<size_t>(state.range(0));
    std::vector<Length> depths;

    for (size_t i = 0; i < count; ++i) {
        depths.push_back(meters(dist(rng) * 100.0)); // 0-10000m depth
    }

    const auto atmospheric_pressure = newtons(101325.0) / (meters(1.0) * meters(1.0));
    const auto water_density = kilograms(1025.0) / (meters(1.0) * meters(1.0) * meters(1.0));
    const auto gravity = meters(9.81) / (seconds(1.0) * seconds(1.0));

    for (auto _ : state) {
        for (size_t i = 0; i < count; ++i) {
            // Calculate pressure at depth: P = P₀ + ρgh
            auto pressure_increase = water_density * gravity * depths[i];
            auto total_pressure = atmospheric_pressure + pressure_increase;
            benchmark::DoNotOptimize(total_pressure);
        }
    }

    state.SetItemsProcessed(state.iterations() * count);
}

/**
 * @brief Compare unit-safe vs raw calculations
 */
static void BM_UnitSafeCalculation(benchmark::State& state) {
    const size_t count = static_cast<size_t>(state.range(0));
    std::vector<Length> distances;
    std::vector<Time> times;

    for (size_t i = 0; i < count; ++i) {
        distances.push_back(meters(dist(rng)));
        times.push_back(seconds(dist(rng)));
    }

    for (auto _ : state) {
        for (size_t i = 0; i < count; ++i) {
            auto velocity = distances[i] / times[i];
            auto kinetic_energy_factor = velocity * velocity; // Partial KE calculation
            benchmark::DoNotOptimize(kinetic_energy_factor);
        }
    }

    state.SetItemsProcessed(state.iterations() * count);
}

static void BM_RawCalculation(benchmark::State& state) {
    const size_t count = static_cast<size_t>(state.range(0));
    std::vector<double> distances, times;

    for (size_t i = 0; i < count; ++i) {
        distances.push_back(dist(rng));
        times.push_back(dist(rng));
    }

    for (auto _ : state) {
        for (size_t i = 0; i < count; ++i) {
            auto velocity = distances[i] / times[i];
            auto kinetic_energy_factor = velocity * velocity;
            benchmark::DoNotOptimize(kinetic_energy_factor);
        }
    }

    state.SetItemsProcessed(state.iterations() * count);
}

// Register benchmarks
BENCHMARK(BM_LengthAddition)->Range(8, 8<<10)->UseRealTime();
BENCHMARK(BM_VelocityCalculation)->Range(8, 8<<10)->UseRealTime();
BENCHMARK(BM_ForceCalculation)->Range(8, 8<<10)->UseRealTime();
BENCHMARK(BM_UnitConversions)->Range(8, 8<<10)->UseRealTime();

// Trigonometry benchmarks
BENCHMARK(BM_TauTrigonometry)->Range(8, 8<<10)->UseRealTime();
BENCHMARK(BM_TraditionalTrigonometry)->Range(8, 8<<10)->UseRealTime();

// Marine robotics benchmarks
BENCHMARK(BM_BuoyancyCalculation)->Range(8, 8<<10)->UseRealTime();
BENCHMARK(BM_PressureAtDepth)->Range(8, 8<<10)->UseRealTime();

// Performance comparison benchmarks
BENCHMARK(BM_UnitSafeCalculation)->Range(8, 8<<10)->UseRealTime();
BENCHMARK(BM_RawCalculation)->Range(8, 8<<10)->UseRealTime();