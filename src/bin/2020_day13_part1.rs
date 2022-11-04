//! Advent of Code 2020 Day 13
//! https://adventofcode.com/2020/day/13
//!
//! Challenge part 1
//!
//! Read current time and available buses from the input file, and work out the next bus that will
//! arrive after that timestamp.

use std::fs;

const INPUT_FILENAME: &str = "2020_day13_input.txt";

fn parse_buses(input: &str) -> Vec<u16> {
    let mut buses = Vec::new();
    let tokens = input.split(',');

    for t in tokens {
        // println!("Token: {}", &t);
        if t != "x" {
            buses.push(t.parse::<u16>().unwrap());
        }
    }

    buses
}

/// Determines which bus will leave first after `timestamp`. Returns the id of this bus and how
/// long after `timestamp` it leaves as a pair of values in this order.
fn find_earliest_bus(buses: &Vec<u16>, timestamp: u32) -> (u16, u32) {
    let mut earliest_bus = u16::MAX;
    let mut earliest_time_delta = u32::MAX;

    for b in buses {
        let time_until_next_bus = *b as u32 - (timestamp % *b as u32);

        if time_until_next_bus < earliest_time_delta {
            earliest_time_delta = time_until_next_bus;
            earliest_bus = *b;
        }
    }

    (earliest_bus, earliest_time_delta)
}

fn do_challenge(input: &str) -> u32 {
    let mut lines = input.lines();
    let timestamp = lines.next().unwrap().parse::<u32>().unwrap();
    let buses = parse_buses(&lines.next().unwrap());

    // println!("Timestamp: {}", timestamp);
    // println!("Buses: {:?}", &buses);

    let bus_and_leaving_time = find_earliest_bus(&buses, timestamp);
    // println!("Bus: {}", bus_and_leaving_time.0);
    // println!("Timestamp it leaves: {}", bus_and_leaving_time.1);

    bus_and_leaving_time.0 as u32 * bus_and_leaving_time.1
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let answer = do_challenge(&input_file);
    println!("The answer to the challenge is {}", answer);
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
939
7,13,x,x,59,x,31,19";

    #[test]
    fn test_0() {
        let answer = do_challenge(&TEST_INPUT);
        assert_eq!(answer, 295);
    }
}
