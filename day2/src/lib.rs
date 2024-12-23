use parse_ints::scalar::parse_until_newline;

// For my puzzle input there are 6488 numbers, and this takes 138us.
// That comes out to ~21ns per number. Not bad, but could probably
// be better. Maybe don't use the `.split` method and instead write
// my own loop that uses the number of bytes consumed by the
// `parse_until_newline` output.

pub fn parse_input(puzzle: &str) -> Vec<Vec<u32>> {
    let output = puzzle
        .split('\n')
        .map(|line| parse_until_newline(line.as_bytes()))
        .collect::<Vec<Vec<_>>>();
    // let total = output.iter().fold(0, |acc, new| acc + new.len());
    // eprintln!("{total}");
    output
}

fn line_is_safe(orig_line: &[u32]) -> bool {
    let len = orig_line.len();
    let line = &orig_line[..(len - 1)];
    let shifted = &orig_line[1..];
    let diffs = line
        .iter()
        .zip(shifted.iter())
        .map(|(x, y)| *x as i32 - *y as i32)
        .collect::<Vec<_>>();
    let valid_decreasing = diffs.iter().all(|x| (*x >= 1) && (*x <= 3));
    let valid_increasing = diffs.iter().all(|x| (*x >= -3) && (*x <= -1));
    valid_increasing || valid_decreasing
}

pub fn solve_puzzle_part1(lines: &[Vec<u32>]) -> u32 {
    let mut count = 0;
    for line in lines.iter() {
        if line_is_safe(line) {
            count += 1;
        }
    }
    count
}
