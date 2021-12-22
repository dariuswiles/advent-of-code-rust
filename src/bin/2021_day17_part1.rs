//! Advent of Code 2021 Day 17
//! https://adventofcode.com/2021/day/17
//!
//! Challenge part 1
//!
//! Determines the highest trajectory a probe can take and still end up within the target area
//! defined in the input data.

use std::collections::{ HashMap, HashSet };
use std::fs;
use std::ops::RangeInclusive;

const INPUT_FILENAME: &str = "2021_day17_input.txt";
const X_INITIAL_MAX: Velocity = 50;  // The highest initial velocity of x to try.
const Y_INITIAL_MAX: Velocity = 100;  // The highest initial velocity of y to try.

type Velocity = i16;
type Position = i16;
type Round = usize;


/// Returns a pair of inclusive ranges for x and y axes of the target area based on the given
/// string.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> (RangeInclusive<Position>, RangeInclusive<Position>) {
    let tokens: Vec<&str> = input.lines().next().unwrap().split(' ').collect();
    assert_eq!(tokens.len(), 4);

    let x_input = tokens[2].strip_prefix("x=").unwrap().strip_suffix(",").unwrap();
    let y_input = tokens[3].strip_prefix("y=").unwrap();

    let x_tokens: Vec<&str> = x_input.split("..").collect();
    let y_tokens: Vec<&str> = y_input.split("..").collect();
    assert_eq!(x_tokens.len(), 2);
    assert_eq!(y_tokens.len(), 2);

    let x_start = i16::from_str_radix(x_tokens[0], 10).unwrap();
    let x_end = i16::from_str_radix(x_tokens[1], 10).unwrap();
    let y_start = i16::from_str_radix(y_tokens[0], 10).unwrap();
    let y_end = i16::from_str_radix(y_tokens[1], 10).unwrap();

    (RangeInclusive::new(x_start, x_end), RangeInclusive::new(y_start, y_end))
}


/// Returns a `HashMap` containing information on initial velocities of y that lead to the probe
/// entering the target. The returned HashMap is indexed by the round the probe is within the
/// target, and the values are a tuple of the initial y velocity and highest y position achieved.
fn possible_y_velocities(y_range: &RangeInclusive<i16>) -> HashMap<Round, (Velocity, Position)> {
    let y_min = *y_range.start();

    let mut results: HashMap<Round, (Velocity, Position)> = HashMap::new();
    for initial_y in 2..Y_INITIAL_MAX {
        let mut round = 0;
        let mut y_pos = 0;
        let mut y_highest_pos = 0;
        let mut y_velocity = initial_y;

        while y_pos > y_min || y_velocity > 0 {
            round += 1;
            y_pos += y_velocity;
            y_highest_pos = y_highest_pos.max(y_pos);
            y_velocity -= 1;

            if y_range.contains(&y_pos) {
                if let Some(r) = results.get(&round) {
                    if r.1 < y_highest_pos {
                        results.insert(round, (initial_y, y_highest_pos));
                    }
                } else {
                    results.insert(round, (initial_y, y_highest_pos));
                }
            }
        }
    }
    results
}


/// Restricts the y_candidates passed to only those where the probe is within the target in the x
/// direction during the same round. Returns a copy of y_candidates, restricted down to only
/// entries meeting both x and y conditions, and with the initial value of x included. The returned
/// HashMap is indexed by the round the probe is within the target (in both x and y axes), and the
/// values are a tuple of the initial x velocity, initial y velocity and highest y position
/// achieved.
///
/// NOTE: the challenge allows negative initial values of x, but this code does not support this.
fn restrict_y_candidates_with_valid_x(
    x_range: &RangeInclusive<Position>,
    y_candidates: HashMap<Round, (Velocity, Position)>
) -> HashMap<Round, (Velocity, Velocity, Position)> {
    let y_round_candidates: HashSet<&Round> = y_candidates.keys().collect();
    let y_round_max = **y_round_candidates.iter().max().unwrap();

    let mut results = HashMap::new();
    for initial_x in 0..X_INITIAL_MAX {

        let mut round = 0;
        let mut x_pos = 0;
        let mut x_velocity = initial_x;

        while round <= y_round_max {
            round += 1;
            x_pos += x_velocity;
            x_velocity = 0.max(x_velocity - 1);

            if x_range.contains(&x_pos) && y_round_candidates.contains(&round) {
                results.insert(round, (initial_x, y_candidates[&round].0, y_candidates[&round].1));
            }
        }
    }
    results
}


/// Returns the answer to the challenge based on the target range definitions in the given input
/// file.
///
/// # Panics
///
/// Panics if the input is malformed or if a valid answer cannot be found.
fn challenge_answer(input: &str) -> Position {
    let (x_range, y_range) = parse_input(&input);
    let y_candidates = possible_y_velocities(&y_range);
    let xy_candidates = restrict_y_candidates_with_valid_x(&x_range, y_candidates);

    xy_candidates.values().map(|c| c.2).max().unwrap()
}


fn main() {
    let input_file =
        fs::read_to_string(INPUT_FILENAME)
            .expect("Error reading input file");

    println!("The highest y position that the probe can reach and pass through the target is {}",
        challenge_answer(&input_file)
    );
}


// Test using data from the examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn parse_test_input() {
        let (x_range, y_range) = parse_input(&TEST_INPUT);

        assert_eq!(x_range, RangeInclusive::new(20, 30));
        assert_eq!(y_range, RangeInclusive::new(-10, -5));
    }

    #[test]
    fn test_y_candidates() {
        let y_range = RangeInclusive::new(-10, -5);
        let y_candidates = possible_y_velocities(&y_range);

        assert_eq!(y_candidates[&7], (2, 3));
        assert_eq!(y_candidates[&9], (3, 6));
        assert_eq!(y_candidates[&10], (4, 10));
        assert_eq!(y_candidates[&12], (5, 15));
        assert_eq!(y_candidates[&14], (6, 21));
        assert_eq!(y_candidates[&16], (7, 28));
        assert_eq!(y_candidates[&18], (8, 36));
        assert_eq!(y_candidates[&20], (9, 45));
    }

    #[test]
    fn test_xy_candidates() {
        let x_range = RangeInclusive::new(20, 30);
        let y_range = RangeInclusive::new(-10, -5);
        let y_candidates = possible_y_velocities(&y_range);
        let xy_candidates = restrict_y_candidates_with_valid_x(&x_range, y_candidates);

        assert_eq!(xy_candidates[&7], (7, 2, 3));
        assert_eq!(xy_candidates[&9], (7, 3, 6));
        assert_eq!(xy_candidates[&10], (7, 4, 10));
        assert_eq!(xy_candidates[&12], (7, 5, 15));
        assert_eq!(xy_candidates[&14], (7, 6, 21));
        assert_eq!(xy_candidates[&16], (7, 7, 28));
        assert_eq!(xy_candidates[&18], (7, 8, 36));
        assert_eq!(xy_candidates[&20], (7, 9, 45));
    }

    #[test]
    fn test_challenge_answer() {
        assert_eq!(challenge_answer(&TEST_INPUT), 45);
    }
}
