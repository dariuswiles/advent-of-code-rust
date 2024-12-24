//! Advent of Code 2022 Day 03
//! https://adventofcode.com/2022/day/3
//!
//! Challenge part 2
//!
//! Reads an input file representing items placed in backpacks, determines which items are in both
//! sides of each backpack and calculates the challenge answer based on this data.

use std::fs;

const INPUT_FILENAME: &str = "2022_day03_input.txt";

type Backpack<'a> = &'a str;

/// Takes a string containing the entire input file, where each line contains letters representing
/// items in a backpack, and returns a `Vec` containing this data.
///
/// # Panics
///
/// Panics if the input is malformed.
/// Panics if the number of backpacks is not divisible by 3.
fn parse_input(input: &str) -> Vec<Backpack> {
    let mut backpacks = Vec::new();

    for line in input.lines() {
        if !line.is_empty() {
            backpacks.push(line);
        }
    }

    assert!(backpacks.len() % 3 == 0);
    backpacks
}

/// Returns the first `char` in `first` that also appears in `second` and `third`. Returns `None`
/// if no `char` appears in all three strings.
fn find_common_item(first: &str, second: &str, third: &str) -> Option<char> {
    first
        .chars()
        .find(|&c| second.contains(c) && third.contains(c))
}

/// Returns the priority of the given `item`, following the challenge rules. Returns None if
/// `item` is not a letter.
fn item_priority(item: char) -> Option<u32> {
    if ('a' as u32..='z' as u32).contains(&(item as u32)) {
        return Some(item as u32 - 'a' as u32 + 1);
    }

    if ('A' as u32..='Z' as u32).contains(&(item as u32)) {
        return Some(item as u32 - 'A' as u32 + 27);
    }

    None
}

/// Returns the sum of the priorities for each common item for each backpack. Backpacks are
/// examined for common items in groups of 3, as per the challenge.
fn sum_all_item_priorities(backpacks: &[Backpack]) -> u32 {
    let mut total_priority = 0;

    for i in (0..backpacks.len()).step_by(3) {
        let common_item =
            find_common_item(backpacks[i], backpacks[i + 1], backpacks[i + 2]).unwrap();
        total_priority += item_priority(common_item).unwrap();
    }

    total_priority
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    let backpacks = parse_input(&input);

    println!(
        "The challenge answer is {}",
        sum_all_item_priorities(&backpacks)
    );
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn test_input_parsing() {
        let backpacks = parse_input(TEST_INPUT);

        assert_eq!(backpacks.len(), 6);
        assert_eq!(backpacks[3], "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
        assert_eq!(backpacks[5], "CrZsJsPPZsGzwwsLwLmpwMDw");
    }

    #[test]
    #[should_panic]
    fn test_input_parsing_malformed() {
        parse_input("abc\ndef");
    }

    #[test]
    fn test_find_common_item() {
        let mut backpacks = TEST_INPUT.lines();

        assert_eq!(
            find_common_item(
                backpacks.next().unwrap(),
                backpacks.next().unwrap(),
                backpacks.next().unwrap(),
            ),
            Some('r')
        );

        assert_eq!(
            find_common_item(
                backpacks.next().unwrap(),
                backpacks.next().unwrap(),
                backpacks.next().unwrap(),
            ),
            Some('Z')
        );
    }

    #[test]
    fn test_item_priority() {
        assert_eq!(item_priority('a'), Some(1));
        assert_eq!(item_priority('z'), Some(26));
        assert_eq!(item_priority('A'), Some(27));
        assert_eq!(item_priority('Z'), Some(52));
        assert_eq!(item_priority('4'), None);
    }

    #[test]
    fn test_sum_all_item_priorities() {
        let backpacks = parse_input(TEST_INPUT);

        assert_eq!(sum_all_item_priorities(&backpacks), 70);
    }
}
