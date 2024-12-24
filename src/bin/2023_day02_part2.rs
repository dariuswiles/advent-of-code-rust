//! Advent of Code 2023 Day 02
//! https://adventofcode.com/2023/day/2
//!
//! Challenge part 2
//!
//! Model a game consisting of a bag of colored cubes, from which several random handfuls of cubes
//! are taken out and shown to us. Part 2 of the challenge consists of determining the minimum
//! number of cubes of each color must be in the bag for the revealed sets of cubes to be possible.
//! The challenge answer is the sum of the product of the number of these cubes for each game.

use std::fs;

const INPUT_FILENAME: &str = "2023_day02_input.txt";

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
            let amount = t[0].parse().unwrap();

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
    println!(
        "The sum of the powers of the minimum cubes required for each game is {}",
        do_challenge(&input)
    );
}

/// Performs all steps required to determine the challenge answer, which is then returned.
fn do_challenge(input: &str) -> u32 {
    let games = parse_input(input);

    games
        .iter()
        .map(|g| cubeset_power(&minimum_cubeset(&g.reveals)))
        .sum()
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
        if !line.is_empty() {
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
    let id = id_raw.parse().unwrap();

    let reveals_raw: Vec<&str> = line_fields[1].split(';').collect();

    let mut reveals = Vec::new();
    for r in reveals_raw {
        reveals.push(CubeSet::from_str(r));
    }

    Game { id, reveals }
}

/// Returns the minimum numbers of red, green and blue cubes that are required for a game to have
/// enough cubes for the `reveals` of cubes passed.
fn minimum_cubeset(reveals: &Vec<CubeSet>) -> CubeSet {
    let mut min_set = CubeSet {
        red: 0,
        green: 0,
        blue: 0,
    };

    for r in reveals {
        min_set.red = min_set.red.max(r.red);
        min_set.green = min_set.green.max(r.green);
        min_set.blue = min_set.blue.max(r.blue);
    }

    min_set
}

/// Returns the "power" of a set of cubes, as defined in the challenge. It is calculated by
/// multiplying the numbers of red, green and blue cubes in the `CubeSet` passed.
fn cubeset_power(c: &CubeSet) -> u32 {
    c.red as u32 * c.green as u32 * c.blue as u32
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
                    blue: 33,
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
                        blue: 3,
                    },
                    CubeSet {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    CubeSet {
                        red: 0,
                        green: 2,
                        blue: 0,
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
                        blue: 1,
                    },
                    CubeSet {
                        red: 1,
                        green: 3,
                        blue: 4,
                    },
                    CubeSet {
                        red: 0,
                        green: 1,
                        blue: 1,
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
                        blue: 6,
                    },
                    CubeSet {
                        red: 4,
                        green: 13,
                        blue: 5,
                    },
                    CubeSet {
                        red: 1,
                        green: 5,
                        blue: 0,
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
                        blue: 6,
                    },
                    CubeSet {
                        red: 6,
                        green: 3,
                        blue: 0,
                    },
                    CubeSet {
                        red: 14,
                        green: 3,
                        blue: 15,
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
    fn minimum_cubeset_game1() {
        assert_eq!(
            CubeSet {
                red: 4,
                green: 2,
                blue: 6,
            },
            minimum_cubeset(&vec![
                CubeSet {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                CubeSet {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                CubeSet {
                    red: 0,
                    green: 2,
                    blue: 0,
                },
            ]),
        );
    }

    #[test]
    fn minimum_cubeset_game2() {
        assert_eq!(
            CubeSet {
                red: 1,
                green: 3,
                blue: 4,
            },
            minimum_cubeset(&vec![
                CubeSet {
                    red: 0,
                    green: 2,
                    blue: 1,
                },
                CubeSet {
                    red: 1,
                    green: 3,
                    blue: 4,
                },
                CubeSet {
                    red: 0,
                    green: 1,
                    blue: 1,
                },
            ]),
        );
    }

    #[test]
    fn minimum_cubeset_game3() {
        assert_eq!(
            CubeSet {
                red: 20,
                green: 13,
                blue: 6,
            },
            minimum_cubeset(&vec![
                CubeSet {
                    red: 20,
                    green: 8,
                    blue: 6,
                },
                CubeSet {
                    red: 4,
                    green: 13,
                    blue: 5,
                },
                CubeSet {
                    red: 1,
                    green: 5,
                    blue: 0,
                },
            ]),
        );
    }

    #[test]
    fn minimum_cubeset_game4() {
        assert_eq!(
            CubeSet {
                red: 14,
                green: 3,
                blue: 15,
            },
            minimum_cubeset(&vec![
                CubeSet {
                    red: 3,
                    green: 1,
                    blue: 6,
                },
                CubeSet {
                    red: 6,
                    green: 3,
                    blue: 0,
                },
                CubeSet {
                    red: 14,
                    green: 3,
                    blue: 15,
                },
            ]),
        );
    }

    #[test]
    fn minimum_cubeset_game5() {
        assert_eq!(
            CubeSet {
                red: 6,
                green: 3,
                blue: 2,
            },
            minimum_cubeset(&vec![
                CubeSet {
                    red: 6,
                    green: 3,
                    blue: 1,
                },
                CubeSet {
                    red: 1,
                    green: 2,
                    blue: 2,
                },
            ]),
        );
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(2286, do_challenge(TEST_INPUT));
    }
}
