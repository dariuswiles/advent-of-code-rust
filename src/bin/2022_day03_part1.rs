//! Advent of Code 2022 Day 03
//! https://adventofcode.com/2022/day/3
//!
//! Challenge part 1
//!
//! Reads an input file representing items placed in backpacks, determines which items are in both
//! sides of each backpack and calculates the challenge answer based on this data.

use std::fs;

const INPUT_FILENAME: &str = "2022_day03_input.txt";

type BackpackItems<'a> = &'a str;
type Backpack<'a> = (BackpackItems<'a>, BackpackItems<'a>);

/// Takes a string containing the entire input file, where each line contains letters representing
/// items in a backpack. The first half of the letters on a line represent items in the first
/// partition of the backpack, and the rest of the letters are items in the second partition.
///
/// This function returns a `Vec` of `Backpack`s containing pairs of string slices for the two
/// partitions of each backpack.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> Vec<Backpack> {
    let mut backpacks = Vec::new();

    for line in input.lines() {
        if line != "" {
            backpacks.push((&line[..line.len() / 2], &line[line.len() / 2..]));
        }
    }
    backpacks
}

/// Returns the first `char` in `first` that also appears in `second`. Returns `None` if no `char`
/// appears in both strings.
fn find_common_item(first: &str, second: &str) -> Option<char> {
    for c in first.chars() {
        if second.contains(c) {
            return Some(c);
        }
    }
    None
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

/// Returns the sum of the priorities for each common item for each backpack.
fn sum_all_item_priorities(backpacks: &Vec<Backpack>) -> u32 {
    let mut total_priority = 0;

    for bp in backpacks {
        let common_item = find_common_item(bp.0, bp.1).unwrap();
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

    const EXPECTED_BACKPACKS: [Backpack; 6] = [
        ("vJrwpWtwJgWr", "hcsFMMfFFhFp"),
        ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"),
        ("PmmdzqPrV", "vPwwTWBwg"),
        ("wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn"),
        ("ttgJtRGJ", "QctTZtZT"),
        ("CrZsJsPPZsGz", "wwsLwLmpwMDw"),
    ];

    #[test]
    fn test_input_parsing() {
        let backpacks = parse_input(TEST_INPUT);

        assert_eq!(backpacks, EXPECTED_BACKPACKS.to_vec());
    }

    #[test]
    fn test_find_common_item() {
        assert_eq!(
            find_common_item(EXPECTED_BACKPACKS[0].0, EXPECTED_BACKPACKS[0].1),
            Some('p')
        );
        assert_eq!(
            find_common_item(EXPECTED_BACKPACKS[1].0, EXPECTED_BACKPACKS[1].1),
            Some('L')
        );
        assert_eq!(
            find_common_item(EXPECTED_BACKPACKS[2].0, EXPECTED_BACKPACKS[2].1),
            Some('P')
        );
        assert_eq!(
            find_common_item(EXPECTED_BACKPACKS[3].0, EXPECTED_BACKPACKS[3].1),
            Some('v')
        );
        assert_eq!(
            find_common_item(EXPECTED_BACKPACKS[4].0, EXPECTED_BACKPACKS[4].1),
            Some('t')
        );
        assert_eq!(
            find_common_item(EXPECTED_BACKPACKS[5].0, EXPECTED_BACKPACKS[5].1),
            Some('s')
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

        assert_eq!(sum_all_item_priorities(&backpacks), 157);
    }
}
