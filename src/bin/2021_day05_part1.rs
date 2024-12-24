//! Advent of Code 2021 Day 05
//! https://adventofcode.com/2021/day/5
//!
//! Challenge part 1
//!
//! Read the end points of a number of lines from a file, where they are defined as x and y
//! coordinates on a 2D grid. Top-left is 0,0 and x is horizontal. Form a map of only horizontal
//! and vertical lines, where each x,y cell contains the number of lines that pass through it.
//! The challenge answer is the number of cells that have more than one line passing through.

use std::cmp::Ordering;
use std::fmt::{Display, Error, Formatter};
use std::fs;

const INPUT_FILENAME: &str = "2021_day05_input.txt";
const MAP_SIZE: usize = 1000;

type Line = (Coordinate, Coordinate);

#[derive(Clone, Copy, Debug, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    /// Return a new `Coordinate` from a string of two comma-separated numbers.
    ///
    /// # Panics
    ///
    /// Panics if the input string is malformed.
    fn new(input: &str) -> Self {
        let tokens: Vec<&str> = input.split(',').collect();

        if tokens.len() != 2 {
            panic!("Cannot parse malformed coordinate string: '{}'", input);
        }

        Self {
            x: tokens[0].parse::<usize>().unwrap(),
            y: tokens[1].parse::<usize>().unwrap(),
        }
    }
}

/// A grid of `cells` that record the number of hydrothermal vents across the ocean floor. The
/// y-axis is the major access, so cells are referenced as cells[y][x].
#[derive(Clone, PartialEq)]
struct Map {
    cells: Vec<Vec<u8>>,
}

impl Map {
    fn new(size: usize) -> Self {
        let mut cells = Vec::new();

        for _ in 0..size {
            let row = vec![0; size];
            cells.push(row);
        }

        Map { cells }
    }

    /// Update each of the `cells` of this `Map` that the given `Line` passes through.
    /// Limited to horizontal and vertical lines only.
    fn draw_line(&mut self, line: &Line) {
        match line.0.x.cmp(&line.1.x) {
            Ordering::Equal => {
                if line.0.y < line.1.y {
                    for row in line.0.y..=line.1.y {
                        self.cells[row][line.0.x] += 1;
                    }
                } else {
                    for row in line.1.y..=line.0.y {
                        self.cells[row][line.0.x] += 1;
                    }
                }
            }
            Ordering::Less => {
                for col in line.0.x..=line.1.x {
                    self.cells[line.0.y][col] += 1;
                }
            }
            Ordering::Greater => {
                for col in line.1.x..=line.0.x {
                    self.cells[line.0.y][col] += 1;
                }
            }
        }
    }

    /// Return the number of cells that have more than one line passing through them.
    fn count_intersections(&self) -> u32 {
        let mut total = 0;
        for row in &self.cells {
            total += row
                .iter()
                .fold(0, |acc, c| if c > &1 { acc + 1 } else { acc });
        }
        total
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut result = Ok(());

        for row in &self.cells {
            let row_as_string = row
                .iter()
                .map(|c| {
                    if c == &0 {
                        '.'.to_string()
                    } else {
                        c.to_string()
                    }
                })
                .collect::<String>();
            result = writeln!(f, "{}", row_as_string);
        }
        result
    }
}

/// Parses an input string consisting of two pairs of comma-separated numbers separated by an
/// arrow. Returns the pairs as a `Line`.
///
/// # Panics
///
/// Panics if the input string is malformed.
fn parse_input(input: &str) -> Vec<Line> {
    let mut coords = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split(" -> ").collect();

        if tokens.len() != 2 {
            panic!("Malformed input: {}", line);
        }

        coords.push((Coordinate::new(tokens[0]), Coordinate::new(tokens[1])));
    }

    coords
}

/// Picks only horizontal and vertical `Line`s from the input passed, and returns as a new `Vec`.
fn filter_horizontal_and_vertical(lines: &Vec<Line>) -> Vec<Line> {
    let mut output = Vec::new();

    for l in lines {
        if (l.0.x == l.1.x) | (l.0.y == l.1.y) {
            output.push(*l);
        }
    }
    output
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let mut map = Map::new(MAP_SIZE);
    let coords = parse_input(&input_file);
    let filtered = filter_horizontal_and_vertical(&coords);

    for l in &filtered {
        map.draw_line(l);
    }

    println!(
        "The number of positions with intersecting geothermal vents is {}",
        map.count_intersections()
    );
}

// Test using data from the examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn parse_test_input() {
        let coords = parse_input(TEST_INPUT);

        assert_eq!(coords.len(), 10);
        assert_eq!(
            coords[0],
            (Coordinate { x: 0, y: 9 }, Coordinate { x: 5, y: 9 })
        );
        assert_eq!(
            coords[4],
            (Coordinate { x: 7, y: 0 }, Coordinate { x: 7, y: 4 })
        );
        assert_eq!(
            coords[9],
            (Coordinate { x: 5, y: 5 }, Coordinate { x: 8, y: 2 })
        );
    }

    #[test]
    fn test_filtering() {
        let coords = parse_input(TEST_INPUT);
        let filtered = filter_horizontal_and_vertical(&coords);

        assert_eq!(filtered.len(), 6);
        assert_eq!(
            filtered[0],
            (Coordinate { x: 0, y: 9 }, Coordinate { x: 5, y: 9 })
        );
        assert_eq!(
            filtered[1],
            (Coordinate { x: 9, y: 4 }, Coordinate { x: 3, y: 4 })
        );
        assert_eq!(
            filtered[5],
            (Coordinate { x: 3, y: 4 }, Coordinate { x: 1, y: 4 })
        );
    }

    #[test]
    fn test_draw_line() {
        let mut map = Map::new(10);

        map.draw_line(&(Coordinate { x: 0, y: 7 }, Coordinate { x: 5, y: 7 }));
        map.draw_line(&(Coordinate { x: 3, y: 4 }, Coordinate { x: 3, y: 9 }));
        assert_eq!(map.cells[7][2], 1);
        assert_eq!(map.cells[7][3], 2);

        map.draw_line(&(Coordinate { x: 5, y: 4 }, Coordinate { x: 2, y: 4 }));
        assert_eq!(map.cells[4][2], 1);
        assert_eq!(map.cells[4][3], 2);
    }

    #[test]
    fn test_answer() {
        let mut map = Map::new(10);
        let coords = parse_input(TEST_INPUT);
        let filtered = filter_horizontal_and_vertical(&coords);

        for l in &filtered {
            map.draw_line(l);
        }

        assert_eq!(map.count_intersections(), 5);
    }
}
