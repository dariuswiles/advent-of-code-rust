//! Advent of Code 2023 Day 06
//! https://adventofcode.com/2023/day/6
//!
//! Challenge part 1
//!
//! The challenge is to determine how many ways a number of model boat races can be won. Each race
//! has a set time in which the boats need to travel as far as possible. Each boat has a button
//! that can be pressed to increase the boats speed when the button is released and it sets off.
//! The longer the button is pressed the faster the boat travels, so every possible button pressing
//! time is considered to examine all possibilities.
//!
//! Each race has an associated current record longest distance. The challenge is to determine the
//! number of ways that can be exceeded for each race, and multiply all ways together to calculate
//! the answer.

use std::fs;
use std::iter::zip;

const INPUT_FILENAME: &str = "2023_day06_input.txt";

/// Stores the details of a single race, namely the duration of the race and the current record
/// longest distance.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    /// Returns the number of ways the current record longest distance for this race (stored in
    /// `self.distance`), can be exceeded. This is determined by calculating how far the boat goes
    /// in the time stored in `self.time` when pushing the button on the boat for each possible
    /// time value between 0 milliseconds and `self.time`.
    fn count_winning_race_options(&self) -> u32 {
        let mut total_distance_achieved = Vec::new();
        total_distance_achieved.push(0); // Pushing the button for 0 milliseconds

        for speed in 1..self.time {
            total_distance_achieved.push((self.time - speed) * speed);
        }

        total_distance_achieved
            .iter()
            .filter(|tda| **tda > self.distance)
            .count() as u32
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    println!(
        "The product of the number of ways each race can be run is {}",
        do_challenge(&input)
    );
}

/// Finds the number of ways the record longest distance for each race can be beaten and multiplies
/// them together to calculate the challenge answer, which is returned.
fn do_challenge(input: &str) -> u32 {
    let races = parse_input(input);

    races
        .iter()
        .fold(1, |product, r| product * r.count_winning_race_options())
}

/// Returns the given input as a `Vec` of `Race`s.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = parse_times(lines.next().unwrap());
    let distances = parse_distances(lines.next().unwrap());

    assert_eq!(
        times.len(),
        distances.len(),
        "Malformed input. There must be the same number of race times as race distances"
    );

    zip(times.iter(), distances.iter())
        .map(|(t, d)| Race {
            time: *t,
            distance: *d,
        })
        .collect()
}

/// Parses an input string containing race times and returns them as a `Vec`.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_times(times: &str) -> Vec<u32> {
    let t = times
        .strip_prefix("Time: ")
        .expect("The first line of input must begin with 'Time: '");

    let mut times = Vec::new();
    for token in t.split(' ') {
        if token.is_empty() {
            continue;
        }

        times.push(token.parse().expect("Could not parse '{token}' as a time"));
    }

    times
}

/// Parses an input string containing race distances and returns them as a `Vec`.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_distances(distances: &str) -> Vec<u32> {
    let d = distances
        .strip_prefix("Distance: ")
        .expect("The second line of input must begin with 'Distance: '");

    let mut distances = Vec::new();
    for token in d.split(' ') {
        if token.is_empty() {
            continue;
        }

        distances.push(
            token
                .parse()
                .expect("Could not parse '{token}' as a distance"),
        );
    }

    distances
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            vec![
                Race {
                    time: 7,
                    distance: 9
                },
                Race {
                    time: 15,
                    distance: 40
                },
                Race {
                    time: 30,
                    distance: 200
                },
            ],
            parse_input(TEST_INPUT)
        );
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(288, do_challenge(TEST_INPUT));
    }
}
