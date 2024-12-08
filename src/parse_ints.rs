#[allow(unused_imports)]
use std::arch::x86_64::*;
use std::ops::ShlAssign;

type Error = anyhow::Error;

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

fn determine_digit_conversion_size(mut pattern: u16) -> u32 {
    let mut digits_per_num = vec![];
    // A sequence of set bits at the end of the pattern could be an incomplete
    // number, so we don't want to count those digits.
    let n_possibly_incomplete_bits = pattern.trailing_ones();
    let max_len = 16;
    let mut shifted = 0;
    loop {
        // We need to bring a sequence of 1 bits to the front of the pattern,
        // so count the leading zeros and shift left by that amount.
        let useless_zeros = pattern.leading_zeros();
        shifted += useless_zeros;
        if (shifted + n_possibly_incomplete_bits) >= max_len {
            break;
        }
        pattern <<= useless_zeros;
        // Now that we have a sequence of set bits, count them.
        let n_digits = pattern.leading_ones();
        digits_per_num.push(n_digits);
        shifted += n_digits;
        pattern <<= n_digits;
    }
    digits_per_num.into_iter().max().unwrap_or(0)
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn identifies_single_digit_conversion_size(idx in 0u16..=15) {
            let pattern = 1 << idx;
            let digits_found = determine_digit_conversion_size(pattern);
            prop_assert_eq!(1, digits_found);
        }
    }
}
