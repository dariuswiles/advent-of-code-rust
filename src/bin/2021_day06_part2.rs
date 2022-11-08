//! Advent of Code 2021 Day 06
//! https://adventofcode.com/2021/day/6
//!
//! Challenge part 2
//!
//! Model lanternfish spawning to determine the number of fish that exist after a given number of
//! days. Part 2 of the challenge increases the number of days to run the simulation, requiring
//! substantial changes to the Part 1 code.

use std::fs;

const INPUT_FILENAME: &str = "2021_day06_input.txt";
const CHALLENGE_DAYS: u32 = 256;
const STARTING_DAYS_TO_SPAWN: DaysToSpawn = 8; // For fish just born
const RESET_DAYS_TO_SPAWN: DaysToSpawn = 6; // For fish that have just spawned

type DaysToSpawn = u8;
type Fish = [u64; STARTING_DAYS_TO_SPAWN as usize + 1];

/// Parses an input string consisting of comma-separated numbers representing the time until fish
/// spawn again. The return value is an array where the array index is the *number* of fish that
/// have that number of days until they next spawn. For example, the index 0 contains the number
/// of fish that have 0 days until they next spawn.
fn parse_input(input: &str) -> Fish {
    let mut fish = [0; STARTING_DAYS_TO_SPAWN as usize + 1];

    let individual_fish = input.lines().collect::<Vec<&str>>()[0]
        .split(",")
        .map(|i| DaysToSpawn::from_str_radix(i, 10).unwrap());

    for i in individual_fish {
        fish[i as usize] += 1;
    }
    fish
}

/// Decrement the days to spawn value for every fish. If a fish is already at 0 days, restart their
/// cycle at 6 days and add a new fish with a cycle of 8 days.
fn decrement_fish(fish: &mut Fish) {
    let new_spawn = fish[0];

    for num_fish in 0..STARTING_DAYS_TO_SPAWN as usize {
        fish[num_fish] = fish[num_fish + 1];
    }

    fish[RESET_DAYS_TO_SPAWN as usize] += new_spawn;
    fish[STARTING_DAYS_TO_SPAWN as usize] = new_spawn;
}

/// Run the simulation for the given number of days and return the number of fish that exist at the
/// end of the process.
fn run_simulation(fish: &mut Fish, days: usize) -> u64 {
    for _ in 0..days {
        decrement_fish(fish);
    }

    fish.iter().fold(0, |acc, f| acc + f)
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    let mut fish = parse_input(&input_file);
    let result = run_simulation(&mut fish, CHALLENGE_DAYS as usize);
    println!(
        "The total number of fish after {} days is {}",
        CHALLENGE_DAYS, result
    );
}

// Test using data from the examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn parse_test_input() {
        let fish = parse_input(&TEST_INPUT);

        assert_eq!(fish, [0, 1, 1, 2, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn test_two_decrements() {
        let mut fish = parse_input(&TEST_INPUT);

        decrement_fish(&mut fish);
        assert_eq!(fish, [1, 1, 2, 1, 0, 0, 0, 0, 0]);

        decrement_fish(&mut fish);
        assert_eq!(fish, [1, 2, 1, 0, 0, 0, 1, 0, 1]);
    }

    #[test]
    fn test_18_decrements() {
        let mut fish = parse_input(&TEST_INPUT);

        for _ in 0..18 {
            decrement_fish(&mut fish);
        }

        assert_eq!(fish, [3, 5, 3, 2, 2, 1, 5, 1, 4]);
    }

    #[test]
    fn challenge_answer() {
        let mut fish = parse_input(&TEST_INPUT);

        assert_eq!(
            run_simulation(&mut fish, CHALLENGE_DAYS as usize),
            26984457539
        );
    }
}
