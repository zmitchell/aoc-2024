use parse_ints::scalar::parse_until_newline;

pub fn parse_input(puzzle: &str) -> Vec<Vec<u32>> {
    let output = puzzle
        .split('\n')
        .map(|line| parse_until_newline(line.as_bytes()))
        .collect::<Vec<Vec<_>>>();
    // let total = output.iter().fold(0, |acc, new| acc + new.len());
    // eprintln!("{total}");
    output
}

// For my puzzle input there are 6488 numbers, and this takes 138us.
// That comes out to ~21ns per number. Not bad, but could probably
// be better. Maybe don't use the `.split` method and instead write
// my own loop that uses the number of bytes consumed by the
// `parse_until_newline` output.
