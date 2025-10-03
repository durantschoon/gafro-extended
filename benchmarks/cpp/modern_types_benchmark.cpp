// SPDX-FileCopyrightText: GAFRO Extended Implementation
//
// SPDX-License-Identifier: MPL-2.0

#include <benchmark/benchmark.h>
#include <random>
#include <vector>

// Remove GAFRO dependency for now to avoid compilation issues
// #include <gafro/modern/GATerm.hpp>
// #include <gafro/modern/GradeIndexed.hpp>
// #include <gafro/modern/PatternMatching.hpp>

// Simplified test structures for benchmarking
namespace gafro::modern {

    template<typename T>
    struct SimpleScalar {
        T value;
        explicit SimpleScalar(T v) : value(v) {}
        SimpleScalar operator+(const SimpleScalar& other) const {
            return SimpleScalar(value + other.value);
        }
        SimpleScalar operator*(const SimpleScalar& other) const {
            return SimpleScalar(value * other.value);
        }
    };

    template<typename T>
    struct SimpleVector {
        std::vector<std::pair<int, T>> components;
        explicit SimpleVector(std::vector<std::pair<int, T>> c) : components(std::move(c)) {}

        SimpleVector operator+(const SimpleVector& other) const {
            auto result = components;
            for (const auto& [idx, coeff] : other.components) {
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
            return SimpleVector(std::move(result));
        }
    };

