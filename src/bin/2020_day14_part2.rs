//! Advent of Code 2020 Day 14
//! https://adventofcode.com/2020/day/14
//!
//! Challenge part 2
//!
//! Execute commands from an input file to update values in a HashMap representing memory, then
//! sum all non-zero values to find the challenge answer. Bitmasks are created to modify memory
//! locations, and presence of wildcards in the bitmask allows one memory set instruction to update
//! multiple locations.

use std::collections::HashMap;
use std::fs;

const INPUT_FILENAME: &str = "2020_day14_input.txt";
const BITMASK_LENGTH: usize = 36;

#[derive(Debug, Default)]
struct Bitmask {
    always_set: u64,
    wildcard: Vec<usize>,
}

impl Bitmask {
    fn from_str(s: &str) -> Self {
        let mut set = 0u64;
        let mut wildcard = Vec::new();

        for (i, c) in s.chars().enumerate() {
            match &c {
                'X' => {
                    wildcard.push(BITMASK_LENGTH - i - 1);
                }
                '0' => {
                   continue;
                }
                '1' => {
                    let new_mask_bit = 1 << (BITMASK_LENGTH - i - 1);
                    set |= new_mask_bit;
                }
                _ => {
                    panic!(format!("Unrecognized character in bitmask '{}'", s));
                }
            }
        }
//         println!("Bitmask created from string '{:0>64}'", s);
//         println!("`always_set` mask i         '{:0>64b}'", set);
//         println!("Positions of wildcard bits are '{:#?}'", &wildcard);

        wildcard.sort_unstable();

        Self { always_set: set, wildcard: wildcard, }
    }

    /// Applies this bitmask to the given memory `location` and returns one or more resultant
    /// memory locations. Multiple locations are returned if the bitmask contains wildcards, i.e.,
    /// `X`s.
    fn apply_bitmask(&self, location: usize) -> Vec<usize> {
        let loc_set = location | self.always_set as usize;
        let mut locs = Vec::new();
        let wildcard_len = self.wildcard.len();



        // To generate all wildcard combinations, an outer loop iterates through enough integers to
        // cover all possible wildcard permutations. An inner loop isolates each bit in the outer
        // loop counter and uses it to modify a bit in the memory `location` passed.
        for control_bits in 0..u64::pow(2, wildcard_len as u32) {
            let mut wildcard_bit_flips = 0u64;

            for (mem_index, wildcard_position) in self.wildcard.iter().enumerate() {
//                 print!("control_bits={:3}; mem_index={}; wildcard_position={:4}; ",
//                 control_bits, mem_index, wildcard_position);

                let mut control_bit = control_bits & 1<<mem_index;

                control_bit <<= wildcard_position - mem_index;

                wildcard_bit_flips |= control_bit;

//                 println!("\tcontrol_bit={:0>36b}; wildcard_bit_flips={:0>36b}", control_bit,
//                     wildcard_bit_flips);
            }

//             println!("              Applying wildcard_bit_flips {:0>36b}", wildcard_bit_flips);
//             println!("    to location with `always_set` applied {:0>36b}", loc_set);
//             println!("                                   giving {:0>36b}", loc_set ^
//                 wildcard_bit_flips as usize);

            locs.push(loc_set ^ wildcard_bit_flips as usize);

        }

        locs
    }
}


/// Parse the `location` and `value` strings representing a command to save a value to a location
/// in memory, and return a pair of values representing validated numeric equivalents.
fn parse_mem_command(location: &str, value: &str) -> (usize, u64) {
//     println!("Entered update_memory with location='{}' and value='{}'", location, value);

    let loc_str: Vec<&str> = location.strip_suffix(']').unwrap().split("[").collect();
    if loc_str.len() != 2 {
        panic!(format!("Unrecognized format of command '{}'", location));
    }

    (loc_str[1].parse::<usize>().unwrap(), value.parse::<u64>().unwrap())
}


/// Reads each line of the input string and executes the commands found. Returns a `HashMap`
/// containing the memory locations and values set as a result of executing the commands.
fn execute_input(input: &str) -> HashMap<usize, u64> {
    let mut mask = Bitmask::default();
    let mut memory = HashMap::new();

    for line in input.lines() {
        if line == "" { continue; }

        let token: Vec<&str> = line.split(" = ").collect();
        if token.len() != 2 {
            panic!(format!("Unrecognized format of line '{}'", &line));
        }

        if token[0].starts_with("mask") {
            mask = Bitmask::from_str(&line.strip_prefix("mask = ").unwrap());
        } else if token[0].starts_with("mem") {
            let loc_val = parse_mem_command(&token[0], &token[1]);

            let masked_locations = mask.apply_bitmask(loc_val.0);

            for loc in masked_locations {
                memory.insert(loc as usize, loc_val.1);
//                 println!("Set memory location {} to value {}", loc, loc_val.1);
            }

        } else {
            panic!(format!("Unrecognized command '{}'", &token[0]));
        }
    }

    memory
}



fn main() {
    let input_file =
        fs::read_to_string(INPUT_FILENAME)
            .expect("Error reading input file");

    let mem = execute_input(&input_file);

    let answer: u64 = mem.values().sum();

    println!("The answer to the challenge is {}", answer);
}


// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_0: &str = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn test_bitmask() {
        let bm = Bitmask::from_str("000000000000000000000000000000X1001X");
        assert_eq!(bm.always_set, 0b000000000000000000000000000000010010);
        assert_eq!(bm.wildcard, vec![0, 5]);

        let mut locations = bm.apply_bitmask(42);
        locations.sort_unstable();

        assert_eq!(locations, vec![26, 27, 58, 59]);
    }

    #[test]
    fn test_execute_input() {
        let mem = execute_input(&TEST_INPUT_0);

        assert_eq!(mem.len(), 10);
        assert_eq!(mem[&16], 1);
        assert_eq!(mem[&17], 1);
        assert_eq!(mem[&18], 1);
        assert_eq!(mem[&19], 1);
        assert_eq!(mem[&24], 1);
        assert_eq!(mem[&25], 1);
        assert_eq!(mem[&26], 1);
        assert_eq!(mem[&27], 1);
        assert_eq!(mem[&58], 100);
        assert_eq!(mem[&59], 100);
    }

    #[test]
    fn test_challenge() {
        let mem = execute_input(&TEST_INPUT_0);
        let answer: u64 = mem.values().sum();

        assert_eq!(answer, 208);
    }
}
