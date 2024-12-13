use criterion::{criterion_group, criterion_main, Criterion};
use day1_simd_parser::{self, parse_input, solve_puzzle};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_bytes!("../../input/day1.txt");
    let (left, right) = parse_input(input);
    let mut group = c.benchmark_group("day1_simd_parser");
    group.bench_function("parse_input_simd_parser", |b| {
        b.iter(|| parse_input(black_box(input)))
    });
    group.bench_function("solve_puzzle_simd_parser", |b| {
        b.iter(|| {
            let left = left.clone();
            let right = right.clone();
            solve_puzzle(&mut black_box(left), &mut black_box(right))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
