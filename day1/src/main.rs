use day1::{parse_input, solve_puzzle_part1, solve_puzzle_part2};

fn main() {
    let input_bytes = include_bytes!("../../input/day1.txt");
    let len = input_bytes.len();
    let input_str = unsafe { std::str::from_utf8_unchecked(&input_bytes[..(len - 1)]) };
    let (mut left, mut right) = parse_input(input_str).unwrap();
    let part1 = solve_puzzle_part1(&mut left, &mut right);
    let part2 = solve_puzzle_part2(&left, &right);
    println!("{part1},{part2}");
}
