//! Advent of Code 2021 Day 21
//! https://adventofcode.com/2021/day/21
//!
//! Challenge part 2
//!
//! Analyzes all possible games of "Dirac Dice" that could be played based on the starting
//! positions given in the input file and calculates the number of wins for each player for every
//! possible permutation of dice throws.
//
// The challenge is worded to encourage a strategy of branching at each roll of the die, but it
// is more efficient to branch for the total of 3 dice rolls, and then factor in the frequency of
// permutations for that total. For example, the total 4 can be achieved by rolling 1, 1 then 2;
// or 1, 2 then 1; or 2, 1 then 1, so the frequency is 3. This code considers dice roll totals and
// uses the normal distribution of these totals to avoid unnecessarily repeating work.
//

use std::collections::HashMap;
use std::fs;
use std::iter::FromIterator;

type Position = u8;

const INPUT_FILENAME: &str = "2021_day21_input.txt";
const WIN_SCORE: u8 = 21;
const BOARD_SIZE: u8 = 10;

// The normal distribution of outcomes of rolling a 3-sided die 3 times. The index is the sum of
// the three rolls and the value is the number of ways that sum can be achieved.
const DIE_NORMAL_DIST: [u8; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];


/// Contains state for both players, recording their position and total score.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct PlayersState {
    p1_position: Position,
    p1_score: u8,
    p2_position: Position,
    p2_score: u8,
}

impl PlayersState {
    fn new(p1_start_position: Position, p2_start_position: Position) -> Self {
        Self {
            p1_position: p1_start_position,
            p1_score: 0,
            p2_position: p2_start_position,
            p2_score: 0,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct GameState {
    turn: u16,
    most_recent_player: u8,
    p1_wins: u64,
    p2_wins: u64,
    perms: HashMap<PlayersState, u64>,
}

impl GameState {
    fn new(p1_start_position: Position, p2_start_position: Position) -> Self {
        Self {
            turn: 0,
            most_recent_player: 2,
            p1_wins: 0,
            p2_wins: 0,
            perms: HashMap::from_iter(
                [(PlayersState::new(p1_start_position, p2_start_position), 1)]
            ),
        }
    }

    /// Returns a new `GameState` object containing the outcomes of all possible moves starting
    /// from the game state in `self`.
    fn make_move(&self) -> Self {
        let player = (self.most_recent_player % 2) + 1;
        let turn = if player == 1 { self.turn + 1 } else { self.turn };

//         println!("\nTurn {} Player {}", turn, player);

        let mut new_perms = HashMap::new();
        let mut new_p1_wins = self.p1_wins;
        let mut new_p2_wins = self.p2_wins;

        for (players_state, occurrences) in &self.perms {
            for dice in 3..=9 {
                let new_score;
                let mut new_position;
                let new_occurrences: u64 = occurrences * DIE_NORMAL_DIST[dice as usize] as u64;

                match player {
                    1 => {
                        new_position = players_state.p1_position + dice;

                        if new_position > BOARD_SIZE {
                            new_position %= BOARD_SIZE;
                        }

                        new_score = players_state.p1_score + new_position;

                        if new_score < WIN_SCORE {
                            let ps = PlayersState {
                                        p1_position: new_position,
                                        p1_score: new_score,
                                        p2_position: players_state.p2_position,
                                        p2_score: players_state.p2_score,
                                    };
                            match new_perms.get_mut(&ps) {
                                Some(state) => {
                                    *state += new_occurrences;
                                }
                                None => {
                                    new_perms.insert(ps, new_occurrences);
                                }
                            }
                        } else {
                            new_p1_wins += new_occurrences;
                        }
                    },
                    2 => {
                       new_position = players_state.p2_position + dice;

                        if new_position > BOARD_SIZE {
                            new_position %= BOARD_SIZE;
                        }

                        new_score = players_state.p2_score + new_position;

                        if new_score < WIN_SCORE {
                            let ps = PlayersState {
                                        p1_position: players_state.p1_position,
                                        p1_score: players_state.p1_score,
                                        p2_position: new_position,
                                        p2_score: new_score,
                                    };
                            match new_perms.get_mut(&ps) {
                                Some(state) => {
                                    *state += new_occurrences;
                                }
                                None => {
                                    new_perms.insert(ps, new_occurrences);
                                }
                            }
                        } else {
                            new_p2_wins += new_occurrences;
                        }
                    },
                    _ => {
                        panic!("Internal error - player id was neither 1 or 2.");
                    }
                }
            }
        }

        Self {
            turn: turn,
            most_recent_player: player,
            p1_wins: new_p1_wins,
            p2_wins: new_p2_wins,
            perms: new_perms,
        }
    }
}


/// Reads the start positions of both players from the string passed and returns as a tuple.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> (Position, Position) {
    let mut lines = input.lines();

    ( lines.next().unwrap()
            .strip_prefix("Player 1 starting position: ").unwrap()
            .parse().unwrap(),
        lines.next().unwrap()
            .strip_prefix("Player 2 starting position: ").unwrap()
            .parse().unwrap()
    )
}


/// Play a game beginning at the starting positions provided until all possible permutation of
/// dice rolls have been considered. Return the number of wins for each player as a tuple.
fn play_game(p1_start: u8, p2_start: u8) -> (u64, u64) {
    let mut game = GameState::new(p1_start, p2_start);

    while game.perms.len() != 0 {
        game = game.make_move();
    }

    (game.p1_wins, game.p2_wins)
}


fn main() {
    let input_file =
        fs::read_to_string(INPUT_FILENAME)
            .expect("Error reading input file");

    let (p1_start, p2_start) = parse_input(&input_file);
    let wins = play_game(p1_start, p2_start);
    println!("Player 1 wins {} times and Player 2 wins {} times", wins.0, wins.1);
    println!("The challenge answer is the larger of these numbers, which is: {}",
        u64::max(wins.0, wins.1)
    );

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
        let (p1_start, p2_start) = parse_input(&TEST_INPUT);

        assert_eq!(p1_start, 4);
        assert_eq!(p2_start, 8);
    }

    #[test]
    fn test_play_game() {
        let (p1_start, p2_start) = parse_input(&TEST_INPUT);
        let wins = play_game(p1_start, p2_start);
        println!("Player 1 wins {} times and Player 2 wins {} times", wins.0, wins.1);

        assert_eq!(wins.0, 444356092776315);
        assert_eq!(wins.1, 341960390180808);
    }
}
