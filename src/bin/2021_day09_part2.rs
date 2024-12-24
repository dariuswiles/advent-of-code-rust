//! Advent of Code 2021 Day 09
//! https://adventofcode.com/2021/day/9
//!
//! Challenge part 2
//!
//! Determine the size of each area of cells within the input data that are separated by the cell
//! value '9', and calculate the product of the three largest to obtain the answer to the
//! challenge.

use std::fs;

const INPUT_FILENAME: &str = "2021_day09_input.txt";

type CellData = u8;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

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
            if line.is_empty() {
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
    fn is_lowest(&self, row: usize, col: usize) -> bool {
        let value = self.cells[row][col];

        if col > 0 && value >= self.cells[row][col - 1] {
            return false;
        }

        if col < self.cells[row].len() - 1 && value >= self.cells[row][col + 1] {
            return false;
        }

        if row > 0 && value >= self.cells[row - 1][col] {
            return false;
        }

        if row < self.cells.len() - 1 && value >= self.cells[row + 1][col] {
            return false;
        }

        true
    }

    /// Returns a Vec containing the value of each low point within this `HeightMap`.
    fn find_low_points(&self) -> Vec<(usize, usize)> {
        let mut low_points = Vec::new();

        for row in 0..self.cells.len() {
            for col in 0..self.cells[row].len() {
                if self.is_lowest(row, col) {
                    low_points.push((row, col));
                }
            }
        }

        low_points
    }

    /// An internal function that should not be called directly. It returns the size of the basin
    /// that contains point `row`, `col`.
    //
    // This is determined by recursively traversing cells surrounding the cell at `row`, `col. For
    // each cell, this function is called recursively for each adjacent cell, except if the
    // `ignore_direction` parameter says to ignore it. This is used to stop loops formed by two
    // adjacent cells forever calling each other. `visited` is a 2D map that is the same size as
    // the `HeightMap` that is used to indicate that a cell is already being considered and should
    // not be considered again.
    fn basin_size_recurse(
        &self,
        row: usize,
        col: usize,
        ignore_direction: Option<Direction>,
        visited: &mut Vec<Vec<bool>>,
    ) -> u32 {
        // println!("basin_size_recurse called with row = {}, col = {}; and ignore_direction = {:?}.",
        //     row, col, ignore_direction
        // );
        if visited[row][col] | (self.cells[row][col] == 9) {
            // println!("\tReturning 0 because this cell has been visited or its value is 9");
            return 0;
        }

        visited[row][col] = true;
        let mut total = 1;

        if col > 0 && ignore_direction != Some(Direction::Left) {
            total += self.basin_size_recurse(row, col - 1, Some(Direction::Right), visited);
        }

        if col < self.cells[row].len() - 1 && ignore_direction != Some(Direction::Right) {
            total += self.basin_size_recurse(row, col + 1, Some(Direction::Left), visited);
        }

        if row > 0 && ignore_direction != Some(Direction::Up) {
            total += self.basin_size_recurse(row - 1, col, Some(Direction::Down), visited);
        }

        if row < self.cells.len() - 1 && ignore_direction != Some(Direction::Down) {
            total += self.basin_size_recurse(row + 1, col, Some(Direction::Up), visited);
        }

        total
    }

    /// Returns the size of the basin that contains point `row`, `col`.
    fn basin_size(&self, row: usize, col: usize) -> u32 {
        let mut visited = Vec::new();

        for _ in 0..self.cells.len() {
            let mut row = Vec::new();
            row.resize(self.cells[0].len(), false);
            visited.push(row);
        }

        self.basin_size_recurse(row, col, None, &mut visited)
    }

    /// Returns a Vec containing the number of cells of each basin in this `HeightMap`, sorted from
    /// largest first.
    fn all_basin_sizes(&self) -> Vec<u32> {
        let low_points = self.find_low_points();

        let mut basin_sizes = Vec::new();
        for (row, col) in low_points {
            basin_sizes.push(self.basin_size(row, col));
        }

        basin_sizes.sort_unstable();
        basin_sizes.reverse();
        basin_sizes
    }
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let hm = HeightMap::new(&input_file);
    let basin_sizes = hm.all_basin_sizes();
    let biggest_basins = &basin_sizes[..3];

    println!(
        "The answer to the challenge is {}",
        biggest_basins.iter().product::<u32>()
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
        let hm = HeightMap::new(TEST_INPUT);

        assert_eq!(hm.cells.len(), 5);
        assert_eq!(hm.cells[0].len(), 10);
    }

    #[test]
    fn test_find_low_points() {
        let hm = HeightMap::new(TEST_INPUT);
        let mut low_points = hm.find_low_points();

        assert_eq!(low_points.len(), 4);

        low_points.sort_unstable();
        assert_eq!(low_points, vec![(0, 1), (0, 9), (2, 2), (4, 6),]);
    }

    #[test]
    fn test_basin_size() {
        let hm = HeightMap::new(TEST_INPUT);

        assert_eq!(hm.basin_size(0, 1), 3);
        assert_eq!(hm.basin_size(0, 9), 9);
        assert_eq!(hm.basin_size(2, 2), 14);
        assert_eq!(hm.basin_size(4, 6), 9);
    }

    #[test]
    fn challenge_answer() {
        let hm = HeightMap::new(TEST_INPUT);
        let basin_sizes = hm.all_basin_sizes();
        let biggest_basins = &basin_sizes[..3];

        assert_eq!(biggest_basins.iter().product::<u32>(), 1134);
    }

    #[test]
    #[should_panic]
    fn different_line_lengths() {
        let _ = HeightMap::new(TEST_INPUT_BAD_LENGTH);
    }
}
