//! Advent of Code 2020 Day 09
//! https://adventofcode.com/2020/day/9
//!
//! Challenge part 2
//!
//! Find an "invalid" integer from an input file of integers. An integer is valid if any pair of
//! integers in the preceding 25 sum to its value. The input file has one invalid number that must
//! be identified.
//!
//! Part 2 of the challenge adds the requirement to find a contiguous set of integers of any size
//! that sum to the "invalid" integer.

use std::fs;

const INPUT_FILENAME: &str = "2020_day09_input.txt";
const INPUT_PREAMBLE_LENGTH: usize = 25;


#[derive(Debug)]
struct Xmas {
    data: Vec<u64>
}

impl Xmas {
    fn create_from_string(input_string: &str) -> Self {
        let mut data = Vec::new();

        for line in input_string.lines() {
            if line.len() == 0 {
                continue;
            }

            data.push(line.parse().unwrap());
        }

        Self { data: data }
    }
}


/// An `Iterator` that is created with a Vec of integers and iterates over the sum of each pair.
/// For example, `SumPairs(vec![5, 7, 11])` calculates the sum of 5+7, 5+11 and 7+11, giving
/// 12, 16 and 18.
struct SumPairs<'a> {
    data: &'a Vec<u64>,
    i: usize,
    j: usize,
}

impl<'a> SumPairs<'a> {
    fn new(data: &'a Vec<u64>) -> Self {
        Self { data: data, i: 0, j: 1 }
    }
}

impl Iterator for SumPairs<'_> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let max_limit = self.data.len();

        if (max_limit == 0) || ((self.i >= max_limit - 1) && (self.j >= max_limit - 1)) {
            return None;
        }

        let ret = self.data[self.i] + self.data[self.j];

        if self.j < max_limit - 1{
            self.j += 1;
        } else if self.i < max_limit - 1 {
            self.i += 1;
            self.j = self.i + 1;
        }

        Some(ret as u64)
    }
}


fn find_invalid_number(input: &Xmas, preamble_len: usize) -> u64 {

    if input.data.len() < (preamble_len + 1) {
        panic!("Insufficient input data to analyze. It must contain more integers than the
            preamble length.");
    }


    for w in 0..input.data.len() - preamble_len {
        let num_to_verify = input.data[w + preamble_len];
//         print!("Checking {:?}. ", num_to_verify);

        let window: &Vec<u64> = &(&input.data[w..w + preamble_len]).to_vec();
//         print!("Window = {:?}. ", window);

        let window_pairs: Vec<u64> = SumPairs::new(&window).collect();
//         println!("Pairs = {:?}", window_pairs);

        if !window_pairs.contains(&num_to_verify) {
            return num_to_verify;
        }
    }

    panic!("No invalid number found.");
}


fn find_contiguous_slice(input: &Xmas, target_num: u64) -> &[u64] {
    let input_len = input.data.len();

    for slice_start in 0..input_len-1 {
        for slice_end in slice_start+1..input_len {
            let slice = &input.data[slice_start..=slice_end];
//             println!("Slice {}..={} is {:?}", slice_start, slice_end, slice);

            let sum: u64 = slice.iter().sum();
            if sum > target_num {
                break;
            }
            if sum == target_num {
                return slice;
            }
        }

    }
    panic!("Cannot find contiguous integers that add up to required `target_num`");
}


fn main() {
    let input_file =
        fs::read_to_string(INPUT_FILENAME)
            .expect("Error reading input file");

    let input = Xmas::create_from_string(&input_file);
    let invalid = find_invalid_number(&input, INPUT_PREAMBLE_LENGTH);
    let result = find_contiguous_slice(&input, invalid);

    println!("The invalid number in the input is {}", invalid);

    let result_min = result.iter().min().unwrap();
    let result_max = result.iter().max().unwrap();
    print!("A sequence of numbers that sum to this value exists. ");
    print!("Its smallest number is {} and its largest {}.", result_min, result_max);
    println!("The sum of these two, and the answer to the challenge, is {}", result_min +
        result_max);

}


// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_search_for_invalid_integer() {
        let input = Xmas::create_from_string(&TEST_INPUT);
        let result = find_invalid_number(&input, 5);

        assert_eq!(result, 127);
    }


    #[test]
    fn test_find_contiguous_sum() {
        let input = Xmas::create_from_string(&TEST_INPUT);
        let invalid = find_invalid_number(&input, 5);
        let result = find_contiguous_slice(&input, invalid);

        assert!((result.first() == Some(&15)) && (result.last() == Some(&40)));
    }


    #[test]
    fn test_iterator_empty() {
        let nums = &vec![];
        let mut sap = SumPairs::new(&nums);

        assert_eq!(sap.next(), None);
        assert_eq!(sap.next(), None);
    }

    #[test]
    fn test_iterator_len1() {
        let nums = vec![13];
        let mut sap = SumPairs::new(&nums);

        assert_eq!(sap.next(), None);
        assert_eq!(sap.next(), None);
    }

    #[test]
    fn test_iterator_len2() {
        let nums = vec![13, 1];
        let mut sap = SumPairs::new(&nums);

        assert_eq!(sap.next(), Some(14));
        assert_eq!(sap.next(), None);
    }

    #[test]
    fn test_iterator_len4() {
        let nums = vec![7, 17, 41, 19];
        let mut sap = SumPairs::new(&nums);

        assert_eq!(sap.next(), Some(24));
        assert_eq!(sap.next(), Some(48));
        assert_eq!(sap.next(), Some(26));
        assert_eq!(sap.next(), Some(58));
        assert_eq!(sap.next(), Some(36));
        assert_eq!(sap.next(), Some(60));
        assert_eq!(sap.next(), None);
    }
}
