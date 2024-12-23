#![feature(iter_array_chunks)]
use parse_ints::simd::{cast_to_lookup_table, parse_ints};

pub fn parse_input(puzzle: &[u8]) -> (Vec<u32>, Vec<u32>) {
    let mut lookup_table = Vec::with_capacity(2 * 1024 * 1024);
    let lookup_table_bytes = include_bytes!("../../input/day1_part1_lookup_table.dat");
    lookup_table.extend_from_slice(lookup_table_bytes);
    let lookup_table = cast_to_lookup_table(lookup_table);
    let numbers = parse_ints(puzzle, &lookup_table);
    let (left, right) = numbers.into_iter().array_chunks::<2>().fold(
        (vec![], vec![]),
        |(mut left, mut right), [n_left, n_right]| {
            left.push(n_left);
            right.push(n_right);
            (left, right)
        },
    );
    (left, right)
}

pub fn solve_puzzle(left: &mut [u32], right: &mut [u32]) -> u32 {
    left.sort_unstable();
    right.sort_unstable();
    left.iter()
        .zip(right.iter())
        .fold(0, |mut summed_diff, (left_num, right_num)| {
            summed_diff += left_num.abs_diff(*right_num);
            summed_diff
        })
}
