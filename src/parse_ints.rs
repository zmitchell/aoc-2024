#[allow(unused_imports)]
use std::arch::x86_64::*;

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
    for i in 0u16..=u16::MAX {
        todo!()
    }
    shuffles
}

struct ExtractedPatternInfo {
    conversion_size: usize,
    digit_ranges: Vec<DigitRange>,
    incomplete_bits: u32,
}

struct DigitRange {
    start: usize,
    size: usize,
}

fn compute_conversion_size(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        2 => 2,
        3 | 4 => 4,
        5 | 6 | 7 | 8 => 8,
        _ => panic!("can't convert integers with more than 8 digits"),
    }
}

fn extract_pattern_info(mut pattern: u16) -> ExtractedPatternInfo {
    let mut digit_ranges = vec![];
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
        // Record the start position of the digits and how many there are.
        digit_ranges.push(DigitRange {
            start: shifted as usize,
            size: n_digits as usize,
        });
        shifted += n_digits;
        pattern <<= n_digits;
    }
    let max_digits = digit_ranges.iter().map(|dr| dr.size).max().unwrap_or(0);
    ExtractedPatternInfo {
        conversion_size: compute_conversion_size(max_digits),
        digit_ranges,
        incomplete_bits: n_possibly_incomplete_bits,
    }
}

fn generate_shuffle_array(pat_info: &ExtractedPatternInfo) -> [u8; 16] {
    eprintln!("cs = {}", pat_info.conversion_size);
    let mut shuffle_array = ZERO_SHUFFLE;
    for (i, digit_range) in pat_info.digit_ranges.iter().enumerate() {
        // Put the cursor at the next block of `conversion_size` bits
        let output_cursor = pat_info.conversion_size as usize * i;
        // We need to pad each set of digits with leading zeros to make it a consistent
        // number of digits for the entire vector. THE ZERO_SHUFFLE array is filled
        // with the bytes that generate zeros already, so we need to skip ahead and
        // only fill out the shuffle pattern bytes that correspond to the actual digits
        // in the source array.
        let zero_bits = pat_info.conversion_size as usize - digit_range.size;
        for j in 0..digit_range.size {
            let src = digit_range.start + j;
            let dest = output_cursor + zero_bits + j;
            eprintln!("cursor + zb + i = {output_cursor} + {zero_bits} + {j} = {dest}");
            // This cast to u8 should be safe because the value is an index into an array
            // of size 16, so we should never see a number larger than that.
            shuffle_array[dest] = src as u8;
        }
    }
    shuffle_array
}

#[derive(Default)]
struct PatternData {
    /// The input array for the `pshufb` instruction
    shuffle_array: [u8; 16],
    /// How many trailing bits there are for this pattern
    skip: u8,
}

fn generate_pattern_lookup_table() -> Vec<PatternData> {
    let mut lookup_table = vec![];
    for i in 0..=u16::MAX {
        eprintln!("pattern: {i:016b}");
        let extracted = extract_pattern_info(i);
        let shuffle = generate_shuffle_array(&extracted);
        let pattern_data = PatternData {
            shuffle_array: shuffle,
            // Safe conversion, more than 256 incomplete bits would mean
            // that we're operating on 256 byte vectors of digits,
            // but we're only operating on 16 byte vectors.
            skip: extracted.incomplete_bits as u8,
        };
        lookup_table.push(pattern_data);
    }
    lookup_table
}

#[cfg(test)]
mod test {
    use std::io::Write;

    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn identifies_one_digit_conversion_size(idx in 1u16..=15) {
            let pattern = 1 << idx;
            let digits_found = extract_pattern_info(pattern);
            prop_assert_eq!(1, digits_found.conversion_size);
        }

        #[test]
        fn identifies_two_digit_conversion_size(idx in 1u16..=14) {
            let pattern = 3 << idx;
            eprintln!("pattern: {pattern:016b}");
            let digits_found = extract_pattern_info(pattern);
            prop_assert_eq!(2, digits_found.conversion_size);
        }

        #[test]
        fn identifies_incomplete_number(n in 0u16..=15) {
            let pattern = 0xFF >> n;
            let digits_found = extract_pattern_info(pattern);
            prop_assert_eq!(0, digits_found.conversion_size);
        }

        #[test]
        fn identifies_two_different_sized_numbers(m in 1u16..=8, n in 1u16..=6) {
            // Input explanation:
            // - I want at most 8 upper bits set, the upper bit of the lower byte unset so it
            //   acts as a separator, and we can't have the lower bit set because it indicates
            //   an incomplete number.
            // - `m` can be all 8 upper bits
            // - If the highest and lowest bit of the lower byte must be unset, then we can have
            //   at most 6 bits set in the lower byte.
            eprintln!("(m, n): ({m}, {n})");
            let shift_size = 8 - m;
            eprintln!("starting m: {:016b}", 0xFF00);
            eprintln!("shift: {shift_size}");
            let pattern_m = 0xFF00 << shift_size; // set the highest m bits of the upper byte
            eprintln!("pattern m: {pattern_m:016b}");
            let shift_size = 8 - n;
            let pattern_n = (0xFF >> shift_size) << 1; // set the lowest n bits of the lower byte, then back one
            eprintln!("pattern n: {pattern_n:016b}");
            let pattern = pattern_n | pattern_m; // merge them
            eprintln!("input pattern: {pattern:016b}");
            let expected_digits = m.max(n);
            let found_digits = extract_pattern_info(pattern);
            prop_assert_eq!(expected_digits as usize, found_digits.conversion_size);
        }
    }

    #[test]
    fn single_one_digit_shuffle_pattern() {
        let pattern = 0x8000; // Just the top bit set
        let mut expected_shuffle = ZERO_SHUFFLE;
        expected_shuffle[0] = 0;
        let computed_shuffle = generate_shuffle_array(&extract_pattern_info(pattern));
        assert_eq!(expected_shuffle, computed_shuffle);
    }

    #[test]
    fn one_digit_two_digit_shuffle_pattern() {
        let pattern = 0b1001100000000000;
        let mut expected_shuffle = ZERO_SHUFFLE;
        // First byte for the first number will be a zero (0x80)
        expected_shuffle[1] = 0;
        // The two digits for the second number
        expected_shuffle[2] = 3;
        expected_shuffle[3] = 4;
        let computed_shuffle = generate_shuffle_array(&extract_pattern_info(pattern));
        assert_eq!(expected_shuffle, computed_shuffle);
    }

    #[test]
    fn generates_lookup_table() {
        generate_pattern_lookup_table();
    }

    #[test]
    #[ignore = "don't write it yet"]
    fn write_lookup_table() {
        let table = generate_pattern_lookup_table();
        let path = std::env::current_dir()
            .unwrap()
            .join("input/2024/day1_part1_lookup_table.dat");
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&path)
            .unwrap();
        let (ptr, length, capacity) = table.into_raw_parts();
        let n_bytes = length * size_of::<PatternData>();
        let raw_bytes: Vec<u8> = unsafe {
            let ptr = ptr as *mut u8;
            Vec::from_raw_parts(ptr, n_bytes, capacity)
        };
        file.write_all(&raw_bytes).unwrap();
    }
}
