//! Advent of Code 2021 Day 17
//! https://adventofcode.com/2021/day/17
//!
//! Challenge part 2
//!
//! Determine the number of valid initial x and y velocity pairs that fire a probe into the target
//! area defined in the input data.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::RangeInclusive;

const INPUT_FILENAME: &str = "2021_day17_input.txt";
const X_INITIAL_BOUND: Velocity = 2000; // The lowest and highest initial velocities of x to try.
const Y_INITIAL_BOUND: Velocity = 2000; // The lowest and highest initial velocities of y to try.

type Velocity = i32;
type Position = i32;
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

    let x_input = tokens[2]
        .strip_prefix("x=")
        .unwrap()
        .strip_suffix(",")
        .unwrap();
    let y_input = tokens[3].strip_prefix("y=").unwrap();

    let x_tokens: Vec<&str> = x_input.split("..").collect();
    let y_tokens: Vec<&str> = y_input.split("..").collect();
    assert_eq!(x_tokens.len(), 2);
    assert_eq!(y_tokens.len(), 2);

    let x_start = Velocity::from_str_radix(x_tokens[0], 10).unwrap();
    let x_end = Velocity::from_str_radix(x_tokens[1], 10).unwrap();
    let y_start = Velocity::from_str_radix(y_tokens[0], 10).unwrap();
    let y_end = Velocity::from_str_radix(y_tokens[1], 10).unwrap();

    (
        RangeInclusive::new(x_start, x_end),
        RangeInclusive::new(y_start, y_end),
    )
}

/// Returns a `HashMap` containing the initial velocities of y that lead to the probe entering the
/// target area. The returned HashMap is indexed by the round the probe is within the target, and
/// the value is a Vec of the initial y velocities.
fn possible_y_velocities(y_range: &RangeInclusive<Velocity>) -> HashMap<Round, Vec<Velocity>> {
    let y_min = *y_range.start();

    let mut results = HashMap::new();
    for initial_y in -Y_INITIAL_BOUND..Y_INITIAL_BOUND {
        let mut round = 0;
        let mut y_pos = 0;
        let mut y_velocity = initial_y;

        while y_pos > y_min || y_velocity > 0 {
            round += 1;
            y_pos += y_velocity;
            y_velocity -= 1;

            if y_range.contains(&y_pos) {
                let results_entry = results.entry(round).or_insert(Vec::new());
                results_entry.push(initial_y);
            }
        }
    }
    results
}

