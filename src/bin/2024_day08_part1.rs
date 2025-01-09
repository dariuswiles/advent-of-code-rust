//! Advent of Code 2024 Day 08
//! https://adventofcode.com/2024/day/8
//!
//! Challenge part 1
//!
//! The input consists of a grid of different types of antennas and the challenge is to find
//! locations on the grid that are specific distances from two antennas of the same type.

use std::collections::{HashMap, HashSet};
use std::fs;

const INPUT_FILENAME: &str = "2024_day08_input.txt";

type Position = (i16, i16);

#[derive(Debug)]
struct Grid {
    antennas: HashMap<char, HashSet<Position>>,
    size: Position,
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    println!("There are {} antinodes in the grid", do_challenge(&input));
}

/// Calculates the `Position` of every antinode for the antennas in the given input. Returns the
/// total number of antinodes.
fn do_challenge(input: &str) -> usize {
    let grid = parse_input(input);
    get_antinodes(&grid).len()
}

/// Returns a `Grid` containing a `HashMap` of every entry for antenna in the given input; and the
/// size of input grid. The former maps the symbol for each antenna to a `HashSet` of the
/// `Position`s of all its associated antennas. Each position is a tuple containing the row and
/// column. The top-left of the grid is (0, 0).
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> Grid {
    let mut antennas: HashMap<char, HashSet<Position>> = HashMap::new();
    let mut width = None;
    let mut row = 0;

    for line in input.lines() {
        if !line.is_empty() {
            if width.is_none() {
                width = Some(line.len());
            }

            for (col, c) in line.chars().enumerate() {
                match c {
                    '.' => {}
                    _ => {
                        let position = (row, col as i16);

                        match antennas.get_mut(&c) {
                            Some(ant) => {
                                ant.insert(position);
                            }
                            None => {
                                let mut location = HashSet::new();
                                location.insert(position);

                                antennas.insert(c, location);
                            }
                        }
                    }
                }
            }
            row += 1;
        }
    }

    Grid {
        antennas,
        size: (width.unwrap() as i16, row),
    }
}

/// Returns a `HashSet` containing the `Position` of all antinodes, calculated from the `Position`s
/// of all the antennas in the given `Grid`. Each antinode is located on a line drawn between a
/// pair of antennas. Its distance from the nearest antenna is the same as the distance between
/// this antenna and the second antenna in the pair. Antinodes that would fall outside the grid are
/// ignored.
fn get_antinodes(grid: &Grid) -> HashSet<Position> {
    let mut antinodes = HashSet::new();

    // for (_, locations) in &grid.antennas {
    for locations in grid.antennas.values() {
        let locations_vec: Vec<_> = locations.iter().collect();
        let location_pairs = all_distinct_pairs(&locations_vec);

        for ((x_0, y_0), (x_1, y_1)) in location_pairs {
            let x_diff = x_1 - x_0;
            let y_diff = y_1 - y_0;

            let x = x_0 - x_diff;
            let y = y_0 - y_diff;
            if (0..grid.size.0).contains(&x) && (0..grid.size.1).contains(&y) {
                antinodes.insert((x, y));
            }

            let x = x_1 + x_diff;
            let y = y_1 + y_diff;
            if (0..grid.size.0).contains(&x) && (0..grid.size.1).contains(&y) {
                antinodes.insert((x, y));
            }
        }
    }

    antinodes
}

/// Returns every pair of elements from the given slice where each pair is formed from different
/// elements. E.g., if slice is [1, 2, 3, 4], the result is a `Vec` containing
/// `(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)`.
fn all_distinct_pairs<T: Clone + Copy>(slice: &[T]) -> Vec<(T, T)> {
    let mut result = Vec::new();

    let slice_len = slice.len();

    for left in 0..slice_len {
        for right in left + 1..slice_len {
            result.push((slice[left], slice[right]));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn test_parse_input() {
        let grid = parse_input(INPUT);
        let antennas = grid.antennas;

        assert_eq!(2, antennas.len());

        let zero = antennas.get(&'0').unwrap();
        assert_eq!(4, zero.len());
        assert!(zero.contains(&(1, 8)));
        assert!(zero.contains(&(2, 5)));
        assert!(zero.contains(&(3, 7)));
        assert!(zero.contains(&(4, 4)));

        let upper_a = antennas.get(&'A').unwrap();
        assert_eq!(3, upper_a.len());
        assert!(upper_a.contains(&(5, 6)));
        assert!(upper_a.contains(&(8, 8)));
        assert!(upper_a.contains(&(9, 9)));

        assert_eq!((12, 12), grid.size);
    }

    #[test]
    fn test_all_distinct_pairs() {
        assert_eq!(
            vec![(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)],
            all_distinct_pairs(&[1, 2, 3, 4])
        );
    }

    #[test]
    fn test_get_antinodes() {
        let grid = parse_input(INPUT);
        let antinodes = get_antinodes(&grid);

        assert_eq!(14, antinodes.len());
        assert!(antinodes.contains(&(0, 6)));
        assert!(antinodes.contains(&(0, 11)));
        assert!(antinodes.contains(&(1, 3)));
        assert!(antinodes.contains(&(2, 4)));
        assert!(antinodes.contains(&(2, 10)));
        assert!(antinodes.contains(&(3, 2)));
        assert!(antinodes.contains(&(4, 9)));
        assert!(antinodes.contains(&(5, 1)));
        assert!(antinodes.contains(&(5, 6)));
        assert!(antinodes.contains(&(6, 3)));
        assert!(antinodes.contains(&(7, 0)));
        assert!(antinodes.contains(&(7, 7)));
        assert!(antinodes.contains(&(10, 10)));
        assert!(antinodes.contains(&(11, 10)));
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(14, do_challenge(INPUT));
    }
}
