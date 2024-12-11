use aoc_runner_derive::{aoc, aoc_generator};
use std::arch::x86_64::*;

use crate::parse_ints::{cast_to_lookup_table, parse_ints};

type Error = anyhow::Error;

#[aoc_generator(day1, part1, naive)]
fn parse_input_day1(puzzle: &str) -> Result<(Vec<i32>, Vec<i32>), Error> {
    let (left, right) = puzzle
        .split('\n')
        .map(|line| {
            let mut nums_on_line = line.split_ascii_whitespace();
            let left = nums_on_line.next().unwrap();
            let right = nums_on_line.next().unwrap();
            (left, right)
        })
        .fold(
            (vec![], vec![]),
            |(mut acc_left, mut acc_right), (left, right)| {
                acc_left.push(left.parse::<i32>().unwrap());
                acc_right.push(right.parse::<i32>().unwrap());
                (acc_left, acc_right)
            },
        );
    Ok((left, right))
}

#[aoc(day1, part1, naive)]
fn solve_puzzle_day1_naive((left, right): &(Vec<i32>, Vec<i32>)) -> u32 {
    let mut left = left.clone();
    let mut right = right.clone();
    left.sort_unstable();
    right.sort_unstable();
    left.iter()
        .zip(right.iter())
        .fold(0, |mut summed_diff, (left_num, right_num)| {
            summed_diff += left_num.abs_diff(*right_num);
            summed_diff
        })
}

#[aoc_generator(day1, part1, simd_parser)]
fn parse_input_with_simd(puzzle: &[u8]) -> Result<(Vec<u32>, Vec<u32>), Error> {
    let mut lookup_table = Vec::with_capacity(2 * 1024 * 1024);
    let lookup_table_bytes = include_bytes!("../input/2024/day1_part1_lookup_table.dat");
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
    Ok((left, right))
}

#[aoc(day1, part1, simd_parser)]
fn solve_puzzle_day1_with_simd_parser((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut left = left.clone();
    let mut right = right.clone();
    left.sort_unstable();
    right.sort_unstable();
    left.iter()
        .zip(right.iter())
        .fold(0, |mut summed_diff, (left_num, right_num)| {
            summed_diff += left_num.abs_diff(*right_num);
            summed_diff
        })
}
