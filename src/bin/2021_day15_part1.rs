//! Advent of Code 2021 Day 15
//! https://adventofcode.com/2021/day/15
//!
//! Challenge part 1
//!
//! Finds the safest path through a grid of cells where every cell has an associated risk.

use std::fmt;
use std::fs;

const INPUT_FILENAME: &str = "2021_day15_input.txt";

type Risk = u32;

#[derive(Debug, PartialEq)]
struct RiskGrid {
    cell: Vec<Vec<Risk>>,
}

impl RiskGrid {
    /// Creates a grid of risks from an input string. The outer Vec is the row, the inner
    /// the column, so self.cell[3][9] is row 3, column 9.
    ///
    /// # Panics
    ///
    /// Panics if the input contains anything other than digits, or if lines do not all have the
    /// same number of digits.
    fn new(input: &str) -> Self {
        let mut cell = Vec::new();
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

            cell.push(
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as Risk)
                    .collect(),
            );
        }
        Self { cell }
    }

    /// Returns the number of rows in this `Grid`.
    fn height(&self) -> usize {
        self.cell.len()
    }
}

impl fmt::Display for RiskGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cell {
            let mut s: String = row.iter().map(|d| d.to_string()).collect();
            s.push('\n');
            f.write_str(&s).unwrap();
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}

/// A 2 dimensional grid of cells where each cell contains the best path to get to it (i.e., the
/// path resulting in the lowest risk), and its associated risk. The latter includes the risk of
/// entering the last cell in the path.
#[derive(Debug)]
struct BestRiskGrid {
    cell: Vec<Vec<Risk>>,
}

impl BestRiskGrid {
    fn new(size: usize) -> Self {
        let mut cell = Vec::new();

        for _ in 0..size {
            let mut row = Vec::new();

            for _ in 0..size {
                row.push(Risk::MAX);
            }

            cell.push(row);
        }

        Self { cell }
    }
}

/// Recursively investigates all four directions from the given cell, defined by the `row` and
/// `column` passed, looking for better paths to each cell in the grid. "Better" means resulting
/// in a lower total risk from the top-left starting cell to the given cell. If this function is
/// called with a higher risk than one already found for this cell, it  immediately returns as
/// there's no point following the path further.
fn walk_path(
    risk_grid: &RiskGrid,
    best_risk: &mut BestRiskGrid,
    row: usize,
    column: usize,
    current_risk: Risk,
) {
    let size = risk_grid.height();

    // Immediately return if `current_risk` is higher than a previous path has found.
    if current_risk >= best_risk.cell[row][column] {
        return;
    }

    // If this is the best risk found so far, record it.
    best_risk.cell[row][column] = current_risk;

    for dir in vec![
        Direction::Down,
        Direction::Left,
        Direction::Right,
        Direction::Up,
    ]
    .iter_mut()
    {
        let new_row;
        let new_column;

        match dir {
            Direction::Down => {
                if row + 1 >= size {
                    continue;
                } else {
                    new_row = row + 1;
                    new_column = column;
                }
            }
            Direction::Left => {
                if column <= 0 {
                    continue;
                } else {
                    new_row = row;
                    new_column = column - 1;
                }
            }
            Direction::Right => {
                if column + 1 >= size {
                    continue;
                } else {
                    new_row = row;
                    new_column = column + 1;
                }
            }
            Direction::Up => {
                if row <= 0 {
                    continue;
                } else {
                    new_row = row - 1;
                    new_column = column;
                }
            }
        }
        let new_risk = current_risk + risk_grid.cell[new_row][new_column];

        walk_path(risk_grid, best_risk, new_row, new_column, new_risk);
    }
}

/// Returns the total risk of the most efficient path through the grid of risks provided as input.
fn challenge_answer(input: &str) -> Risk {
    let risk_grid = RiskGrid::new(&input);
    let grid_size = risk_grid.height();
    let mut best_risk = BestRiskGrid::new(grid_size);

    walk_path(&risk_grid, &mut best_risk, 0, 0, 0);

    best_risk.cell[grid_size - 1][grid_size - 1]
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    println!(
        "The total risk of the most efficient path is {}",
        challenge_answer(&input_file)
    );
}

// Test using data from the examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    const TEST_INPUT_BAD_LINE_LENGTH: &str = "\
1163751742
1381373672
2136511328
3694
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn parse_test_input() {
        let grid = RiskGrid::new(&TEST_INPUT);
        println!("{}", grid);
        assert_eq!(grid.cell[2][3], 6);
        assert_eq!(grid.cell[5][1], 3);
    }

    #[test]
    fn test_challenge_answer() {
        assert_eq!(challenge_answer(&TEST_INPUT), 40);
    }

    #[test]
    #[should_panic]
    fn incorrect_line_lengths() {
        let _ = RiskGrid::new(&TEST_INPUT_BAD_LINE_LENGTH);
    }
}
