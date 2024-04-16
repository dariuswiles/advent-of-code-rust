//! Advent of Code 2023 Day 02
//! https://adventofcode.com/2023/day/2
//!
//! Challenge part 1
//!
//! Model a game consisting of a bag of colored cubes, from which several random handfuls of cubes
//! are taken out and shown to us. The challenge consists of determining whether a game is possible
//! given limits on each color of cube. The challenge answer is the sum of the ids of possible
//! games.

use std::fs;

const INPUT_FILENAME: &str = "2023_day02_input.txt";
const CUBE_LIMITS: CubeSet = CubeSet {
    red: 12,
    green: 13,
    blue: 14,
};

/// A single game, comprising a game `id` and a `Vec` of `CubeSet`s representing the handfuls of
/// cubes revealed during the game.
#[derive(Debug, PartialEq)]
struct Game {
    id: u8,
    reveals: Vec<CubeSet>,
}

/// The number of red, green and blue cubes in a set of cubes.
#[derive(Debug, PartialEq)]
struct CubeSet {
    red: u8,
    green: u8,
    blue: u8,
}

impl CubeSet {
    /// Creates a `CubeSet` from a comma-delimited string containing the number of red, blue and
    /// green cubes. These can be specified in any order. One spaces is required before and after
    /// every number. Example:
    /// " 1 red, 2 green, 6 blue"
    ///
    /// # Panics
    ///
    /// Panics on malformed input.
    fn from_str(s: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let tokens: Vec<_> = s.trim().split(" ").collect();

        for t in tokens.chunks(2) {
            let amount = u8::from_str_radix(t[0], 10).unwrap();

            match t[1].trim_end_matches(',') {
                "red" => {
                    red = amount;
                }
                "green" => {
                    green = amount;
                }
                "blue" => {
                    blue = amount;
                }
                _ => {
                    panic!("Unexpected token in input: '{}'", t[1]);
                }
            }
        }

        Self { red, green, blue }
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    println!("The sum of all possible games is {}", do_challenge(&input));
}

/// Performs all steps required to determine the challenge answer, which is then returned.
fn do_challenge(input: &str) -> u32 {
    let games = parse_input(&input);
    let limits = CUBE_LIMITS;
    let possible_games = find_possible_game_ids(&games, &limits);
    possible_games.iter().map(|&n| n as u32).sum()
}

/// Takes a string containing the entire input file and converts each line into a `Game` struct.
/// A `Vec` of these `Game`s is returned.
///
/// # Panics
///
/// Panics on malformed input.
fn parse_input(input: &str) -> Vec<Game> {
    let mut games = Vec::new();

    for line in input.lines() {
        if line != "" {
            games.push(parse_line(line));
        }
    }

    games
}

/// Takes a string containing the one line of input and converts it into a `Game` struct which is
/// then returned.
///
/// # Panics
///
/// Panics on malformed input.
fn parse_line(s: &str) -> Game {
    let line_fields: Vec<&str> = s.split(':').collect();
    assert_eq!(
        2,
        line_fields.len(),
        "Each line of input should contain exactly 1 colon: {s}"
    );

    let id_raw = line_fields[0].strip_prefix("Game ").unwrap();
    let id = u8::from_str_radix(id_raw, 10).unwrap();

    let reveals_raw: Vec<&str> = line_fields[1].split(';').collect();

    let mut reveals = Vec::new();
    for r in reveals_raw {
        reveals.push(CubeSet::from_str(r));
    }

    Game { id, reveals }
}

/// Compares each `Game` passed in `games` to the limits for red, green and blue cubes given in
/// `limits`. Returns a `Vec` of the game ids whose revealed handfuls all have red, green and blue
/// amounts that do not exceed the limits.
fn find_possible_game_ids(games: &Vec<Game>, limits: &CubeSet) -> Vec<u8> {
    let mut possible_games = Vec::new();

    for g in games {
        let mut impossible_found = false;

        for r in &g.reveals {
            if r.red > limits.red || r.green > limits.green || r.blue > limits.blue {
                impossible_found = true;
            }
        }

        if !impossible_found {
            possible_games.push(g.id);
        }
    }

    possible_games
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_parse_line() {
        assert_eq!(
            Game {
                id: 99,
                reveals: vec![CubeSet {
                    red: 11,
                    green: 22,
                    blue: 33
                }]
            },
            parse_line("Game 99: 11 red, 22 green, 33 blue")
        );
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);

        assert_eq!(5, result.len());
        assert_eq!(
            Game {
                id: 1,
                reveals: vec![
                    CubeSet {
                        red: 4,
                        green: 0,
                        blue: 3
                    },
                    CubeSet {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    CubeSet {
                        red: 0,
                        green: 2,
                        blue: 0
                    },
                ]
            },
            result[0]
        );
        assert_eq!(
            Game {
                id: 2,
                reveals: vec![
                    CubeSet {
                        red: 0,
                        green: 2,
                        blue: 1
                    },
                    CubeSet {
                        red: 1,
                        green: 3,
                        blue: 4
                    },
                    CubeSet {
                        red: 0,
                        green: 1,
                        blue: 1
                    },
                ]
            },
            result[1]
        );
        assert_eq!(
            Game {
                id: 3,
                reveals: vec![
                    CubeSet {
                        red: 20,
                        green: 8,
                        blue: 6
                    },
                    CubeSet {
                        red: 4,
                        green: 13,
                        blue: 5
                    },
                    CubeSet {
                        red: 1,
                        green: 5,
                        blue: 0
                    },
                ]
            },
            result[2]
        );
        assert_eq!(
            Game {
                id: 4,
                reveals: vec![
                    CubeSet {
                        red: 3,
                        green: 1,
                        blue: 6
                    },
                    CubeSet {
                        red: 6,
                        green: 3,
                        blue: 0
                    },
                    CubeSet {
                        red: 14,
                        green: 3,
                        blue: 15
                    },
                ]
            },
            result[3]
        );
        assert_eq!(
            Game {
                id: 5,
                reveals: vec![
                    CubeSet {
                        red: 6,
                        green: 3,
                        blue: 1
                    },
                    CubeSet {
                        red: 1,
                        green: 2,
                        blue: 2
                    },
                ]
            },
            result[4]
        );
    }

    #[test]
    fn test_find_possible_game_ids() {
        let games = parse_input(TEST_INPUT);
        let limits = CubeSet {
            red: 12,
            green: 13,
            blue: 14,
        };
        let possible_games = find_possible_game_ids(&games, &limits);

        assert_eq!(vec![1, 2, 5], possible_games);
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(8, do_challenge(&TEST_INPUT));
    }
}
