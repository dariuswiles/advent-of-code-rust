//! Advent of Code 2021 Day 21
//! https://adventofcode.com/2021/day/21
//!
//! Challenge part 1
//!
//! Play a game of "Dirac Dice" until one of the two players wins, then return a value based on
//! the score of the losing player and the number of turns played.

use std::fs;

const INPUT_FILENAME: &str = "2021_day21_input.txt";
const MAX_DIE_VALUE: Int = 100;
const WIN_SCORE: Int = 1000;

type Int = u32;

#[derive(Debug)]
/// A die that returns numbers from 1..MAX_DIE_VALUE, then loops back to 1 and starts over.
struct DeterministicDie {
    value: Int,
}

impl DeterministicDie {
    fn new() -> Self {
        Self { value: 0 }
    }
}

impl Iterator for DeterministicDie {
    type Item = Int;

    fn next(&mut self) -> Option<Self::Item> {
        self.value = (self.value % MAX_DIE_VALUE) + 1;

        Some(self.value)
    }
}

struct Game {
    die: DeterministicDie,
    die_rolls: Int,
    player1: Player,
    player2: Player,
}

impl Game {
    fn new(p1_start: Int, p2_start: Int) -> Self {
        Self {
            die: DeterministicDie::new(),
            die_rolls: 0,
            player1: Player {
                position: p1_start,
                score: 0,
            },
            player2: Player {
                position: p2_start,
                score: 0,
            },
        }
    }

    fn make_move(&mut self, player_id: u8) -> bool {
        let p = if player_id == 1 {
            &mut self.player1
        } else {
            &mut self.player2
        };

        self.die_rolls += 3;
        let move_distance =
            self.die.next().unwrap() + self.die.next().unwrap() + self.die.next().unwrap();
        p.position = (p.position + move_distance) % 10;
        if p.position == 0 {
            p.position = 10
        };
        p.score += p.position;
        p.score >= WIN_SCORE
    }

    fn play_game(&mut self) -> u32 {
        loop {
            if self.make_move(1) {
                return self.die_rolls * self.player2.score;
            }

            if self.make_move(2) {
                return self.die_rolls * self.player1.score;
            }
        }
    }
}

struct Player {
    position: Int,
    score: Int,
}

/// Returns the start positions of both players as a tuple.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> (Int, Int) {
    let mut lines = input.lines();

    (
        lines
            .next()
            .unwrap()
            .strip_prefix("Player 1 starting position: ")
            .unwrap()
            .parse()
            .unwrap(),
        lines
            .next()
            .unwrap()
            .strip_prefix("Player 2 starting position: ")
            .unwrap()
            .parse()
            .unwrap(),
    )
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let (p1_start, p2_start) = parse_input(&input_file);
    let mut game = Game::new(p1_start, p2_start);

    println!("The challenge answer is {}", game.play_game());
}

// Test using data from the examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn parse_test_input() {
        let (p1_start, p2_start) = parse_input(TEST_INPUT);

        assert_eq!(p1_start, 4);
        assert_eq!(p2_start, 8);
    }

    #[test]
    fn test_play_game() {
        let (p1_start, p2_start) = parse_input(TEST_INPUT);

        let mut game = Game::new(p1_start, p2_start);
        assert_eq!(game.play_game(), 739785);
    }
}
