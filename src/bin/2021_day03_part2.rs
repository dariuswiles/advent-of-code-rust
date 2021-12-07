//! Advent of Code 2021 Day 03
//! https://adventofcode.com/2021/day/3
//!
//! Challenge part 2
//!
//! Filter the challenge data based on whether each bit contains the most popular value at that
//! position (for the oxygen generator rating), or least popular (for the CO2 scrubber rating).
//! Multiply the decimal equivalent of these two ratings to obtain the challenge answer.

use std::fs;

const INPUT_FILENAME: &str = "2021_day03_input.txt";

enum Rating {
    OxygenGenerator,
    CO2Scrubber,
}

#[derive(Clone, Debug)]
struct DiagnosticReport {
    data: Vec<Vec<u8>>,
}

impl DiagnosticReport {
    /// Creates a new `DiagnosticReport` from the string passed.
    fn new(input: &str) -> Self {
        let mut data = Vec::new();
        let mut bits_per_line = None;

        for line in input.lines() {
            if line == "" {
                continue;
            }

            if bits_per_line == None {
                bits_per_line = Some(line.len());
            } else {
                if bits_per_line.unwrap() != line.len() {
                    panic!("All input lines must contain the same number of bits");
                }
            }

            data.push(line.chars().map(|c| c.to_digit(2).unwrap() as u8).collect());
        }
        Self { data }
    }
}


/// Contains references to the data in a `DiagnosticReport` struct, and methods to filter these
/// down following the process required by the challenge. References are used to avoid copying
/// the lines of bits during each stage of the whittling process.
#[derive(Clone, Debug)]
struct FilteredReport<'a> {
    data: Vec<&'a Vec<u8>>,
}

impl<'a> FilteredReport<'a> {
    fn new(r: &'a DiagnosticReport) -> Self {
        let mut refs: Vec<&Vec<u8>> = Vec::new();
        for d in &r.data {
            refs.push(&d);
        }

        FilteredReport { data: refs }
    }


    /// Returns the count of '1' characters in the given index position of every string in the
    /// data set.
    fn count_ones_in_position(&self, position: usize) -> u32 {
        self.data.iter().fold(0, |acc, x| acc + x[position] as u32)
    }


    /// Returns the most commonly occurring bit, either 0 or 1, at the given index position of
    /// every string in the data set. If there are equal occurrences, return 1.
    fn most_common_bit_in_position(&self, position: usize) -> u8 {
        let data_length = self.data.len();

        if self.count_ones_in_position(position) * 2 >= data_length as u32 {
            return 1;
        } else {
            return 0;
        }
    }


    /// Returns the least commonly occurring bit, either 0 or 1, at the given index position of
    /// every string in the data set. If there are equal occurrences, return 0.
    fn least_common_bit_in_position(&self, position: usize) -> u8 {
        let data_length = self.data.len();

        if data_length == 1 {
            return self.data[0][position];
        }

        if self.count_ones_in_position(position) * 2 >= data_length as u32 {
            return 0;
        } else {
            return 1;
        }
    }


    /// Modifies `self` to only contain data lines having the most commonly occurring bit in
    /// `position`. For example, if `position` is 3, and 0 occurs more often than 1 in that
    /// position, the modified`DiagnosticReport` only contains data lines with a 0 in position 3.
    fn filter_most_common(&mut self, position: usize) {
        let required_bit = self.most_common_bit_in_position(position);
        let mut new_data: Vec<&Vec<u8>> = Vec::new();

        for d in &self.data {
            if *d.iter().nth(position).unwrap() == required_bit {
                new_data.push(d);
            }
        }

        self.data = new_data;
    }

    /// Modifies `self` to only contain data lines having the least commonly occurring bit in
    /// `position`. For example, if `position` is 3, and 0 occurs less often than 1 in that
    /// position, the modified`DiagnosticReport` only contains data lines with a 0 in position 3.
    fn filter_least_common(&mut self, position: usize) {
        let required_bit = self.least_common_bit_in_position(position);
        let mut new_data: Vec<&Vec<u8>> = Vec::new();

        for d in &self.data {
            if *d.iter().nth(position).unwrap() == required_bit {
                new_data.push(d);
            }
        }

        self.data = new_data;
    }
}


/// Examine each bit position in turn, from left to right, for all data. If calculating the oxygen
/// generator rating, as specified in `r`, determine the most common value (0 or 1) for each bit,
/// and keep only data with this value in this bit position. If calculating the CO2 scrubber
/// rating, use the same process except keep data with the least common value. After this process
/// is performed for all bits, there should only be one value remaining, which is returned as a
/// `u32`.
///
/// # Panics
///
/// Panics if the result of filtering all bits is not exactly one line of data.
fn calculate_rating(original_data: &DiagnosticReport, r: &Rating) -> u32 {
    let mut current_data = FilteredReport::new(original_data);

    for b in 0..current_data.data[0].len() {
        match r {
            Rating::OxygenGenerator => { current_data.filter_most_common(b); }
            Rating::CO2Scrubber => { current_data.filter_least_common(b); }
        }
    }

    assert!(current_data.data.len() == 1);

    let s = current_data.data[0].iter().map(|i| i.to_string()).collect::<String>();
    u32::from_str_radix(&s, 2).unwrap()
}


