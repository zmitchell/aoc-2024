#![feature(vec_into_raw_parts)]
use aoc_runner_derive::aoc_lib;

#[cfg(target_arch = "x86_64")]
mod day1;
#[cfg(target_arch = "x86_64")]
mod parse_ints;

aoc_lib! { year = 2024 }

// #[cfg(test)]
// mod tests {
//     use super::*;

// }
