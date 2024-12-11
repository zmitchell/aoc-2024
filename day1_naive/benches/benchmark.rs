use criterion::{criterion_group, criterion_main, Criterion};
use day1_naive::{self, parse_input, solve_puzzle};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_bytes!("../../input/day1.txt");
    let len = input.len();
    let input_str = std::str::from_utf8(&input[..(len - 1)]).unwrap();
    let (mut left, mut right) = parse_input(input_str).unwrap();
    c.bench_function("parse_input", |b| {
        b.iter(|| parse_input(black_box(input_str)))
    });
    c.bench_function("solve_puzzle", |b| {
        b.iter(|| {
            let mut left = left.clone();
            let mut right = right.clone();
            solve_puzzle(&mut black_box(left), &mut black_box(right))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
