use day1_simd_parser::{parse_input, solve_puzzle};

fn main() {
    let input = include_bytes!("../../input/day1.txt");
    let (mut left, mut right) = parse_input(input.as_slice());
    let output = solve_puzzle(&mut left, &mut right);
    println!("{output}");
}
