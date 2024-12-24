//! Advent of Code 2022 Day 08
//! https://adventofcode.com/2022/day/8
//!
//! Challenge part 1
//!
//! Reads an input file containing a grid of single digits, each digit representing the height of
//! a tree. Determines the number of trees which are visible from outside the grid, where a tree
//! is visible if it is higher than all trees between it and an edge. The visibility analysis is
//! performed vertically and horizontally, but not diagonally. The challenge answer is the number
//! of visible trees.

use std::fs;

type Height = i8;

const INPUT_FILENAME: &str = "2022_day08_input.txt";

/// Takes a string containing lines of tightly packed single digits and returns them as a
/// two-dimensional vector of integers.
///
/// # Panics
///
/// Panics if the input is malformed.
/// Panics if every row does not have the same number of columns.
fn parse_input(input: &str) -> Vec<Vec<Height>> {
    let mut grid: Vec<Vec<Height>> = Vec::new();

    for line in input.lines() {
        if !line.is_empty() {
            grid.push(
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as Height)
                    .collect(),
            );
        }
    }

    let num_columns: usize = grid[0].len();
    assert!(grid.iter().all(|row| row.len() == num_columns));

    grid
}

/// Returns a two-dimensional vector with the same dimensions as `tree_grid`. Each cell contains a
/// Boolean indicating if the tree at the same location in `tree_grid` is visible.
//
// This is determined by looking along each row and column from both directions to determine if
// each tree is visible from outside the grid.
fn find_visible_trees(tree_grid: &[Vec<Height>]) -> Vec<Vec<bool>> {
    let num_rows = tree_grid.len();
    let num_columns = tree_grid[0].len();
    let mut visible_trees = Vec::new();

    // Initialize all trees to indicate they are hidden, represented with Boolean `false`.
    for _ in 0..num_rows {
        visible_trees.push(vec![false; num_columns]);
    }

    // Left to right
    for r in 0..num_rows {
        let mut highest_so_far = -1;
        for c in 0..num_columns {
            let tree_height = tree_grid[r][c];

            if tree_height > highest_so_far {
                visible_trees[r][c] |= true;
                highest_so_far = tree_height;
            }
        }
    }

    // Right to left
    for r in 0..num_rows {
        let mut highest_so_far = -1;
        for c in (0..num_columns).rev() {
            let tree_height = tree_grid[r][c];

            if tree_height > highest_so_far {
                visible_trees[r][c] |= true;
                highest_so_far = tree_height;
            }
        }
    }

    // Top to bottom
    for c in 0..num_columns {
        let mut highest_so_far = -1;
        for r in 0..num_rows {
            let tree_height = tree_grid[r][c];

            if tree_height > highest_so_far {
                visible_trees[r][c] |= true;
                highest_so_far = tree_height;
            }
        }
    }

    // Bottom to top
    for c in 0..num_columns {
        let mut highest_so_far = -1;
        for r in (0..num_rows).rev() {
            let tree_height = tree_grid[r][c];

            if tree_height > highest_so_far {
                visible_trees[r][c] |= true;
                highest_so_far = tree_height;
            }
        }
    }

    visible_trees
}

/// Returns the number of visible trees in `visible_trees`.
fn challenge_answer(visible_trees: &[Vec<bool>]) -> usize {
    visible_trees.iter().flatten().filter(|&t| *t).count()
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    let tree_grid = parse_input(&input);
    let visible_trees = find_visible_trees(&tree_grid);

    println!(
        "The number of visible trees is {}",
        challenge_answer(&visible_trees),
    );
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
30373
25512
65332
33549
35390
";

    #[test]
    fn test_parse_input() {
        let grid = parse_input(TEST_INPUT);

        #[rustfmt::skip]
        assert_eq!(grid,
            vec![
                vec![3, 0, 3, 7, 3, ],
                vec![2, 5, 5, 1, 2, ],
                vec![6, 5, 3, 3, 2, ],
                vec![3, 3, 5, 4, 9, ],
                vec![3, 5, 3, 9, 0, ],
            ]
        );
    }

    #[test]
    #[should_panic]
    fn test_parse_input_unequal_lines() {
        #[rustfmt::skip]
        let _ = parse_input("303\n25512\n653");
    }

    #[test]
    fn test_find_visible_trees() {
        let tree_grid = parse_input(TEST_INPUT);
        let visible_trees = find_visible_trees(&tree_grid);

        #[rustfmt::skip]
        assert_eq!(visible_trees,
            vec![
                vec![true,  true,  true,  true,  true, ],
                vec![true,  true,  true, false,  true, ],
                vec![true,  true, false,  true,  true, ],
                vec![true, false,  true, false,  true, ],
                vec![true,  true,  true,  true,  true, ],
            ]
        );
    }

    #[test]
    fn test_challenge_answer() {
        let tree_grid = parse_input(TEST_INPUT);
        let visible_trees = find_visible_trees(&tree_grid);

        assert_eq!(challenge_answer(&visible_trees), 21);
    }
}
