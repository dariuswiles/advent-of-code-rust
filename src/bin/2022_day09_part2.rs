//! Advent of Code 2022 Day 09
//! https://adventofcode.com/2022/day/9
//!
//! Challenge part 2
//!
//! Reads an input file containing movement instructions (called "motions") for a rope, models the
//! position of all segments of the rope, and outputs the number of unique positions the tail
//! visited. Part 2 of the challenge extends the rope's length from 1 unit to 10.

use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;

const INPUT_FILENAME: &str = "2022_day09_input.txt";
const ROPE_LENGTH: usize = 10;

type Distance = u8;

#[derive(Clone, Debug, PartialEq)]
enum Motion {
    Down(Distance),
    Left(Distance),
    Right(Distance),
    Up(Distance),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i16,
    y: i16,
}

impl Position {
    fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Rope {
    knots: [Position; ROPE_LENGTH],
    history: HashSet<Position>,
}

impl Rope {
    fn new() -> Self {
        Self {
            knots: [Position::new(0, 0); ROPE_LENGTH],
            history: HashSet::from_iter(vec![Position::new(0, 0)]),
        }
    }

    /// Moves the head of the rope one unit at a time in the direction indicated by `motion`,
    /// then updates the following knots in the rope.
    fn execute_motion(&mut self, motion: &Motion) {
        match motion {
            Motion::Down(distance) => {
                for _ in 0..*distance {
                    self.knots[0].y -= 1;
                    self.update_tail();
                }
            }
            Motion::Left(distance) => {
                for _ in 0..*distance {
                    self.knots[0].x -= 1;
                    self.update_tail();
                }
            }
            Motion::Right(distance) => {
                for _ in 0..*distance {
                    self.knots[0].x += 1;
                    self.update_tail();
                }
            }
            Motion::Up(distance) => {
                for _ in 0..*distance {
                    self.knots[0].y += 1;
                    self.update_tail();
                }
            }
        }
    }

    /// Performs every `Motion` in the `motions` vector passed.
    fn execute_motions(&mut self, motions: &Vec<Motion>) {
        for motion in motions {
            self.execute_motion(motion);
        }
    }

    /// Examines the position of all knots in the rope except the first, and updates them if
    /// necessary to ensure they are all in adjacent positions. Records the position of the last
    /// knot in the rope.
    fn update_tail(&mut self) {
        for i in 0..ROPE_LENGTH - 1 {
            Self::update_knot(&self.knots[i].clone(), &mut self.knots[i + 1]);
        }

        self.history.insert(self.knots[ROPE_LENGTH - 1]);
    }

    /// Compares the positions of the two knots passed, where `leader` should be closer to the
    /// head of the rope than `follower`. If they are not adjacent, moves `follower` closer to
    /// `leader`. If they have the same `x` coordinates, only `follower`'s `y` coordinate
    /// is changed. If they have the same `y` coordinates, only `follower`'s `x` coordinate
    /// is changed. Otherwise `follower` moves diagonally.
    fn update_knot(leader: &Position, follower: &mut Position) {
        let rope_offset_horizontal = leader.x - follower.x;
        let rope_offset_vertical = leader.y - follower.y;

        // If `leader` and `follower` are in adjacent positions, `follower` does not need to be
        // moved.
        if i16::abs(rope_offset_horizontal) <= 1 && i16::abs(rope_offset_vertical) <= 1 {
            return;
        }

        follower.y += match rope_offset_vertical.cmp(&0) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            _ => 0,
        };

        follower.x += match rope_offset_horizontal.cmp(&0) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            _ => 0,
        };
    }
}

/// Takes a string containing the entire input file and converts it into vector of `Motion`s. Each
/// line of input must be a motion, e.g., "R 6" means "Right 6".
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> Vec<Motion> {
    let mut motion = Vec::new();

    for line in input.lines() {
        if !line.is_empty() {
            let tokens: Vec<&str> = line.split(' ').collect();
            assert_eq!(tokens.len(), 2);

            let distance = tokens[1].parse().unwrap();
            match tokens[0] {
                "D" => {
                    motion.push(Motion::Down(distance));
                }
                "L" => {
                    motion.push(Motion::Left(distance));
                }
                "R" => {
                    motion.push(Motion::Right(distance));
                }
                "U" => {
                    motion.push(Motion::Up(distance));
                }
                _ => {
                    panic!("Unrecognized motion instruction in input.");
                }
            }
        }
    }

    motion
}

