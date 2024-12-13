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

pub fn solve_puzzle_part1(left: &mut [u32], right: &mut [u32]) -> u32 {
    left.sort_unstable();
    right.sort_unstable();
    left.iter()
        .zip(right.iter())
        .fold(0, |mut summed_diff, (left_num, right_num)| {
            summed_diff += left_num.abs_diff(*right_num);
            summed_diff
        })
}

pub fn solve_puzzle_part2(left: &[u32], right: &[u32]) -> u32 {
    let mut sum = 0;
    let mut left_cursor = 0;
    let mut right_cursor = 0;
    let max_left_idx = left.len();
    let max_right_idx = right.len();
    let mut left_num = left[left_cursor];
    while (left_cursor < max_left_idx) && (right_cursor < max_right_idx) {
        let right_num = right[right_cursor];
        if left_num < right_num {
            left_cursor += 1;
            left_num = left[left_cursor];
            continue;
        }
        if left_num == right_num {
            sum += left_num;
            right_cursor += 1;
            continue;
        }
        right_cursor += 1;
    }
    sum
}
