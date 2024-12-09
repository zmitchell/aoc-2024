#[allow(unused_imports)]
use std::arch::x86_64::*;
use std::{
    io::{Read, Write},
    path::Path,
};

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

/// A byte of 0x80 tells the pshufb instruction to put a zero at the corresponding location.
/// We'll use this as the basis for shuffles and set individual bytes to particular values.
const ZERO_SHUFFLE: [u8; 16] = [
    0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
];

/// The location of a sequence of digits and how many digits there are.
struct DigitRange {
    start: usize,
    size: usize,
}

/// Which span size a number of digits falls into.
fn compute_conversion_size(digits: usize) -> usize {
    match digits {
        0 => 0,
        1 => 1,
        2 => 2,
        3 | 4 => 4,
        5 | 6 | 7 | 8 => 8,
        _ => panic!("can't convert integers with more than 8 digits"),
    }
}

/// How many ranges are consumable from a pattern and at what size.
#[derive(Clone, Default)]
struct ConsumableRanges {
    /// How many ranges are consumable from the pattern.
    n_ranges: usize,
    /// At what span size should the digits be consumed.
    conversion_size: usize,
}

/// Determine the ranges that are consumable from the total set of ranges
/// extracted from a pattern.
fn consumable_ranges(digit_ranges: &[DigitRange]) -> ConsumableRanges {
    let mut biggest_span_size = 0;
    let mut n_spans_at_biggest_size = 0;
    for span_size in [1usize, 2, 4, 8].iter() {
        let mut consumable_ranges_at_this_size = 0;
        for dr in digit_ranges.iter() {
            if dr.size <= *span_size {
                consumable_ranges_at_this_size += 1;
            } else {
                // Stop on the first digit range that won't fit in this span
                break;
            }
        }
        if (consumable_ranges_at_this_size > 0)
            && (consumable_ranges_at_this_size > n_spans_at_biggest_size)
            && (consumable_ranges_at_this_size * span_size <= 16)
        {
            biggest_span_size = *span_size;
            n_spans_at_biggest_size = consumable_ranges_at_this_size;
        }
    }
    ConsumableRanges {
        n_ranges: n_spans_at_biggest_size,
        conversion_size: biggest_span_size,
    }
}

/// Information about which ranges were extracted from a pattern, at what size,
/// and whether any bits from the pattern couldn't be consumed.
struct ExtractedPatternInfo {
    consumable_ranges: ConsumableRanges,
    digit_ranges: Vec<DigitRange>,
    incomplete_bits: usize,
}

