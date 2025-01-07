//! Advent of Code 2024 Day 07
//! https://adventofcode.com/2024/day/7
//!
//! Challenge part 1
//!
//! The input consists of a list of test values and associated sequences of numbers. The challenge
//! is to determine which sequences can total their test value by inserting all permutations of
//! multiplication and addition operators between the numbers. The equations are always evaluated
//! left-to-right rather than by using the usual math precedence rules. The challenge answer is the
//! sum of the test values of all equations that can equal their associated test value.

use std::fs;

const INPUT_FILENAME: &str = "2024_day07_input.txt";

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    println!(
        "The sum of the test values of all equations that can possible be true is {}",
        do_challenge(&input)
    );
}

/// Determines which sequences can total their test value by inserting all permutations of
/// multiplication and addition operators between the numbers. The equations are always evaluated
/// left-to-right rather than by using the usual math precedence rules. Returns the challenge
/// answer, i.e., the sum of the test values of all equations that can equal their associated test
/// value.
fn do_challenge(input: &str) -> u64 {
    let test_value_equations = parse_input(input);

    let mut total = 0;
    for (test_value, equation) in test_value_equations {
        if check_equation_validity(test_value, &equation) > 0 {
            total += test_value;
        }
    }

    total
}

/// Returns a `Vec` where each entry is a tuple containing the test value required and the integers
/// in its associated equation.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    let mut result = Vec::new();

    for line in input.lines() {
        if !line.is_empty() {
            let tokens: Vec<_> = line.split(": ").collect();

            assert_eq!(
                tokens.len(),
                2,
                "Each line of input must contain exactly one colon"
            );
            let test_value = tokens[0]
                .parse()
                .expect("Malformed test value {test_value}");
            let integers: Vec<u64> = tokens[1]
                .split(" ")
                .map(|s| s.parse().expect("Malformed integer {s}"))
                .collect();

            result.push((test_value, integers));
        }
    }

    result
}

/// Exhaustively generates totals from all permutations of multiplying and adding every number in
/// `equation`. Returns the number of permutations that total to `test_value`.
fn check_equation_validity(test_value: u64, equation: &[u64]) -> u64 {
    _check_equation_validity_internal(test_value, &equation[1..], equation[0])
}

/// Internal function that recursively generates all totals that can be obtained from multiplying
/// and/or adding the `subtotal` to all other values in `equation`. Returns the number of ways
/// `test_value` is obtained by trying all combinations of multipyling and adding.
fn _check_equation_validity_internal(test_value: u64, equation: &[u64], subtotal: u64) -> u64 {
    if equation.is_empty() {
        // println!("Subtotal = {subtotal}");
        if subtotal == test_value {
            return 1;
        } else {
            return 0;
        }
    }

    let current: u64 = equation[0];
    let rest: &[u64] = &equation[1..];

    _check_equation_validity_internal(test_value, rest, subtotal * current)
        + _check_equation_validity_internal(test_value, rest, subtotal + current)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test_parse_input() {
        let test_value_equations = parse_input(INPUT);

        assert_eq!(9, test_value_equations.len());
        assert_eq!((190, vec![10, 19]), test_value_equations[0]);
        assert_eq!((3267, vec![81, 40, 27]), test_value_equations[1]);
        assert_eq!((83, vec![17, 5]), test_value_equations[2]);
        assert_eq!((156, vec![15, 6]), test_value_equations[3]);
        assert_eq!((7290, vec![6, 8, 6, 15]), test_value_equations[4]);
        assert_eq!((161011, vec![16, 10, 13]), test_value_equations[5]);
        assert_eq!((192, vec![17, 8, 14]), test_value_equations[6]);
        assert_eq!((21037, vec![9, 7, 18, 13]), test_value_equations[7]);
        assert_eq!((292, vec![11, 6, 16, 20]), test_value_equations[8]);
    }

    #[test]
    fn test_check_equation_validity() {
        assert_eq!(1, check_equation_validity(190, &vec![10, 19]));
        assert_eq!(2, check_equation_validity(3267, &vec![81, 40, 27]));
        assert_eq!(0, check_equation_validity(83, &vec![17, 5]));
        assert_eq!(0, check_equation_validity(156, &vec![15, 6]));
        assert_eq!(0, check_equation_validity(7290, &vec![6, 8, 6, 15]));
        assert_eq!(0, check_equation_validity(161011, &vec![16, 10, 13]));
        assert_eq!(0, check_equation_validity(192, &vec![17, 8, 14]));
        assert_eq!(0, check_equation_validity(21037, &vec![9, 7, 18, 13]));
        assert_eq!(1, check_equation_validity(292, &vec![11, 6, 16, 20]));
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(3749, do_challenge(INPUT));
    }
}
