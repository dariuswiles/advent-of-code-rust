//! Advent of Code 2021 Day 09
//! https://adventofcode.com/2021/day/9
//!
//! Challenge part 1
//!
//! Find the values in a 2D array of data that are lower than adjacent data, and sum them to
//! generate an overall `risk` score.

use std::fs;

const INPUT_FILENAME: &str = "2021_day09_input.txt";

type CellData = u8;

#[derive(Debug, PartialEq)]
struct HeightMap {
    cells: Vec<Vec<CellData>>,
}

impl HeightMap {
    /// Creates a new `HeightMap` from an input string.
    fn new(input: &str) -> Self {
        let mut cells = Vec::new();
        let mut line_length = None;

        for line in input.lines() {
            if line == "" {
                continue;
            }

            if let Some(prior_length) = line_length {
                if prior_length != line.len() {
                    panic!("All input lines must contain the same number of digits");
                }
            } else {
                line_length = Some(line.len());
            }

            cells.push(
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as CellData)
                    .collect(),
            );
        }
        Self { cells }
    }

    /// Determines if the cell at `row` and `col` is lower in value than the cells above, below,
    /// left and right. If it is, its value is returned in an Option, otherwise `None` is
    /// returned.
    fn is_lowest(&self, row: usize, col: usize) -> Option<CellData> {
        let value = self.cells[row][col];

        if col > 0 {
            if value >= self.cells[row][col - 1] {
                return None;
            }
        }

        if col < self.cells[row].len() - 1 {
            if value >= self.cells[row][col + 1] {
                return None;
            }
        }

        if row > 0 {
            if value >= self.cells[row - 1][col] {
                return None;
            }
        }

        if row < self.cells.len() - 1 {
            if value >= self.cells[row + 1][col] {
                return None;
            }
        }

        Some(value)
    }

    /// Returns a Vec containing the value of each low point within this `HeightMap`.
    fn find_low_points(&self) -> Vec<CellData> {
        let mut low_points = Vec::new();

        for row in 0..self.cells.len() {
            for col in 0..self.cells[row].len() {
                if let Some(value) = self.is_lowest(row, col) {
                    low_points.push(value);
                }
            }
        }

        low_points
    }
}

/// Returns the total risk by summing the value of low point, plus one, as per the challenge
/// instructions.
fn calculate_risk(low_points: &Vec<CellData>) -> u32 {
    low_points.iter().fold(0, |acc, &i| acc + (i as u32) + 1)
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let hm = HeightMap::new(&input_file);
    println!(
        "The total risk is {}",
        calculate_risk(&hm.find_low_points())
    );
}

// Test using data from the examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

    const TEST_INPUT_BAD_LENGTH: &str = "\
2199943210
39
9856789892
8767896789
9899965678";

    #[test]
    fn parse_test_input() {
        let hm = HeightMap::new(&TEST_INPUT);

        assert_eq!(hm.cells.len(), 5);
        assert_eq!(hm.cells[0].len(), 10);
    }

    #[test]
    fn test_find_low_points() {
        let hm = HeightMap::new(&TEST_INPUT);
        let mut low_points = hm.find_low_points();

        assert_eq!(low_points.len(), 4);

        low_points.sort_unstable();
        assert_eq!(low_points, vec![0, 1, 5, 5]);
    }

    #[test]
    fn test_calculate_risk() {
        let hm = HeightMap::new(&TEST_INPUT);
        assert_eq!(calculate_risk(&hm.find_low_points()), 15);
    }

    #[test]
    #[should_panic]
    fn different_line_lengths() {
        let _ = HeightMap::new(&TEST_INPUT_BAD_LENGTH);
    }
}
