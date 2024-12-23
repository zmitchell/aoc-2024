use day2::{parse_input, solve_puzzle_part1};

fn main() {
    let input_bytes = include_bytes!("../../input/day2.txt");
    let len = input_bytes.len();
    let input_str = unsafe { std::str::from_utf8_unchecked(&input_bytes[..(len - 1)]) };
    let lines = parse_input(input_str);
    let count = solve_puzzle_part1(&lines);
    println!("{count}");
}
