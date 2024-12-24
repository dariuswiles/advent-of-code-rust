//! Advent of Code 2020 Day 14
//! https://adventofcode.com/2020/day/14
//!
//! Challenge part 1
//!
//! Execute commands from an input file to update values in a HashMap representing memory, then
//! sum all non-zero values to find the challenge answer. Bitmasks are created to modify the
//! values being saved to memory.

use std::collections::HashMap;
use std::fs;

const INPUT_FILENAME: &str = "2020_day14_input.txt";
const BITMASK_LENGTH: usize = 36;

#[derive(Debug, Default)]
struct Bitmask {
    always_set: u64,
    always_clear: u64,
}

impl Bitmask {
    fn from_str(s: &str) -> Self {
        let mut clear = u64::MAX;
        let mut set = 0u64;

        for (i, c) in s.chars().enumerate() {
            match &c {
                'X' => {
                    continue;
                }
                '0' => {
                    let new_mask_bit = 1 << (BITMASK_LENGTH - i - 1);
                    clear ^= new_mask_bit;
                }
                '1' => {
                    let new_mask_bit = 1 << (BITMASK_LENGTH - i - 1);
                    set |= new_mask_bit;
                }
                _ => {
                    panic!("Unrecognized character in bitmask '{}'", s);
                }
            }
        }
        // println!("Bitmask created from string '{:>64}'", s);
        // println!("Always clear mask is        '{:64b}'", clear);
        // println!("Always set mask is          '{:0>64b}'", set);

        Self {
            always_set: set,
            always_clear: clear,
        }
    }

    fn apply_bitmask(&self, num: u64) -> u64 {
        (num & self.always_clear) | self.always_set
    }
}

/// Parse the `location` and `value` strings representing a command to save a value to a location
/// in memory, and return a pair of values representing validation numeric equivalents.
fn parse_mem_command(location: &str, value: &str) -> (u32, u64) {
    // println!("Entered update_memory with location='{}' and value='{}'", location, value);

    let loc_str: Vec<&str> = location.strip_suffix(']').unwrap().split("[").collect();
    if loc_str.len() != 2 {
        panic!("Unrecognized format of command '{}'", location);
    }

    (
        loc_str[1].parse::<u32>().unwrap(),
        value.parse::<u64>().unwrap(),
    )
}

/// Reads each line of the input string and executes the commands found. Returns a `HashMap`
/// containing the results of executing the commands.
fn execute_input(input: &str) -> HashMap<u32, u64> {
    let mut mask = Bitmask::default();
    let mut memory = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let token: Vec<&str> = line.split(" = ").collect();
        if token.len() != 2 {
            panic!("Unrecognized format of line '{}'", &line);
        }

        if token[0].starts_with("mask") {
            mask = Bitmask::from_str(line.strip_prefix("mask = ").unwrap());
        } else if token[0].starts_with("mem") {
            let loc_val = parse_mem_command(token[0], token[1]);

            let masked_val = mask.apply_bitmask(loc_val.1);
            memory.insert(loc_val.0, masked_val);

        // println!("Set memory location {} to value {}", loc_val.0, masked_val);
        } else {
            panic!("Unrecognized command '{}'", &token[0]);
        }
    }

    memory
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let mem = execute_input(&input_file);

    let answer: u64 = mem.values().sum();

    println!("The answer to the challenge is {}", answer);
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_0: &str = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn test_bitmask() {
        let bm = Bitmask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(bm.always_set, 0b000000000000000000000000000001000000);
        assert_eq!(bm.always_clear, u64::MAX - 2);

        assert_eq!(bm.apply_bitmask(11), 73);
        assert_eq!(bm.apply_bitmask(101), 101);
        assert_eq!(bm.apply_bitmask(0), 64);
    }

    #[test]
    fn test_execute_input() {
        let mem = execute_input(TEST_INPUT_0);

        assert_eq!(mem[&7], 101);
        assert_eq!(mem[&8], 64);
    }
}
