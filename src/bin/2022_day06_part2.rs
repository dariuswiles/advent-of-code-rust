//! Advent of Code 2022 Day 06
//! https://adventofcode.com/2022/day/6
//!
//! Challenge part 2
//!
//! Finds the first occurrence of a sequence of 14 characters in the input where all 14
//! characters differ.

use std::collections::hash_set::HashSet;
use std::fs;

const INPUT_FILENAME: &str = "2022_day06_input.txt";

/// Returns true if all characters passed are different from each other.
fn all_unique(chars: Vec<char>) -> bool {
    let mut hs = HashSet::new();

    for c in chars {
        if !hs.insert(c) {
            return false;
        }
    }

    true
}

/// Finds the first sequence of 14 characters in the input that are all different from each other.
/// Returns the position of the last of the 14 characters, where the numbering starts at 1, as per
/// the challenge.
///
/// # Panics
///
/// Panics if the input does not contain a sequence of 14 different characters.
fn find_first_message_start(s: &str) -> usize {
    let w1: Vec<char> = s.chars().collect();
    for (idx, w) in w1.windows(14).enumerate() {
        if all_unique(w.to_vec()) {
            return idx + 14;
        }
    }

    panic!("A sequence of 14 different was not found in the input");
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    println!(
        "The challenge answer is {}",
        find_first_message_start(&input)
    );
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT0: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const TEST_INPUT1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const TEST_INPUT2: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const TEST_INPUT3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const TEST_INPUT4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_all_unique() {
        assert!(all_unique("abcdefghijklmn".chars().collect()));
        assert!(!all_unique("aacdefghijklmn".chars().collect()));
        assert!(!all_unique("abcdefahijklmn".chars().collect()));
        assert!(!all_unique("abcdefghijklma".chars().collect()));
        assert!(!all_unique("abcdafghijalan".chars().collect()));
        assert!(!all_unique("abcddfghijklmn".chars().collect()));
        assert!(!all_unique("abcdefghijkldn".chars().collect()));
        assert!(!all_unique("abcdefghijklbn".chars().collect()));
        assert!(!all_unique("abcdefggijklmn".chars().collect()));
        assert!(!all_unique("abcdefghijklnn".chars().collect()));
        assert!(!all_unique("aacdefghijklmn".chars().collect()));
        assert!(!all_unique("aaaaaaaaaaaaaa".chars().collect()));
    }

    #[test]
    fn test_find_first_message_start() {
        assert_eq!(find_first_message_start(TEST_INPUT0), 19);
        assert_eq!(find_first_message_start(TEST_INPUT1), 23);
        assert_eq!(find_first_message_start(TEST_INPUT2), 23);
        assert_eq!(find_first_message_start(TEST_INPUT3), 29);
        assert_eq!(find_first_message_start(TEST_INPUT4), 26);
    }
}
