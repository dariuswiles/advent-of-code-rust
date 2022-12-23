//! Advent of Code 2022 Day 06
//! https://adventofcode.com/2022/day/6
//!
//! Challenge part 1
//!
//! Finds the first occurrence of a sequence of 4 characters in the input where all 4
//! characters differ.

use std::fs;

const INPUT_FILENAME: &str = "2022_day06_input.txt";

/// Returns true if all characters in the 4-character string slice passed are different.
///
/// # Panics
///
/// Panics if the slice passed is not 4 characters in length.
fn all_unique(slice: &[char]) -> bool {
    assert_eq!(slice.len(), 4);

    (slice[0] != slice[1])
        && (slice[0] != slice[2])
        && (slice[0] != slice[3])
        && (slice[1] != slice[2])
        && (slice[1] != slice[3])
        && (slice[2] != slice[3])
}

/// Finds the first sequence of 4 characters in the input that are all different from each other.
/// Returns the position of the last of the 4 characters, where the numbering starts at 1, as per
/// the challenge.
///
/// # Panics
///
/// Panics if the input does not contain a sequence of 4 different characters.
fn find_first_packet_start(s: &str) -> usize {
    let w1: Vec<char> = s.chars().collect();
    for (idx, w) in w1.windows(4).enumerate() {
        if all_unique(w) {
            return idx + 4;
        }
    }

    panic!("A sequence of 4 different was not found in the input");
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    println!(
        "The challenge answer is {}",
        find_first_packet_start(&input)
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
        assert!(all_unique(&['a', 'b', 'c', 'd']));
        assert!(!all_unique(&['a', 'b', 'a', 'd']));
        assert!(!all_unique(&['a', 'b', 'b', 'd']));
        assert!(!all_unique(&['a', 'b', 'c', 'a']));
        assert!(!all_unique(&['a', 'b', 'c', 'b']));
        assert!(!all_unique(&['a', 'b', 'c', 'c']));
        assert!(!all_unique(&['a', 'a', 'a', 'a']));
        assert!(!all_unique(&['z', 'a', 'a', 'a']));
        assert!(!all_unique(&['a', 'z', 'a', 'a']));
        assert!(!all_unique(&['a', 'a', 'z', 'a']));
        assert!(!all_unique(&['a', 'a', 'a', 'z']));
    }

    #[test]
    fn test_find_first_packet_start() {
        assert_eq!(find_first_packet_start(TEST_INPUT0), 7);
        assert_eq!(find_first_packet_start(TEST_INPUT1), 5);
        assert_eq!(find_first_packet_start(TEST_INPUT2), 6);
        assert_eq!(find_first_packet_start(TEST_INPUT3), 10);
        assert_eq!(find_first_packet_start(TEST_INPUT4), 11);
    }
}
