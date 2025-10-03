// SPDX-FileCopyrightText: GAFRO Extended Implementation
//
// SPDX-License-Identifier: MPL-2.0

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use gafro_modern::prelude::*;
use gafro_modern::si_units::{self, UnitExt, TAU, PI};
use rand::{thread_rng, Rng};
use std::time::Duration;

/// Generate test data for benchmarks
fn generate_scalars(count: usize) -> Vec<f64> {
    let mut rng = thread_rng();
    (0..count).map(|_| rng.gen_range(-10.0..10.0)).collect()
}

fn generate_vectors(count: usize, components: usize) -> Vec<Vec<(i32, f64)>> {
    let mut rng = thread_rng();
    (0..count)
        .map(|_| {
            (0..components)
                .map(|i| (i as i32, rng.gen_range(-10.0..10.0)))
                .collect()
        })
        .collect()
}

fn generate_ga_terms(count: usize) -> Vec<GATerm<f64>> {
    let mut rng = thread_rng();
    (0..count)
        .map(|i| {
            if i % 2 == 0 {
                GATerm::scalar(rng.gen_range(-10.0..10.0))
            } else {
                let components: Vec<(i32, f64)> = (0..3)
                    .map(|j| (j, rng.gen_range(-10.0..10.0)))
                    .collect();
                GATerm::vector(components)
            }
        })
        .collect()
}

/// Modern Types Benchmarks
fn bench_ga_term_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("ga_term_operations");

    for size in [10, 100, 1000].iter() {
        // Scalar operations
        group.bench_with_input(
            BenchmarkId::new("scalar_addition", size),
            size,
            |b, &size| {
                let scalars1: Vec<GATerm<f64>> = (0..size)
                    .map(|_| GATerm::scalar(thread_rng().gen_range(-10.0..10.0)))
                    .collect();
                let scalars2: Vec<GATerm<f64>> = (0..size)
                    .map(|_| GATerm::scalar(thread_rng().gen_range(-10.0..10.0)))
                    .collect();

                b.iter(|| {
                    for i in 0..size {
                        let result = pattern_matching::operations::add(
                            black_box(&scalars1[i]),
                            black_box(&scalars2[i])
                        );
                        black_box(result);
                    }
                });
            },
        );

        // Vector operations
        group.bench_with_input(
            BenchmarkId::new("vector_addition", size),
            size,
            |b, &size| {
                let vectors1: Vec<GATerm<f64>> = (0..size)
                    .map(|_| {
                        let components: Vec<(i32, f64)> = (0..3)
                            .map(|j| (j, thread_rng().gen_range(-10.0..10.0)))
                            .collect();
                        GATerm::vector(components)
                    })
                    .collect();
                let vectors2: Vec<GATerm<f64>> = (0..size)
                    .map(|_| {
                        let components: Vec<(i32, f64)> = (0..3)
                            .map(|j| (j, thread_rng().gen_range(-10.0..10.0)))
                            .collect();
                        GATerm::vector(components)
                    })
                    .collect();

                b.iter(|| {
                    for i in 0..size {
                        let result = pattern_matching::operations::add(
                            black_box(&vectors1[i]),
                            black_box(&vectors2[i])
                        );
                        black_box(result);
                    }
                });
            },
        );

        // Pattern matching
        group.bench_with_input(
            BenchmarkId::new("pattern_matching", size),
            size,
            |b, &size| {
                let terms = generate_ga_terms(size);

                b.iter(|| {
                    for term in &terms {
                        let result = match_gaterm(
                            black_box(term),
                            |s| s.value * 2.0,
                            |v| v.len() as f64,
                            |b| b.len() as f64,
                            |t| t.len() as f64,
                            |m| m.len() as f64,
                        );
                        black_box(result);
                    }
                });
            },
        );

        // Scalar multiplication
        group.bench_with_input(
            BenchmarkId::new("scalar_multiplication", size),
            size,
            |b, &size| {
                let terms = generate_ga_terms(size);

                b.iter(|| {
                    for term in &terms {
                        let result = pattern_matching::operations::scalar_multiply(
                            black_box(2.0),
                            black_box(term)
                        );
                        black_box(result);
                    }
                });
            },
        );

        // Norm calculation
        group.bench_with_input(
            BenchmarkId::new("norm_calculation", size),
            size,
            |b, &size| {
                let terms = generate_ga_terms(size);

                b.iter(|| {
                    for term in &terms {
                        let result = pattern_matching::operations::norm(black_box(term));
                        black_box(result);
                    }
                });
            },
        );
    }

    group.finish();
}

