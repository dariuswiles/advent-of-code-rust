//! Advent of Code 2020 Day 01
//! https://adventofcode.com/2020/day/1
//!
//! Challenge part 2
//!
//! Read an input file with one unsigned integer per line, find the three integers that add up to a
//! given value and print the result of multiplying those numbers.

use std::fs;

const INPUT_FILENAME: &str = "2020_day01_input.txt";
const REQUIRED_SUM: u32 = 2020;

/// Given a string containing a list of integers, one per line, and a `required_sum` find the three
/// integers that sum to that number. Return these as a tuple wrapped in an Option, or `None` if
/// no integers sum to `required_sum`.
fn find_sum_three(input_file: &str, required_sum: u32) -> Option<(u32, u32, u32)> {
    let input: Vec<u32> = input_file.lines().map(|s| s.parse().unwrap()).collect();

    let input_count = input.len();
    //     println!("Input contains {} numbers", input_count);

    let mut i_num: u32;
    let mut j_num: u32;
    let mut k_num: u32;
    for i in 0..input_count {
        for j in i..input_count {
            for k in j..input_count {
                i_num = *input.get(i).unwrap();
                j_num = *input.get(j).unwrap();
                k_num = *input.get(k).unwrap();

                if i_num + j_num + k_num == required_sum {
                    return Some((i_num, j_num, k_num));
                }
            }
        }
    }

    None
}

/// Returns the result of multiplying the three integers passed in the `integers` tuple.
fn product(integers: (u32, u32, u32)) -> u32 {
    integers.0 * integers.1 * integers.2
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    if let Some(r) = find_sum_three(&input, REQUIRED_SUM) {
        println!(
            "Integers {}, {} and {} sum to required total, and multiplying them gives {}",
            r.0,
            r.1,
            r.2,
            product(r)
        );
    } else {
        println!(
            "Error: Input did not contain three integers whose sum is {}",
            REQUIRED_SUM
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_0: &str = "1721
979
366
299
675
1456";

    #[test]
    fn find_answer_integers() {
        assert_eq!(find_sum_three(INPUT_0, REQUIRED_SUM), Some((979, 366, 675)));
    }

    #[test]
    fn find_answer_product() {
        assert_eq!(
            product(find_sum_three(INPUT_0, REQUIRED_SUM).unwrap()),
            241861950
        );
    }

    #[test]
    fn no_solution() {
        assert_eq!(find_sum_three(INPUT_0, 1456), None);
    }
}
