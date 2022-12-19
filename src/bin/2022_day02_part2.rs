//! Advent of Code 2022 Day 02
//! https://adventofcode.com/2022/day/2
//!
//! Challenge part 2
//!
//! Plays games of rock, paper, scissors based on input which gives the opponent's moves and states
//! if the response should be to win, draw or lose. Calculates the score of each round based on
//! these criteria, and prints the total score of all rounds of the game.

use std::fs;

const INPUT_FILENAME: &str = "2022_day02_input.txt";

type Score = u32;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum GameResult {
    Lose,
    Draw,
    Win,
}

const SHAPE_SCORE: [(Shape, u32); 3] = [(Shape::Rock, 1), (Shape::Paper, 2), (Shape::Scissors, 3)];

const OPPONENT_MOVE: [(char, Shape); 3] = [
    ('A', Shape::Rock),
    ('B', Shape::Paper),
    ('C', Shape::Scissors),
];

const GAME_RESULT_CODE: [(char, GameResult); 3] = [
    ('X', GameResult::Lose),
    ('Y', GameResult::Draw),
    ('Z', GameResult::Win),
];

const GAME_RESULT_SCORE: [(GameResult, Score); 3] = [
    (GameResult::Lose, 0),
    (GameResult::Draw, 3),
    (GameResult::Win, 6),
];

/// Takes a string containing a pair for each game round, where each pair is the opponent's move
/// and the desired outcome of the round for me. Returns a `Vec` of tuples with the same data
/// represented using the `Shape` and `GameResult' enums. The move must be 'A', 'B' or 'C' and the
/// desired outcome must be 'X', 'Y', 'Z'. The two characters must be separated by a single space.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> Vec<(Shape, GameResult)> {
    let mut moves = Vec::new();

    for line in input.lines() {
        if line != "" {
            assert_eq!(line.len(), 3);

            let mut chars = line.chars();
            let opp_char = chars.next().unwrap();
            let opp_move = OPPONENT_MOVE.iter().find(|&c| c.0 == opp_char).unwrap().1;

            assert_eq!(chars.next().unwrap(), ' ');

            let desired_outcome_char = chars.next().unwrap();
            let desired_outcome = GAME_RESULT_CODE
                .iter()
                .find(|&grc| grc.0 == desired_outcome_char)
                .unwrap()
                .1;

            moves.push((opp_move, desired_outcome));
        }
    }
    moves
}

/// Returns the `Shape` I need to play to achieve the given `desired_outcome` given the `Shape`
/// chosen by the opponent.
fn choose_response(opponent_move: Shape, desired_outcome: GameResult) -> Shape {
    match desired_outcome {
        GameResult::Lose => match opponent_move {
            Shape::Rock => {
                return Shape::Scissors;
            }
            Shape::Paper => {
                return Shape::Rock;
            }
            Shape::Scissors => {
                return Shape::Paper;
            }
        },

        GameResult::Draw => {
            return opponent_move;
        }

        GameResult::Win => match opponent_move {
            Shape::Rock => {
                return Shape::Paper;
            }
            Shape::Paper => {
                return Shape::Scissors;
            }
            Shape::Scissors => {
                return Shape::Rock;
            }
        },
    }
}

/// Returns the score for a round given the `Shape` I chose for the round and whether I won.
fn score_round(my_move: Shape, round_result: GameResult) -> Score {
    SHAPE_SCORE.iter().find(|&ss| ss.0 == my_move).unwrap().1
        + GAME_RESULT_SCORE
            .iter()
            .find(|&grs| grs.0 == round_result)
            .unwrap()
            .1
}

/// Returns the total score for all rounds of the `game` passed.
fn score_all_rounds(game: Vec<(Shape, GameResult)>) -> Score {
    let mut total_score = 0;

    for round in game {
        total_score += score_round(choose_response(round.0, round.1), round.1);
    }

    total_score
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    let input_as_enums: Vec<(Shape, GameResult)> = parse_input(&input);

    println!(
        "My total score for the game is {}",
        score_all_rounds(input_as_enums)
    );
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_GAME: &str = "\
A Y
B X
C Z";

    #[test]
    fn test_input_parsing() {
        let input_as_enums: Vec<(Shape, GameResult)> = parse_input(TEST_GAME);
        assert_eq!(
            input_as_enums,
            vec![
                (Shape::Rock, GameResult::Draw),
                (Shape::Paper, GameResult::Lose),
                (Shape::Scissors, GameResult::Win),
            ]
        );
    }

    #[test]
    fn test_choose_response() {
        assert_eq!(
            choose_response(Shape::Rock, GameResult::Lose),
            Shape::Scissors
        );
        assert_eq!(choose_response(Shape::Paper, GameResult::Lose), Shape::Rock);
        assert_eq!(
            choose_response(Shape::Scissors, GameResult::Lose),
            Shape::Paper
        );

        assert_eq!(choose_response(Shape::Rock, GameResult::Draw), Shape::Rock);
        assert_eq!(
            choose_response(Shape::Paper, GameResult::Draw),
            Shape::Paper
        );
        assert_eq!(
            choose_response(Shape::Scissors, GameResult::Draw),
            Shape::Scissors
        );

        assert_eq!(choose_response(Shape::Rock, GameResult::Win), Shape::Paper);
        assert_eq!(
            choose_response(Shape::Paper, GameResult::Win),
            Shape::Scissors
        );
        assert_eq!(
            choose_response(Shape::Scissors, GameResult::Win),
            Shape::Rock
        );
    }

    #[test]
    fn test_score_round() {
        assert_eq!(score_round(Shape::Rock, GameResult::Lose), 1);
        assert_eq!(score_round(Shape::Paper, GameResult::Lose), 2);
        assert_eq!(score_round(Shape::Scissors, GameResult::Lose), 3);

        assert_eq!(score_round(Shape::Rock, GameResult::Draw), 4);
        assert_eq!(score_round(Shape::Paper, GameResult::Draw), 5);
        assert_eq!(score_round(Shape::Scissors, GameResult::Draw), 6);

        assert_eq!(score_round(Shape::Rock, GameResult::Win), 7);
        assert_eq!(score_round(Shape::Paper, GameResult::Win), 8);
        assert_eq!(score_round(Shape::Scissors, GameResult::Win), 9);
    }

    #[test]
    fn test_score_all_rounds() {
        let input_as_enums: Vec<(Shape, GameResult)> = parse_input(TEST_GAME);

        assert_eq!(score_all_rounds(input_as_enums), 12);
    }
}