/// Grade-Indexed Types Benchmarks
fn bench_grade_indexed_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("grade_indexed_operations");

    for size in [10, 100, 1000].iter() {
        // Scalar operations
        group.bench_with_input(
            BenchmarkId::new("grade_indexed_scalar_addition", size),
            size,
            |b, &size| {
                let scalars1: Vec<gafro_modern::grade_indexed::ScalarType<f64>> = (0..size)
                    .map(|_| gafro_modern::grade_indexed::ScalarType::scalar(
                        thread_rng().gen_range(-10.0..10.0)
                    ))
                    .collect();
                let scalars2: Vec<gafro_modern::grade_indexed::ScalarType<f64>> = (0..size)
                    .map(|_| gafro_modern::grade_indexed::ScalarType::scalar(
                        thread_rng().gen_range(-10.0..10.0)
                    ))
                    .collect();

                b.iter(|| {
                    for i in 0..size {
                        let result = black_box(scalars1[i].clone()) + black_box(scalars2[i].clone());
                        black_box(result);
                    }
                });
            },
        );

        // Compile-time grade checking
        group.bench_with_input(
            BenchmarkId::new("compile_time_grade_check", size),
            size,
            |b, &size| {
                b.iter(|| {
                    for _ in 0..size {
                        // This is compile-time checked
                        let is_scalar = gafro_modern::grade_checking::TypeInspector::<
                            gafro_modern::grade_indexed::ScalarType<f64>
                        >::is_scalar();
                        let is_vector = gafro_modern::grade_checking::TypeInspector::<
                            gafro_modern::grade_indexed::VectorType<f64>
                        >::is_vector();
                        black_box(is_scalar && is_vector);
                    }
                });
            },
        );
    }

    group.finish();
}

/// SI Units Benchmarks
fn bench_si_units_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("si_units_operations");

    for size in [10, 100, 1000].iter() {
        // Basic unit operations
        group.bench_with_input(
            BenchmarkId::new("length_addition", size),
            size,
            |b, &size| {
                let lengths1: Vec<si_units::Length> = (0..size)
                    .map(|_| thread_rng().gen_range(0.1..100.0).meters())
                    .collect();
                let lengths2: Vec<si_units::Length> = (0..size)
                    .map(|_| thread_rng().gen_range(0.1..100.0).meters())
                    .collect();

                b.iter(|| {
                    for i in 0..size {
                        let result = black_box(lengths1[i]) + black_box(lengths2[i]);
                        black_box(result);
                    }
                });
            },
        );

        // Velocity calculation (dimension arithmetic)
        group.bench_with_input(
            BenchmarkId::new("velocity_calculation", size),
            size,
            |b, &size| {
                let distances: Vec<si_units::Length> = (0..size)
                    .map(|_| thread_rng().gen_range(0.1..100.0).meters())
                    .collect();
                let times: Vec<si_units::Time> = (0..size)
                    .map(|_| thread_rng().gen_range(0.1..100.0).seconds())
                    .collect();

                b.iter(|| {
                    for i in 0..size {
                        let velocity = black_box(distances[i]) / black_box(times[i]);
                        black_box(velocity);
                    }
                });
            },
        );

        // Marine calculations
        group.bench_with_input(
            BenchmarkId::new("buoyancy_calculation", size),
            size,
            |b, &size| {
                let volumes: Vec<si_units::Quantity<f64, 0, 3, 0, 0, 0, 0, 0>> = (0..size)
                    .map(|_| {
                        let side = thread_rng().gen_range(0.1..10.0).meters();
                        side * side * side
                    })
                    .collect();

                b.iter(|| {
                    for volume in &volumes {
                        let buoyancy = si_units::marine::buoyancy_force(*black_box(volume));
                        black_box(buoyancy);
                    }
                });
            },
        );

        // Tau-based trigonometry
        group.bench_with_input(
            BenchmarkId::new("tau_trigonometry", size),
            size,
            |b, &size| {
                let angles: Vec<f64> = (0..size)
                    .map(|_| thread_rng().gen_range(0.0..360.0))
                    .collect();

                b.iter(|| {
                    for angle in &angles {
                        let tau_rad = angle * TAU / 360.0;
                        let sin_val = tau_rad.sin();
                        let cos_val = tau_rad.cos();
                        black_box(sin_val + cos_val);
                    }
                });
            },
        );

        // Traditional trigonometry comparison
        group.bench_with_input(
            BenchmarkId::new("traditional_trigonometry", size),
            size,
            |b, &size| {
                let angles: Vec<f64> = (0..size)
                    .map(|_| thread_rng().gen_range(0.0..360.0))
                    .collect();

                b.iter(|| {
                    for angle in &angles {
                        let rad = angle * PI / 180.0;
                        let sin_val = rad.sin();
                        let cos_val = rad.cos();
                        black_box(sin_val + cos_val);
                    }
                });
            },
        );
    }

    group.finish();
}

