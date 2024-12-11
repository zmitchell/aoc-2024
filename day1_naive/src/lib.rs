type Error = anyhow::Error;

pub fn parse_input(puzzle: &str) -> Result<(Vec<u32>, Vec<u32>), Error> {
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
                acc_left.push(left.parse::<u32>().unwrap());
                acc_right.push(right.parse::<u32>().unwrap());
                (acc_left, acc_right)
            },
        );
    Ok((left, right))
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
