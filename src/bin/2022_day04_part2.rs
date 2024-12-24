//! Advent of Code 2022 Day 04
//! https://adventofcode.com/2022/day/4
//!
//! Challenge part 2
//!
//! Reads an input file containing one pair of ranges per line and determines how many of these
//! pairs overlap.

use std::fs;
use std::ops::RangeInclusive;

const INPUT_FILENAME: &str = "2022_day04_input.txt";

/// Takes a string containing the entire input file, where each line contains a pair of
/// inclusive ranges. Returns a `Vec` containing pairs of ranges, one pair for each line of input.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    let mut ranges = Vec::new();

    for line in input.lines() {
        if !line.is_empty() {
            let both_ranges: Vec<&str> = line.split(',').collect();
            assert_eq!(both_ranges.len(), 2);

            let left: Vec<u32> = both_ranges[0]
                .split('-')
                .map(|n| n.parse().unwrap())
                .collect();
            let right: Vec<u32> = both_ranges[1]
                .split('-')
                .map(|n| n.parse().unwrap())
                .collect();

            assert_eq!(left.len(), 2);
            assert_eq!(right.len(), 2);

            ranges.push((left[0]..=left[1], right[0]..=right[1]));
        }
    }
    ranges
}

/// Returns `true` if the two ranges passed overlap, e.g., 3..=7 and 6..=8.
fn do_ranges_overlap(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    !(a.start() > b.end() || a.end() < b.start())
}

/// Returns the number of pairs of ranges in the `Vec` passed where the ranges overlap.
fn count_overlaps(range_pairs: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> u32 {
    range_pairs
        .iter()
        .filter(|rp| do_ranges_overlap(&rp.0, &rp.1))
        .count() as u32
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    let ranges = parse_input(&input);

    println!("The challenge answer is {}", count_overlaps(&ranges));
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test_input_parsing() {
        let ranges = parse_input(TEST_INPUT);

        assert_eq!(ranges[0], ((2..=4), (6..=8)));
        assert_eq!(ranges[1], ((2..=3), (4..=5)));
        assert_eq!(ranges[2], ((5..=7), (7..=9)));
        assert_eq!(ranges[3], ((2..=8), (3..=7)));
        assert_eq!(ranges[4], ((6..=6), (4..=6)));
        assert_eq!(ranges[5], ((2..=6), (4..=8)));
    }

    #[test]
    fn test_do_ranges_overlap() {
        assert!(do_ranges_overlap(&(1..=4), &(2..=5)));
        assert!(!do_ranges_overlap(&(3..=4), &(5..=5)));
        assert!(do_ranges_overlap(&(1..=6), &(2..=5)));
        assert!(do_ranges_overlap(&(4..=4), &(4..=9)));
        assert!(!do_ranges_overlap(&(7..=9), &(2..=5)));
    }

    #[test]
    fn test_count_overlaps() {
        let ranges = parse_input(TEST_INPUT);

        assert_eq!(count_overlaps(&ranges), 4);
    }
}
