//! Advent of Code 2024 Day 04
//! https://adventofcode.com/2024/day/4
//!
//! Challenge part 2
//!
//! Counts the number of times two instances of the string "MAS" are on diagonals such that they
//! cross each other on the shared letter "A".

use std::fs;

const INPUT_FILENAME: &str = "2024_day04_input.txt";

#[derive(Debug, PartialEq)]
struct WordSearch {
    cell: Vec<Vec<char>>,
    size: usize,
}

impl WordSearch {
    /// Creates a new `WordSearch` from the input string.
    ///
    /// # Panics
    ///
    /// Panics if the input is invalid. Rows must be the same length and must be the same length
    /// as the number of rows.
    fn new(input: &str) -> Self {
        let mut cell = Vec::new();
        let mut size = 0;

        for line in input.lines() {
            if line.is_empty() {
                continue;
            }

            if size == 0 {
                size = line.len();
            } else {
                assert_eq!(size, line.len(), "All input lines must be the same length");
            }

            cell.push(line.chars().collect());
        }

        if size != cell.len() {
            panic!("The input must have the same number of rows as columns");
        }

        Self { cell, size }
    }

    /// Returns the number of times the X-MAS pattern appears in this `WordSearch`.
    fn count_xmas(&self) -> u32 {
        let mut count = 0;

        for row in 1..self.size - 1 {
            for column in 1..self.size - 1 {
                if self.cell[row][column] == 'A' {
                    let top_left = self.cell[row - 1][column - 1];
                    let top_right = self.cell[row - 1][column + 1];
                    let bottom_left = self.cell[row + 1][column - 1];
                    let bottom_right = self.cell[row + 1][column + 1];

                    if ((top_left == 'M' && bottom_right == 'S')
                        || (top_left == 'S' && bottom_right == 'M'))
                        && ((top_right == 'M' && bottom_left == 'S')
                            || (top_right == 'S' && bottom_left == 'M'))
                    {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    println!(
        "The X-MAS pattern appears in the input wordsearch {} times",
        do_challenge(&input)
    );
}

/// Creates a new `WordSearch` from the input data, and returns the number of times the X-MAS
/// pattern appears in it.
fn do_challenge(input: &str) -> u32 {
    let ws = WordSearch::new(input);
    ws.count_xmas()
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_wordsearch_new() {
        let ws = WordSearch::new(TEST_INPUT);

        assert_eq!(10, ws.size);
        assert_eq!(
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            ws.cell[2]
        );
    }

    #[test]
    fn test_count_xmas() {
        let ws = WordSearch::new(TEST_INPUT);

        assert_eq!(9, ws.count_xmas());
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(do_challenge(TEST_INPUT), 9);
    }
}
