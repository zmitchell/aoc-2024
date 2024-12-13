use criterion::{criterion_group, criterion_main, Criterion};
use day1::{self, parse_input, solve_puzzle_part1, solve_puzzle_part2};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_bytes!("../../input/day1.txt");
    let len = input.len();
    let input_str = std::str::from_utf8(&input[..(len - 1)]).unwrap();
    let (left, right) = parse_input(input_str).unwrap();
    let mut group = c.benchmark_group("day1");
    group.bench_function("parse_input", |b| {
        b.iter(|| parse_input(black_box(input_str)))
    });
    group.bench_function("solve_part1", |b| {
        b.iter(|| {
            let left = left.clone();
            let right = right.clone();
            solve_puzzle_part1(&mut black_box(left), &mut black_box(right))
        })
    });
    group.bench_function("solve_part2", |b| {
        b.iter(|| {
            let left = left.clone();
            let right = right.clone();
            solve_puzzle_part2(&black_box(left), &black_box(right))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
