//! Advent of Code 2023 Day 06
//! https://adventofcode.com/2023/day/6
//!
//! Challenge part 2
//!
//! The challenge is to determine how many ways a model boat race can be won. The race has a set
//! time in which the boat needs to travel as far as possible. The boat has a button that can be
//! pressed to increase its speed when the button is released and it sets off. The longer the button
//! is pressed the faster the boat travels for the remainder of the race time.
//!
//! The race has an associated current record longest distance. The challenge is to determine the
//! number of unique durations the button can pressed for the boat to exceeded this distance.
//!
//! Part 1 of the challenge treated space-delimited numbers as separate races. Part 2 states that
//! spaces should be ignored, concatenating multiple numbers down to a single race time and race
//! record distance.

use std::fs;

const INPUT_FILENAME: &str = "2023_day06_input.txt";

/// Stores the details of a single race, namely the duration of the race and the current record
/// longest distance.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    /// Creates and returns a new `Race` from the given input string. Spaces between numbers are
    /// ignored, as per part 2 of the challenge, giving a single large number for the time and for
    /// the distance.
    ///
    /// # Panics
    ///
    /// Panics if the input is malformed.
    fn from_str(s: &str) -> Self {
        let mut lines = s.lines();
        let time = parse_time(&lines.next().unwrap());
        let distance = parse_distance(&lines.next().unwrap());

        Self { time, distance }
    }

    /// Returns the number of ways the current record longest distance for this race (stored in
    /// `self.distance`), can be exceeded. This is determined by solving the quadratic equation
    /// defining the distance travelled, i.e.:
    ///     distance_travelled = b * (race_time - b)
    ///
    /// where 'b' is the length of time the button is held.
    //
    // The code from part 1 of the challenge generates the same result but is much slower due to
    // iterating through all possible values of time for holding the boat button down.
    fn count_winning_race_options(&self) -> u64 {
        let sqrt_term = ((self.time as f64).powi(2) - 4.0 * self.distance as f64).sqrt();

        let lower_bound = ((self.time as f64 - sqrt_term) / 2.0).ceil() as u64;
        let upper_bound = ((self.time as f64 + sqrt_term) / 2.0).floor() as u64;
        upper_bound - lower_bound + 1
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    println!(
        "The race can be won in {} different ways",
        do_challenge(&input)
    );
}

/// Returns the number of ways the record longest distance for the race passed as string input can
/// be exceeded.
fn do_challenge(input: &str) -> u64 {
    let race = Race::from_str(input);
    race.count_winning_race_options()
}

/// Returns the alloted race time in the given string, applying the new rule in part 2 of the
/// challenge that spaces between numbers should be ignored and the result treated as a single
/// number.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_time(time: &str) -> u64 {
    let t = time
        .strip_prefix("Time: ")
        .expect("The first line of input must begin with 'Time: '")
        .replace(' ', "");

    u64::from_str_radix(&t, 10).expect("Could not parse '{t}' as a time")
}

/// Returns the race distance in the given string, applying the new rule in part 2 of the challenge
/// that spaces between numbers should be ignored and the result treated as a single number.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_distance(distance: &str) -> u64 {
    let d = distance
        .strip_prefix("Distance: ")
        .expect("The second line of input must begin with 'Distance: '")
        .replace(' ', "");

    u64::from_str_radix(&d, 10).expect("Could not parse '{t}' as a distance")
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
            Race {
                time: 71530,
                distance: 940200,
            },
            Race::from_str(&TEST_INPUT)
        );
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(71503, do_challenge(&TEST_INPUT));
    }
}
