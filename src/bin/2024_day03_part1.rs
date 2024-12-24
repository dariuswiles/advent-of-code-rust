//! Advent of Code 2024 Day 03
//! https://adventofcode.com/2024/day/3
//!
//! Challenge part 1
//!
//! Finds all valid multiplication instructions in the input, where valid is defined as having the
//! form "mul(000,000)", where 000 is a number between 1 and 3 digits (inclusive). The challenge
//! answer is the sum of the result of each multiplication instruction.

use std::fs;

const INPUT_FILENAME: &str = "2024_day03_input.txt";

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    println!(
        "The sum of the result of each multiplication instruction is {}",
        do_challenge(&input)
    );
}

/// Finds all valid multiplication instructions in the given `input` and returns the sum of the
/// result of each multiplication instruction. Valid instructions have the form "mul(000,000)",
/// where 000 is a number between 1 and 3 digits (inclusive).
fn do_challenge(input: &str) -> u32 {
    let mut total = 0;

    for token in input.split("mul(") {
        if let Some((parameters, _)) = token.split_once(')') {
            if let Some((first_str, second_str)) = parameters.split_once(',') {
                if let (Ok(first), Ok(second)) =
                    (first_str.parse::<u32>(), second_str.parse::<u32>())
                {
                    if first < 1000 && second < 1000 {
                        total += first * second;
                    }
                }
            }
        }
    }

    total
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_do_challenge() {
        assert_eq!(do_challenge(TEST_INPUT), 161);
    }
}
