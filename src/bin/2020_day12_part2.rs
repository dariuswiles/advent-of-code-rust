//! Advent of Code 2020 Day 12
//! https://adventofcode.com/2020/day/12
//!
//! Challenge part 2
//!
//! Move the position of a ship based on commands in the input file. Determine the Manhattan
//! distance to its final position, which is the solution to the challenge. Part 2 differs from
//! part 1 by introducing a waypoint and modifying the meaning of a few commands.

use std::fs;

const INPUT_FILENAME: &str = "2020_day12_input.txt";
const ACCEPTABLE_DIRECTION: [u16; 4] = [0, 90, 180, 270];

/// A ship, consisting of an absolute position expressed as `latitude` and `longitude`, and a
/// waypoint that is always relative to the current position of the ship. Positive latitude is
/// north and positive longitude is east.
#[derive(Clone, Copy, Debug)]
struct Ship {
    latitude: i32,
    longitude: i32,
    way_latitude: i32,
    way_longitude: i32,
}

impl Ship {
    fn new() -> Self {
        Self {
            latitude: 0,
            longitude: 0,
            way_latitude: 1,
            way_longitude: 10,
        }
    }

    fn move_forward(&mut self, iterations: i32) {
        self.latitude += iterations * self.way_latitude;
        self.longitude += iterations * self.way_longitude;
    }

    fn turn_left(&mut self, degrees: u16) {
        if ACCEPTABLE_DIRECTION.contains(&degrees) {
            for _ in 0..degrees / 90 {
                let tmp = self.way_longitude;
                self.way_longitude = -self.way_latitude;
                self.way_latitude = tmp;
            }
        } else {
            panic!("turn_left() passed unrecognized value");
        }
    }

    fn turn_right(&mut self, degrees: u16) {
        if ACCEPTABLE_DIRECTION.contains(&degrees) {
            for _ in 0..degrees / 90 {
                let tmp = self.way_latitude;
                self.way_latitude = -self.way_longitude;
                self.way_longitude = tmp;
            }
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

                    self.way_latitude += distance;
                    // print!("Shifting waypoint north {} units.", distance);
                    // println!("Position is now ({}, {})", self.way_latitude, self.way_longitude);
                }
                'S' => {
                    let distance: i32 = command_chars[1..]
                        .iter()
                        .collect::<String>()
                        .parse()
                        .unwrap();

                    self.way_latitude -= distance;
                    // print!("Shifting waypoint south {} units.", distance);
                    // println!("Position is now ({}, {})", self.way_latitude, self.way_longitude);
                }
                'E' => {
                    let distance: i32 = command_chars[1..]
                        .iter()
                        .collect::<String>()
                        .parse()
                        .unwrap();

                    self.way_longitude += distance;
                    // print!("Shifting waypoint east {} units.", distance);
                    // println!("Position is now ({}, {})", self.way_latitude, self.way_longitude);
                }
                'W' => {
                    let distance: i32 = command_chars[1..]
                        .iter()
                        .collect::<String>()
                        .parse()
                        .unwrap();

                    self.way_longitude -= distance;
                    // print!("Shifting waypoint west {} units.", distance);
                    // println!("Position is now ({}, {})", self.way_latitude, self.way_longitude);
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
        assert_eq!(ship.latitude, -72);
        assert_eq!(ship.longitude, 214);
        assert_eq!(ship.manhatten_distance(), 286);
    }

    #[test]
    fn test_turn_left() {
        let mut ship = Ship::new();
        ship.turn_left(90);
        assert_eq!(ship.way_latitude, 10);
        assert_eq!(ship.way_longitude, -1);
    }

    #[test]
    fn test_turn_right() {
        let mut ship = Ship::new();
        ship.turn_right(90);
        assert_eq!(ship.way_latitude, -10);
        assert_eq!(ship.way_longitude, 1);
    }

    #[test]
    fn test_move_forward() {
        let mut ship = Ship::new();
        ship.move_forward(10);
        assert_eq!(ship.latitude, 10);
        assert_eq!(ship.longitude, 100);
    }
}
