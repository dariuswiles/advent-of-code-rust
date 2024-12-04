//! Advent of Code 2024 Day 01
//! https://adventofcode.com/2024/day/1
//!
//! Challenge part 1
//!
//! The input consists of two columns of numbers. The challenge is to determine the difference
//! between the smallest number in the left column and the smallest number in the right column, then
//! the difference between the two second smallest numbers, etc. The challenge answer is the sum of
//! all differences.

use std::fs;
use std::iter::zip;

const INPUT_FILENAME: &str = "2024_day01_input.txt";

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    println!(
        "The total distance between the two columns of numbers is {}",
        do_challenge(&input)
    );
}

/// Parses the two columns of numbers in the input. Returns the challenge answer which is the
/// difference between the smallest number in the left column and the smallest number in the
/// right column, added to the second smallest numbers, etc.
fn do_challenge(input: &str) -> u64 {
    let number_pairs = parse_input(input);
    sum_distances(number_pairs)
}

/// Reads the input, which is expected to consist of one pair of integers on each line. Returns
/// the first column of integers in `left` and the second column in `right`.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let nums: Vec<&str> = line.split(' ').filter(|token| token != &"").collect();
        assert_eq!(nums.len(), 2, "Each line of input must contain exactly two numbers");

        left.push(nums[0].parse::<u32>().unwrap());
        right.push(nums[1].parse::<u32>().unwrap());
    }

    (left, right)
}

/// Takes a pair of `Vec`s of numbers, and determines the difference between the smallest pair of
/// numbers in the `Vec`s, the difference between the second smallest pair, etc. Returns the sum of
/// the differences.
///
/// # Panics
///
/// Panics if the `Vec`s are different lengths.
fn sum_distances(number_pairs: (Vec<u32>, Vec<u32>)) -> u64 {
    let (mut left, mut right) = number_pairs;

    assert_eq!(left.len(), right.len(),
        "The two columns of numbers must be the same length"
    );

    left.sort();
    right.sort();

    zip(left, right).map(|(x, y)| Into::<u64>::into(x.abs_diff(y))).sum()
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(&TEST_INPUT),
            (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])
        );
    }

    #[test]
    fn test_sum_distances() {
        assert_eq!(sum_distances((vec![3], vec![5])), 2);
        assert_eq!(sum_distances((vec![1, 3], vec![11, 28])), 35);
        assert_eq!(sum_distances((vec![3, 1], vec![11, 28])), 35);
        assert_eq!(sum_distances((vec![1, 3], vec![28, 11])), 35);
        assert_eq!(sum_distances((vec![3, 1], vec![28, 11])), 35);
        assert_eq!(sum_distances(parse_input(&TEST_INPUT)), 11);
    }
}