/// Determine the ranges that are extractable from a pattern and whether any bits
/// couldn't be consumed.
fn extract_pattern_info(mut pattern: u16) -> ExtractedPatternInfo {
    let mut digit_ranges = vec![];
    // A sequence of set bits at the end of the pattern could be an incomplete
    // number, so we don't want to count those digits.
    let n_trailing_bits = pattern.trailing_ones();
    let max_len = 16;
    let mut shifted = 0;
    loop {
        // We need to bring a sequence of 1 bits to the front of the pattern,
        // so count the leading zeros and shift left by that amount.
        let useless_zeros = pattern.leading_zeros();
        shifted += useless_zeros;
        if (shifted + n_trailing_bits) >= max_len {
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
    if digit_ranges.is_empty() {
        return ExtractedPatternInfo {
            consumable_ranges: ConsumableRanges::default(),
            digit_ranges: Vec::new(),
            incomplete_bits: 0,
        };
    }
    let ranges_consumable = consumable_ranges(&digit_ranges);
    let incomplete_bits = if ranges_consumable.n_ranges < digit_ranges.len() {
        16 - digit_ranges[ranges_consumable.n_ranges].start
    } else {
        n_trailing_bits as usize
    };

    ExtractedPatternInfo {
        consumable_ranges: ranges_consumable,
        digit_ranges,
        incomplete_bits,
    }
}

/// From the consumable ranges of a pattern, generate a shuffle array that can be used
/// to parse them. Note that a 0x80 byte in the shuffle array means to place a zero in
/// the destination.
fn generate_shuffle_array(pat_info: &ExtractedPatternInfo) -> [u8; 16] {
    let mut shuffle_array = ZERO_SHUFFLE;
    if pat_info.consumable_ranges.n_ranges == 0 {
        return shuffle_array;
    }
    let conversion_size = pat_info.consumable_ranges.conversion_size;
    for i in 0..pat_info.consumable_ranges.n_ranges {
        let digit_range = pat_info.digit_ranges.get(i).unwrap();
        // Put the cursor at the next block of `conversion_size` bits
        let output_cursor = conversion_size * i;
        // We need to pad each set of digits with leading zeros to make it a consistent
        // number of digits for the entire vector. THE ZERO_SHUFFLE array is filled
        // with the bytes that generate zeros already, so we need to skip ahead and
        // only fill out the shuffle pattern bytes that correspond to the actual digits
        // in the source array.
        let zero_bits = conversion_size - digit_range.size;
        for j in 0..digit_range.size {
            let src = digit_range.start + j;
            let dest = output_cursor + zero_bits + j;
            // This cast to u8 should be safe because the value is an index into an array
            // of size 16, so we should never see a number larger than that.
            shuffle_array[dest] = src as u8;
        }
    }
    shuffle_array
}

/// A lookup table entry corresponding to a 16 bit pattern.
#[derive(Default, Clone, Copy)]
struct PatternData {
    /// The input array for the `pshufb` instruction
    shuffle_array: [u8; 16],
    /// How many trailing bits there are for this pattern
    skip: u8,
}

/// Generate a lookup table for shuffles of every 16 bit pattern.
fn generate_pattern_lookup_table() -> Vec<PatternData> {
    let mut lookup_table = vec![];
    for i in 0..=u16::MAX {
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

/// Serializes the lookup table as a flat array of bytes and saves it to
/// the provided path.
fn write_lookup_table(table: Vec<PatternData>, path: &Path) -> Result<(), Error> {
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
    file.write_all(&raw_bytes)?;
    Ok(())
}

/// Loads the lookup table from the provided path.
fn load_lookup_table(path: &Path) -> Result<Vec<PatternData>, Error> {
    let mut file = std::fs::OpenOptions::new().read(true).open(&path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let (ptr, _, capacity) = buffer.into_raw_parts();
    let lookup_table: Vec<PatternData> = unsafe {
        let ptr = ptr as *mut PatternData;
        Vec::from_raw_parts(ptr, 65536, capacity)
    };
    Ok(lookup_table)
}

#[inline]
fn load_slice_to_vector(bytes: &[u8]) -> __m128i {
    unsafe { _mm_loadu_si128(bytes.as_ptr() as *const __m128i) }
}

#[inline]
fn bitmask_to_vector(mask: u16) -> __m128i {
    unsafe { _mm_movm_epi8(mask) }
}

#[inline]
fn vector_to_bitmask(mask: __m128i) -> u16 {
    unsafe { _mm_movemask_epi8(mask) as u16 }
}

#[inline]
fn broadcast_to_vector(byte: u8) -> __m128i {
    unsafe { _mm_set1_epi8(byte as i8) }
}

/// Stores a 16 byte vector to a slice.
///
/// SAFETY: The slice must be at least 16 bytes long.
#[inline]
fn vector_to_slice(vector: __m128i, buffer: &mut [u8]) {
    let ptr = buffer.as_mut_ptr();
    unsafe {
        _mm_storeu_epi8(ptr as *mut i8, vector);
    }
}

/// Returns a mask of the locations of the digits
fn detect_digits(input: __m128i) -> __m128i {
    unsafe {
        let ascii_zero = _mm_set1_epi8(b'0' as i8);
        let ascii_nine = _mm_set1_epi8((b'9' + 1) as i8);
        let less_than_zero = _mm_cmplt_epi8(input, ascii_zero);
        let less_than_nine = _mm_cmplt_epi8(input, ascii_nine);
        // The name of this instruction is confusing, it's really "notand".
        // For arguments (x, y) it computes (~x) & y.
        _mm_andnot_si128(less_than_zero, less_than_nine)
    }
}

#[inline]
fn shuffle_digits(input: __m128i, pat: &PatternData) -> __m128i {
    let shuffle_vector = load_slice_to_vector(&pat.shuffle_array);
    unsafe { _mm_shuffle_epi8(input, shuffle_vector) }
}

fn parse_ints(bytes: &[u8], lookup_table: &[PatternData]) -> Vec<u16> {
    let vector_size = 16; // bytes
    let n_bytes = bytes.len();
    let mut cursor = 0;
    while cursor < n_bytes {
        let input = load_slice_to_vector(&bytes[cursor..(cursor + vector_size)]);
        let digit_vector_mask = detect_digits(input);
        let digit_bitmask = vector_to_bitmask(digit_vector_mask);
        let pattern_data = lookup_table[digit_bitmask as usize];
        let shuffled = shuffle_digits(input, &pattern_data);
    }
    todo!()
}

#[cfg(test)]
mod test {
    use super::{write_lookup_table, *};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn identifies_one_digit_conversion_size(idx in 1u16..=15) {
            let pattern = 1 << idx;
            let pat_info = extract_pattern_info(pattern);
            prop_assert_eq!(1, pat_info.consumable_ranges.conversion_size);
        }

        #[test]
        fn identifies_two_digit_conversion_size(idx in 1u16..=14) {
            let pattern = 3 << idx;
            let pat_info = extract_pattern_info(pattern);
            prop_assert_eq!(2, pat_info.consumable_ranges.conversion_size);
        }

        #[test]
        fn identifies_incomplete_number(n in 0u16..=15) {
            let pattern = 0xFF >> n;
            let pat_info = extract_pattern_info(pattern);
            prop_assert_eq!(0, pat_info.consumable_ranges.conversion_size);
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
            let shift_size = 8 - m;
            let pattern_m = 0xFF00 << shift_size; // set the highest m bits of the upper byte
            let shift_size = 8 - n;
            let pattern_n = (0xFF >> shift_size) << 1; // set the lowest n bits of the lower byte, then back one
            let pattern = pattern_n | pattern_m; // merge them
            let expected_digits = compute_conversion_size(m.max(n) as usize);
            let pat_info = extract_pattern_info(pattern);
            prop_assert_eq!(expected_digits as usize, pat_info.consumable_ranges.conversion_size);
        }

        #[test]
        fn detects_digits(mask in any::<u16>()) {
            // Sets the corresponding byte for each set bit in the mask
            let vector = bitmask_to_vector(mask);
            let ones = broadcast_to_vector(b'1');
            // Turn the bytes that are set into '1' chars
            let digits = unsafe {
              _mm_and_si128(vector, ones)
            };
            // Now we get a mask of the bytes that were digits,
            // which we'll turn back into a u16 mask to compare
            // against the one we started with.
            let detected_vector = detect_digits(digits);
            let detected_bitmask = unsafe {
                // I don't know why this returns an i32,
                // there are only 16 bytes in an __m128i
                _mm_movemask_epi8(detected_vector) as u16
            };
            prop_assert_eq!(mask, detected_bitmask);
        }
    }

    #[test]
    fn conversion_size_3x3_digits() {
        let pattern = 0b011101110111000;
        let pat_info = extract_pattern_info(pattern);
        assert_eq!(pat_info.consumable_ranges.n_ranges, 3);
        assert_eq!(pat_info.consumable_ranges.conversion_size, 4);
    }

    #[test]
    fn regression_pattern_conversion_size() {
        let pattern = 0b1000000000111110;
        let pat_info = extract_pattern_info(pattern);
        assert_eq!(pat_info.consumable_ranges.n_ranges, 2);
        assert_eq!(pat_info.consumable_ranges.conversion_size, 8);
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
    #[ignore = "don't write it"]
    fn dummy_test_write_lookup_table() {
        let table = generate_pattern_lookup_table();
        let path = std::env::current_dir()
            .unwrap()
            .join("input/2024/day1_part1_lookup_table.dat");
        write_lookup_table(table, &path).unwrap();
    }
}
