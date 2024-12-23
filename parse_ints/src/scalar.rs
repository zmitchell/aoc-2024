use std::{
    arch::x86_64::{_mm_madd_epi16, _mm_maddubs_epi16, _mm_packus_epi32},
    simd::{u16x8, u8x16},
};

use crate::simd::{print_vec_u16, print_vec_u32, print_vec_u8};

const TWO_DIGITS: u8x16 =
    u8x16::from_array([10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1]);
const FOUR_DIGITS: u16x8 = u16x8::from_array([100, 1, 100, 1, 100, 1, 100, 1]);
// const EIGHT_DIGITS: u32x4 = u32x4::from_array([10000, 1, 10000, 1]);
// const SIXTEEN_DIGITS: u64x2 = u64x2::from_array([100000000, 1]);

pub fn parse_until_newline(input_raw: &[u8]) -> Vec<u32> {
    let mut output = Vec::new();
    let mut input = input_raw;
    let mut cursor = 0;
    while let Some(found) = extract_digits(input) {
        cursor += found.consumed;
        input = &input_raw[cursor..];
        let num = convert_digits(&found.digits);
        output.push(num);
    }
    output
}

#[derive(Debug)]
struct FoundNumber {
    consumed: usize,
    _n_digits: usize,
    digits: [u8; 16],
}

fn extract_digits(input: &[u8]) -> Option<FoundNumber> {
    let mut cursor = 0;
    let mut n_digits = 0;
    let newline = 13;
    let mut digits_array: [u8; 16] = [0; 16];
    for byte in input.iter() {
        cursor += 1;
        // End of input we're concerned about
        if *byte == newline {
            if n_digits > 0 {
                break;
            } else {
                return None;
            }
        }
        // Found a digit byte
        if (*byte <= b'9') && (*byte >= b'0') {
            digits_array[n_digits] = *byte - b'0';
            n_digits += 1;
            continue;
        }
        // Transition from digits to separators
        if n_digits > 0 {
            break;
        }
    }
    if n_digits > 0 {
        digits_array.rotate_right(16 - n_digits);
        Some(FoundNumber {
            consumed: cursor,
            _n_digits: n_digits,
            digits: digits_array,
        })
    } else {
        None
    }
}

fn convert_digits(digits: &[u8; 16]) -> u32 {
    let vector = u8x16::from_array(*digits);
    convert_four_digits(vector)[7] as u32
}

#[inline]
#[allow(dead_code)]
fn convert_two_digits(vector: u8x16) -> [u16; 8] {
    let two_converted = unsafe { _mm_maddubs_epi16(vector.into(), TWO_DIGITS.into()) };
    let portable = u16x8::from(two_converted);
    portable.to_array()
}

#[inline]
fn convert_four_digits(vector: u8x16) -> [u16; 8] {
    print_vec_u8(vector.into(), "input");
    let output_vec = unsafe {
        // Turns u8x16 into u16x8 in the process
        let two_converted = _mm_maddubs_epi16(vector.into(), TWO_DIGITS.into());
        print_vec_u16(two_converted, "two");
        // Turns u16x8 into u32x4 in the process
        let four_converted = _mm_madd_epi16(two_converted, FOUR_DIGITS.into());
        print_vec_u32(four_converted, "four");
        // Turn u32x4 back into u16x8
        _mm_packus_epi32(four_converted, four_converted)
    };
    print_vec_u16(output_vec, "output");
    let portable = u16x8::from(output_vec);
    portable.to_array()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn finds_leading_digits() {
        let input = "123 \n";
        let found = extract_digits(input.as_bytes()).unwrap();
        assert_eq!(found._n_digits, 3);
        assert_eq!(found.digits[13], 1);
        assert_eq!(found.digits[14], 2);
        assert_eq!(found.digits[15], 3);
    }
    #[test]
    fn finds_trailing_digits() {
        let input = "    123\n";
        let found = extract_digits(input.as_bytes()).unwrap();
        assert_eq!(found._n_digits, 3);
        assert_eq!(found.digits[13], 1);
        assert_eq!(found.digits[14], 2);
        assert_eq!(found.digits[15], 3);
    }

    #[test]
    fn finds_middle_digits() {
        let input = "   123 \n";
        let found = extract_digits(input.as_bytes()).unwrap();
        assert_eq!(found._n_digits, 3);
        assert_eq!(found.digits[13], 1);
        assert_eq!(found.digits[14], 2);
        assert_eq!(found.digits[15], 3);
    }

    #[test]
    fn converts_two_digits_raw() {
        let mut input = [0; 16];
        input[14] = 1;
        input[15] = 2;
        let vector = u8x16::from_array(input);
        let output = convert_two_digits(vector);
        assert_eq!(output[7], 12);
    }

    #[test]
    fn converts_four_digits_raw() {
        let mut input = [0; 16];
        input[12] = 1;
        input[13] = 2;
        input[14] = 3;
        input[15] = 4;
        let vector = u8x16::from_array(input);
        let output = convert_four_digits(vector);
        assert_eq!(output[7], 1234);
    }

    #[test]
    fn converts_1_digit() {
        let mut input = [0; 16];
        input[15] = 5;
        let output = convert_digits(&input);
        assert_eq!(5, output);
    }

    #[test]
    fn converts_2_digits() {
        let mut input = [0; 16];
        input[14] = 1;
        input[15] = 2;
        let output = convert_digits(&input);
        assert_eq!(12, output);
    }

    #[test]
    fn converts_3_digits() {
        let mut input = [0; 16];
        input[13] = 1;
        input[14] = 2;
        input[15] = 3;
        let output = convert_digits(&input);
        assert_eq!(123, output);
    }

    #[test]
    fn converts_4_digits() {
        let mut input = [0; 16];
        input[12] = 1;
        input[13] = 2;
        input[14] = 3;
        input[15] = 4;
        let output = convert_digits(&input);
        assert_eq!(1234, output);
    }

    #[test]
    fn terminates_at_end_of_input() {
        // Input with no newline character
        let input = " ";
        parse_until_newline(input.as_bytes());
    }

    #[test]
    fn parses_1_digit() {
        let input = "  1            \n";
        let output = parse_until_newline(input.as_bytes());
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], 1);
    }

    #[test]
    fn parses_2_digits() {
        let input = "  12            \n";
        let output = parse_until_newline(input.as_bytes());
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], 12);
    }

    #[test]
    fn parses_4_digits() {
        let input = "  1234            \n";
        let output = parse_until_newline(input.as_bytes());
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], 1234);
    }

    #[test]
    fn parses_multiple_numbers() {
        let input = "  1 23 456 7890           \n";
        let output = parse_until_newline(input.as_bytes());
        assert_eq!(output.len(), 4);
        assert_eq!(output[0], 1);
        assert_eq!(output[1], 23);
        assert_eq!(output[2], 456);
        assert_eq!(output[3], 7890);
    }
}
