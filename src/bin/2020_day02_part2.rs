//! Advent of Code 2020 Day 02
//! https://adventofcode.com/2020/day/2
//!
//! Challenge part 2
//!
//! Read an input file containing a space delimited list of:
//!     1. the positions that a given character must appear in a string, with the first position
//!         being `1` (not 0), and the positions being separated with a hyphen, e.g., 7-8;
//!     2. the given character followed by a colon; and
//!     3. the string that must contain the given character.
//!
//! Example: `7-8 x: qxrxmxccxxx`
//!
//! A valid string has the given character in just one of the two given positions. The result
//! is the number of strings considered valid. The example above will not be counted because
//! positions `7` and `8` (mapping to 6 and 7 in Rust terms), contain `c` and `c`.

use std::fs;

const INPUT_FILENAME: &str = "2020_day02_input.txt";

/// Validate the strings in the `input` passed against the rules specified in the challenge.
/// Return the number of valid strings.
fn validate_input(input: &str) -> u32 {
    let mut valid_string_count = 0;
    for (line_num, line) in input.lines().enumerate() {
        // println!("Line #{}, {}", line_num, line);
        let line_fields: Vec<&str> = line.split(' ').collect();
        // println!("\t{:?}", line_fields);

        if line_fields.len() != 3 {
            println!(
                "Line {} does not have three space delimited fields, which is required",
                line_num
            );
            break;
        }

        let required_char = line_fields.get(1).unwrap().get(..1).unwrap();
        // println!("\tRequired character is '{}'", required_char);

        let positions: Vec<&str> = line_fields.get(0).unwrap().split('-').collect();
        let position0 = positions.get(0).unwrap().parse::<usize>().unwrap() - 1;
        let position1 = positions.get(1).unwrap().parse::<usize>().unwrap() - 1;
        // println!("\tPositions (in Rust terms) are {} and {}", position0, position1);

        if positions.len() == 2 {
            let s = line_fields.get(2).unwrap();

            let char0 = s.get(position0..position0 + 1).unwrap();
            let char1 = s.get(position1..position1 + 1).unwrap();
            // println!("\tCharacters at those positions are {} and {}", char0, char1);

            if ((char0 == required_char) | (char1 == required_char)) & (char0 != char1) {
                valid_string_count += 1;
                // println!("\tValidation successful");
            // } else {
                // println!("\tValidation failed");
            }
        } else {
            println!(
                "Line {} does not contain two positions, which is required",
                line_num
            );
        }
    }
    valid_string_count
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let valid_string_count = validate_input(&input);

    println!("{} strings are valid", valid_string_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_0: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn success() {
        assert_eq!(validate_input(INPUT_0), 1);
    }
}
