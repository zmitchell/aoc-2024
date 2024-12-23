use criterion::{criterion_group, criterion_main, Criterion};
use day2::{self, parse_input};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_bytes!("../../input/day2.txt");
    let len = input.len();
    let input_str = std::str::from_utf8(&input[..(len - 1)]).unwrap();
    let _lines = parse_input(input_str);
    let mut group = c.benchmark_group("day2");
    group.bench_function("parse_input", |b| {
        b.iter(|| parse_input(black_box(input_str)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
