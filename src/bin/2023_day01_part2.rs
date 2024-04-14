//! Advent of Code 2023 Day 01
//! https://adventofcode.com/2023/day/1
//!
//! Challenge part 2
//!
//! For each line of the input, find the first and last numbers on the line. Numbers can be
//! numerical digits, e.g., 6, and English language numbers from "one" to "nine", e.g., "six".
//! The first and last numbers found are concatenated to make a 2-digit number.
//!
//! The challenge answer is the sum of all the 2-digit numbers.

use std::fs;

const INPUT_FILENAME: &str = "2023_day01_input.txt";

const ALPHA_DIGITS: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    let input_as_ints = parse_input(&input);
    let answer: u32 = sum_vec_ints(&input_as_ints);

    println!("The sum of all 2-digit numbers is {answer}",);
}

/// For each non-empty line of input, finds the first and last digit looking for both digits and
/// English language versions, e.g., six.  These are concatenated to make a 2-digit number, and a
/// `Vec` containing the 2-digit number for each line is returned.
fn parse_input(input: &str) -> Vec<u8> {
    let mut calibration_values = Vec::new();

    for line in input.lines() {
        if line != "" {
            let first = find_first_number(&line).unwrap();
            let last = find_last_number(&line).unwrap();

            calibration_values.push(first * 10 + last);
        }
    }

    calibration_values
}

/// Returns the first number in the given string, regardless of whether it is a digit or the written
/// English of a digit, e.g., "one". Returns `None` if neither form of a digit is found.
fn find_first_number(s: &str) -> Option<u8> {
    let mut first_number = None;
    let mut first_number_pos = s.find(char::is_numeric);

    if first_number_pos.is_some() {
        let f = first_number_pos.unwrap();
        first_number = Some(s.get(f..=f).unwrap().parse::<u8>().unwrap());
    }

    // Skip 'zero' as it is never used in the challenge input
    for i in 1..ALPHA_DIGITS.len() {
        let matches: Vec<_> = s.match_indices(ALPHA_DIGITS[i]).collect();
        if matches.len() > 0 {
            if first_number_pos.is_none() || matches[0].0 < first_number_pos.unwrap() {
                first_number_pos = Some(matches[0].0);
                first_number = Some(i as u8);
            }
        }
    }

    first_number
}

/// Returns the last number in the given string, regardless of whether it is a digit or the written
/// English of a digit, e.g., "one". Returns `None` if neither form of a digit is found.
fn find_last_number(s: &str) -> Option<u8> {
    let mut last_number = None;
    let mut last_number_pos = s.rfind(char::is_numeric);

    if last_number_pos.is_some() {
        let f = last_number_pos.unwrap();
        last_number = Some(s.get(f..=f).unwrap().parse::<u8>().unwrap());
    }

    // Skip 'zero' as it is never used in the challenge input
    for i in 1..ALPHA_DIGITS.len() {
        let rmatches: Vec<_> = s.rmatch_indices(ALPHA_DIGITS[i]).collect();
        if rmatches.len() > 0 {
            if last_number_pos.is_none() || rmatches[0].0 > last_number_pos.unwrap() {
                last_number_pos = Some(rmatches[0].0);
                last_number = Some(i as u8);
            }
        }
    }

    last_number
}

/// Returns the sum of the integers in the `Vec` passed.
fn sum_vec_ints(vec_ints: &Vec<u8>) -> u32 {
    vec_ints.iter().map(|&n| n as u32).sum()
}

// Test data based partially on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_0: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const TEST_INPUT_1: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_find_first_number() {
        assert_eq!(Some(1), find_first_number("onetwo3fourfive"));
        assert_eq!(Some(1), find_first_number("abconetwo3fourfivedef"));
        assert_eq!(Some(1), find_first_number("1two3fourfivedef"));
        assert_eq!(Some(1), find_first_number("abc1two3fourfivedef"));
    }

    #[test]
    fn test_find_first_number_none() {
        assert_eq!(None, find_first_number("abcdefghi"));
        assert_eq!(None, find_first_number("ontwthrefoufivsiseveighnin"));
    }

    #[test]
    fn test_find_last_number() {
        assert_eq!(Some(5), find_last_number("onetwo3fourfive"));
        assert_eq!(Some(5), find_last_number("abconetwo3fourfivedef"));
        assert_eq!(Some(5), find_last_number("1two3fourfivedef"));
        assert_eq!(Some(5), find_last_number("abc1two3fourfivedef"));
    }

    #[test]
    fn test_find_last_number_none() {
        assert_eq!(None, find_last_number("abcdefghi"));
        assert_eq!(None, find_last_number("ontwthrefoufivsiseveighnin"));
    }

    #[test]
    fn test_parse_input_0() {
        let two_digit_numbers = parse_input(TEST_INPUT_0);
        assert_eq!(vec![12, 38, 15, 77], two_digit_numbers);
    }

    #[test]
    fn test_parse_input_1() {
        let two_digit_numbers = parse_input(TEST_INPUT_1);
        assert_eq!(vec![29, 83, 13, 24, 42, 14, 76], two_digit_numbers);
    }

    #[test]
    fn test_sum_vec_ints_0() {
        let two_digit_numbers = parse_input(TEST_INPUT_0);

        assert_eq!(142, sum_vec_ints(&two_digit_numbers));
    }

    #[test]
    fn test_sum_vec_ints_1() {
        let two_digit_numbers = parse_input(TEST_INPUT_1);

        assert_eq!(281, sum_vec_ints(&two_digit_numbers));
    }
}
