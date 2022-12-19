//! Advent of Code 2022 Day 02
//! https://adventofcode.com/2022/day/2
//!
//! Challenge part 1
//!
//! Plays games of rock, paper, scissors and determines my total score based on who won each
//! round and the shape I chose.

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

const MY_MOVE: [(char, Shape); 3] = [
    ('X', Shape::Rock),
    ('Y', Shape::Paper),
    ('Z', Shape::Scissors),
];

const GAME_RESULT_SCORE: [(GameResult, Score); 3] = [
    (GameResult::Lose, 0),
    (GameResult::Draw, 3),
    (GameResult::Win, 6),
];

/// Takes a string containing pairs of moves, one pair per line, and returns a `Vec` of tuples
/// with the same data represented using the `Shape` enum. The first move must be 'A', 'B' or 'C'
/// and the responding move must be 'X', 'Y', 'Z'. They must be separated by a single space.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> Vec<(Shape, Shape)> {
    let mut moves = Vec::new();

    for line in input.lines() {
        if line != "" {
            assert_eq!(line.len(), 3);

            let mut chars = line.chars();
            let opp_char = chars.next().unwrap();
            let opp_move = OPPONENT_MOVE.iter().find(|&c| c.0 == opp_char).unwrap().1;

            assert_eq!(chars.next().unwrap(), ' ');

            let my_char = chars.next().unwrap();
            let my_move = MY_MOVE.iter().find(|&c| c.0 == my_char).unwrap().1;

            moves.push((opp_move, my_move));
        }
    }
    moves
}

/// Returns a `GameResult` enum indicating whether the shapes chosen this round result in a win,
/// loss or draw for me.
fn play_round(opponent_move: Shape, my_move: Shape) -> GameResult {
    if opponent_move == my_move {
        return GameResult::Draw;
    }

    if (opponent_move == Shape::Rock && my_move == Shape::Paper)
        || (opponent_move == Shape::Paper && my_move == Shape::Scissors)
        || (opponent_move == Shape::Scissors && my_move == Shape::Rock)
    {
        return GameResult::Win;
    }

    GameResult::Lose
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
fn score_all_rounds(game: Vec<(Shape, Shape)>) -> Score {
    let mut total_score = 0;

    for round in game {
        total_score += score_round(round.1, play_round(round.0, round.1));
    }

    total_score
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    let input_as_enums: Vec<(Shape, Shape)> = parse_input(&input);

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
        let input_as_enums: Vec<(Shape, Shape)> = parse_input(TEST_GAME);
        assert_eq!(
            input_as_enums,
            vec![
                (Shape::Rock, Shape::Paper),
                (Shape::Paper, Shape::Rock),
                (Shape::Scissors, Shape::Scissors),
            ]
        );
    }

    #[test]
    fn test_play_round() {
        assert_eq!(play_round(Shape::Rock, Shape::Paper), GameResult::Win);
        assert_eq!(play_round(Shape::Paper, Shape::Scissors), GameResult::Win);
        assert_eq!(play_round(Shape::Scissors, Shape::Rock), GameResult::Win);

        assert_eq!(play_round(Shape::Rock, Shape::Rock), GameResult::Draw);
        assert_eq!(play_round(Shape::Paper, Shape::Paper), GameResult::Draw);
        assert_eq!(
            play_round(Shape::Scissors, Shape::Scissors),
            GameResult::Draw
        );

        assert_eq!(play_round(Shape::Rock, Shape::Scissors), GameResult::Lose);
        assert_eq!(play_round(Shape::Paper, Shape::Rock), GameResult::Lose);
        assert_eq!(play_round(Shape::Scissors, Shape::Paper), GameResult::Lose);
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
        let input_as_enums: Vec<(Shape, Shape)> = parse_input(TEST_GAME);

        assert_eq!(score_all_rounds(input_as_enums), 15);
    }
}
