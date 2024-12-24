//! Advent of Code 2021 Day 06
//! https://adventofcode.com/2021/day/6
//!
//! Challenge part 1
//!
//! Model lanternfish spawning to determine the number of fish that exist after a given number of
//! days.

use std::fs;

const INPUT_FILENAME: &str = "2021_day06_input.txt";
const CHALLENGE_DAYS: u32 = 80;

type Fish = u8;

/// Parses an input string consisting of comma-separated numbers representing the time until fish
/// spawn again.
fn parse_input(input: &str) -> Vec<Fish> {
    input.lines().collect::<Vec<&str>>()[0]
        .split(",")
        .map(|i| i.parse().unwrap())
        .collect()
}

/// Decrement the days to spawn value for every fish. If a fish is already at 0 days, restart their
/// cycle at 6 days and add a new fish with a cycle of 8 days.
fn decrement_fish(fish: &mut Vec<Fish>) {
    let mut spawn = 0;

    for f in fish.iter_mut() {
        if f == &0 {
            *f = 6;
            spawn += 1;
        } else {
            *f -= 1;
        }
    }

    for _ in 0..spawn {
        fish.push(8);
    }
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    let mut fish = parse_input(&input_file);

    for _ in 0..CHALLENGE_DAYS {
        decrement_fish(&mut fish);
    }

    println!(
        "The total number of fish after {} days is {}",
        CHALLENGE_DAYS,
        fish.len()
    );
}

// Test using data from the examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn parse_test_input() {
        let fish = parse_input(TEST_INPUT);

        assert_eq!(fish, vec![3, 4, 3, 1, 2]);
    }

    #[test]
    fn test_decrement() {
        let mut fish = parse_input(TEST_INPUT);

        for _ in 0..18 {
            decrement_fish(&mut fish);
        }

        assert_eq!(
            fish,
            vec![6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 0, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 8, 8, 8]
        );

        println!(
            "The total number of fish after {} days is {}",
            CHALLENGE_DAYS,
            fish.len()
        );
    }

    #[test]
    fn challenge_answer() {
        let mut fish = parse_input(TEST_INPUT);

        for _ in 0..CHALLENGE_DAYS {
            decrement_fish(&mut fish);
        }

        assert_eq!(fish.len(), 5934);
    }
}
