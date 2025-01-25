//! Advent of Code 2024 Day 10
//! https://adventofcode.com/2024/day/10
//!
//! Challenge part 2
//!
//! The input represents a topological map where each cell is a single digit representing its
//! height. The challenge is to count the number of trails on the map, where each trail starts at
//! height 0 and ends at height 9.
//!
//! Part 2 of the challenge requires every distinct route between a trailhead and destination cell
//! to be counted separately.

use std::fs;

const INPUT_FILENAME: &str = "2024_day10_input.txt";

type Altitude = u8;
type Position = (usize, usize);

/// A topographic map
#[derive(Debug, PartialEq)]
struct TopoMap {
    cells: Vec<Vec<Altitude>>,
    height: usize,
    width: usize,
}

impl TopoMap {
    /// Creates a new `TopoMap` from an input string.
    fn new(input: &str) -> Self {
        let mut cells = Vec::new();
        let mut line_length = None;

        for line in input.lines() {
            if line.is_empty() {
                continue;
            }

            if let Some(prior_length) = line_length {
                if prior_length != line.len() {
                    panic!("All lines of input must contain the same number of digits");
                }
            } else {
                line_length = Some(line.len());
            }

            cells.push(
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as Altitude)
                    .collect(),
            );
        }

        let height = cells.len();
        Self {
            cells,
            height,
            width: line_length.unwrap(),
        }
    }

    /// Counts all distinct trails from the given trailhead position to a destination cell that is
    /// height 9. The trailhead position must be a cell containing 0 or the result will be
    /// incorrect. Part 2 of the challenge requires every distinct route between a trailhead and
    /// destination cell to be counted separately.
    fn count_trails_from_trailhead(&self, p: Position) -> u16 {
        let mut todo = Vec::new();
        let mut trail_count = 0;

        todo.push(p);

        while let Some((current_row, current_col)) = todo.pop() {
            let current_value = self.cells[current_row][current_col];

            if self.cells[current_row][current_col] == 9 {
                trail_count += 1;
            } else {
                if current_row > 0 && self.cells[current_row - 1][current_col] == current_value + 1
                {
                    todo.push((current_row - 1, current_col));
                }

                if current_col > 0 && self.cells[current_row][current_col - 1] == current_value + 1
                {
                    todo.push((current_row, current_col - 1));
                }

                if current_row + 1 < self.height
                    && self.cells[current_row + 1][current_col] == current_value + 1
                {
                    todo.push((current_row + 1, current_col));
                }

                if current_col + 1 < self.width
                    && self.cells[current_row][current_col + 1] == current_value + 1
                {
                    todo.push((current_row, current_col + 1));
                }
            }
        }

        trail_count
    }

    /// Returns a `Vec` containing all trailheads in this `TopoMap`.
    fn find_all_trailheads(&self) -> Vec<Position> {
        let mut trailheads = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[x][y] == 0 {
                    trailheads.push((x, y));
                }
            }
        }

        trailheads
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    println!(
        "The sum of the ratings of all trailheads is {}",
        do_challenge(&input)
    );
}

/// Finds all trailheads in the input and sums the number of distinct trails from each to give the
/// challenge answer.
fn do_challenge(input: &str) -> u16 {
    let topo = TopoMap::new(input);

    topo.find_all_trailheads()
        .iter()
        .map(|t| topo.count_trails_from_trailhead(*t))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_0: &str = "\
2222202
2243212
2252222
2265432
2272242
2287652
2292222";

    const INPUT_1: &str = "\
1190119
1111198
1112117
6543456
7651987
8761111
9871111";

    const INPUT_2: &str = "\
012345
123456
234567
345678
416789
567891";

    const INPUT_3: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            TopoMap {
                cells: vec![
                    vec![2, 2, 2, 2, 2, 0, 2],
                    vec![2, 2, 4, 3, 2, 1, 2],
                    vec![2, 2, 5, 2, 2, 2, 2],
                    vec![2, 2, 6, 5, 4, 3, 2],
                    vec![2, 2, 7, 2, 2, 4, 2],
                    vec![2, 2, 8, 7, 6, 5, 2],
                    vec![2, 2, 9, 2, 2, 2, 2],
                ],
                height: 7,
                width: 7,
            },
            TopoMap::new(INPUT_0)
        );

        assert_eq!(
            TopoMap {
                cells: vec![
                    vec![1, 1, 9, 0, 1, 1, 9],
                    vec![1, 1, 1, 1, 1, 9, 8],
                    vec![1, 1, 1, 2, 1, 1, 7],
                    vec![6, 5, 4, 3, 4, 5, 6],
                    vec![7, 6, 5, 1, 9, 8, 7],
                    vec![8, 7, 6, 1, 1, 1, 1],
                    vec![9, 8, 7, 1, 1, 1, 1],
                ],
                height: 7,
                width: 7,
            },
            TopoMap::new(INPUT_1)
        );

        assert_eq!(
            TopoMap {
                cells: vec![
                    vec![0, 1, 2, 3, 4, 5],
                    vec![1, 2, 3, 4, 5, 6],
                    vec![2, 3, 4, 5, 6, 7],
                    vec![3, 4, 5, 6, 7, 8],
                    vec![4, 1, 6, 7, 8, 9],
                    vec![5, 6, 7, 8, 9, 1],
                ],
                height: 6,
                width: 6,
            },
            TopoMap::new(INPUT_2)
        );

        assert_eq!(
            TopoMap {
                cells: vec![
                    vec![8, 9, 0, 1, 0, 1, 2, 3],
                    vec![7, 8, 1, 2, 1, 8, 7, 4],
                    vec![8, 7, 4, 3, 0, 9, 6, 5],
                    vec![9, 6, 5, 4, 9, 8, 7, 4],
                    vec![4, 5, 6, 7, 8, 9, 0, 3],
                    vec![3, 2, 0, 1, 9, 0, 1, 2],
                    vec![0, 1, 3, 2, 9, 8, 0, 1],
                    vec![1, 0, 4, 5, 6, 7, 3, 2],
                ],
                height: 8,
                width: 8,
            },
            TopoMap::new(INPUT_3)
        );
    }

    #[test]
    fn test_count_trails_from_trailhead() {
        assert_eq!(3, TopoMap::new(INPUT_0).count_trails_from_trailhead((0, 5)));
        assert_eq!(
            13,
            TopoMap::new(INPUT_1).count_trails_from_trailhead((0, 3))
        );
        assert_eq!(
            227,
            TopoMap::new(INPUT_2).count_trails_from_trailhead((0, 0))
        );

        let topo3 = TopoMap::new(INPUT_3);
        assert_eq!(20, topo3.count_trails_from_trailhead((0, 2)));
        assert_eq!(24, topo3.count_trails_from_trailhead((0, 4)));
        assert_eq!(10, topo3.count_trails_from_trailhead((2, 4)));
        assert_eq!(4, topo3.count_trails_from_trailhead((4, 7)));
        assert_eq!(1, topo3.count_trails_from_trailhead((5, 2)));
        assert_eq!(4, topo3.count_trails_from_trailhead((5, 5)));
        assert_eq!(5, topo3.count_trails_from_trailhead((6, 0)));
        assert_eq!(8, topo3.count_trails_from_trailhead((6, 6)));
        assert_eq!(5, topo3.count_trails_from_trailhead((7, 1)));
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(81, do_challenge(INPUT_3));
    }
}
