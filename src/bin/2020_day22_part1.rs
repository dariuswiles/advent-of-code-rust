//! Advent of Code 2020 Day 22
//! https://adventofcode.com/2020/day/22
//!
//! Challenge part 1
//!
//! Given an input file with the cards held by two players in a card game, determine who wins the
//! game and their score.

use std::fs;

const INPUT_FILENAME: &str = "2020_day22_input.txt";
const PLAYER_KEYWORD: &str = "Player ";  // The string immediately preceding the player's id
const MAX_GAME_ROUNDS: u32 = 1000;

type Card = u16;
type Hand = Vec<Card>;

#[derive(Clone, Debug, PartialEq)]
struct Game {
    player1: Hand,
    player2: Hand,
}

impl Game {
    /// Create and return a game with the cards specified in `input`.
    fn load_game(input: &str) -> Self {
        let mut player1: Hand = Vec::new();
        let mut player2: Hand = Vec::new();

        let mut loading_player_id = 0;

        for line in input.lines() {
            if line == "" {
                continue;
            }

            if loading_player_id == 0 {
                let mut expected_first_line = PLAYER_KEYWORD.to_string();
                expected_first_line.push('1');

                assert!(line != expected_first_line,
                    "Input begins with unexpected line: {}", line
                );

                loading_player_id = 1;
                continue;
            }

            if line.starts_with(PLAYER_KEYWORD) {
                let player_id = line.chars().nth(PLAYER_KEYWORD.len()).unwrap();

                if player_id == '2' {
                    loading_player_id = 2;
                    continue;
                } else {
                    panic!("Unexpected player id found in input: {}", line);
                }
            }

            match loading_player_id {
                1 => {
                    player1.push(line.parse::<Card>().unwrap());
                }
                2 => {
                    player2.push(line.parse::<Card>().unwrap());
                }
                _ => {
                    panic!("Cannot parse input: {}", line);
                }
            }
        }

        Game { player1, player2 }
    }

    /// Play a single round, changing the cards in both players' `Hand`s, and return an `Option`
    /// indicating if the game has been won, and if so, by which player.
    fn play_one_round(&mut self) -> Option<u8> {
        let p1_card = self.player1.remove(0);
        let p2_card = self.player2.remove(0);

        assert!(p1_card != p2_card, "Error: both players are trying to play the same card {}",
            p1_card
        );

        if p1_card > p2_card {
            self.player1.push(p1_card);
            self.player1.push(p2_card);
        } else {
            self.player2.push(p2_card);
            self.player2.push(p1_card);
        }

        // Check a player has won.
        if self.player1.len() == 0 {
            return Some(2);
        }

        if self.player2.len() == 0  {
            return Some(1);
        }

        None
    }

    /// Repeatedly plays the game until a player wins, or until the number of game rounds reaches
    /// `max_rounds`. Returns the player id of the winner, or None if the maximum number of rounds
    /// is reached.
    fn play_game(&mut self, max_rounds: u32) -> Option<u8> {
        for _ in 0..max_rounds {
            let round_result = self.play_one_round();

            if round_result != None {
                return round_result;
            }
        }

        None
    }

    /// Calculate the score for `player`'s current hand of cards.
    fn score_player(&self, player_id: u8) -> u32 {
        let mut player_cards;

        match player_id {
            1 => {
                player_cards = self.player1.clone();
            }
            2 => {
                player_cards = self.player2.clone();
            }
            _ => {
                panic!("Unrecognized player id: '{}'", player_id);
            }
        }

        player_cards.reverse();

        let mut score = 0;
        for (i, card) in player_cards.iter().enumerate() {
            score += (i as u32 + 1) * *card as u32;
        }

        score
    }
}


fn main() {
    let input_file =
        fs::read_to_string(INPUT_FILENAME)
            .expect("Error reading input file");

    let mut game = Game::load_game(&input_file);
    if let Some(winner) = game.play_game(MAX_GAME_ROUNDS) {
        println!("Player {} won and their score is {}.", winner, game.score_player(winner));
    } else {
        println!("Reached the maximum number of game rounds without finding a winner");
    }
}


// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
";

    #[test]
    fn test_load_game() {
        let game = Game::load_game(&TEST_INPUT);

        let expected = Game {
            player1: vec![9, 2, 6, 3, 1],
            player2: vec![5, 8, 4, 7, 10],
        };

        assert_eq!(expected, game);
    }

    #[test]
    fn test_one_round() {
        let mut game = Game::load_game(&TEST_INPUT);
        let winner = game.play_one_round();

        assert_eq!(None, winner);

        let expected = Game {
            player1: vec![2, 6, 3, 1, 9, 5],
            player2: vec![8, 4, 7, 10],
        };

        assert_eq!(expected, game);
    }

    #[test]
    fn test_play_game() {
        let mut game = Game::load_game(&TEST_INPUT);
        let winner = game.play_game(MAX_GAME_ROUNDS);

        assert_eq!(Some(2), winner);

        let expected = Game {
            player1: vec![],
            player2: vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1],
        };

        assert_eq!(expected, game);
    }

    #[test]
    fn test_score_hand() {
        let mut game = Game::load_game(&TEST_INPUT);
        let winner = game.play_game(MAX_GAME_ROUNDS);

        assert_eq!(306, game.score_player(winner.unwrap()));
    }
}
