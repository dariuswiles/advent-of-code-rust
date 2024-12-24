//! Advent of Code 2022 Day 01
//! https://adventofcode.com/2022/day/1
//!
//! Challenge part 2
//!
//! Sums each block of numbers in the input file and returns the largest sum.

use std::fs;

const INPUT_FILENAME: &str = "2022_day01_input.txt";

type Calories = u32;

/// Takes a string containing sets of numbers, one per line, with each block separated by one blank
/// lines. Returns a `Vec` containing each block. Each block consists of an inner `Vec` where each
/// element is one number.
fn parse_input(input: &str) -> Vec<Vec<Calories>> {
    let mut all_elves = Vec::new();
    let mut calories_vec = Vec::new();

    for line in input.lines() {
        if !line.is_empty() {
            calories_vec.push(line.parse().unwrap());
        } else {
            all_elves.push(calories_vec);
            calories_vec = Vec::new();
        }
    }

    if !calories_vec.is_empty() {
        all_elves.push(calories_vec);
    }

    all_elves
}

/// Takes blocks of numbers, sums the numbers of each block, and returns a `Vec` of these sums.
fn sum_calorie_blocks(blocks: &Vec<Vec<Calories>>) -> Vec<Calories> {
    let mut block_totals = Vec::new();

    for block in blocks {
        block_totals.push(block.iter().sum::<Calories>());
    }

    block_totals
}

/// Returns the largest 3 numbers in the `Vec` passed (which must contain more than 3 numbers).
///
/// # Panics
///
/// Panics if the `Vec` passed contains less than 3 elements.
fn largest_3(v: &[Calories]) -> Vec<Calories> {
    let v_len = v.len();
    assert!(v_len >= 3);

    let mut v_clone = v.to_owned();
    v_clone.sort_unstable();
    v_clone[v_len - 3..].to_vec()
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    let input_as_ints = parse_input(&input);
    let calories_per_elf = sum_calorie_blocks(&input_as_ints);

    println!(
        "The sum of the largest sum of elf calories is: {} calories",
        largest_3(&calories_per_elf).iter().sum::<Calories>()
    );
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const CALORIE_SETS: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_input_parsing() {
        let input_as_ints = parse_input(CALORIE_SETS);
        assert_eq!(
            input_as_ints,
            vec![
                vec![1000, 2000, 3000],
                vec![4000],
                vec![5000, 6000],
                vec![7000, 8000, 9000],
                vec![10000],
            ]
        );
    }

    #[test]
    fn test_sum_calorie_blocks() {
        let input_as_ints = parse_input(CALORIE_SETS);
        let elves = sum_calorie_blocks(&input_as_ints);

        assert_eq!(elves, vec![6000, 4000, 11000, 24000, 10000]);
    }

    #[test]
    fn test_largest_3() {
        let input_as_ints = parse_input(CALORIE_SETS);
        let elves = sum_calorie_blocks(&input_as_ints);
        let top3 = largest_3(&elves);

        assert_eq!(top3.len(), 3);
        assert!(top3.contains(&11000));
        assert!(top3.contains(&24000));
        assert!(top3.contains(&10000));
    }
}
