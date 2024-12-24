//! Advent of Code 2021 Day 02
//! https://adventofcode.com/2021/day/2
//!
//! Challenge part 1
//!
//! Read a file of submarine commands and execute them to determine the product of the submarine's
//! final horizontal position and depth.

use std::fs;

const INPUT_FILENAME: &str = "2021_day02_input.txt";

type Horizontal = u32;
type Depth = u32;

#[derive(Debug, PartialEq)]
enum Command {
    Down(u32),
    Forward(u32),
    Up(u32),
}

#[derive(Debug, PartialEq)]
struct Commands {
    commands: Vec<Command>,
}

impl Commands {
    fn parse_commands(code: &str) -> Self {
        let mut commands = Vec::new();

        for line in code.lines() {
            if line.is_empty() {
                continue;
            }

            let tokens: Vec<&str> = line.split(" ").collect();

            if tokens.len() != 2 {
                panic!("Malformed command: {}", &line);
            }

            match tokens[0] {
                "down" => {
                    commands.push(Command::Down(tokens[1].parse().unwrap()));
                }
                "forward" => {
                    commands.push(Command::Forward(tokens[1].parse().unwrap()));
                }
                "up" => {
                    commands.push(Command::Up(tokens[1].parse().unwrap()));
                }
                _ => {
                    panic!("Unrecognized command: {}", &line);
                }
            }
        }

        Self { commands }
    }

    /// Executes the commands in this struct and returns the resultant horizontal position and
    /// depth in a pair.
    fn execute_commands(&self) -> (Horizontal, Depth) {
        let mut horizontal = 0;
        let mut depth = 0;

        for c in &self.commands {
            match c {
                Command::Down(d) => {
                    depth += d;
                }
                Command::Forward(f) => {
                    horizontal += f;
                }
                Command::Up(u) => {
                    depth -= u;
                }
            }
        }

        (horizontal, depth)
    }
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let position = Commands::parse_commands(&input_file).execute_commands();

    println!(
        "The product of the submarine's final position is {}",
        position.0 * position.1
    );
}

// Test using data from the examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn parse_test_input() {
        let result = Commands::parse_commands(TEST_INPUT);
        let mut result_iter = result.commands.iter();

        assert_eq!(result_iter.next(), Some(&Command::Forward(5)));
        assert_eq!(result_iter.next(), Some(&Command::Down(5)));
        assert_eq!(result_iter.next(), Some(&Command::Forward(8)));
        assert_eq!(result_iter.next(), Some(&Command::Up(3)));
        assert_eq!(result_iter.next(), Some(&Command::Down(8)));
        assert_eq!(result_iter.next(), Some(&Command::Forward(2)));
        assert_eq!(result_iter.next(), None);
    }

    #[test]
    fn check_horizontal_and_depth() {
        let c = Commands::parse_commands(TEST_INPUT);

        assert_eq!(c.execute_commands(), (15, 10));
    }
}
