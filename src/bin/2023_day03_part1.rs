//! Advent of Code 2023 Day 03
//! https://adventofcode.com/2023/day/3
//!
//! Challenge part 1
//!
//! Interprets the input as a 2D schematic containing multi-digit part numbers and symbols. Part
//! numbers adjacent to at least one symbol are summed to calculate the challenge answer.

use std::fs;

const INPUT_FILENAME: &str = "2023_day03_input.txt";
const CELL_EMPTY: char = '.';

#[derive(Debug, PartialEq)]
enum Cell {
    Empty,
    Digit(u32),
    Symbol,
}

/// Represents a schematic as defined in the challenge. The first line of `cells` is ordered such
/// that row 0 is the top of the schematic.
#[derive(Debug, PartialEq)]
struct Schematic {
    cells: Vec<Vec<Cell>>,
    width: usize,
}

impl Schematic {
    /// Returns a `Schematic` object representing the `input` provided.
    ///
    /// # Panics
    ///
    /// Panics if non-empty lines do not all contain exactly the same number of characters.
    fn from_string(input: &str) -> Self {
        let mut cells = Vec::new();
        let mut width = None;

        for line in input.lines() {
            if line == "" {
                continue;
            }

            let mut row = Vec::new();
            let chars: Vec<char> = line.chars().collect();

            if let Some(line_length) = width {
                if chars.len() != line_length {
                    panic!("All image data lines must be the same length, but are not.");
                }
            } else {
                width = Some(chars.len());
            }

            for c in &chars {
                if c == &CELL_EMPTY {
                    row.push(Cell::Empty);
                } else if c.is_digit(10) {
                    row.push(Cell::Digit(c.to_digit(10).unwrap() as u32));
                } else {
                    row.push(Cell::Symbol);
                };
            }

            cells.push(row);
        }

        Self {
            cells,
            width: width.unwrap(),
        }
    }

    /// Create a mask indicating which cells are symbols or adjacent to symbols (including
    /// diagonals). The mask takes the form of a grid of booleans of the same size as the cells in
    /// `self`. A value of `true` indicates that the corresponding cell is a symbol or adjacent to
    /// a symbol.
    fn create_symbol_adjacency_mask(&self) -> Vec<Vec<bool>> {
        let mut mask = Vec::new();
        let mask_height = self.cells.len();

        // Create empty mask
        for _ in 0..mask_height {
            let mut row = Vec::new();
            row.resize(self.width, false);
            mask.push(row);
        }

        for row in 0..mask_height {
            for column in 0..self.width {
                if Cell::Symbol == self.cells[row][column] {
                    let mut min_row = 0;
                    if row > 0 {
                        min_row = row - 1;
                    }

                    let mut max_row = mask_height - 1;
                    if row < max_row {
                        max_row = row + 1;
                    }

                    let mut min_column = 0;
                    if column > 0 {
                        min_column = column - 1;
                    }

                    let mut max_column = self.width - 1;
                    if column < max_column {
                        max_column = column + 1;
                    }

                    // Set the adjacency mask for the cell containing the symbol and all the 8
                    // adjacent cells, providing they are within the bounds of the cell grid.
                    for r in min_row..=max_row {
                        for c in min_column..=max_column {
                            mask[r][c] = true;
                        }
                    }
                }
            }
        }

        mask
    }

    /// Convert contiguous digits to numbers and sum the numbers that are adjacent to one or more
    /// symbols. The adjacency check is performed using the mask passed in the `m` argument. See
    /// the `create_symbol_adjacency_mask()` method for more details. Returns the sum of adjacent
    /// numbers.
    fn sum_adjacent_numbers(&self, m: &Vec<Vec<bool>>) -> u32 {
        let mut total = 0;

        for row in 0..self.cells.len() {
            let mut n = 0;
            let mut adjacent = false;
            for column in 0..self.width {
                if let Cell::Digit(d) = self.cells[row][column] {
                    n = n * 10 + d;
                    adjacent |= m[row][column];
                } else {
                    if n > 0 {
                        if adjacent {
                            total += n;
                        }

                        n = 0;
                        adjacent = false;
                    }
                }
            }

            if adjacent {
                total += n;
            }
        }

        total
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    println!(
        "The sum of all part numbers adjacent to a symbol is {}",
        do_challenge(&input)
    );
}

/// Performs all steps required to determine the challenge answer, which is then returned.
fn do_challenge(input: &str) -> u32 {
    let s = Schematic::from_string(input);
    let m = s.create_symbol_adjacency_mask();
    s.sum_adjacent_numbers(&m)
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn schematic_from_string() {
        let s = Schematic::from_string(&TEST_INPUT);

        assert_eq!(Cell::Digit(4), s.cells[0][0]);
        assert_eq!(Cell::Digit(6), s.cells[0][1]);
        assert_eq!(Cell::Digit(7), s.cells[0][2]);
        assert_eq!(Cell::Empty, s.cells[0][3]);
        assert_eq!(Cell::Empty, s.cells[0][4]);
        assert_eq!(Cell::Digit(1), s.cells[0][5]);
        assert_eq!(Cell::Digit(1), s.cells[0][6]);
        assert_eq!(Cell::Digit(4), s.cells[0][7]);
        assert_eq!(Cell::Empty, s.cells[0][8]);
        assert_eq!(Cell::Empty, s.cells[0][9]);

        assert_eq!(Cell::Empty, s.cells[1][2]);
        assert_eq!(Cell::Symbol, s.cells[1][3]);

        assert_eq!(Cell::Symbol, s.cells[5][5]);
        assert_eq!(Cell::Empty, s.cells[5][6]);
        assert_eq!(Cell::Digit(5), s.cells[5][7]);
        assert_eq!(Cell::Digit(8), s.cells[5][8]);
        assert_eq!(Cell::Empty, s.cells[5][9]);

        assert_eq!(Cell::Digit(8), s.cells[9][7]);
        assert_eq!(Cell::Empty, s.cells[9][9]);
    }

    #[test]
    fn test_create_symbol_adjacency_mask() {
        let s = Schematic::from_string(&TEST_INPUT);
        let m = s.create_symbol_adjacency_mask();

        assert_eq!(
            vec![false, false, true, true, true, false, false, false, false, false],
            m[0]
        );

        assert_eq!(
            vec![false, false, true, true, true, true, true, true, false, false],
            m[3]
        );

        assert_eq!(
            vec![false, false, true, true, true, true, true, false, false, false],
            m[9]
        );
    }

    #[test]
    fn test_sum_adjacent_numbers() {
        let s = Schematic::from_string(&TEST_INPUT);
        let m = s.create_symbol_adjacency_mask();

        assert_eq!(4361, s.sum_adjacent_numbers(&m));
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(4361, do_challenge(&TEST_INPUT));
    }
}
