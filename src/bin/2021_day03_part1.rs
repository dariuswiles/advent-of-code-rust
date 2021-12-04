//! Advent of Code 2021 Day 03
//! https://adventofcode.com/2021/day/3
//!
//! Challenge part 1
//!
//! For each bit position in a sequence of numbers, determine whether 0 or 1 is most common, use
//! this to derive the gamma and epsilon rates defined in the challenge and multiply them to get
//! the answer.

use std::collections::HashMap;
use std::fs;

const INPUT_FILENAME: &str = "2021_day03_input.txt";

fn calculate_gamma_epsilon(input: &str) -> (String, String) {
    let mut line_count = 0;
    let mut count_of_ones = HashMap::new();
    let mut bits_per_line = None;


    for line in input.lines() {
        if line == "" {
            continue;
        }

        line_count += 1;

        if bits_per_line == None {
            bits_per_line = Some(line.len());

            for i in 0..bits_per_line.unwrap() {
                count_of_ones.insert(i, 0);
            }
        } else {
            if bits_per_line.unwrap() != line.len() {
                panic!("All input lines must contain the same number of bits");
            }
        }

        for (position, bit) in line.chars().enumerate() {
            if bit == '1' {
                *count_of_ones.get_mut(&position).unwrap() += 1;
            }
        }
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();
    for i in 0..bits_per_line.unwrap() {
        let count = count_of_ones[&i];

        if count < line_count / 2 {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }

    (gamma, epsilon)
}


fn multiply_gamma_epsilon(gamma: &str, epsilon: &str) -> u32 {
    u32::from_str_radix(&gamma, 2).unwrap() * u32::from_str_radix(&epsilon, 2).unwrap()
}


fn main() {
    let input_file =
        fs::read_to_string(INPUT_FILENAME)
            .expect("Error reading input file");

    let (gamma, epsilon) = calculate_gamma_epsilon(&input_file);

    println!("gamma = {}, epsilon = {}", gamma, epsilon);

    let answer = multiply_gamma_epsilon(&gamma, &epsilon);
    println!("The submarine's power consupmtion is {}", answer);
}


// Test using data from the examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

    const TEST_INPUT_BAD_LENGTH: &str =
r#"00100
11110
101
10111"#;

    #[test]
    fn parse_test_input() {
        let (gamma, epsilon) = calculate_gamma_epsilon(&TEST_INPUT);

        assert_eq!(gamma, "10110");
        assert_eq!(epsilon, "01001");
    }

    #[test]
    fn result() {
        let (gamma, epsilon) = calculate_gamma_epsilon(&TEST_INPUT);
        assert_eq!(multiply_gamma_epsilon(&gamma, &epsilon), 198);
    }

    #[test]
    #[should_panic]
    fn different_line_lengths() {
        calculate_gamma_epsilon(&TEST_INPUT_BAD_LENGTH);
    }
}