/// Restricts the y_candidates passed to only those where the probe is within the target in the x
/// direction during the same round. Returns a copy of y_candidates, restricted down to only
/// entries meeting both x and y conditions, and with the initial value of x included. The returned
/// HashMap is indexed by the round the probe is within the target (in both x and y axes), and the
/// values are a tuple of the initial x velocity and initial y velocity.
fn restrict_y_candidates_with_valid_x(
    x_range: &RangeInclusive<Position>,
    y_candidates: &HashMap<Round, Vec<Velocity>>,
) -> HashSet<(Velocity, Velocity)> {
    let y_round_candidates: &HashSet<Round> = &y_candidates.keys().cloned().collect();
    let y_round_max = *y_round_candidates.iter().max().unwrap();

    let mut results = HashSet::new();
    for initial_x in -X_INITIAL_BOUND..X_INITIAL_BOUND {
        let mut round = 0;
        let mut x_pos = 0;
        let mut x_velocity = initial_x;

        while round <= y_round_max {
            round += 1;
            x_pos += x_velocity;
            x_velocity -= x_velocity.signum();

            if x_range.contains(&x_pos) && y_round_candidates.contains(&round) {
                for initial_y in &y_candidates[&round] {
                    results.insert((initial_x as Velocity, *initial_y as Velocity));
                }
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
fn challenge_answer(input: &str) -> usize {
    let (x_range, y_range) = parse_input(&input);
    let y_candidates = possible_y_velocities(&y_range);
    let xy_candidates = restrict_y_candidates_with_valid_x(&x_range, &y_candidates);

    xy_candidates.len()
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    println!(
        "The number of initial (x, y) velocities that land the within the target is {}",
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

        assert_eq!(y_candidates[&1], vec![-10, -9, -8, -7, -6, -5]);
        assert_eq!(y_candidates[&2], vec![-4, -3, -2]);
        assert_eq!(y_candidates[&3], vec![-2, -1]);
        assert_eq!(y_candidates[&4], vec![-1, 0]);
        assert_eq!(y_candidates[&5], vec![0, 1]);
        assert_eq!(y_candidates[&6], vec![1]);
        assert_eq!(y_candidates[&7], vec![2]);
        assert_eq!(y_candidates[&9], vec![3]);
        assert_eq!(y_candidates[&10], vec![4]);
        assert_eq!(y_candidates[&12], vec![5]);
        assert_eq!(y_candidates[&14], vec![6]);
        assert_eq!(y_candidates[&16], vec![7]);
        assert_eq!(y_candidates[&18], vec![8]);
        assert_eq!(y_candidates[&20], vec![9]);
    }

    #[test]
    fn test_xy_candidates() {
        let x_range = RangeInclusive::new(20, 30);
        let y_range = RangeInclusive::new(-10, -5);
        let y_candidates = possible_y_velocities(&y_range);
        let xy_candidates = restrict_y_candidates_with_valid_x(&x_range, &y_candidates);

        assert_eq!(xy_candidates.len(), 112); // Challenge answer
        assert_eq!(
            xy_candidates,
            vec![
                (23, -10),
                (25, -9),
                (27, -5),
                (29, -6),
                (22, -6),
                (21, -7),
                (9, 0),
                (27, -7),
                (24, -5),
                (25, -7),
                (26, -6),
                (25, -5),
                (6, 8),
                (11, -2),
                (20, -5),
                (29, -10),
                (6, 3),
                (28, -7),
                (8, 0),
                (30, -6),
                (29, -8),
                (20, -10),
                (6, 7),
                (6, 4),
                (6, 1),
                (14, -4),
                (21, -6),
                (26, -10),
                (7, -1),
                (7, 7),
                (8, -1),
                (21, -9),
                (6, 2),
                (20, -7),
                (30, -10),
                (14, -3),
                (20, -8),
                (13, -2),
                (7, 3),
                (28, -8),
                (29, -9),
                (15, -3),
                (22, -5),
                (26, -8),
                (25, -8),
                (25, -6),
                (15, -4),
                (9, -2),
                (15, -2),
                (12, -2),
                (28, -9),
                (12, -3),
                (24, -6),
                (23, -7),
                (25, -10),
                (7, 8),
                (11, -3),
                (26, -7),
                (7, 1),
                (23, -9),
                (6, 0),
                (22, -10),
                (27, -6),
                (8, 1),
                (22, -8),
                (13, -4),
                (7, 6),
                (28, -6),
                (11, -4),
                (12, -4),
                (26, -9),
                (7, 4),
                (24, -10),
                (23, -8),
                (30, -8),
                (7, 0),
                (9, -1),
                (10, -1),
                (26, -5),
                (22, -9),
                (6, 5),
                (7, 5),
                (23, -6),
                (28, -10),
                (10, -2),
                (11, -1),
                (20, -9),
                (14, -2),
                (29, -7),
                (13, -3),
                (23, -5),
                (24, -8),
                (27, -9),
                (30, -7),
                (28, -5),
                (21, -10),
                (7, 9),
                (6, 6),
                (21, -5),
                (27, -10),
                (7, 2),
                (30, -9),
                (21, -8),
                (22, -7),
                (24, -9),
                (20, -6),
                (6, 9),
                (29, -5),
                (8, -2),
                (27, -8),
                (30, -5),
                (24, -7)
            ]
            .iter()
            .cloned()
            .collect()
        );
    }

    #[test]
    fn test_challenge_answer() {
        assert_eq!(challenge_answer(&TEST_INPUT), 112);
    }
}
