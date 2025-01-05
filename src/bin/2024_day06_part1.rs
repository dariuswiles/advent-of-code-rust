//! Advent of Code 2020 Day 06
//! https://adventofcode.com/2020/day/6
//!
//! Challenge part 1
//!
//! Determine the number of distinct locations a guard visits while patrolling a rectanguler grid
//! containing obstacles. The guard walks straight until encountering an obstacle, at which point
//! they turn 90° right.

use std::collections::HashSet;
use std::fs;

const INPUT_FILENAME: &str = "2024_day06_input.txt";
const EMPTY: char = '.';
const GUARD: char = '^';
const OBSTACLE: char = '#';

type Position = (u8, u8);
type Obstacles = HashSet<Position>;

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    println!(
        "The guard visits {} distinct locations in the grid",
        do_challenge(&input)
    );
}

/// Returns the number of distinct locations a guard visits while patrolling a rectangular grid
/// containing obstacles.
fn do_challenge(input: &str) -> u32 {
    let (obstacles, guard, boundary) = parse_input(input);
    trace_guard_path(&obstacles, &guard, &boundary)
}

/// Returns a tuple containing: a `HashSet` of the `Position` of each obstacle; the `Position` of
/// the guard, and a tuple of the width and height of the grid.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> (Obstacles, Position, Position) {
    let mut obstacles = HashSet::new();
    let mut guard = None;
    let mut row = 0;
    let mut width = None;

    for line in input.lines() {
        if !line.is_empty() {
            if width.is_none() {
                width = Some(line.len());
            }

            for (col, c) in line.chars().enumerate() {
                match c {
                    EMPTY => {}
                    OBSTACLE => {
                        obstacles.insert((u8::try_from(col).unwrap(), row));
                    }
                    GUARD => match guard {
                        None => {
                            guard = Some((u8::try_from(col).unwrap(), row));
                        }
                        Some(_) => {
                            panic!("The guard's location must only appear once in the input");
                        }
                    },
                    _ => {
                        panic!("Input contains unrecognized character '{c}'");
                    }
                }
            }
            row += 1;
        }
    }

    (
        obstacles,
        guard.expect("The input contained no guard"),
        (u8::try_from(width.unwrap()).unwrap(), row),
    )
}

/// Returns the number of unique `Position`s the guard walks through, including their starting
/// location.
fn trace_guard_path(obstacles: &Obstacles, guard: &Position, boundary: &Position) -> u32 {
    enum Direction {
        East,
        North,
        South,
        West,
    }

    let mut path = HashSet::new();
    path.insert(*guard);
    let mut direction = Direction::North;
    let mut pos = *guard;
    let mut within_area = true;

    while within_area {
        match direction {
            Direction::East => {
                if pos.0 < boundary.0 - 1 {
                    match obstacles.get(&(pos.0 + 1, pos.1)) {
                        None => {
                            pos.0 += 1;
                        }
                        Some(_) => {
                            direction = Direction::South;
                        }
                    }
                } else {
                    within_area = false;
                }
            }
            Direction::North => {
                if pos.1 > 0 {
                    match obstacles.get(&(pos.0, pos.1 - 1)) {
                        None => {
                            pos.1 -= 1;
                        }
                        Some(_) => {
                            direction = Direction::East;
                        }
                    }
                } else {
                    within_area = false;
                }
            }
            Direction::South => {
                if pos.1 < boundary.1 - 1 {
                    match obstacles.get(&(pos.0, pos.1 + 1)) {
                        None => {
                            pos.1 += 1;
                        }
                        Some(_) => {
                            direction = Direction::West;
                        }
                    }
                } else {
                    within_area = false;
                }
            }

            Direction::West => {
                if pos.0 > 0 {
                    match obstacles.get(&(pos.0 - 1, pos.1)) {
                        None => {
                            pos.0 -= 1;
                        }
                        Some(_) => {
                            direction = Direction::North;
                        }
                    }
                } else {
                    within_area = false;
                }
            }
        }

        path.insert(pos);
    }

    path.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_parse_input() {
        let (obstacles, guard, boundary) = parse_input(INPUT);

        assert_eq!(8, obstacles.len());
        assert!(obstacles.contains(&(4, 0)));
        assert!(obstacles.contains(&(9, 1)));
        assert!(obstacles.contains(&(2, 3)));
        assert!(obstacles.contains(&(7, 4)));
        assert!(obstacles.contains(&(1, 6)));
        assert!(obstacles.contains(&(8, 7)));
        assert!(obstacles.contains(&(0, 8)));
        assert!(obstacles.contains(&(6, 9)));

        assert_eq!((4, 6), guard);
        assert_eq!((10, 10), boundary);
    }

    #[test]
    fn test_trace_guard_path() {
        let (obstacles, guard, boundary) = parse_input(INPUT);

        assert_eq!(41, trace_guard_path(&obstacles, &guard, &boundary));
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(41, do_challenge(INPUT));
    }
}
