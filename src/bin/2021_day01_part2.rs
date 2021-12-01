//! Advent of Code 2021 Day 01
//! https://adventofcode.com/2021/day/1
//!
//! Challenge part 2
//!
//! Reads a file of integers, one per line, and sums each consecutive set of three lines. The
//! summing is performed using a sliding window, so an input of 1, 2, 3, 4, 5 generates sums of
//! 6, 9 and 12. These sums are compared and the number of sums that are greater than the preceding
//! is returned as the challenge answer.

use std::fs;

const INPUT_FILENAME: &str = "2021_day01_input.txt";


/// Takes an `input_file` string that has one integer per line, sums each consecutive set of three
/// lines and returns the number of sums that are greater than the preceding one.
fn count_greater_ints(input_file: &str) -> u16 {
    input_file
        .lines()
        .map(|x| x.parse::<u16>().unwrap())
        .collect::<Vec<u16>>()
        .windows(3)
        .map(|x| x[0] + x[1] + x[2])
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
        assert_eq!(count_greater_ints(&TEST_INPUT), 5);
    }

    #[test]
    fn check_single_triple() {
        assert_eq!(count_greater_ints("1\n2\n3"), 0);
    }
}
