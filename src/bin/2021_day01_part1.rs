//! Advent of Code 2021 Day 01
//! https://adventofcode.com/2021/day/1
//!
//! Challenge part 1
//!
//! Read a file of integers and count the number of integers that are greater than the preceding
//! one.

use std::fs;

const INPUT_FILENAME: &str = "2021_day01_input.txt";

/// Takes an `input_file` of integers, one per line, and returns the number of integers that are
/// greater than the preceding one.
fn count_greater_ints(input_file: &str) -> u16 {
    input_file
        .lines()
        .map(|x| x.parse::<u16>().unwrap())
        .collect::<Vec<u16>>()
        .windows(2)
        .fold(0, |acc, x| if x[1] > x[0] { acc + 1 } else { acc })
}


fn main() {
    let input_file =
        fs::read_to_string(INPUT_FILENAME)
            .expect("Error reading input file");

    let greater_ints_total = count_greater_ints(&input_file);

    println!("{} integers are greater than their preceding integer", greater_ints_total);
}


// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
r#"199
200
208
210
200
207
240
269
260
263"#;

    #[test]
    fn verify_test_input() {
        assert_eq!(count_greater_ints(&TEST_INPUT), 7);
    }

    #[test]
    fn check_pairs() {
        assert_eq!(count_greater_ints("13\n26"), 1);
        assert_eq!(count_greater_ints("26\n13"), 0);
        assert_eq!(count_greater_ints("13\n13"), 0);
    }
}