    template<typename T>
    using SimpleGATerm = std::variant<SimpleScalar<T>, SimpleVector<T>>;

} // namespace gafro::modern

using namespace gafro::modern;

// Global test data
std::mt19937 rng(42);
std::uniform_real_distribution<double> dist(-10.0, 10.0);

std::vector<SimpleScalar<double>> generateScalars(size_t count) {
    std::vector<SimpleScalar<double>> scalars;
    scalars.reserve(count);
    for (size_t i = 0; i < count; ++i) {
        scalars.emplace_back(dist(rng));
    }
    return scalars;
}

std::vector<SimpleVector<double>> generateVectors(size_t count, size_t components) {
    std::vector<SimpleVector<double>> vectors;
    vectors.reserve(count);
    for (size_t i = 0; i < count; ++i) {
        std::vector<std::pair<int, double>> comps;
        for (size_t j = 0; j < components; ++j) {
            comps.emplace_back(static_cast<int>(j), dist(rng));
        }
        vectors.emplace_back(std::move(comps));
    }
    return vectors;
}

std::vector<SimpleGATerm<double>> generateGATerms(size_t count) {
    std::vector<SimpleGATerm<double>> terms;
    terms.reserve(count);
    for (size_t i = 0; i < count; ++i) {
        if (i % 2 == 0) {
            terms.emplace_back(SimpleScalar<double>(dist(rng)));
        } else {
            std::vector<std::pair<int, double>> comps;
            for (int j = 0; j < 3; ++j) {
                comps.emplace_back(j, dist(rng));
            }
            terms.emplace_back(SimpleVector<double>(std::move(comps)));
        }
    }
    return terms;
}

/**
 * @brief Benchmark scalar operations
 */
static void BM_ScalarAddition(benchmark::State& state) {
    const size_t count = static_cast<size_t>(state.range(0));
    auto scalars1 = generateScalars(count);
    auto scalars2 = generateScalars(count);

    for (auto _ : state) {
        for (size_t i = 0; i < count; ++i) {
            auto result = scalars1[i] + scalars2[i];
            benchmark::DoNotOptimize(result);
        }
    }

    state.SetItemsProcessed(state.iterations() * count);
    state.SetBytesProcessed(state.iterations() * count * sizeof(SimpleScalar<double>) * 2);
}

static void BM_ScalarMultiplication(benchmark::State& state) {
    const size_t count = static_cast<size_t>(state.range(0));
    auto scalars1 = generateScalars(count);
    auto scalars2 = generateScalars(count);

    for (auto _ : state) {
        for (size_t i = 0; i < count; ++i) {
            auto result = scalars1[i] * scalars2[i];
            benchmark::DoNotOptimize(result);
        }
    }

    state.SetItemsProcessed(state.iterations() * count);
    state.SetBytesProcessed(state.iterations() * count * sizeof(SimpleScalar<double>) * 2);
}

/**
 * @brief Benchmark vector operations
 */
static void BM_VectorAddition(benchmark::State& state) {
    const size_t count = static_cast<size_t>(state.range(0));
    const size_t components = 3;
    auto vectors1 = generateVectors(count, components);
    auto vectors2 = generateVectors(count, components);

    for (auto _ : state) {
        for (size_t i = 0; i < count; ++i) {
            auto result = vectors1[i] + vectors2[i];
            benchmark::DoNotOptimize(result);
        }
    }

    state.SetItemsProcessed(state.iterations() * count);
    state.SetBytesProcessed(state.iterations() * count * sizeof(SimpleVector<double>) * 2);
}

/**
 * @brief Benchmark pattern matching
 */
static void BM_PatternMatching(benchmark::State& state) {
    const size_t count = static_cast<size_t>(state.range(0));
    auto terms = generateGATerms(count);

    for (auto _ : state) {
        for (size_t i = 0; i < count; ++i) {
            std::visit([](const auto& value) {
                using T = std::decay_t<decltype(value)>;
                if constexpr (std::is_same_v<T, SimpleScalar<double>>) {
                    return value.value * 2.0;
                } else {
                    return static_cast<double>(value.components.size());
                }
            }, terms[i]);
        }
    }

    state.SetItemsProcessed(state.iterations() * count);
    state.SetBytesProcessed(state.iterations() * count * sizeof(SimpleGATerm<double>));
}

/**
 * @brief Benchmark memory allocation patterns
 */
static void BM_VectorCreation(benchmark::State& state) {
    const size_t count = static_cast<size_t>(state.range(0));
    const size_t components = 3;

    for (auto _ : state) {
        for (size_t i = 0; i < count; ++i) {
            std::vector<std::pair<int, double>> comps;
            for (size_t j = 0; j < components; ++j) {
                comps.emplace_back(static_cast<int>(j), dist(rng));
            }
            SimpleVector<double> vector(std::move(comps));
            benchmark::DoNotOptimize(vector);
        }
    }

    state.SetItemsProcessed(state.iterations() * count);
}

static void BM_GATermCreation(benchmark::State& state) {
    const size_t count = static_cast<size_t>(state.range(0));

    for (auto _ : state) {
        for (size_t i = 0; i < count; ++i) {
            if (i % 2 == 0) {
                SimpleGATerm<double> term = SimpleScalar<double>(dist(rng));
                benchmark::DoNotOptimize(term);
            } else {
                std::vector<std::pair<int, double>> comps;
                for (int j = 0; j < 3; ++j) {
                    comps.emplace_back(j, dist(rng));
                }
                SimpleGATerm<double> term = SimpleVector<double>(std::move(comps));
                benchmark::DoNotOptimize(term);
            }
        }
    }

    state.SetItemsProcessed(state.iterations() * count);
}

/**
 * @brief Benchmark compilation-time vs runtime performance
 */
template<size_t N>
static void BM_CompileTimeGradeChecking(benchmark::State& state) {
    static_assert(N >= 0 && N <= 3, "Invalid grade");

    const size_t count = static_cast<size_t>(state.range(0));

    for (auto _ : state) {
        for (size_t i = 0; i < count; ++i) {
            // Simulate compile-time grade checking
            constexpr bool is_scalar = (N == 0);
            constexpr bool is_vector = (N == 1);
            constexpr bool is_bivector = (N == 2);
            constexpr bool is_trivector = (N == 3);

            double result = 0.0;
            if constexpr (is_scalar) {
                result = 1.0;
            } else if constexpr (is_vector) {
                result = 2.0;
            } else if constexpr (is_bivector) {
                result = 3.0;
            } else if constexpr (is_trivector) {
                result = 4.0;
            }

            benchmark::DoNotOptimize(result);
        }
    }

    state.SetItemsProcessed(state.iterations() * count);
}

static void BM_RuntimeGradeChecking(benchmark::State& state) {
    const size_t count = static_cast<size_t>(state.range(0));
    std::uniform_int_distribution<int> grade_dist(0, 3);

    for (auto _ : state) {
        for (size_t i = 0; i < count; ++i) {
            int grade = grade_dist(rng);

            double result = 0.0;
            switch (grade) {
                case 0: result = 1.0; break;
                case 1: result = 2.0; break;
                case 2: result = 3.0; break;
                case 3: result = 4.0; break;
                default: result = 0.0; break;
            }

            benchmark::DoNotOptimize(result);
        }
    }

    state.SetItemsProcessed(state.iterations() * count);
}

// Register benchmarks with different sizes
BENCHMARK(BM_ScalarAddition)->Range(8, 8<<10)->UseRealTime();
BENCHMARK(BM_ScalarMultiplication)->Range(8, 8<<10)->UseRealTime();
BENCHMARK(BM_VectorAddition)->Range(8, 8<<10)->UseRealTime();
BENCHMARK(BM_PatternMatching)->Range(8, 8<<10)->UseRealTime();
BENCHMARK(BM_VectorCreation)->Range(8, 8<<10)->UseRealTime();
BENCHMARK(BM_GATermCreation)->Range(8, 8<<10)->UseRealTime();

// Grade checking benchmarks
BENCHMARK_TEMPLATE(BM_CompileTimeGradeChecking, 0)->Range(8, 8<<10)->UseRealTime();
BENCHMARK_TEMPLATE(BM_CompileTimeGradeChecking, 1)->Range(8, 8<<10)->UseRealTime();
BENCHMARK_TEMPLATE(BM_CompileTimeGradeChecking, 2)->Range(8, 8<<10)->UseRealTime();
BENCHMARK_TEMPLATE(BM_CompileTimeGradeChecking, 3)->Range(8, 8<<10)->UseRealTime();
BENCHMARK(BM_RuntimeGradeChecking)->Range(8, 8<<10)->UseRealTime();

// Custom main function to add more benchmark configuration
int main(int argc, char** argv) {
    // Set custom benchmark configuration
    benchmark::Initialize(&argc, argv);

    if (benchmark::ReportUnrecognizedArguments(argc, argv)) {
        return 1;
    }

    // Print system information
    std::cout << "=== GAFRO Modern Types Benchmarks ===" << std::endl;
    std::cout << "Compiler: " <<
#ifdef __clang__
        "Clang " << __clang_major__ << "." << __clang_minor__ << "." << __clang_patchlevel__
#elif defined(__GNUC__)
        "GCC " << __GNUC__ << "." << __GNUC_MINOR__ << "." << __GNUC_PATCHLEVEL__
#elif defined(_MSC_VER)
        "MSVC " << _MSC_VER
#else
        "Unknown"
#endif
        << std::endl;

    std::cout << "C++ Standard: " << __cplusplus << std::endl;
    std::cout << "Mathematical Convention: τ (tau = 2π) = " << (2.0 * 3.141592653589793) << std::endl;
    std::cout << "=======================================" << std::endl;

    benchmark::RunSpecifiedBenchmarks();
    benchmark::Shutdown();

    return 0;
}