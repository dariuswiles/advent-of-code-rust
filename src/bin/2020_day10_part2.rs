//! Advent of Code 2020 Day 10
//! https://adventofcode.com/2020/day/10
//!
//! Challenge part 2
//!
//! Determine the number of combinations of integers that meet the challenge criteria.

use std::fs;

const INPUT_FILENAME: &str = "2020_day10_input.txt";
const DIVIDE_CONQUER_LENGTH: usize = 10;
const MAX_ALLOWED_DIFF: u32 = 3;

/// Convert a string containing one unsigned integer per line into a vector of integers.
fn parse_str_to_nums(input: &str) -> Vec<u32> {
    let mut result = Vec::new();

    for line in input.lines() {
        if line.len() == 0 {
            continue;
        }

        result.push(line.parse::<u32>().unwrap());
    }
    result
}

/// The challenge requires integers of 0 and 3 greater than the highest integer in the input file
/// to be added to the vector of integers.
fn add_outlet_and_device(v: &mut Vec<u32>) {
    v.insert(0, 0);
    let last_val = *v.last().unwrap();

    v.push(last_val + 3);
}

/// Given a vector of integers, calculates the number of combinations of integers that meet the
/// challenge criteria, namely that there must be a chain of integers from 0 to the largest integer
/// where the difference between each pair of integers in the chain must be no greater than 3. This
/// is calculated using recursion.
fn calculate_combinations_inner(ints: &[u32]) -> u64 {
    // If only one element remains in the `ints` slice, we have successfully found a combination of
    // integers from 0 to this final value, so return 1.
    if ints.len() == 1 {
        return 1;
    }

    let mut total = 0;
    let new_ints = &ints[1..];

    for (idx, int) in new_ints.iter().enumerate() {
        if *int > ints[0] + MAX_ALLOWED_DIFF {
            break;
        }

        total += calculate_combinations_inner(&new_ints[idx..]);
    }
    total
}

/// Given a vector of integers, calculates the number of combinations of integers that meet the
/// challenge criteria, namely that there must be a chain of integers from 0 to the largest integer
/// where the difference between each pair of integers in the chain must be no greater than 3.
//
// To improve performance, groups of `ints` are calculated individually and the results combined.
// Groups are divided only at integers that are the maximum difference from the previous integer,
// meaning that all solutions *must* incorporate them.
fn calculate_combinations(ints: &[u32]) -> u64 {
    let mut total = 1u64;

    // Calculate the differences between pairs of elements in `ints`. For example, [0, 3, 4, 7]
    // results in vec![0, 3, 1, 3].
    let ints_diffs: Vec<u32> = ints
        .iter()
        .scan(0, |previous, current| {
            let diff = *current - *previous;
            *previous = *current;
            Some(diff)
        })
        .collect();

    // println!("{:#?}", &ints_diffs);

    let mut work_idx = 0; // Index of the last int included in the last calculation.
    while work_idx < ints.len() - 1 {
        let mut next_group_end = 0;
        for i in work_idx + DIVIDE_CONQUER_LENGTH..ints.len() - 1 {
            if ints_diffs[i] == MAX_ALLOWED_DIFF {
                next_group_end = i;
                break;
            }
        }

        // The end of `ints` was reached in the above loop.
        if next_group_end == 0 {
            next_group_end = ints.len() - 1;
        }

        if work_idx == next_group_end {
            // println!("Breaking because work_idx and next_group_end are the same: {}", work_idx);
            break;
        }

        // println!("Calculating combinations over range {}..={}", work_idx, next_group_end);
        // println!("\ttotal before call is {}", total);
        total *= calculate_combinations_inner(&ints[work_idx..=next_group_end]) as u64;
        // println!("\ttotal after call is {}", total);
        work_idx = next_group_end;
    }
    total
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let mut input = parse_str_to_nums(&input_file);

    input.sort_unstable();
    add_outlet_and_device(&mut input);

    let result = calculate_combinations(&input);

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

        let result = calculate_combinations(&input);

        assert_eq!(result, 8);
    }

    #[test]
    fn test_1() {
        let mut input = parse_str_to_nums(&TEST_INPUT_1);

        input.sort_unstable();
        add_outlet_and_device(&mut input);
        println!("{:#?}", &input);

        let result = calculate_combinations(&input);

        assert_eq!(result, 19208);
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
}
