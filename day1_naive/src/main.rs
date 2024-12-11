use day1_naive::{parse_input, solve_puzzle};

fn main() {
    let input_bytes = include_bytes!("../../input/day1.txt");
    let len = input_bytes.len();
    let input_str = unsafe { std::str::from_utf8_unchecked(&input_bytes[..(len - 1)]) };
    let (mut left, mut right) = parse_input(input_str).unwrap();
    let output = solve_puzzle(&mut left, &mut right);
    println!("{output}");
}
