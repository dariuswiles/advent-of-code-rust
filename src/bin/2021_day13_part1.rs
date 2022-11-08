//! Advent of Code 2021 Day 13
//! https://adventofcode.com/2021/day/13
//!
//! Challenge part 1
//!
//! Place dots on a grid at positions given in the input, simulate folding the grid
//! horizontally and vertically along lines given in the input, and return the resulting number of
//! visible dots.

use std::collections::HashSet;
use std::fs;

const INPUT_FILENAME: &str = "2021_day13_input.txt";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: u16,
    y: u16,
}

/// A `Grid` is a `HashSet` of dots. Top-left is (0, 0) and positive x extends horizontally to the
/// right.
#[derive(Debug, PartialEq)]
struct Grid {
    dots: HashSet<Coord>,
}

impl Grid {
    /// Returns a new `Grid` created from an input string containing an arbitrary number of lines,
    /// where each line contains a single x,y coordinate in the form "x,y", e.g., "6,10".
    ///
    /// # Panics
    ///
    /// Panics if the input is malformed.
    fn new(input: &Vec<&str>) -> Self {
        let mut dots = HashSet::new();
        for dot in input {
            let x_y: Vec<&str> = dot.split(',').collect();
            if x_y.len() != 2 {
                panic!("Malformed coordinates for dot: {}", dot);
            }

            dots.insert(Coord {
                x: u16::from_str_radix(x_y[0], 10).unwrap(),
                y: u16::from_str_radix(x_y[1], 10).unwrap(),
            });
        }

        Self { dots }
    }

    /// Modifies this grid by folding it in accordance with the `Fold` instruction passed.
    fn perform_fold(&mut self, fold: &Fold) {
        let mut new_dots = HashSet::new();

        match fold.axis {
            'x' => {
                for d in &self.dots {
                    if d.x < fold.location {
                        new_dots.insert(*d);
                    } else {
                        new_dots.insert(Coord {
                            x: fold.location * 2 - d.x,
                            y: d.y,
                        });
                    }
                }
            }
            'y' => {
                for d in &self.dots {
                    if d.y < fold.location {
                        new_dots.insert(*d);
                    } else {
                        new_dots.insert(Coord {
                            x: d.x,
                            y: fold.location * 2 - d.y,
                        });
                    }
                }
            }
            _ => {
                panic!(
                    "Internal error: `Coord` contains unexpected axis '{}'",
                    fold.axis
                );
            }
        }

        self.dots = new_dots;
    }
}

/// Contains details of a fold instruction, i.e., the fold axis and location.
#[derive(Debug, PartialEq)]
struct Fold {
    axis: char,
    location: u16,
}

impl Fold {
    /// Returns a new `Fold` created from the given string.
    ///
    /// # Panics
    ///
    /// Panics if the input is malformed.
    fn new(input: &str) -> Self {
        let substring = input.strip_prefix("fold along ").unwrap();

        let fold_details: Vec<&str> = substring.split('=').collect();
        assert_eq!(fold_details.len(), 2);

        let axis;
        let location;

        axis = fold_details[0].chars().next().unwrap();
        assert!(axis == 'x' || axis == 'y');

        location = u16::from_str_radix(fold_details[1], 10).unwrap();

        Self { axis, location }
    }
}

/// Parses a string consisting of lines of comma separated coordinates, then a blank line, then
/// lines with fold information. Returns a `Grid` containing dots at the coordinates, and a `Vec`
/// containing the individual `Fold` instructions.
fn parse_input(input: &str) -> (Grid, Vec<Fold>) {
    let mut dots = Vec::new();
    let mut folds = Vec::new();
    let mut line = input.lines();

    while let Some(l) = line.next() {
        if l.len() == 0 {
            break;
        }
        dots.push(l);
    }

    let grid = Grid::new(&dots);

    while let Some(l) = line.next() {
        if l.len() > 0 {
            folds.push(Fold::new(l));
        }
    }

    (grid, folds)
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let (mut grid, folds) = parse_input(&input_file);
    grid.perform_fold(&folds[0]);

    println!(
        "The number of visible dots in the grid is {}",
        grid.dots.len()
    );
}

