//! Advent of Code 2020 Day 13
//! https://adventofcode.com/2020/day/13
//!
//! Challenge part 2
//!
//! Read current time and available buses from the input file, and find the time when all buses
//! leave a certain number of minutes after that time. The time in minutes is determined by the
//! position of the bus in the input, so the first bus (at index 0), leaves at time 't'. The bus
//! at index 1 leaves t+1, etc. Many index values are 'x', meaning that position can be ignored.

use std::fs;

const INPUT_FILENAME: &str = "2020_day13_input.txt";

/// A bus, identified by its `id` and the `delay` in minutes that it must leave after a given time.
/// The latter is determined from the buses position in the input file.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Bus {
    id: u64,
    delay: u64,
}

#[derive(Debug, PartialEq)]
struct Buses {
    buses: Vec<Bus>,
}

impl Buses {
    /// Create and return `Buses` from an input string. The first line of the input is discarded as
    /// it contains the timestamp, which is not used for this part of the challenge.
    fn from_input(input: &str) -> Self {
        let mut lines = input.lines();
        let _ = lines.next(); // Discard line containing timestamp.

        let mut buses = Vec::new();
        let tokens = lines.next().unwrap().split(',');

        for (i, t) in tokens.enumerate() {
            // println!("Index {} contains bus id: {}", i, &t);
            if t != "x" {
                buses.push(Bus {
                    id: t.parse::<u64>().unwrap(),
                    delay: i as u64,
                });
            }
        }

        Self { buses }
    }

    /// Sort the `buses` vector by bus `id`, largest to smallest.
    fn sort_descending(&mut self) {
        self.buses.sort_by(|a, b| a.id.cmp(&b.id));
        self.buses.reverse();
    }
}

/// Given a vector of buses sorted by bus `id`, largest first, returns a timestamp that meets the
/// challenge criteria, namely that each bus departs `delay` minutes after the timestamp. For
/// example, if we have buses: bus id 7 with delay 1; and bus id 5 with delay 2; this can be
/// represented as:
///
/// Timestamp:  0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26
/// Bus 7 (-1)  -  -  -  -  -  -  Y  -  -  -  -  -  -  Y  -  -  -  -  -  -  Y  -  -  -  -  -  -
/// Bus 5 (-2)  -  -  -  Y  -  -  -  -  Y  -  -  -  -  Y  -  -  -  -  Y  -  -  -  -  Y  -  -  -
///
/// Timestamp = 13 is the answer, because bus 7 leaves one minute later at t = 14 (which is
/// divisible by 7), and bus 5 leaves two minutes later at t = 15 (which is divisible by 5).
//
// To improve performance, the outer loop iterates over timestamp values that meet the criteria
// of the bus with the highest bus `id`. For example, if the highest bus `id` is 900 and its
// associated delay is 10 minutes, the timestamps considered are 890, 1790, 2690, etc. This
// eliminates needing to loop over timestamps from 0-889, 891-1789, etc., that would be wasted
// work.
fn find_challenge_answer(buses: &Buses) -> u64 {
    // println!("Sorted list of buses: {:#?}", buses);

    let loop_bus = &buses.buses[0];
    let buses_without_first = &buses.buses[1..];
    let mut t = loop_bus.id - (loop_bus.delay % loop_bus.id);
    'outer: loop {
        // print!("t = {}", t);

        for b in buses_without_first {
            if (t + b.delay) % b.id != 0 {
                // println!("\tCriteria not met for bus {} with delay {}", b.id, b.delay);
                t += loop_bus.id;
                continue 'outer;
            }
        }
        // println!("Solution found! {}");
        return t;
    }
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let mut buses = Buses::from_input(&input_file);
    buses.sort_descending();

    let answer = find_challenge_answer(&buses);
    println!("The answer to the challenge is {}", answer);
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_0: &str = "\
939
7,13,x,x,59,x,31,19";

    const TEST_INPUT_1: &str = "\
0
17,x,13,19";

    const TEST_INPUT_2: &str = "\
0
67,7,59,61";

    const TEST_INPUT_3: &str = "\
0
67,x,7,59,61";

    const TEST_INPUT_4: &str = "\
0
67,7,x,59,61";

    const TEST_INPUT_5: &str = "\
0
1789,37,47,1889";

    #[test]
    fn test_0() {
        let mut buses = Buses::from_input(TEST_INPUT_0);
        buses.sort_descending();
        let answer = find_challenge_answer(&buses);

        assert_eq!(answer, 1068781);
    }

    #[test]
    fn test_1() {
        let mut buses = Buses::from_input(TEST_INPUT_1);
        buses.sort_descending();
        let answer = find_challenge_answer(&buses);

        assert_eq!(answer, 3417);
    }

    #[test]
    fn test_2() {
        let mut buses = Buses::from_input(TEST_INPUT_2);
        buses.sort_descending();
        let answer = find_challenge_answer(&buses);

        assert_eq!(answer, 754018);
    }

    #[test]
    fn test_3() {
        let mut buses = Buses::from_input(TEST_INPUT_3);
        buses.sort_descending();
        let answer = find_challenge_answer(&buses);

        assert_eq!(answer, 779210);
    }

    #[test]
    fn test_4() {
        let mut buses = Buses::from_input(TEST_INPUT_4);
        buses.sort_descending();
        let answer = find_challenge_answer(&buses);

        assert_eq!(answer, 1261476);
    }

    #[test]
    fn test_5() {
        let mut buses = Buses::from_input(TEST_INPUT_5);
        buses.sort_descending();
        let answer = find_challenge_answer(&buses);

        assert_eq!(answer, 1202161486);
    }

    #[test]
    fn bus_parse() {
        let buses = Buses::from_input(TEST_INPUT_0);

        assert_eq!(
            buses,
            Buses {
                buses: vec!(
                    Bus { id: 7, delay: 0 },
                    Bus { id: 13, delay: 1 },
                    Bus { id: 59, delay: 4 },
                    Bus { id: 31, delay: 6 },
                    Bus { id: 19, delay: 7 },
                )
            }
        );
    }

    #[test]
    fn bus_sort() {
        let mut buses = Buses::from_input(TEST_INPUT_0);
        buses.sort_descending();

        assert_eq!(
            buses,
            Buses {
                buses: vec!(
                    Bus { id: 59, delay: 4 },
                    Bus { id: 31, delay: 6 },
                    Bus { id: 19, delay: 7 },
                    Bus { id: 13, delay: 1 },
                    Bus { id: 7, delay: 0 },
                )
            }
        );
    }
}
