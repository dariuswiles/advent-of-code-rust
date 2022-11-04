//! Advent of Code 2020 Day 10
//! https://adventofcode.com/2020/day/10
//!
//! Challenge part 1
//!
//! Determine the differences between pairs of numbers in an ordered set.

use std::collections::HashMap;
use std::fs;

const INPUT_FILENAME: &str = "2020_day10_input.txt";

/// Convert a string containing one unsigned integer per line into a vector of integers.
fn parse_str_to_nums(input: &str) -> Vec<i32> {
    let mut result = Vec::new();

    for line in input.lines() {
        if line.len() == 0 {
            continue;
        }

        result.push(line.parse::<i32>().unwrap());
    }
    result
}

/// The challenge requires integers of 0 and 3 greater than the highest integer in the input file
/// to be added to the vector of integers.
fn add_outlet_and_device(v: &mut Vec<i32>) {
    v.insert(0, 0);
    let last_val = *v.last().unwrap();

    v.push(last_val + 3);
}

/// Returns a vector representing the difference between every pair of numbers in `v`. For example,
/// if `v` is 2, 4 and 7, a new vector containing 2 and 3 is returned.
fn generate_pair_deltas(v: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();

    let mut num_vec = v.clone();
    let mut previous = num_vec.remove(0);

    for i in &num_vec {
        result.push(*i - previous);
        previous = *i;
    }

    result
}

/// Returns a HashMap whose keys are the integers in `v` and whose associated values are totals of
/// the number of occurrences. For example, if `v` contains ten instances of the integer '3', the
/// HashMap returned will contain a key of 3 with value 10.
fn count_occurrences(v: &Vec<i32>) -> HashMap<&i32, u16> {
    let mut counts = HashMap::new();

    for i in v {
        let count = counts.entry(i).or_insert(0);
        *count += 1;
    }

    counts
}

/// Performs the steps specified in the challenge, including the final multiplication.
fn do_challenge(input_str: &str) -> i32 {
    let mut input = parse_str_to_nums(&input_str);
    input.sort_unstable();
    add_outlet_and_device(&mut input);
    let deltas = generate_pair_deltas(&input);
    let totals = count_occurrences(&deltas);

    (totals[&1] * totals[&3]).into()
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let result = do_challenge(&input_file);
    println!("The answer to the challenge is {}", result);
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_0: &str = "\
16
10
15
5
1
11
7
19
6
12
4";

    const TEST_INPUT_1: &str = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_0() {
        let mut input = parse_str_to_nums(&TEST_INPUT_0);

        input.sort_unstable();
        add_outlet_and_device(&mut input);
        println!("{:#?}", &input);

        let deltas = generate_pair_deltas(&input);
        println!("{:#?}", &deltas);

        let totals = count_occurrences(&deltas);

        assert_eq!(totals[&1], 7);
        assert_eq!(totals[&3], 5);
    }

    #[test]
    fn test_1() {
        let mut input = parse_str_to_nums(&TEST_INPUT_1);

        input.sort_unstable();
        add_outlet_and_device(&mut input);
        println!("{:#?}", &input);

        let deltas = generate_pair_deltas(&input);
        println!("{:#?}", &deltas);

        let totals = count_occurrences(&deltas);

        assert_eq!(totals[&1], 22);
        assert_eq!(totals[&3], 10);
    }

    #[test]
    fn test_parse_str_to_nums() {
        let mut input = parse_str_to_nums(
            "\
13

7
79
",
        );

        input.sort_unstable();
        assert_eq!(input[0], 7);
        assert_eq!(input[1], 13);
        assert_eq!(input[2], 79);
    }

    #[test]
    fn test_add_outlet_and_device() {
        let mut input = parse_str_to_nums(
            "\
17
55",
        );

        input.sort_unstable();
        add_outlet_and_device(&mut input);

        assert_eq!(input[0], 0);
        assert_eq!(input[1], 17);
        assert_eq!(input[2], 55);
        assert_eq!(input[3], 58);
    }

    #[test]
    fn test_count_occurrences() {
        let input = vec![3, 0, 1, 3, 3, 0, 3, 1, 3];
        let result = count_occurrences(&input);

        assert_eq!(result[&0], 2);
        assert_eq!(result[&1], 2);
        assert_eq!(result[&3], 5);
    }
}
