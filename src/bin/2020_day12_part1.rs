//! Advent of Code 2020 Day 12
//! https://adventofcode.com/2020/day/12
//!
//! Challenge part 1
//!
//! Move the position and orientation of a ship based on commands in the input file. Determine the
//! Manhattan distance to its final position, which is the solution to the challenge.

use std::fs;

const INPUT_FILENAME: &str = "2020_day12_input.txt";
const ACCEPTABLE_DIRECTION: [u16; 4] = [0, 90, 180, 270];

/// A ship, consisting of integer `latitude` and `longitude`, and the direction the boat is facing.
/// The latter is limited to 0, 90, 180 and 270. Positive latitude is north and positive longitude
/// is east.
#[derive(Clone, Copy, Debug)]
struct Ship {
    latitude: i32,
    longitude: i32,
    facing: u16,
}

impl Ship {
    fn new() -> Self {
        Self {
            latitude: 0,
            longitude: 0,
            facing: 90,
        }
    }

    fn move_forward(&mut self, distance: i32) {
        match self.facing {
            0 => {
                self.latitude += distance;
            }
            90 => {
                self.longitude += distance;
            }
            180 => {
                self.latitude -= distance;
            }
            270 => {
                self.longitude -= distance;
            }
            _ => {
                panic!("Ship is facing an unexpected direction");
            }
        }
    }

    fn turn_left(&mut self, degrees: u16) {
        if ACCEPTABLE_DIRECTION.contains(&degrees) {
            self.facing = (self.facing + 360 - degrees) % 360;
        } else {
            panic!("turn_left() passed unrecognized value");
        }
    }

    fn turn_right(&mut self, degrees: u16) {
        if ACCEPTABLE_DIRECTION.contains(&degrees) {
            self.facing = (self.facing + degrees) % 360;
        } else {
            panic!("turn_right() passed unrecognized value");
        }
    }

    fn execute_single_command(&mut self, command: &str) {
        if command != "" {
            let command_chars: Vec<char> = command.chars().collect();
            let command = command_chars[0];

            match &command {
                'N' => {
                    let distance: i32 = command_chars[1..]
                        .iter()
                        .collect::<String>()
                        .parse()
                        .unwrap();

                    self.latitude += distance;
                    // print!("Shifting north {} units.", distance);
                    // println!("Position is now ({}, {})", self.latitude, self.longitude);
                }
                'S' => {
                    let distance: i32 = command_chars[1..]
                        .iter()
                        .collect::<String>()
                        .parse()
                        .unwrap();

                    self.latitude -= distance;
                    // print!("Shifting south {} units.", distance);
                    // println!("Position is now ({}, {})", self.latitude, self.longitude);
                }
                'E' => {
                    let distance: i32 = command_chars[1..]
                        .iter()
                        .collect::<String>()
                        .parse()
                        .unwrap();

                    self.longitude += distance;
                    // print!("Shifting east {} units.", distance);
                    // println!("Position is now ({}, {})", self.latitude, self.longitude);
                }
                'W' => {
                    let distance: i32 = command_chars[1..]
                        .iter()
                        .collect::<String>()
                        .parse()
                        .unwrap();

                    self.longitude -= distance;
                    // print!("Shifting west {} units.", distance);
                    // println!("Position is now ({}, {})", self.latitude, self.longitude);
                }
                'F' => {
                    let distance: i32 = command_chars[1..]
                        .iter()
                        .collect::<String>()
                        .parse()
                        .unwrap();
                    self.move_forward(distance);
                    // print!("Moving forward {} units.", distance);
                    // println!("Position is now ({}, {})", self.latitude, self.longitude);
                }
                'L' => {
                    let rotation: u16 = command_chars[1..]
                        .iter()
                        .collect::<String>()
                        .parse()
                        .unwrap();
                    self.turn_left(rotation);
                    // print!("Rotating left {} units.", rotation);
                    // println!("Ship is now facing {} degrees", self.facing);
                }
                'R' => {
                    let rotation: u16 = command_chars[1..]
                        .iter()
                        .collect::<String>()
                        .parse()
                        .unwrap();
                    self.turn_right(rotation);
                    // print!("Rotating right {} units.", rotation);
                    // println!("Ship is now facing {} degrees", self.facing);
                }
                _ => {
                    panic!("Unrecognized command {}", &command);
                }
            }
        }
    }

    fn execute_multiple_commands(&mut self, commands: &str) {
        for cmd in commands.lines() {
            if cmd != "" {
                self.execute_single_command(&cmd);
            }
        }
    }

    fn manhatten_distance(&self) -> u32 {
        (i32::abs(self.latitude) + i32::abs(self.longitude)) as u32
    }
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let mut ship = Ship::new();

    ship.execute_multiple_commands(&input_file);

    println!(
        "The answer to the challenge is {}",
        ship.manhatten_distance()
    );
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
F10
N3
F7
R90
F11";

    #[test]
    fn test_0() {
        let mut ship = Ship::new();

        ship.execute_multiple_commands(&TEST_INPUT);
        assert_eq!(ship.latitude, -8);
        assert_eq!(ship.longitude, 17);
        assert_eq!(ship.facing, 180);
        assert_eq!(ship.manhatten_distance(), 25);
    }

    #[test]
    fn test_turn_left() {
        let mut ship = Ship::new();
        assert_eq!(ship.facing, 90);
        ship.turn_left(180);
        assert_eq!(ship.facing, 270);
        ship.turn_left(90);
        assert_eq!(ship.facing, 180);
    }

    #[test]
    fn test_turn_right() {
        let mut ship = Ship::new();
        assert_eq!(ship.facing, 90);
        ship.turn_right(180);
        assert_eq!(ship.facing, 270);
        ship.turn_right(90);
        assert_eq!(ship.facing, 0);
    }

    #[test]
    fn test_move_forward() {
        let mut ship = Ship::new();

        ship.move_forward(5);
        assert_eq!(ship.latitude, 0);
        assert_eq!(ship.longitude, 5);

        ship.turn_right(90);
        assert_eq!(ship.facing, 180);

        ship.move_forward(8);
        assert_eq!(ship.latitude, -8);
        assert_eq!(ship.longitude, 5);
    }
}
