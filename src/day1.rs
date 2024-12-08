use aoc_runner_derive::{aoc, aoc_generator};
use std::arch::x86_64::*;

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

/// The maximum number of digits each integer in a vector can have.
#[derive(PartialEq, Eq)]
enum DigitConversion {
    Zero,
    One,
    Two,
    Four,
    Eight,
}

struct ShuffleData {}

/// A byte of 0x80 tells the pshufb instruction to put a zero at the corresponding location.
/// We'll use this as the basis for shuffles and set individual bytes to particular values.
const ZERO_SHUFFLE: [u8; 16] = [
    0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
];

fn precompute_shuffle_table() -> Vec<ShuffleData> {
    let mut shuffles = vec![];
    for i in (0u16..u16::MAX) {
        todo!()
    }
    shuffles
}

fn parse_ints(puzzle: &[u8]) -> Result<Vec<i32>, Error> {
    todo!()
}
