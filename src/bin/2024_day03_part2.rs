//! Advent of Code 2024 Day 03
//! https://adventofcode.com/2024/day/3
//!
//! Challenge part 2
//!
//! Finds all valid multiplication instructions in the input, where valid is defined as having the
//! form "mul(000,000)", where 000 is a number between 1 and 3 digits (inclusive). The challenge
//! answer is the sum of the result of each multiplication instruction.
//!
//! Part 2 adds "do" and "don't" keywords. The latter disables all multiplication instructions that
//! follow until the next "do".

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
/// where 000 is a number between 1 and 3 digits (inclusive). Ignores all instructions that follow
/// the "don't" keyword until a "do" keyword is encountered.
fn do_challenge(input: &str) -> u32 {
    let mut total = 0;

    for token_do in input.split("do") {
        if !token_do.starts_with("n't") {
            for token_mul in token_do.split("mul(") {
                if let Some((parameters, _)) = token_mul.split_once(')') {
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
        }
    }

    total
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_do_challenge() {
        assert_eq!(do_challenge(TEST_INPUT), 48);
    }
}
