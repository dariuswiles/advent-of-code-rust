//! Advent of Code 2020 Day 23
//! https://adventofcode.com/2020/day/23
//!
//! Challenge part 2
//!
//! Model the cup game described in the challenge and determine the final order of cups after
//! making the required number of moves. Part 2 significantly increases the number of cups and
//! number of game rounds required to determine the answer.
//!
//! Note: This implementation takes about 2 hours to run when "--release" version is used. Running
//!       the tests also takes this long.

use std::fs;
use std::iter;

const INPUT_FILENAME: &str = "2020_day23_input.txt";
const TOTAL_CUPS: usize = 1_000_000;
const GAME_ROUNDS: usize = 10_000_000;

type Cup = u32;

/// `Game` holds the state of a game. The `cups` `Vec` lists the cups in a clockwise order. The
/// challenge refers to the cups based on their position, where the first cup is cup 1, whereas
/// this is stored in position 0 in the `Vec`, following standard Rust convention. This leads to
/// translations when converting between these two systems.
#[derive(Clone, Debug, PartialEq)]
struct Game {
    cups: Vec<Cup>,
    cups_len: usize,
    current_cup_index: usize,
}

impl Game {
    /// Create and return a game with the cups ordered as per `input`.
    fn load_game(input: &str, cups_len: usize) -> Self {
        let mut lines = input.lines();
        let l = lines.next().unwrap();

        let mut cups: Vec<Cup> = l.chars().map(|c| c.to_digit(10).unwrap() as Cup).collect();

        for c in cups.iter().max().unwrap() + 1..=cups_len as Cup {
            cups.push(c as Cup);
        }

        Game {
            cups,
            cups_len,
            current_cup_index: 0,
        }
    }

    /// Perform a single move to reorganize the cups based on the rules described in the challenge.
    fn perform_one_move(&mut self) {
        let value_at_current_cup_index = self.cups[self.current_cup_index];
        let mut destination_id = value_at_current_cup_index - 1;
        let mut picked_up_cups = remove_three(&mut self.cups, self.current_cup_index + 1);

        while (picked_up_cups.contains(&destination_id)) || (destination_id == 0) {
            if destination_id == 0 {
                destination_id = *self.cups.iter().max().unwrap();
            } else {
                destination_id -= 1;
            }
        }

        let insert_after_position = self.cups.iter().position(|&x| x == destination_id).unwrap();
        insert_three(&mut self.cups, insert_after_position, &mut picked_up_cups);

        self.current_cup_index = (self
            .cups
            .iter()
            .position(|&x| x == value_at_current_cup_index)
            .unwrap()
            + 1)
            % self.cups_len;
    }

    /// Performs `moves` moves of the game.
    fn play_game(&mut self, moves: usize) {
        for _ in 0..moves {
            self.perform_one_move();
        }
    }

    /// Returns an integer representing the current game state in the format required for the final
    /// challenge answer.
    fn get_challenge_answer(&self) -> u64 {
        let start_pos = self.cups.iter().position(|&x| x == 1).unwrap();

        self.cups[(start_pos + 1) % self.cups_len] as u64
            * self.cups[(start_pos + 2) % self.cups_len] as u64
    }
}

/// Remove and return three elements from `v`, starting at `position`. If `position` is such that
/// the end of `v` is reached, the elements at the beginning of `v` are removed instead. For
/// example, if `v` is [1, 3, 5, 7, 9] and `position` is 3, `v` becomes [3, 5] and [7, 9, 1] is
/// returned. If `position` is past the end of `v`, it is wrapped back to the beginning of `v`.
///
/// # Panics
///
/// Panics if the length of `v` is less than 3.
fn remove_three<T>(v: &mut Vec<T>, position: usize) -> Vec<T> {
    assert!(v.len() >= 3);

    let mut pos = position;
    if position < (v.len() - 3) {
        v.splice(pos..pos + 3, iter::empty()).collect::<Vec<T>>()
    } else {
        let mut result = Vec::new();

        pos %= v.len();
        result.push(v.remove(pos));
        pos %= v.len();
        result.push(v.remove(pos));
        pos %= v.len();
        result.push(v.remove(pos));

        result
    }
}

