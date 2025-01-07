//! Advent of Code 2024 Day 06
//! https://adventofcode.com/2024/day/6
//!
//! Challenge part 2
//!
//! Determine the number of distinct locations a guard visits while patrolling a rectanguler grid
//! containing obstacles. The guard walks straight until encountering an obstacle, at which point
//! she turns 90° right.
//!
//! Part 2 of the challenge requires one obstacle to be added such that the guard's patrol route
//! turns into an endless loop. The challenge answer is the number of places the additional
//! obstacle can be added to cause a loop.

use std::collections::HashSet;
use std::fs;

const INPUT_FILENAME: &str = "2024_day06_input.txt";
const EMPTY: char = '.';
const GUARD: char = '^';
const OBSTACLE: char = '#';

type Position = (u8, u8);
type Obstacles = HashSet<Position>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    East,
    North,
    South,
    West,
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    println!(
        "Their are {} locations where an obstacle can be added to cause an endless patrol loop",
        do_challenge(&input)
    );
}

/// Returns the number of places an additional obstacle can be added to a guard's patrol route to
/// cause the route to become an endless loop.
fn do_challenge(input: &str) -> u32 {
    let (obstacles, guard, boundary) = parse_input(input);
    loop_guard_path(&obstacles, &guard, &boundary)
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

/// Returns the number of places an additional obstacle can be added to the `Obstacles` passed in
/// order for the guard's patrol route to become an endless loop.
fn loop_guard_path(obstacles: &Obstacles, guard: &Position, boundary: &Position) -> u32 {
    // Collects the set of positions the guard walks before any additional obstacles are added. A
    // set is used to eliminate duplicate positions caused by the guard visiting the same `Position`
    // while walking different `Direction`s.
    let guard_path_positions: HashSet<Position> = trace_guard_path(obstacles, guard, boundary)
        .unwrap()
        .iter()
        .map(|(pos, _)| *pos)
        .collect();

    let mut obs = obstacles.clone();

    // Adds an obstacle at each `Position` the guard walks on her original route, then checks if
    // this causes the modified route to be an endless loop. The obstacle is then removed before
    // the process is repeated.
    let mut loop_count = 0;
    for new_obs_pos in guard_path_positions {
        obs.insert(new_obs_pos);

        if trace_guard_path(&obs, guard, boundary).is_none() {
            loop_count += 1;
        }

        obs.remove(&new_obs_pos);
    }

    loop_count
}

/// Returns the unique `Position`s the guard visits while walking her route. It excludes the guard's
/// starting position. Returns `Some` and the number of positions visited if the guard exits the
/// patrol area, or `None` if she starts endlessly walking some part of her route.
fn trace_guard_path(
    obstacles: &Obstacles,
    guard: &Position,
    boundary: &Position,
) -> Option<HashSet<(Position, Direction)>> {
    let mut path = HashSet::new();
    let mut direction = Direction::North;
    let mut pos = *guard;

    loop {
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
                    break;
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
                    break;
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
                    break;
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
                    break;
                }
            }
        }

        if !path.insert((pos, direction)) {
            return None;
        }
    }

    Some(path)
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
        let guard_path = trace_guard_path(&obstacles, &guard, &boundary).unwrap();

        assert_eq!(54, guard_path.len());
        assert!(guard_path.contains(&((4, 5), Direction::North)));
        assert!(guard_path.contains(&((4, 4), Direction::North)));
        assert!(guard_path.contains(&((4, 3), Direction::North)));
        assert!(guard_path.contains(&((4, 2), Direction::North)));
        assert!(guard_path.contains(&((4, 1), Direction::North)));
        assert!(guard_path.contains(&((4, 1), Direction::East)));
        assert!(guard_path.contains(&((5, 1), Direction::East)));
        assert!(guard_path.contains(&((6, 1), Direction::East)));
        assert!(guard_path.contains(&((7, 1), Direction::East)));
        assert!(guard_path.contains(&((8, 1), Direction::East)));
        assert!(guard_path.contains(&((8, 1), Direction::South)));
        assert!(guard_path.contains(&((8, 2), Direction::South)));
        assert!(guard_path.contains(&((8, 3), Direction::South)));
        assert!(guard_path.contains(&((8, 4), Direction::South)));
        assert!(guard_path.contains(&((8, 5), Direction::South)));
        assert!(guard_path.contains(&((8, 6), Direction::South)));
        assert!(guard_path.contains(&((8, 6), Direction::West)));
        assert!(guard_path.contains(&((7, 6), Direction::West)));
        assert!(guard_path.contains(&((6, 6), Direction::West)));
        assert!(guard_path.contains(&((5, 6), Direction::West)));
        assert!(guard_path.contains(&((4, 6), Direction::West)));
        assert!(guard_path.contains(&((3, 6), Direction::West)));
        assert!(guard_path.contains(&((2, 6), Direction::West)));
        assert!(guard_path.contains(&((2, 6), Direction::North)));
        assert!(guard_path.contains(&((2, 5), Direction::North)));
        assert!(guard_path.contains(&((2, 4), Direction::North)));
        assert!(guard_path.contains(&((2, 4), Direction::East)));
        assert!(guard_path.contains(&((3, 4), Direction::East)));
        assert!(guard_path.contains(&((4, 4), Direction::East)));
        assert!(guard_path.contains(&((5, 4), Direction::East)));
        assert!(guard_path.contains(&((6, 4), Direction::East)));
        assert!(guard_path.contains(&((6, 4), Direction::South)));
        assert!(guard_path.contains(&((6, 5), Direction::South)));
        assert!(guard_path.contains(&((6, 6), Direction::South)));
        assert!(guard_path.contains(&((6, 7), Direction::South)));
        assert!(guard_path.contains(&((6, 8), Direction::South)));
        assert!(guard_path.contains(&((6, 8), Direction::West)));
        assert!(guard_path.contains(&((5, 8), Direction::West)));
        assert!(guard_path.contains(&((4, 8), Direction::West)));
        assert!(guard_path.contains(&((3, 8), Direction::West)));
        assert!(guard_path.contains(&((2, 8), Direction::West)));
        assert!(guard_path.contains(&((1, 8), Direction::West)));
        assert!(guard_path.contains(&((1, 8), Direction::North)));
        assert!(guard_path.contains(&((1, 7), Direction::North)));
        assert!(guard_path.contains(&((1, 7), Direction::East)));
        assert!(guard_path.contains(&((2, 7), Direction::East)));
        assert!(guard_path.contains(&((3, 7), Direction::East)));
        assert!(guard_path.contains(&((4, 7), Direction::East)));
        assert!(guard_path.contains(&((5, 7), Direction::East)));
        assert!(guard_path.contains(&((6, 7), Direction::East)));
        assert!(guard_path.contains(&((7, 7), Direction::East)));
        assert!(guard_path.contains(&((7, 7), Direction::South)));
        assert!(guard_path.contains(&((7, 8), Direction::South)));
        assert!(guard_path.contains(&((7, 9), Direction::South)));
    }

    #[test]
    fn test_loop_guard_path() {
        let (obstacles, guard, boundary) = parse_input(INPUT);

        assert_eq!(6, loop_guard_path(&obstacles, &guard, &boundary));
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(6, do_challenge(INPUT));
    }
}
