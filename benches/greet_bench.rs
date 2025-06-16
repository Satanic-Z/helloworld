//! Benchmarks for the HelloWorld library
//!
//! Run with: cargo bench

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use helloworld::{greet, greet_formal};

fn benchmark_greet(c: &mut Criterion) {
    c.bench_function("greet", |b| b.iter(|| greet(black_box("Benchmark"))));
}

fn benchmark_greet_formal(c: &mut Criterion) {
    c.bench_function("greet_formal", |b| {
        b.iter(|| greet_formal(black_box("Benchmark")))
    });
}

fn benchmark_greet_various_lengths(c: &mut Criterion) {
    let short_name = "A";
    let medium_name = "AverageLength";
    let long_name = "AVeryLongNameThatMightImpactPerformance";

    c.bench_function("greet_short", |b| b.iter(|| greet(black_box(short_name))));

    c.bench_function("greet_medium", |b| b.iter(|| greet(black_box(medium_name))));

    c.bench_function("greet_long", |b| b.iter(|| greet(black_box(long_name))));
}

criterion_group!(
    benches,
    benchmark_greet,
    benchmark_greet_formal,
    benchmark_greet_various_lengths
);
criterion_main!(benches);
