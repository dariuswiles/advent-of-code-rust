//! Advent of Code 2023 Day 01
//! https://adventofcode.com/2023/day/1
//!
//! Challenge part 1
//!
//! For each line of the input, find the first digit and the last digit. Concatenate these to form
//! a 2-digit number. If a line only contains one digit, it is used as both digits of the 2-digit
//! number.
//!
//! The challenge answer is the sum of all the 2-digit numbers.

use std::fs;

const INPUT_FILENAME: &str = "2023_day01_input.txt";

/// For each non-empty line of input, finds the first and last digit. These are concatenated to make
/// a 2-digit number, and a `Vec` contain the 2-digit number for each line is returned.
fn parse_input(input: &str) -> Vec<u32> {
    let mut calibration_values = Vec::new();

    for line in input.lines() {
        if line != "" {
            let digits: Vec<&str> = line.matches(char::is_numeric).collect();

            let first = digits.first().unwrap().parse::<u32>().unwrap();
            let last = digits.last().unwrap().parse::<u32>().unwrap();

            calibration_values.push(first * 10 + last);
        }
    }

    calibration_values
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    let input_as_ints = parse_input(&input);
    let answer: u32 = input_as_ints.iter().sum();

    println!(
        "The sum of all 2-digit numbers is {answer}",
    );
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_parse_input() {
        let two_digit_numbers = parse_input(TEST_INPUT);
        assert_eq!(two_digit_numbers, vec![12, 38, 15, 77]);
    }

    #[test]
    fn test_sum() {
        let two_digit_numbers = parse_input(TEST_INPUT);
        let answer: u32 = two_digit_numbers.iter().sum();

        assert_eq!(answer, 142);
    }
}