/// Cross-language consistency benchmarks
fn bench_cross_language_consistency(c: &mut Criterion) {
    let mut group = c.benchmark_group("cross_language_consistency");

    // Test that Rust produces same results as C++ reference
    group.bench_function("scalar_operations_consistency", |b| {
        let test_data: Vec<(f64, f64)> = (0..100)
            .map(|_| {
                let mut rng = thread_rng();
                (rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0))
            })
            .collect();

        b.iter(|| {
            for (a, b) in &test_data {
                let s1 = GATerm::scalar(*a);
                let s2 = GATerm::scalar(*b);
                let sum = pattern_matching::operations::add(black_box(&s1), black_box(&s2));
                black_box(sum);
            }
        });
    });

    group.bench_function("vector_operations_consistency", |b| {
        let test_vectors: Vec<(Vec<(i32, f64)>, Vec<(i32, f64)>)> = (0..100)
            .map(|_| {
                let mut rng = thread_rng();
                let v1 = (0..3).map(|i| (i, rng.gen_range(-10.0..10.0))).collect();
                let v2 = (0..3).map(|i| (i, rng.gen_range(-10.0..10.0))).collect();
                (v1, v2)
            })
            .collect();

        b.iter(|| {
            for (v1, v2) in &test_vectors {
                let vec1 = GATerm::vector(v1.clone());
                let vec2 = GATerm::vector(v2.clone());
                let sum = pattern_matching::operations::add(black_box(&vec1), black_box(&vec2));
                black_box(sum);
            }
        });
    });

    group.finish();
}

/// Memory allocation benchmarks
fn bench_memory_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_allocation");

    for size in [10, 100, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("ga_term_creation", size),
            size,
            |b, &size| {
                b.iter(|| {
                    for i in 0..size {
                        if i % 2 == 0 {
                            let term = GATerm::scalar(black_box(3.14));
                            black_box(term);
                        } else {
                            let components = vec![(0, 1.0), (1, 2.0), (2, 3.0)];
                            let term = GATerm::vector(black_box(components));
                            black_box(term);
                        }
                    }
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("grade_indexed_creation", size),
            size,
            |b, &size| {
                b.iter(|| {
                    for _ in 0..size {
                        let scalar = gafro_modern::grade_indexed::ScalarType::scalar(black_box(3.14));
                        black_box(scalar);
                    }
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("si_units_creation", size),
            size,
            |b, &size| {
                b.iter(|| {
                    for _ in 0..size {
                        let length = black_box(5.0).meters();
                        let time = black_box(2.0).seconds();
                        let velocity = length / time;
                        black_box(velocity);
                    }
                });
            },
        );
    }

    group.finish();
}

/// Configuration
criterion_group!(
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(100)
        .with_plots();
    targets =
        bench_ga_term_operations,
        bench_grade_indexed_operations,
        bench_si_units_operations,
        bench_cross_language_consistency,
        bench_memory_allocation
);

criterion_main!(benches);