fn main() {
    let input_file =
        fs::read_to_string(INPUT_FILENAME)
            .expect("Error reading input file");

    let diag_report = DiagnosticReport::new(&input_file);

    let answer = calculate_rating(&diag_report, &Rating::OxygenGenerator) *
        calculate_rating(&diag_report, &Rating::CO2Scrubber);

    println!("The submarine's life support rating is {}", answer);
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
        let diag_report = DiagnosticReport::new(&TEST_INPUT);

        assert_eq!(diag_report.data[0], vec![0, 0, 1, 0, 0]);
        assert_eq!(diag_report.data[1], vec![1, 1, 1, 1, 0]);
        assert_eq!(diag_report.data[2], vec![1, 0, 1, 1, 0]);
        assert_eq!(diag_report.data[3], vec![1, 0, 1, 1, 1]);
        assert_eq!(diag_report.data[4], vec![1, 0, 1, 0, 1]);
        assert_eq!(diag_report.data[5], vec![0, 1, 1, 1, 1]);
        assert_eq!(diag_report.data[6], vec![0, 0, 1, 1, 1]);
        assert_eq!(diag_report.data[7], vec![1, 1, 1, 0, 0]);
        assert_eq!(diag_report.data[8], vec![1, 0, 0, 0, 0]);
        assert_eq!(diag_report.data[9], vec![1, 1, 0, 0, 1]);
        assert_eq!(diag_report.data[10], vec![0, 0, 0, 1, 0]);
        assert_eq!(diag_report.data[11], vec![0, 1, 0, 1, 0]);
    }

    #[test]
    fn test_count_ones_in_position() {
        let diag_report = DiagnosticReport::new(&TEST_INPUT);
        let report = FilteredReport::new(&diag_report);

        assert_eq!(report.count_ones_in_position(0), 7);
        assert_eq!(report.count_ones_in_position(1), 5);
        assert_eq!(report.count_ones_in_position(2), 8);
        assert_eq!(report.count_ones_in_position(3), 7);
        assert_eq!(report.count_ones_in_position(4), 5);
    }

    #[test]
    fn test_most_common_bit_in_position() {
        let diag_report = DiagnosticReport::new(&TEST_INPUT);
        let report = FilteredReport::new(&diag_report);

        assert_eq!(report.most_common_bit_in_position(0), 1);
        assert_eq!(report.most_common_bit_in_position(1), 0);
        assert_eq!(report.most_common_bit_in_position(2), 1);
        assert_eq!(report.most_common_bit_in_position(3), 1);
        assert_eq!(report.most_common_bit_in_position(4), 0);
    }

    #[test]
    fn test_least_common_bit_in_position() {
        let diag_report = DiagnosticReport::new(&TEST_INPUT);
        let report = FilteredReport::new(&diag_report);

        assert_eq!(report.least_common_bit_in_position(0), 0);
        assert_eq!(report.least_common_bit_in_position(1), 1);
        assert_eq!(report.least_common_bit_in_position(2), 0);
        assert_eq!(report.least_common_bit_in_position(3), 0);
        assert_eq!(report.least_common_bit_in_position(4), 1);
    }

    #[test]
    fn test_filter_most_common() {
        let diag_report = DiagnosticReport::new(&TEST_INPUT);
        let mut filtered_report = FilteredReport::new(&diag_report);
        filtered_report.filter_most_common(0);

        assert_eq!(filtered_report.data.len(), 7);
        assert_eq!(filtered_report.data[0], &vec![1, 1, 1, 1, 0]);
        assert_eq!(filtered_report.data[1], &vec![1, 0, 1, 1, 0]);
        assert_eq!(filtered_report.data[2], &vec![1, 0, 1, 1, 1]);
        assert_eq!(filtered_report.data[3], &vec![1, 0, 1, 0, 1]);
        assert_eq!(filtered_report.data[4], &vec![1, 1, 1, 0, 0]);
        assert_eq!(filtered_report.data[5], &vec![1, 0, 0, 0, 0]);
        assert_eq!(filtered_report.data[6], &vec![1, 1, 0, 0, 1]);

        filtered_report.filter_most_common(1);
        assert_eq!(filtered_report.data.len(), 4);
        assert_eq!(filtered_report.data[0], &vec![1, 0, 1, 1, 0]);
        assert_eq!(filtered_report.data[1], &vec![1, 0, 1, 1, 1]);
        assert_eq!(filtered_report.data[2], &vec![1, 0, 1, 0, 1]);
        assert_eq!(filtered_report.data[3], &vec![1, 0, 0, 0, 0]);
    }

    #[test]
    fn test_oxygen_generator_rating() {
        let diag_report = DiagnosticReport::new(&TEST_INPUT);
        assert_eq!(calculate_rating(&diag_report, &Rating::OxygenGenerator), 23);
    }

    #[test]
    fn test_co0_scrubber_rating() {
        let diag_report = DiagnosticReport::new(&TEST_INPUT);
        assert_eq!(calculate_rating(&diag_report, &Rating::CO2Scrubber), 10);
    }

        #[test]
    fn challenge_answer() {
        let diag_report = DiagnosticReport::new(&TEST_INPUT);
        assert_eq!(
            calculate_rating(&diag_report, &Rating::OxygenGenerator) *
            calculate_rating(&diag_report, &Rating::CO2Scrubber), 230
        );
    }

    #[test]
    #[should_panic]
    fn different_line_lengths() {
        DiagnosticReport::new(&TEST_INPUT_BAD_LENGTH);
    }
}
