//! Advent of Code 2020 Day 02
//! https://adventofcode.com/2020/day/2
//!
//! Challenge part 1
//!
//! Read an input file containing a space delimited list of:
//!     1. the number of times a given character must appear in a string, as an inclusive range,
//!         e.g., 7-8;
//!     2. the given character followed by a colon; and
//!     3. the string that must contain the given character.
//!
//! Example: `7-8 x: qxrxmxccxxx`
//!
//! The result is the number of strings that contain their associated character the given number of
//! times. The example above will not be counted because it contains **6** occurrences of `x`, but
//! requires **7** or **8**.

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

        let repeat_range: Vec<&str> = line_fields.get(0).unwrap().split('-').collect();
        let range_lower: usize = repeat_range.get(0).unwrap().parse().unwrap();
        let range_upper: usize = repeat_range.get(1).unwrap().parse().unwrap();
        // println!("\tRange is from {} to {} (inclusive)", range_lower, range_upper);

        let char_to_count = line_fields.get(1).unwrap().get(..1).unwrap();
        // println!("\tCharacter to count is '{}'", char_to_count);

        let num_matches = line_fields
            .get(2)
            .unwrap()
            .matches(char_to_count)
            .collect::<Vec<&str>>()
            .len();
        // println!("\tCharacter occurs {:?} times in the string", num_matches);

        if (num_matches >= range_lower) & (num_matches <= range_upper) {
            valid_string_count += 1;
            // println!("\tValidation successful");
            // } else {
            // println!("\tValidation failed");
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
        assert_eq!(validate_input(INPUT_0), 2);
    }
}
