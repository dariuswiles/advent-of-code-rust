//! Advent of Code 2023 Day 03
//! https://adventofcode.com/2023/day/3
//!
//! Challenge part 2
//!
//! Interprets the input as a 2D schematic containing multi-digit part numbers and symbols. Gear
//! symbols adjacent to exactly two numbers are considered "gears". For each gear, its two
//! adjacent numbers are multiplied to give the gear's "power". The powers are summed to give the
//! challenge answer.

use std::collections::{HashMap, HashSet};
use std::fs;

type Position = (usize, usize); // (row, column)

const INPUT_FILENAME: &str = "2023_day03_input.txt";
const CELL_EMPTY: char = '.';
const GEAR_SYMBOL: char = '*';

#[derive(Debug, PartialEq)]
enum Cell {
    Empty,
    Digit(u32),
    GearSymbol,
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
                } else if c == &GEAR_SYMBOL {
                    row.push(Cell::GearSymbol);
                } else {
                    // Cells containing symbols that aren't gears are treated as empty
                    row.push(Cell::Empty);
                }
            }

            cells.push(row);
        }

        Self {
            cells,
            width: width.unwrap(),
        }
    }

    /// Creates a `HashMap` containing every `Position` of the input data that is adjacent to a
    /// gear symbol. Each `Position` is formed from its row and column index. Each `Position` is
    /// mapped to the position of its adjacent gear(s). These are potential gears because the
    /// challenge mandates that a gear symbol must have exactly two adjacent numbers to be
    /// considered a genuine gear. Further checks need to be performed to determine this.
    fn create_gear_adjacency_map(&self) -> HashMap<Position, HashSet<Position>> {
        let mut map: HashMap<Position, HashSet<_>> = HashMap::new();
        let mask_height = self.cells.len();

        for row in 0..mask_height {
            for column in 0..self.width {
                if Cell::GearSymbol == self.cells[row][column] {
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
                            match map.get_mut(&(r, c)) {
                                Some(entry) => {
                                    entry.insert((row, column));
                                }
                                None => {
                                    map.insert((r, c), HashSet::from_iter(vec![(row, column)]));
                                }
                            }
                        }
                    }
                }
            }
        }

        map
    }

    /// Builds complete numbers from individual digits in the `Schematic` and determines which
    /// gear symbols, if any, are adjacent to each number. Returns a `HashMap` mapping each gear
    /// symbol's position to the number(s) it is adjacent to. Gear symbols that are not adjacent to
    /// any numbers are not included.
    fn map_gears_to_numbers(
        &self,
        m: &HashMap<Position, HashSet<Position>>,
    ) -> HashMap<Position, HashSet<u32>> {
        let mut gear_to_number_map = HashMap::new();

        for row in 0..self.cells.len() {
            let mut n = 0;
            let mut adjacent_gears: HashSet<Position> = HashSet::new();
            for column in 0..self.width {
                if let Cell::Digit(d) = self.cells[row][column] {
                    n = n * 10 + d;

                    if let Some(gear_positions) = m.get(&(row, column)) {
                        adjacent_gears =
                            HashSet::from_iter(adjacent_gears.union(gear_positions).map(|g| *g));
                    }
                } else {
                    if n > 0 {
                        if adjacent_gears.len() > 0 {
                            add_number_to_gears(&mut gear_to_number_map, n, &mut adjacent_gears);
                        }

                        n = 0;
                        adjacent_gears = HashSet::new();
                    }
                }
            }

            if adjacent_gears.len() > 0 {
                add_number_to_gears(&mut gear_to_number_map, n, &mut adjacent_gears);
            }
        }

        gear_to_number_map
    }
}

/// Associates `number` to every gear symbol that is adjacent in the gear_to_number_map passed.
fn add_number_to_gears(
    gear_to_number_map: &mut HashMap<Position, HashSet<u32>>,
    number: u32,
    adjacent_gears: &mut HashSet<Position>,
) {
    for gear_position in adjacent_gears.iter() {
        match gear_to_number_map.get_mut(&gear_position) {
            Some(entry) => {
                entry.insert(number);
            }
            None => {
                gear_to_number_map.insert(*gear_position, HashSet::from_iter([number]));
            }
        }
    }
}

/// Returns `true` if the set of numbers associated with a single gear symbol passed, meet the
/// challenge's criteria for a gear, i.e., there are exactly 2 numbers.
fn is_gear(numbers: &HashSet<u32>) -> bool {
    2 == numbers.len()
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    println!(
        "The sum of the power of all the gears is {}",
        do_challenge(&input)
    );
}

