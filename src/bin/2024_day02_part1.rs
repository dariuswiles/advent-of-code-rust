//! Advent of Code 2024 Day 02
//! https://adventofcode.com/2024/day/2
//!
//! Challenge part 1
//!
//! The input consists of a list of reports, one per line, where each report consists of several
//! numeric levels. The challenge is to determine how many reports have levels that either:
//!     - increase by between 1 and 3 (inclusive) each level; or
//!     - decrease by between 1 and 3 (inclusive) each level.

use std::cmp::Ordering;
use std::fs;

const INPUT_FILENAME: &str = "2024_day02_input.txt";

#[derive(Debug, Eq, PartialEq)]
struct Report {
    levels: Vec<u8>,
}

impl Report {
    /// Creates a new `Report` from the string passed. The string must be a space-separated sequence
    /// of numbers.
    ///
    /// # Panics
    ///
    /// Panics if the input is malformed or if no numbers are found in the string passed.
    fn from_str(levels_str: &str) -> Self {
        let levels: Vec<u8> = levels_str
            .split(' ')
            .filter_map(|s| s.parse().ok())
            .collect();

        assert!(!levels.is_empty());

        Self { levels }
    }

    /// Returns `true` if the `levels` in this `Report` are "safe", where the challenge defines safe
    /// as being a report where:
    ///     1. Every level is between 1 and 3 greater than the preceding level (inclusive); or
    ///     2. Every level is between 1 and 3 less than the preceding level (inclusive).
    fn is_safe(&self) -> bool {
        if self.levels.len() < 2 {
            return true;
        }

        // Create a `Vec` containing the differences between adjacent levels.
        let differences: Vec<_> = self
            .levels
            .windows(2)
            .map(|w| i16::from(w[1]) - i16::from(w[0]))
            .collect();

        match differences[0].cmp(&0) {
            Ordering::Greater => {
                differences.iter().min().unwrap() >= &1 && differences.iter().max().unwrap() <= &3
            }
            Ordering::Less => {
                differences.iter().min().unwrap() >= &-3 && differences.iter().max().unwrap() <= &-1
            }
            Ordering::Equal => false,
        }
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    println!(
        "The number of reports whose levels are safe is {}",
        do_challenge(&input)
    );
}

/// Parses the input into one `Report` per line. Returns the number of `Report`s that are considered
/// "safe", using the definition in the challenge.
fn do_challenge(input: &str) -> u64 {
    let reports = parse_input(input);

    let mut safe_report_count = 0;

    for r in reports {
        if r.is_safe() {
            safe_report_count += 1;
        }
    }

    safe_report_count
}

/// Reads the input, which is expected to consist of 5 space-separated numbers per line.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> Vec<Report> {
    let mut reports = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        reports.push(Report::from_str(line));
    }

    reports
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn create_report() {
        assert_eq!(
            Report::from_str("7 6 4 2 1"),
            Report {
                levels: vec![7, 6, 4, 2, 1]
            }
        );
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(TEST_INPUT),
            vec![
                Report {
                    levels: vec![7, 6, 4, 2, 1]
                },
                Report {
                    levels: vec![1, 2, 7, 8, 9]
                },
                Report {
                    levels: vec![9, 7, 6, 2, 1]
                },
                Report {
                    levels: vec![1, 3, 2, 4, 5]
                },
                Report {
                    levels: vec![8, 6, 4, 4, 1]
                },
                Report {
                    levels: vec![1, 3, 6, 7, 9]
                },
            ]
        );
    }

    #[test]
    fn report_is_safe() {
        assert!(Report::from_str("7 6 4 2 1").is_safe());
        assert!(!Report::from_str("1 2 7 8 9").is_safe());
        assert!(!Report::from_str("9 7 6 2 1").is_safe());
        assert!(!Report::from_str("1 3 2 4 5").is_safe());
        assert!(!Report::from_str("8 6 4 4 1").is_safe());
        assert!(Report::from_str("1 3 6 7 9").is_safe());
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(do_challenge(TEST_INPUT), 2);
    }
}