/// Moves a `Rope` following the `motions` passed, and returns the number of unique positions that
/// the tail passed through.
fn challenge_answer(motions: &Vec<Motion>) -> usize {
    let mut rope = Rope::new();
    rope.execute_motions(motions);

    rope.history.len()
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    let motions = parse_input(&input);

    println!(
        "The rope tail passed through {} unique positions",
        challenge_answer(&motions)
    );
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_0: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    const TEST_INPUT_1: &str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test]
    fn test_parse_input() {
        let motions = parse_input(TEST_INPUT_0);

        assert_eq!(
            motions,
            vec![
                Motion::Right(4),
                Motion::Up(4),
                Motion::Left(3),
                Motion::Down(1),
                Motion::Right(4),
                Motion::Down(1),
                Motion::Left(5),
                Motion::Right(2),
            ]
        );
    }

    #[test]
    fn test_rope_execute_motion() {
        let mut rope = Rope::new();

        rope.execute_motion(&Motion::Right(4));
        assert_eq!(
            rope.knots,
            [
                Position { x: 4, y: 0 },
                Position { x: 3, y: 0 },
                Position { x: 2, y: 0 },
                Position { x: 1, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
            ]
        );

        rope.execute_motion(&Motion::Up(4));
        assert_eq!(
            rope.knots,
            [
                Position { x: 4, y: 4 },
                Position { x: 4, y: 3 },
                Position { x: 4, y: 2 },
                Position { x: 3, y: 2 },
                Position { x: 2, y: 2 },
                Position { x: 1, y: 1 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
            ]
        );

        rope.execute_motion(&Motion::Left(3));
        assert_eq!(
            rope.knots,
            [
                Position { x: 1, y: 4 },
                Position { x: 2, y: 4 },
                Position { x: 3, y: 3 },
                Position { x: 3, y: 2 },
                Position { x: 2, y: 2 },
                Position { x: 1, y: 1 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
            ]
        );

        rope.execute_motion(&Motion::Down(1));
        assert_eq!(
            rope.knots,
            [
                Position { x: 1, y: 3 },
                Position { x: 2, y: 4 },
                Position { x: 3, y: 3 },
                Position { x: 3, y: 2 },
                Position { x: 2, y: 2 },
                Position { x: 1, y: 1 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
            ]
        );

        rope.execute_motion(&Motion::Right(4));
        assert_eq!(
            rope.knots,
            [
                Position { x: 5, y: 3 },
                Position { x: 4, y: 3 },
                Position { x: 3, y: 3 },
                Position { x: 3, y: 2 },
                Position { x: 2, y: 2 },
                Position { x: 1, y: 1 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
            ]
        );

        rope.execute_motion(&Motion::Down(1));
        assert_eq!(
            rope.knots,
            [
                Position { x: 5, y: 2 },
                Position { x: 4, y: 3 },
                Position { x: 3, y: 3 },
                Position { x: 3, y: 2 },
                Position { x: 2, y: 2 },
                Position { x: 1, y: 1 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
            ]
        );

        rope.execute_motion(&Motion::Left(5));
        assert_eq!(
            rope.knots,
            [
                Position { x: 0, y: 2 },
                Position { x: 1, y: 2 },
                Position { x: 2, y: 2 },
                Position { x: 3, y: 2 },
                Position { x: 2, y: 2 },
                Position { x: 1, y: 1 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
            ]
        );

        rope.execute_motion(&Motion::Right(2));
        assert_eq!(
            rope.knots,
            [
                Position { x: 2, y: 2 },
                Position { x: 1, y: 2 },
                Position { x: 2, y: 2 },
                Position { x: 3, y: 2 },
                Position { x: 2, y: 2 },
                Position { x: 1, y: 1 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
            ]
        );
        assert_eq!(
            rope.history,
            HashSet::from_iter(vec![Position { x: 0, y: 0 },])
        );
    }

    #[test]
    fn test_rope_execute_motions() {
        let motions = parse_input(TEST_INPUT_1);
        let mut rope = Rope::new();
        rope.execute_motions(&motions);

        assert_eq!(
            rope.knots,
            [
                Position { x: -11, y: 15 },
                Position { x: -11, y: 14 },
                Position { x: -11, y: 13 },
                Position { x: -11, y: 12 },
                Position { x: -11, y: 11 },
                Position { x: -11, y: 10 },
                Position { x: -11, y: 9 },
                Position { x: -11, y: 8 },
                Position { x: -11, y: 7 },
                Position { x: -11, y: 6 },
            ]
        );
        assert_eq!(
            rope.history,
            HashSet::from_iter(vec![
                Position { x: 0, y: 0 },
                Position { x: 1, y: 1 },
                Position { x: 2, y: 2 },
                Position { x: 1, y: 3 },
                Position { x: 2, y: 4 },
                Position { x: 3, y: 5 },
                Position { x: 4, y: 5 },
                Position { x: 5, y: 5 },
                Position { x: 6, y: 4 },
                Position { x: 7, y: 3 },
                Position { x: 8, y: 2 },
                Position { x: 9, y: 1 },
                Position { x: 10, y: 0 },
                Position { x: 9, y: -1 },
                Position { x: 8, y: -2 },
                Position { x: 7, y: -3 },
                Position { x: 6, y: -4 },
                Position { x: 5, y: -5 },
                Position { x: 4, y: -5 },
                Position { x: 3, y: -5 },
                Position { x: 2, y: -5 },
                Position { x: 1, y: -5 },
                Position { x: 0, y: -5 },
                Position { x: -1, y: -5 },
                Position { x: -2, y: -5 },
                Position { x: -3, y: -4 },
                Position { x: -4, y: -3 },
                Position { x: -5, y: -2 },
                Position { x: -6, y: -1 },
                Position { x: -7, y: 0 },
                Position { x: -8, y: 1 },
                Position { x: -9, y: 2 },
                Position { x: -10, y: 3 },
                Position { x: -11, y: 4 },
                Position { x: -11, y: 5 },
                Position { x: -11, y: 6 },
            ])
        );
    }

    #[test]
    fn test_challenge_answer_0() {
        let tree = parse_input(TEST_INPUT_0);

        assert_eq!(challenge_answer(&tree), 1);
    }
    #[test]
    fn test_challenge_answer_1() {
        let tree = parse_input(TEST_INPUT_1);

        assert_eq!(challenge_answer(&tree), 36);
    }
}