/// Returns the challenge answer. This is generated by creating a `Schematic` from the input passed,
/// identifying gear symbols and creating a separate map listing which of them are adjacent to each
/// position in the schematic. This is used to create a map of gear symbol positions to adjacent
/// number(s). Finally, gear symbols with exactly two adjacent numbers have those numbers multiplied
/// to produce the "power" of the gear, and these are summed to give the challenge answer.
fn do_challenge(input: &str) -> u32 {
    let s = Schematic::from_string(input);
    let m = s.create_gear_adjacency_map();
    let g2nums = s.map_gears_to_numbers(&m);

    g2nums
        .values()
        .filter(|g| is_gear(g))
        .map(|g| g.iter().fold(1, |power, n| power * n))
        .sum()
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
        assert_eq!(Cell::GearSymbol, s.cells[1][3]);

        assert_eq!(Cell::GearSymbol, s.cells[4][3]);

        assert_eq!(Cell::Empty, s.cells[5][5]);
        assert_eq!(Cell::Empty, s.cells[5][6]);
        assert_eq!(Cell::Digit(5), s.cells[5][7]);
        assert_eq!(Cell::Digit(8), s.cells[5][8]);
        assert_eq!(Cell::Empty, s.cells[5][9]);

        assert_eq!(Cell::GearSymbol, s.cells[8][5]);

        assert_eq!(Cell::Digit(8), s.cells[9][7]);
        assert_eq!(Cell::Empty, s.cells[9][9]);
    }

    #[test]
    fn test_create_gear_adjacency_map() {
        let s = Schematic::from_string(&TEST_INPUT);
        let m = s.create_gear_adjacency_map();

        assert_eq!(27, m.len());
        assert_eq!(None, m.get(&(0, 0)));

        let gear_1_3 = HashSet::from_iter(vec![(1, 3)].iter().cloned());
        assert_eq!(Some(&gear_1_3), m.get(&(0, 2)));
        assert_eq!(Some(&gear_1_3), m.get(&(0, 3)));
        assert_eq!(Some(&gear_1_3), m.get(&(0, 4)));
        assert_eq!(Some(&gear_1_3), m.get(&(1, 2)));
        assert_eq!(Some(&gear_1_3), m.get(&(1, 3)));
        assert_eq!(Some(&gear_1_3), m.get(&(1, 4)));
        assert_eq!(Some(&gear_1_3), m.get(&(2, 2)));
        assert_eq!(Some(&gear_1_3), m.get(&(2, 3)));
        assert_eq!(Some(&gear_1_3), m.get(&(2, 4)));

        let gear_4_3 = HashSet::from_iter(vec![(4, 3)].iter().cloned());
        assert_eq!(Some(&gear_4_3), m.get(&(3, 2)));
        assert_eq!(Some(&gear_4_3), m.get(&(3, 3)));
        assert_eq!(Some(&gear_4_3), m.get(&(3, 4)));
        assert_eq!(Some(&gear_4_3), m.get(&(4, 2)));
        assert_eq!(Some(&gear_4_3), m.get(&(4, 3)));
        assert_eq!(Some(&gear_4_3), m.get(&(4, 4)));
        assert_eq!(Some(&gear_4_3), m.get(&(5, 2)));
        assert_eq!(Some(&gear_4_3), m.get(&(5, 3)));
        assert_eq!(Some(&gear_4_3), m.get(&(5, 4)));

        let gear_8_5 = HashSet::from_iter(vec![(8, 5)].iter().cloned());
        assert_eq!(Some(&gear_8_5), m.get(&(7, 4)));
        assert_eq!(Some(&gear_8_5), m.get(&(7, 5)));
        assert_eq!(Some(&gear_8_5), m.get(&(7, 6)));
        assert_eq!(Some(&gear_8_5), m.get(&(8, 4)));
        assert_eq!(Some(&gear_8_5), m.get(&(8, 5)));
        assert_eq!(Some(&gear_8_5), m.get(&(8, 6)));
        assert_eq!(Some(&gear_8_5), m.get(&(9, 4)));
        assert_eq!(Some(&gear_8_5), m.get(&(9, 5)));
        assert_eq!(Some(&gear_8_5), m.get(&(9, 6)));
    }

    #[test]
    fn test_map_gears_to_numbers() {
        let s = Schematic::from_string(&TEST_INPUT);
        let m = s.create_gear_adjacency_map();
        let g2nums = s.map_gears_to_numbers(&m);

        assert_eq!(None, g2nums.get(&(0, 0)));

        assert_eq!(3, g2nums.len());
        assert_eq!(Some(&HashSet::from_iter([467, 35])), g2nums.get(&(1, 3)));
        assert_eq!(Some(&HashSet::from_iter([617])), g2nums.get(&(4, 3)));
        assert_eq!(Some(&HashSet::from_iter([755, 598])), g2nums.get(&(8, 5)));
    }

    #[test]
    fn test_is_gear() {
        assert!(is_gear(&HashSet::from_iter([467, 35])));
        assert!(!is_gear(&HashSet::from_iter([617])));
        assert!(is_gear(&HashSet::from_iter([755, 598])));
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(467835, do_challenge(&TEST_INPUT));
    }
}
