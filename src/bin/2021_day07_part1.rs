//! Advent of Code 2021 Day 07
//! https://adventofcode.com/2021/day/7
//!
//! Challenge part 1
//!
//! Find the optimal way for a group of positions to move to the same position such that the
//! total movement is minimized.

use std::fs;

const INPUT_FILENAME: &str = "2021_day07_input.txt";

type Position = u32;

/// Parses an input string consisting of comma-separated numbers representing the crabs' initial
/// positions.
fn parse_input(input: &str) -> Vec<Position> {
    input
        .lines().collect::<Vec<&str>>()[0].split(",")
        .map(|i| Position::from_str_radix(i, 10).unwrap()).collect()
}


/// Find the least fuel that can be used to move all the given crabs to the same position.
fn minimum_fuel(crabs: &Vec<Position>) -> u32 {
    let sum = crabs.iter().fold(0, |acc, i| acc + i);
    let mean = f32::round(sum as f32 / crabs.len() as f32 / 2.0) as u32;

    let mut best_position = mean;
    let mut best_fuel = total_fuel_cost(crabs, best_position);

    loop {
        let next_fuel = total_fuel_cost(crabs, best_position + 1);
        if next_fuel > best_fuel {
            break;
        }

        best_fuel = next_fuel;
        best_position += 1;
    }

    loop {
        let next_fuel = total_fuel_cost(crabs, best_position - 1);
        if next_fuel > best_fuel {
            break;
        }

        best_fuel = next_fuel;
        best_position -= 1;
    }
    best_fuel
}


/// Calculate the total fuel used to move the given crabs to given position p.
fn total_fuel_cost(crabs: &Vec<Position>, p: Position) -> u32 {
    let mut total_fuel = 0;

    for c in crabs {
        if *c > p {
            total_fuel += *c - p;
        } else {
            total_fuel += p - *c;
        }
    }

    total_fuel
}


fn main() {
    let input_file =
        fs::read_to_string(INPUT_FILENAME)
            .expect("Error reading input file");

    let positions = parse_input(&input_file);

    println!("The total fuel cost is {}", minimum_fuel(&positions));
}


// Test using data from the examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn parse_test_input() {
        let crabs = parse_input(&TEST_INPUT);

        assert_eq!(crabs,  vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
    }

    #[test]
    fn test_minimum_total_movement() {
        let positions = parse_input(&TEST_INPUT);

        assert_eq!(minimum_fuel(&positions), 37);
    }
}
