// #![feature(portable_simd)]

use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

// Import your functions
use AdventOfCode2024::day_04::{second_part_one_line, second_part_one_line_simd};

fn benchmark_simd(c: &mut Criterion) {
    c.bench_function("example", |b| {
        b.iter(|| {
            // Your benchmarking logic here
            42
        });
    });
    // println!("hello there");
    let mut group = c.benchmark_group("Grid Processing");

    // Set the measurement time for more precise results
    // group.measurement_time(Duration::from_secs(10));

    // Benchmark the original scalar function
    group.bench_function("Scalar", |b| b.iter(|| second_part_one_line()));

    // Benchmark the SIMD implementation
    group.bench_function("SIMD", |b| b.iter(|| second_part_one_line_simd()));

    group.finish();
}

// Define the criterion group and main function
criterion_group!(benches, benchmark_simd);
criterion_main!(benches);