// Test using data from the examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_parse_input() {
        let (grid, folds) = parse_input(&TEST_INPUT);

        assert_eq!(grid.dots.len(), 18);
        assert!(grid.dots.contains(&Coord { x: 3, y: 0 }));
        assert!(grid.dots.contains(&Coord { x: 6, y: 0 }));
        assert!(grid.dots.contains(&Coord { x: 9, y: 0 }));
        assert!(grid.dots.contains(&Coord { x: 4, y: 1 }));
        assert!(grid.dots.contains(&Coord { x: 0, y: 3 }));
        assert!(grid.dots.contains(&Coord { x: 3, y: 4 }));
        assert!(grid.dots.contains(&Coord { x: 8, y: 4 }));
        assert!(grid.dots.contains(&Coord { x: 10, y: 4 }));
        assert!(grid.dots.contains(&Coord { x: 1, y: 10 }));
        assert!(grid.dots.contains(&Coord { x: 6, y: 10 }));
        assert!(grid.dots.contains(&Coord { x: 8, y: 10 }));
        assert!(grid.dots.contains(&Coord { x: 9, y: 10 }));
        assert!(grid.dots.contains(&Coord { x: 4, y: 11 }));
        assert!(grid.dots.contains(&Coord { x: 6, y: 12 }));
        assert!(grid.dots.contains(&Coord { x: 10, y: 12 }));
        assert!(grid.dots.contains(&Coord { x: 0, y: 13 }));
        assert!(grid.dots.contains(&Coord { x: 0, y: 14 }));
        assert!(grid.dots.contains(&Coord { x: 2, y: 14 }));
        assert_eq!(folds.len(), 2);
        assert_eq!(
            folds[0],
            Fold {
                axis: 'y',
                location: 7
            }
        );
        assert_eq!(
            folds[1],
            Fold {
                axis: 'x',
                location: 5
            }
        );
    }

    #[test]
    fn test_perform_fold_1() {
        let (mut grid, folds) = parse_input(&TEST_INPUT);
        grid.perform_fold(&folds[0]);

        assert_eq!(grid.dots.len(), 17);
        assert!(grid.dots.contains(&Coord { x: 3, y: 0 }));
        assert!(grid.dots.contains(&Coord { x: 6, y: 0 }));
        assert!(grid.dots.contains(&Coord { x: 9, y: 0 }));
        assert!(grid.dots.contains(&Coord { x: 4, y: 1 }));
        assert!(grid.dots.contains(&Coord { x: 0, y: 3 }));
        assert!(grid.dots.contains(&Coord { x: 3, y: 4 }));
        assert!(grid.dots.contains(&Coord { x: 8, y: 4 }));
        assert!(grid.dots.contains(&Coord { x: 10, y: 4 }));

        assert!(grid.dots.contains(&Coord { x: 1, y: 4 }));
        assert!(grid.dots.contains(&Coord { x: 6, y: 4 }));
        assert!(grid.dots.contains(&Coord { x: 9, y: 4 }));
        assert!(grid.dots.contains(&Coord { x: 4, y: 3 }));
        assert!(grid.dots.contains(&Coord { x: 6, y: 2 }));
        assert!(grid.dots.contains(&Coord { x: 10, y: 2 }));
        assert!(grid.dots.contains(&Coord { x: 0, y: 1 }));
        assert!(grid.dots.contains(&Coord { x: 0, y: 0 }));
        assert!(grid.dots.contains(&Coord { x: 2, y: 0 }));
    }

    #[test]
    fn test_perform_fold_2() {
        let (mut grid, folds) = parse_input(&TEST_INPUT);
        grid.perform_fold(&folds[0]);
        grid.perform_fold(&folds[1]);

        assert_eq!(grid.dots.len(), 16);
        assert!(grid.dots.contains(&Coord { x: 0, y: 0 }));
        assert!(grid.dots.contains(&Coord { x: 0, y: 1 }));
        assert!(grid.dots.contains(&Coord { x: 0, y: 3 }));
        assert!(grid.dots.contains(&Coord { x: 0, y: 2 }));
        assert!(grid.dots.contains(&Coord { x: 0, y: 4 }));
        assert!(grid.dots.contains(&Coord { x: 1, y: 0 }));
        assert!(grid.dots.contains(&Coord { x: 1, y: 4 }));
        assert!(grid.dots.contains(&Coord { x: 2, y: 0 }));
        assert!(grid.dots.contains(&Coord { x: 2, y: 4 }));
        assert!(grid.dots.contains(&Coord { x: 3, y: 0 }));
        assert!(grid.dots.contains(&Coord { x: 3, y: 4 }));
        assert!(grid.dots.contains(&Coord { x: 4, y: 0 }));
        assert!(grid.dots.contains(&Coord { x: 4, y: 1 }));
        assert!(grid.dots.contains(&Coord { x: 4, y: 2 }));
        assert!(grid.dots.contains(&Coord { x: 4, y: 3 }));
        assert!(grid.dots.contains(&Coord { x: 4, y: 4 }));
    }
}
