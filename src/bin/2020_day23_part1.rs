//! Advent of Code 2020 Day 23
//! https://adventofcode.com/2020/day/23
//!
//! Challenge part 1
//!
//! Model the cup game described in the challenge and determine the final order of cups after
//! making the required number of moves.

use std::fs;
use std::iter;

const INPUT_FILENAME: &str = "2020_day23_input.txt";
const GAME_ROUNDS: usize = 100;

type Cup = u8;

/// `Game` holds the state of a game. The `cups` `Vec` lists the cups in a clockwise order. The
/// challenge refers to the cups based on their position, where the first cup is cup 1, whereas
/// this is stored in position 0 in the `Vec`, following standard Rust convention. This leads to
/// translations when converting between these two systems.
#[derive(Clone, Debug, PartialEq)]
struct Game {
    cups: Vec<Cup>,
    current_cup_index: usize,
}

impl Game {
    /// Create and return a game with the cups ordered as per `input`.
    fn load_game(input: &str) -> Self {
        let mut lines = input.lines();
        let l = lines.next().unwrap();

        Game {
            cups: l.chars().map(|c| c.to_digit(10).unwrap() as Cup).collect(),
            current_cup_index: 0,
        }
    }

    /// Perform a single move to reorganize the cups based on the rules described in the challenge.
    fn perform_one_move(&mut self) {
        let cup_count = self.cups.len();
        let value_at_current_cup_index = self.cups[self.current_cup_index];
        let mut destination_id = value_at_current_cup_index - 1;
        let mut picked_up_cups = remove_three(&mut self.cups, self.current_cup_index + 1);

        while (picked_up_cups.contains(&destination_id)) || (destination_id <= 0) {
            if destination_id <= 0 {
                destination_id = *self.cups.iter().max().unwrap();
            } else {
                destination_id -= 1;
            }
        }

        let insert_after_position = self.cups.iter().position(|&x| x == destination_id).unwrap();
        insert_three(&mut self.cups, insert_after_position, &mut picked_up_cups);

        self.restore_current_cup_value_position(value_at_current_cup_index);

        self.current_cup_index = (self.current_cup_index + 1) % cup_count;
    }

    /// Rotate the cups to restore the given cup `value` to the position in self.current_cup_index.
    ///
    /// # Panics
    ///
    /// Panics if `value` is not a valid `Cup` id.
    fn restore_current_cup_value_position(&mut self, value: Cup) {
        let cups_len = self.cups.len();
        let bad_pos = self.cups.iter().position(|&x| x == value).unwrap();

        let required_shifts_right = (self.current_cup_index + cups_len - bad_pos) % cups_len;

        self.cups.rotate_right(required_shifts_right);
    }

    /// Performs `moves` moves of the game.
    fn play_game(&mut self, moves: usize) {
        for _ in 0..moves {
            self.perform_one_move();
        }
    }

    /// Returns a string representing the current game state in the format required for the final
    /// challenge answer.
    fn get_challenge_answer(&self) -> String {
        let start_pos = self.cups.iter().position(|&x| x == 1).unwrap();

        let mut result = Vec::new();
        for i in 1..self.cups.len() {
            result.push(self.cups[(start_pos + i) % self.cups.len()]);
        }

        result.iter().map(|x| x.to_string()).collect()
    }
}

/// Remove and return three elements from `v`, starting at `position`. If `position` is such that
/// the end of `v` is reached, the elements at the beginning of `v` are removed instead. For
/// example, if `v` is [1, 3, 5, 7, 9] and `position` is 3, `v` becomes [7, 9, 1] and [3, 5] is
/// returned. If `position` is past the end of `v`, it is wrapped back to the beginning of `v`.
///
/// # Panics
///
/// Panics if the length of `v` is less than 3.
fn remove_three<T>(v: &mut Vec<T>, position: usize) -> Vec<T> {
    assert!(v.len() >= 3);

    let mut pos = position;
    if position < (v.len() - 3) {
        return v.splice(pos..pos + 3, iter::empty()).collect::<Vec<T>>();
    } else {
        let mut result = Vec::new();

        pos %= v.len();
        result.push(v.remove(pos));
        pos %= v.len();
        result.push(v.remove(pos));
        pos %= v.len();
        result.push(v.remove(pos));

        return result;
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
        v.splice(p..p, elements.iter().cloned().collect::<Vec<T>>());
    }
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let mut game = Game::load_game(&input_file);

    game.play_game(GAME_ROUNDS);
    println!("Challenge answer is {}", game.get_challenge_answer());
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "389125467";

    #[test]
    fn test_load_game() {
        let game = Game::load_game(&TEST_INPUT);

        let expected = Game {
            cups: vec![3, 8, 9, 1, 2, 5, 4, 6, 7],
            current_cup_index: 0,
        };

        assert_eq!(expected, game);
    }

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
    fn test_one_move() {
        let mut game = Game::load_game(&TEST_INPUT);

        game.perform_one_move();
        assert_eq!(
            Game {
                cups: vec![3, 2, 8, 9, 1, 5, 4, 6, 7],
                current_cup_index: 1
            },
            game
        );
        game.perform_one_move();
        assert_eq!(
            Game {
                cups: vec![3, 2, 5, 4, 6, 7, 8, 9, 1],
                current_cup_index: 2
            },
            game
        );
        game.perform_one_move();
        assert_eq!(
            Game {
                cups: vec![7, 2, 5, 8, 9, 1, 3, 4, 6],
                current_cup_index: 3
            },
            game
        );
        game.perform_one_move();
        assert_eq!(
            Game {
                cups: vec![3, 2, 5, 8, 4, 6, 7, 9, 1],
                current_cup_index: 4
            },
            game
        );
        game.perform_one_move();
        assert_eq!(
            Game {
                cups: vec![9, 2, 5, 8, 4, 1, 3, 6, 7],
                current_cup_index: 5
            },
            game
        );
        game.perform_one_move();
        assert_eq!(
            Game {
                cups: vec![7, 2, 5, 8, 4, 1, 9, 3, 6],
                current_cup_index: 6
            },
            game
        );
        game.perform_one_move();
        assert_eq!(
            Game {
                cups: vec![8, 3, 6, 7, 4, 1, 9, 2, 5],
                current_cup_index: 7
            },
            game
        );
        game.perform_one_move();
        assert_eq!(
            Game {
                cups: vec![7, 4, 1, 5, 8, 3, 9, 2, 6],
                current_cup_index: 8
            },
            game
        );
        game.perform_one_move();
        assert_eq!(
            Game {
                cups: vec![5, 7, 4, 1, 8, 3, 9, 2, 6],
                current_cup_index: 0
            },
            game
        );
        game.perform_one_move();
        assert_eq!(
            Game {
                cups: vec![5, 8, 3, 7, 4, 1, 9, 2, 6],
                current_cup_index: 1
            },
            game
        );
    }

    #[test]
    fn test_play_game() {
        let mut game = Game::load_game(&TEST_INPUT);

        game.play_game(GAME_ROUNDS);
        assert_eq!("67384529", game.get_challenge_answer());
    }
}
