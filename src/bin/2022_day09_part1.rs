//! Advent of Code 2022 Day 09
//! https://adventofcode.com/2022/day/9
//!
//! Challenge part 1
//!
//! Reads an input file containing movement instructions (called "motions") for a short rope,
//! models the positions of the rope's head and tail, and outputs the number of unique positions
//! the tail visited.

use std::collections::HashSet;
use std::fs;

const INPUT_FILENAME: &str = "2022_day09_input.txt";

type Distance = u8;

#[derive(Clone, Debug, PartialEq)]
enum Motion {
    Down(Distance),
    Left(Distance),
    Right(Distance),
    Up(Distance),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
    head: Position,
    tail: Position,
    history: HashSet<Position>,
}

impl Rope {
    fn new() -> Self {
        Self {
            head: Position::new(0, 0),
            tail: Position::new(0, 0),
            history: HashSet::from_iter(vec![Position::new(0, 0)]),
        }
    }

    /// Moves `head` one unit at a time in the direction indicated by the `motion` and updates
    /// `tail` so that it is always adjacent.
    fn execute_motion(&mut self, motion: &Motion) {
        match motion {
            Motion::Down(distance) => {
                for _ in 0..*distance {
                    self.head.y -= 1;
                    self.update_tail();
                }
            }
            Motion::Left(distance) => {
                for _ in 0..*distance {
                    self.head.x -= 1;
                    self.update_tail();
                }
            }
            Motion::Right(distance) => {
                for _ in 0..*distance {
                    self.head.x += 1;
                    self.update_tail();
                }
            }
            Motion::Up(distance) => {
                for _ in 0..*distance {
                    self.head.y += 1;
                    self.update_tail();
                }
            }
        }
    }

    fn execute_motions(&mut self, motions: &Vec<Motion>) {
        for motion in motions {
            self.execute_motion(motion);
        }
    }

    /// Compares the positions of `head` and `tail` and if they are not adjacent, moves `tail`
    /// closer to `head`. If they have the same `x` coordinates, only `tail`'s `y` coordinate
    /// is changed. If they have the same `y` coordinates, only `tail`'s `x` coordinate
    /// is changed. Otherwise `tail` moves diagonally.
    fn update_tail(&mut self) {
        let rope_offset_horizontal = self.head.x - self.tail.x;
        let rope_offset_vertical = self.head.y - self.tail.y;

        // If `head` and `tail` are in adjacent positions, `tail` does not need to be moved.
        if i16::abs(rope_offset_horizontal) <= 1 && i16::abs(rope_offset_vertical) <= 1 {
            return;
        }

        if rope_offset_vertical < 0 {
            self.tail.y -= 1;
        } else if rope_offset_vertical > 0 {
            self.tail.y += 1;
        }

        if rope_offset_horizontal < 0 {
            self.tail.x -= 1;
        } else if rope_offset_horizontal > 0 {
            self.tail.x += 1;
        }

        self.history.insert(self.tail.clone());
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
        if line != "" {
            let tokens: Vec<&str> = line.split(' ').collect();
            assert_eq!(tokens.len(), 2);

            let distance = Distance::from_str_radix(tokens[1], 10).unwrap();
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
    rope.execute_motions(&motions);

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

    const TEST_INPUT: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn test_parse_input() {
        let motions = parse_input(TEST_INPUT);

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
        assert_eq!(rope.head, Position { x: 0, y: 0 });
        assert_eq!(rope.tail, Position { x: 0, y: 0 });
        assert_eq!(rope.history.len(), 1);
        assert!(rope.history.contains(&Position { x: 0, y: 0 }));

        rope.execute_motion(&Motion::Right(1));
        assert_eq!(rope.head, Position { x: 1, y: 0 });
        assert_eq!(rope.tail, Position { x: 0, y: 0 });
        assert_eq!(rope.history.len(), 1);
        assert!(rope.history.contains(&Position { x: 0, y: 0 }));

        rope.execute_motion(&Motion::Right(1));
        assert_eq!(rope.head, Position { x: 2, y: 0 });
        assert_eq!(rope.tail, Position { x: 1, y: 0 });
        assert_eq!(rope.history.len(), 2);
        assert!(rope.history.contains(&Position { x: 0, y: 0 }));
        assert!(rope.history.contains(&Position { x: 1, y: 0 }));

        rope.execute_motion(&Motion::Left(1));
        assert_eq!(rope.head, Position { x: 1, y: 0 });
        assert_eq!(rope.tail, Position { x: 1, y: 0 });
        assert_eq!(rope.history.len(), 2);
        assert!(rope.history.contains(&Position { x: 0, y: 0 }));
        assert!(rope.history.contains(&Position { x: 1, y: 0 }));

        rope.execute_motion(&Motion::Up(2));
        assert_eq!(rope.head, Position { x: 1, y: 2 });
        assert_eq!(rope.tail, Position { x: 1, y: 1 });
        assert_eq!(rope.history.len(), 3);
        assert!(rope.history.contains(&Position { x: 0, y: 0 }));
        assert!(rope.history.contains(&Position { x: 1, y: 0 }));
        assert!(rope.history.contains(&Position { x: 1, y: 1 }));

        rope.execute_motion(&Motion::Left(2));
        assert_eq!(rope.head, Position { x: -1, y: 2 });
        assert_eq!(rope.tail, Position { x: 0, y: 2 });
        assert_eq!(rope.history.len(), 4);
        assert!(rope.history.contains(&Position { x: 0, y: 0 }));
        assert!(rope.history.contains(&Position { x: 1, y: 0 }));
        assert!(rope.history.contains(&Position { x: 1, y: 1 }));
        assert!(rope.history.contains(&Position { x: 0, y: 2 }));
    }

    #[test]
    fn test_rope_execute_motions() {
        let motions = parse_input(TEST_INPUT);
        let mut rope = Rope::new();
        rope.execute_motions(&motions);

        assert_eq!(rope.head, Position { x: 2, y: 2 });
        assert_eq!(rope.tail, Position { x: 1, y: 2 });
        assert_eq!(
            rope.history,
            HashSet::from_iter(vec![
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(2, 0),
                Position::new(3, 0),
                Position::new(4, 1),
                Position::new(1, 2),
                Position::new(2, 2),
                Position::new(3, 2),
                Position::new(4, 2),
                Position::new(3, 3),
                Position::new(4, 3),
                Position::new(2, 4),
                Position::new(3, 4),
            ])
        );
    }

    #[test]
    fn test_challenge_answer() {
        let tree = parse_input(TEST_INPUT);

        assert_eq!(challenge_answer(&tree), 13);
    }
}