/// Inserts the three elements in `elements` into `v`, starting at the index one *after* `position`.
/// The elements are moved, not copied, so `elements` is emptied during this process.
///
/// # Panics
///
/// Panics if the length of `elements` is not 3 or if `position` is not a valid index into `v`.
fn insert_three<T: Clone>(v: &mut Vec<T>, position: usize, elements: &mut Vec<T>) {
    assert!(elements.len() == 3);

    if position == v.len() {
        v.append(elements);
    } else {
        let p = position + 1;
        v.splice(p..p, elements.to_vec());
    }
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let mut game = Game::load_game(&input_file, TOTAL_CUPS);

    game.play_game(GAME_ROUNDS);
    println!("Challenge answer is {}", game.get_challenge_answer());
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "389125467";

    #[test]
    fn test_remove_three() {
        let v = vec![1, 3, 5, 7, 9];

        let mut v1 = v.clone();
        let removed1 = remove_three(&mut v1, 0);
        assert_eq!(vec![1, 3, 5], removed1);
        assert_eq!(vec![7, 9], v1);

        let mut v2 = v.clone();
        let removed2 = remove_three(&mut v2, 2);
        assert_eq!(vec![5, 7, 9], removed2);
        assert_eq!(vec![1, 3], v2);

        let mut v3 = v.clone();
        let removed3 = remove_three(&mut v3, 3);
        assert_eq!(vec![7, 9, 1], removed3);
        assert_eq!(vec![3, 5], v3);

        let mut v4 = v.clone();
        let removed4 = remove_three(&mut v4, 4);
        assert_eq!(vec![9, 1, 3], removed4);
        assert_eq!(vec![5, 7], v4);

        let mut v5 = v.clone();
        let removed5 = remove_three(&mut v5, 5);
        assert_eq!(vec![1, 3, 5], removed5);
        assert_eq!(vec![7, 9], v5);
    }

    #[test]
    fn test_insert_three() {
        let v = vec![1, 3, 5, 7, 9];

        let mut v1 = v.clone();
        insert_three(&mut v1, 0, &mut vec![2, 4, 6]);
        assert_eq!(vec![1, 2, 4, 6, 3, 5, 7, 9], v1);

        let mut v2 = v.clone();
        insert_three(&mut v2, 3, &mut vec![2, 4, 6]);
        assert_eq!(vec![1, 3, 5, 7, 2, 4, 6, 9], v2);

        let mut v3 = v.clone();
        insert_three(&mut v3, 4, &mut vec![2, 4, 6]);
        assert_eq!(vec![1, 3, 5, 7, 9, 2, 4, 6], v3);
    }

    #[test]
    fn get_challenge_answer1() {
        let game = Game {
            cups: vec![14, 97, 34, 21, 3, 87, 1, 22, 5, 92, 77, 38],
            cups_len: 9,
            current_cup_index: 2,
        };

        assert_eq!(110, game.get_challenge_answer());
    }

    #[test]
    fn get_challenge_answer2() {
        let game = Game {
            cups: vec![14, 97, 34, 21, 3, 87, 1, 3],
            cups_len: 8,
            current_cup_index: 1,
        };
        assert_eq!(42, game.get_challenge_answer());
    }

    #[test]
    fn get_challenge_answer3() {
        let game = Game {
            cups: vec![14, 97, 34, 21, 3, 87, 1],
            cups_len: 7,
            current_cup_index: 4,
        };
        assert_eq!(1358, game.get_challenge_answer());
    }

    #[test]
    fn test_one_move() {
        let cups_len = 9;
        let mut game = Game::load_game(TEST_INPUT, cups_len);

        game.perform_one_move();
        assert_eq!(
            Game {
                cups: vec![3, 2, 8, 9, 1, 5, 4, 6, 7],
                cups_len,
                current_cup_index: 1
            },
            game
        );
        game.perform_one_move();
        assert_eq!(
            Game {
                cups: vec![3, 2, 5, 4, 6, 7, 8, 9, 1],
                cups_len,
                current_cup_index: 2
            },
            game
        );
        game.perform_one_move();
        assert_eq!(
            Game {
                cups: vec![3, 4, 6, 7, 2, 5, 8, 9, 1],
                cups_len,
                current_cup_index: 6
            },
            game
        );
        game.perform_one_move();
        assert_eq!(
            Game {
                cups: vec![4, 6, 7, 9, 1, 3, 2, 5, 8],
                cups_len,
                current_cup_index: 0
            },
            game
        );
        game.perform_one_move();
        assert_eq!(
            Game {
                cups: vec![4, 1, 3, 6, 7, 9, 2, 5, 8],
                cups_len,
                current_cup_index: 1
            },
            game
        );
        game.perform_one_move();
        assert_eq!(
            Game {
                cups: vec![4, 1, 9, 3, 6, 7, 2, 5, 8],
                cups_len,
                current_cup_index: 2
            },
            game
        );
        game.perform_one_move();
        assert_eq!(
            Game {
                cups: vec![4, 1, 9, 2, 5, 8, 3, 6, 7],
                cups_len,
                current_cup_index: 3
            },
            game
        );
        game.perform_one_move();
        assert_eq!(
            Game {
                cups: vec![4, 1, 5, 8, 3, 9, 2, 6, 7],
                cups_len,
                current_cup_index: 7
            },
            game
        );
        game.perform_one_move();
        assert_eq!(
            Game {
                cups: vec![5, 7, 4, 1, 8, 3, 9, 2, 6],
                cups_len,
                current_cup_index: 0
            },
            game
        );
        game.perform_one_move();
        assert_eq!(
            Game {
                cups: vec![5, 8, 3, 7, 4, 1, 9, 2, 6],
                cups_len,
                current_cup_index: 1
            },
            game
        );
    }

    #[test]
    fn play_game() {
        let cups_len = 9;
        let mut game = Game::load_game(TEST_INPUT, cups_len);
        let mut game_move = game.clone();

        game.play_game(1);
        game_move.perform_one_move();

        assert_eq!(&game, &game_move);
    }

    #[test]
    fn play_part1_game() {
        let cups_len = 9;
        let mut game = Game::load_game(TEST_INPUT, cups_len);
        game.play_game(100);
        assert_eq!(
            Game {
                cups: vec![2, 9, 1, 6, 7, 3, 8, 4, 5],
                cups_len,
                current_cup_index: 2
            },
            game
        );
    }

    #[test]
    fn play_part2_game() {
        let mut game = Game::load_game(TEST_INPUT, TOTAL_CUPS);

        assert!(game.cups.len() == TOTAL_CUPS);
        game.play_game(GAME_ROUNDS);
        assert_eq!(149245887792, game.get_challenge_answer());
    }
}
