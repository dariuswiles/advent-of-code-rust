//! Advent of Code 2022 Day 17
//! https://adventofcode.com/2022/day/17
//!
//! Challenge part 1
//!
//! Determine the height of a stack of differently shaped rocks that are pushed left and right as
//! they fall before coming to rest.

use std::fmt::{self, Display};
use std::fs;
use std::iter::Cycle;
use std::str::Chars;

type WidthType = u8;
type HeightType = usize;
type Row = [char; CHAMBER_WIDTH as usize];

const INPUT_FILENAME: &str = "2022_day17_input.txt";
const CHAMBER_WIDTH: WidthType = 7;
const REPETITIONS: usize = 2022;

// Define the rock shapes specified in the challenge but with rows of the shape ordered from
// the bottom of the shape to the top. This is the opposite order given in the challenge but is
// required as the `Chamber` is defined with row 0 being the bottom row. This actually only affects
// 'ROCK_SHAPE_2' as all other shapes are horizontally symmetrical.
const ROCK_HORIZONTAL_LINE: &[&str; 1] = &["####"];

#[rustfmt::skip]
const ROCK_PLUS: &[&str; 3] = &[
    ".#.",
    "###",
    ".#."];

#[rustfmt::skip]
const ROCK_L: &[&str; 3] = &[
    "###",
    "..#",
    "..#"];

#[rustfmt::skip]
const ROCK_VERTICAL_LINE: &[&str; 4] = &[
    "#",
    "#",
    "#",
    "#"];

#[rustfmt::skip]
const ROCK_SQUARE: &[&str; 2] = &[
    "##",
    "##"];

#[derive(Clone, Copy, Debug, PartialEq)]
enum RockShape {
    HorizontalLine,
    Plus,
    L,
    VerticalLine,
    Square,
}

const ROCK_SHAPE_ORDER: [RockShape; 5] = [
    RockShape::HorizontalLine,
    RockShape::Plus,
    RockShape::L,
    RockShape::VerticalLine,
    RockShape::Square,
];

/// Holds the empty space and at-rest rocks in the `Chamber`'s cavern. The rows are indexed with
/// the lowest empty row being index 0.
#[derive(Clone)]
struct Chamber {
    cavern: Vec<Row>,
}

impl Display for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cavern_row_count = self.cavern.len();

        let top_rows;

        if cavern_row_count > 20 {
            top_rows = &self.cavern[cavern_row_count - 20..];
        } else {
            top_rows = &self.cavern[..];
        }

        for row in top_rows.iter().rev() {
            _ = write!(f, "|{}|\n", row.iter().collect::<String>());
        }

        if cavern_row_count > 20 {
            write!(f, "~~~~~~~~~ {cavern_row_count}")
        } else {
            write!(f, "+-------+")
        }
    }
}

impl Chamber {
    /// Returns an empty `Chamber`, i.e., one that contains no rocks.
    fn new() -> Self {
        Self { cavern: Vec::new() }
    }

    /// Adds a rock of the given shape to this `Chamber` at the given coordinates. Additional rows
    /// are added to the top of the `Chamber` if required.
    fn put_rock(&mut self, rock: RockShape, left_edge: WidthType, bottom_edge: HeightType) {
        let rock_cells = match rock {
            RockShape::HorizontalLine => ROCK_HORIZONTAL_LINE.to_vec(),
            RockShape::Plus => ROCK_PLUS.to_vec(),
            RockShape::L => ROCK_L.to_vec(),
            RockShape::VerticalLine => ROCK_VERTICAL_LINE.to_vec(),
            RockShape::Square => ROCK_SQUARE.to_vec(),
        };

        let rock_height = rock_cells.len();
        let highest_row_needed = bottom_edge + rock_height;

        if highest_row_needed > self.cavern.len() {
            let extra_rows_needed = bottom_edge + rock_height - 1 - self.cavern.len();

            for _ in 0..=extra_rows_needed {
                self.cavern.push(['.', '.', '.', '.', '.', '.', '.']);
            }
        }

        for (y, rock_row) in rock_cells.iter().enumerate() {
            for (x, cell) in rock_row.chars().enumerate() {
                if cell == '#' {
                    self.cavern[y + bottom_edge as usize][x + left_edge as usize] = cell;
                }
            }
        }
    }

    /// Returns true if any rocky cell of the 'rock' passed is in the same position as a rock
    /// within this `Chamber`.
    fn overlaps(&self, rock: RockShape, left_edge: WidthType, bottom_edge: HeightType) -> bool {
        let rock_cells = match rock {
            RockShape::HorizontalLine => ROCK_HORIZONTAL_LINE.to_vec(),
            RockShape::Plus => ROCK_PLUS.to_vec(),
            RockShape::L => ROCK_L.to_vec(),
            RockShape::VerticalLine => ROCK_VERTICAL_LINE.to_vec(),
            RockShape::Square => ROCK_SQUARE.to_vec(),
        };

        let chamber_height = self.cavern.len();

        for (y, rock_row) in rock_cells.iter().enumerate() {
            if y + bottom_edge as usize >= chamber_height {
                break;
            }

            for (x, cell) in rock_row.chars().enumerate() {
                let offset_x = x + left_edge as usize;
                if offset_x >= CHAMBER_WIDTH as usize {
                    return true;
                }

                if cell == '#' && self.cavern[y + bottom_edge as usize][offset_x] == '#' {
                    return true;
                }
            }
        }

        false
    }

    /// Returns the index of the lowest empty row. Returns 0 if the chamber is completely
    /// empty.
    fn lowest_empty_row(&self) -> usize {
        let top_row = self.cavern.len();

        if top_row == 0 {
            return 0;
        }

        for row_index in 0..top_row {
            if self.cavern[row_index] == ['.', '.', '.', '.', '.', '.', '.'] {
                return row_index - 1;
            }
        }

        top_row
    }
}

/// Holds the shape, horizontal offset and bottom row of a falling rock. 'horizontal_offset' is
/// specified as the column that the leftmost part of the rock occupies. 'bottom row' is the row
/// occupied by the lowest part of the rock (i.e., nearest the floor).
#[derive(Clone, Debug, PartialEq)]
struct FallingRock {
    shape: RockShape,
    left_edge: WidthType,
    bottom_edge: HeightType,
}

impl FallingRock {
    /// Creates a new falling rock of the given shape whose lowest part (meaning nearest the floor)
    /// is 'bottom_edge'. As per the challenge, the left edge of the rock begins two units in from
    /// the `Chamber`'s left wall.
    fn new(shape: RockShape, bottom_edge: HeightType) -> Self {
        Self {
            shape,
            left_edge: 2,
            bottom_edge,
        }
    }

    /// Moves this `FallingRock` object one unit to the left, providing this does not result in
    /// colliding with an existing rock in the 'chamber', or the chamber's left wall. If there is a
    /// collision, make no changes to the position of this `FallingRock`.
    fn move_left(&mut self, chamber: &Chamber) {
        if self.left_edge > 0 {
            if !chamber.overlaps(self.shape, self.left_edge - 1, self.bottom_edge) {
                self.left_edge -= 1;
            }
        }
    }

    /// Moves this `FallingRock` object one unit to the right, following the same process as
    /// explained for 'move_left'.
    fn move_right(&mut self, chamber: &Chamber) {
        let shape_width = match self.shape {
            RockShape::HorizontalLine => ROCK_HORIZONTAL_LINE[0].len(),
            RockShape::Plus => ROCK_PLUS[0].len(),
            RockShape::L => ROCK_L[0].len(),
            RockShape::VerticalLine => ROCK_VERTICAL_LINE[0].len(),
            RockShape::Square => ROCK_SQUARE[0].len(),
        };

        if self.left_edge as usize + shape_width < CHAMBER_WIDTH as usize {
            if !chamber.overlaps(self.shape, self.left_edge + 1, self.bottom_edge) {
                self.left_edge += 1;
            }
        }
    }

    /// Moves this `FallingRock` object one unit down, providing this does not result in colliding
    /// with an existing rock in the 'chamber' or reaching the floor.
    /// Returns `true` if the move was successful.
    fn move_down(&mut self, chamber: &Chamber) -> bool {
        if self.bottom_edge == 0 {
            return false;
        }

        if chamber.overlaps(self.shape, self.left_edge, self.bottom_edge - 1) {
            return false;
        }

        self.bottom_edge -= 1;
        true
    }

    fn place(self, chamber: &mut Chamber) {
        chamber.put_rock(self.shape, self.left_edge, self.bottom_edge);
    }
}

/// Models the movement of the `FallingRock` as it gets pushed horizontally by the jets and falls
/// due to gravity until it comes to rest on rocks that have already settled in the `Chamber` or
/// on the chamber's floor. 'chamber' is updated with the final resting place of the rock, and the
/// rock object passed is consumed by this operation. The `jets` object is modified as jet data is
/// read from it, as required by the challenge.
fn land_one_rock(chamber: &mut Chamber, mut rock: FallingRock, jets: &mut Cycle<Chars>) {
    loop {
        match jets.next().unwrap() {
            '<' => {
                rock.move_left(&chamber);
            }
            '>' => {
                rock.move_right(&chamber);
            }
            _ => {
                panic!("Unexpected character found in input");
            }
        }

        if !rock.move_down(&chamber) {
            rock.place(chamber);
            return;
        }
    }
}

/// Creates a new `Chamber` and returns its state after modeling `count` rocks falling and coming
/// to rest. `input` is the single line of characters representing the configuration of jets that
/// push the rocks horizontally as they fall.
fn do_challenge(input: &str, count: usize) -> Chamber {
    let mut chamber = Chamber::new();
    let mut jets = input.chars().cycle();
    let mut rock_shapes = ROCK_SHAPE_ORDER.iter().cycle();

    for _ in 0..count {
        let falling_rock =
            FallingRock::new(*rock_shapes.next().unwrap(), chamber.lowest_empty_row() + 3);

        land_one_rock(&mut chamber, falling_rock, &mut jets);
    }

    chamber
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let chamber = do_challenge(&input_file.trim(), REPETITIONS);

    println!(
        "The number of rows in the cavern containing rocks is {}",
        chamber.lowest_empty_row(),
    );
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_rocks_cycling() {
        let mut rocks: Cycle<_> = ROCK_SHAPE_ORDER.iter().cycle();

        assert_eq!(rocks.next(), Some(&RockShape::HorizontalLine));
        assert_eq!(rocks.next(), Some(&RockShape::Plus));
        assert_eq!(rocks.next(), Some(&RockShape::L));
        assert_eq!(rocks.next(), Some(&RockShape::VerticalLine));
        assert_eq!(rocks.next(), Some(&RockShape::Square));
        assert_eq!(rocks.next(), Some(&RockShape::HorizontalLine));
        assert_eq!(rocks.next(), Some(&RockShape::Plus));
        assert_eq!(rocks.next(), Some(&RockShape::L));
    }

    #[test]
    fn test_input_cycling() {
        let mut input: Cycle<Chars> = INPUT.chars().cycle();

        assert_eq!(input.next(), Some('>'));
        assert_eq!(input.nth(37), Some('>')); // Get penultimate char in INPUT
        assert_eq!(input.next(), Some('>'));
        assert_eq!(input.next(), Some('>')); // Wrap around to the first char in INPUT
        assert_eq!(input.next(), Some('>'));
        assert_eq!(input.next(), Some('>'));
        assert_eq!(input.next(), Some('<'));
    }

    #[test]
    fn test_empty_chamber_display() {
        let chamber = Chamber::new();
        assert_eq!(&format!("{}", chamber), &"+-------+");
    }

    #[test]
    fn test_chamber_display() {
        let mut chamber = Chamber::new();
        chamber.put_rock(RockShape::L, 3, 0);
        let result = format!("{}", chamber);
        let result_lines: Vec<_> = result.lines().rev().collect();
        assert_eq!(&result_lines[3], &"|.....#.|");
        assert_eq!(&result_lines[2], &"|.....#.|");
        assert_eq!(&result_lines[1], &"|...###.|");
        assert_eq!(&result_lines[0], &"+-------+");
    }

    #[test]
    fn test_lowest_empty_row() {
        let mut chamber = Chamber::new();
        assert_eq!(chamber.lowest_empty_row(), 0);

        chamber.put_rock(RockShape::L, 3, 0);
        assert_eq!(chamber.lowest_empty_row(), 3);
    }

    #[test]
    fn test_overlaps() {
        let empty_chamber = Chamber::new();
        assert_eq!(empty_chamber.overlaps(RockShape::Plus, 2, 3), false);

        let mut chamber_vertical_rock = Chamber::new();
        chamber_vertical_rock.put_rock(RockShape::VerticalLine, 4, 0);
        assert_eq!(chamber_vertical_rock.overlaps(RockShape::Plus, 0, 0), false);
        assert_eq!(chamber_vertical_rock.overlaps(RockShape::Plus, 1, 0), false);
        assert_eq!(chamber_vertical_rock.overlaps(RockShape::Plus, 2, 0), true);
        assert_eq!(chamber_vertical_rock.overlaps(RockShape::Plus, 2, 2), true);
        assert_eq!(chamber_vertical_rock.overlaps(RockShape::Plus, 2, 3), false);
    }

    #[test]
    fn test_move_left() {
        let chamber = Chamber::new();
        let mut falling_rock = FallingRock::new(RockShape::Plus, 2);

        falling_rock.move_left(&chamber);
        assert_eq!(
            falling_rock,
            FallingRock {
                shape: RockShape::Plus,
                left_edge: 1,
                bottom_edge: 2,
            }
        );

        falling_rock.move_left(&chamber);
        assert_eq!(
            falling_rock,
            FallingRock {
                shape: RockShape::Plus,
                left_edge: 0,
                bottom_edge: 2,
            }
        );

        falling_rock.move_left(&chamber);
        assert_eq!(
            falling_rock,
            FallingRock {
                shape: RockShape::Plus,
                left_edge: 0,
                bottom_edge: 2,
            }
        );
    }

    #[test]
    fn test_move_right() {
        let chamber = Chamber::new();
        let mut falling_rock = FallingRock::new(RockShape::Plus, 3);

        falling_rock.move_right(&chamber);
        assert_eq!(
            falling_rock,
            FallingRock {
                shape: RockShape::Plus,
                left_edge: 3,
                bottom_edge: 3,
            }
        );

        falling_rock.move_right(&chamber);
        assert_eq!(
            falling_rock,
            FallingRock {
                shape: RockShape::Plus,
                left_edge: 4,
                bottom_edge: 3,
            }
        );

        falling_rock.move_right(&chamber);
        assert_eq!(
            falling_rock,
            FallingRock {
                shape: RockShape::Plus,
                left_edge: 4,
                bottom_edge: 3,
            }
        );
    }

    #[test]
    fn test_move_down() {
        let mut chamber = Chamber::new();
        let mut falling_rock_0 = FallingRock::new(RockShape::L, 2);

        assert!(falling_rock_0.move_down(&chamber));
        assert_eq!(
            falling_rock_0,
            FallingRock {
                shape: RockShape::L,
                left_edge: 2,
                bottom_edge: 1,
            }
        );

        assert!(falling_rock_0.move_down(&chamber));
        assert_eq!(
            falling_rock_0,
            FallingRock {
                shape: RockShape::L,
                left_edge: 2,
                bottom_edge: 0,
            }
        );

        assert!(!falling_rock_0.move_down(&chamber));
        assert_eq!(
            falling_rock_0,
            FallingRock {
                shape: RockShape::L,
                left_edge: 2,
                bottom_edge: 0,
            }
        );

        falling_rock_0.place(&mut chamber);
        let mut falling_rock_1 = FallingRock::new(RockShape::Plus, 4);

        assert!(falling_rock_1.move_down(&chamber));
        assert_eq!(
            falling_rock_1,
            FallingRock {
                shape: RockShape::Plus,
                left_edge: 2,
                bottom_edge: 3,
            }
        );

        assert!(falling_rock_1.move_down(&chamber));
        assert_eq!(
            falling_rock_1,
            FallingRock {
                shape: RockShape::Plus,
                left_edge: 2,
                bottom_edge: 2,
            }
        );

        assert!(!falling_rock_1.move_down(&chamber));
        assert_eq!(
            falling_rock_1,
            FallingRock {
                shape: RockShape::Plus,
                left_edge: 2,
                bottom_edge: 2,
            }
        );

        falling_rock_1.place(&mut chamber);
        let result = format!("{}", chamber);
        let result_lines: Vec<_> = result.lines().rev().collect();

        assert_eq!(&result_lines[5], &"|...#...|");
        assert_eq!(&result_lines[4], &"|..###..|");
        assert_eq!(&result_lines[3], &"|...##..|");
        assert_eq!(&result_lines[2], &"|....#..|");
        assert_eq!(&result_lines[1], &"|..###..|");
        assert_eq!(&result_lines[0], &"+-------+");
    }

    #[test]
    fn test_land_one_rock() {
        let mut jets: Cycle<Chars> = INPUT.chars().cycle();
        let mut chamber = Chamber::new();
        let falling_rock_0 = FallingRock::new(RockShape::HorizontalLine, 3);

        // Clone the chamber and falling rock to generate a snapshot of the current situation
        // which can be compared with that expected in the challenge.
        let mut test_chamber_0 = chamber.clone();
        falling_rock_0.clone().place(&mut test_chamber_0);
        let result_t0 = format!("{}", test_chamber_0);
        let result_lines_t0: Vec<_> = result_t0.lines().rev().collect();
        assert_eq!(&result_lines_t0[4], &"|..####.|");
        assert_eq!(&result_lines_t0[3], &"|.......|");
        assert_eq!(&result_lines_t0[2], &"|.......|");
        assert_eq!(&result_lines_t0[1], &"|.......|");
        assert_eq!(&result_lines_t0[0], &"+-------+");

        land_one_rock(&mut chamber, falling_rock_0, &mut jets);
        let result_t1 = format!("{}", chamber);
        let result_lines_t1: Vec<_> = result_t1.lines().rev().collect();
        assert_eq!(&result_lines_t1[1], &"|..####.|");
        assert_eq!(&result_lines_t1[0], &"+-------+");

        let falling_rock_1 = FallingRock::new(RockShape::Plus, 4);
        land_one_rock(&mut chamber, falling_rock_1, &mut jets);
        let result_t2 = format!("{}", chamber);
        let result_lines_t2: Vec<_> = result_t2.lines().rev().collect();
        assert_eq!(&result_lines_t2[4], &"|...#...|");
        assert_eq!(&result_lines_t2[3], &"|..###..|");
        assert_eq!(&result_lines_t2[2], &"|...#...|");
        assert_eq!(&result_lines_t2[1], &"|..####.|");
        assert_eq!(&result_lines_t2[0], &"+-------+");
    }

    #[test]
    fn test_do_challenge_2() {
        let chamber = do_challenge(&INPUT, 2);
        let result = format!("{}", chamber);
        let result_lines: Vec<_> = result.lines().rev().collect();

        assert_eq!(&result_lines[4], &"|...#...|");
        assert_eq!(&result_lines[3], &"|..###..|");
        assert_eq!(&result_lines[2], &"|...#...|");
        assert_eq!(&result_lines[1], &"|..####.|");
        assert_eq!(&result_lines[0], &"+-------+");
    }

    #[test]
    fn test_do_challenge_3() {
        let chamber = do_challenge(&INPUT, 3);
        let result = format!("{}", chamber);
        let result_lines: Vec<_> = result.lines().rev().collect();

        assert_eq!(&result_lines[6], &"|..#....|");
        assert_eq!(&result_lines[5], &"|..#....|");
        assert_eq!(&result_lines[4], &"|####...|");
        assert_eq!(&result_lines[3], &"|..###..|");
        assert_eq!(&result_lines[2], &"|...#...|");
        assert_eq!(&result_lines[1], &"|..####.|");
        assert_eq!(&result_lines[0], &"+-------+");
    }

    #[test]
    fn test_do_challenge_4() {
        let chamber = do_challenge(&INPUT, 4);
        let result = format!("{}", chamber);
        let result_lines: Vec<_> = result.lines().rev().collect();

        assert_eq!(&result_lines[7], &"|....#..|");
        assert_eq!(&result_lines[6], &"|..#.#..|");
        assert_eq!(&result_lines[5], &"|..#.#..|");
        assert_eq!(&result_lines[4], &"|#####..|");
        assert_eq!(&result_lines[3], &"|..###..|");
        assert_eq!(&result_lines[2], &"|...#...|");
        assert_eq!(&result_lines[1], &"|..####.|");
        assert_eq!(&result_lines[0], &"+-------+");
    }

    #[test]
    fn test_do_challenge_5() {
        let chamber = do_challenge(&INPUT, 5);
        let result = format!("{}", chamber);
        let result_lines: Vec<_> = result.lines().rev().collect();

        assert_eq!(&result_lines[9], &"|....##.|");
        assert_eq!(&result_lines[8], &"|....##.|");
        assert_eq!(&result_lines[7], &"|....#..|");
        assert_eq!(&result_lines[6], &"|..#.#..|");
        assert_eq!(&result_lines[5], &"|..#.#..|");
        assert_eq!(&result_lines[4], &"|#####..|");
        assert_eq!(&result_lines[3], &"|..###..|");
        assert_eq!(&result_lines[2], &"|...#...|");
        assert_eq!(&result_lines[1], &"|..####.|");
        assert_eq!(&result_lines[0], &"+-------+");
    }

    #[test]
    fn test_do_challenge_6() {
        let chamber = do_challenge(&INPUT, 6);
        let result = format!("{}", chamber);
        let result_lines: Vec<_> = result.lines().rev().collect();

        assert_eq!(&result_lines[10], &"|.####..|");
        assert_eq!(&result_lines[9], &"|....##.|");
        assert_eq!(&result_lines[8], &"|....##.|");
        assert_eq!(&result_lines[7], &"|....#..|");
        assert_eq!(&result_lines[6], &"|..#.#..|");
        assert_eq!(&result_lines[5], &"|..#.#..|");
        assert_eq!(&result_lines[4], &"|#####..|");
        assert_eq!(&result_lines[3], &"|..###..|");
        assert_eq!(&result_lines[2], &"|...#...|");
        assert_eq!(&result_lines[1], &"|..####.|");
        assert_eq!(&result_lines[0], &"+-------+");
    }

    #[test]
    fn test_do_challenge_7() {
        let chamber = do_challenge(&INPUT, 7);
        let result = format!("{}", chamber);
        let result_lines: Vec<_> = result.lines().rev().collect();

        assert_eq!(&result_lines[13], &"|..#....|");
        assert_eq!(&result_lines[12], &"|.###...|");
        assert_eq!(&result_lines[11], &"|..#....|");
        assert_eq!(&result_lines[10], &"|.####..|");
        assert_eq!(&result_lines[9], &"|....##.|");
        assert_eq!(&result_lines[8], &"|....##.|");
        assert_eq!(&result_lines[7], &"|....#..|");
        assert_eq!(&result_lines[6], &"|..#.#..|");
        assert_eq!(&result_lines[5], &"|..#.#..|");
        assert_eq!(&result_lines[4], &"|#####..|");
        assert_eq!(&result_lines[3], &"|..###..|");
        assert_eq!(&result_lines[2], &"|...#...|");
        assert_eq!(&result_lines[1], &"|..####.|");
        assert_eq!(&result_lines[0], &"+-------+");
    }

    #[test]
    fn test_do_challenge_8() {
        let chamber = do_challenge(&INPUT, 8);
        let result = format!("{}", chamber);
        let result_lines: Vec<_> = result.lines().rev().collect();

        assert_eq!(&result_lines[15], &"|.....#.|");
        assert_eq!(&result_lines[14], &"|.....#.|");
        assert_eq!(&result_lines[13], &"|..####.|");
        assert_eq!(&result_lines[12], &"|.###...|");
        assert_eq!(&result_lines[11], &"|..#....|");
        assert_eq!(&result_lines[10], &"|.####..|");
        assert_eq!(&result_lines[9], &"|....##.|");
        assert_eq!(&result_lines[8], &"|....##.|");
        assert_eq!(&result_lines[7], &"|....#..|");
        assert_eq!(&result_lines[6], &"|..#.#..|");
        assert_eq!(&result_lines[5], &"|..#.#..|");
        assert_eq!(&result_lines[4], &"|#####..|");
        assert_eq!(&result_lines[3], &"|..###..|");
        assert_eq!(&result_lines[2], &"|...#...|");
        assert_eq!(&result_lines[1], &"|..####.|");
        assert_eq!(&result_lines[0], &"+-------+");
    }

    #[test]
    fn test_do_challenge_9() {
        let chamber = do_challenge(&INPUT, 9);
        let result = format!("{}", chamber);
        let result_lines: Vec<_> = result.lines().rev().collect();

        assert_eq!(&result_lines[17], &"|....#..|");
        assert_eq!(&result_lines[16], &"|....#..|");
        assert_eq!(&result_lines[15], &"|....##.|");
        assert_eq!(&result_lines[14], &"|....##.|");
        assert_eq!(&result_lines[13], &"|..####.|");
        assert_eq!(&result_lines[12], &"|.###...|");
        assert_eq!(&result_lines[11], &"|..#....|");
        assert_eq!(&result_lines[10], &"|.####..|");
        assert_eq!(&result_lines[9], &"|....##.|");
        assert_eq!(&result_lines[8], &"|....##.|");
        assert_eq!(&result_lines[7], &"|....#..|");
        assert_eq!(&result_lines[6], &"|..#.#..|");
        assert_eq!(&result_lines[5], &"|..#.#..|");
        assert_eq!(&result_lines[4], &"|#####..|");
        assert_eq!(&result_lines[3], &"|..###..|");
        assert_eq!(&result_lines[2], &"|...#...|");
        assert_eq!(&result_lines[1], &"|..####.|");
        assert_eq!(&result_lines[0], &"+-------+");
    }

    #[test]
    fn test_do_challenge_10() {
        let chamber = do_challenge(&INPUT, 10);
        let result = format!("{}", chamber);
        let result_lines: Vec<_> = result.lines().rev().collect();

        assert_eq!(&result_lines[17], &"|....#..|");
        assert_eq!(&result_lines[16], &"|....#..|");
        assert_eq!(&result_lines[15], &"|....##.|");
        assert_eq!(&result_lines[14], &"|##..##.|");
        assert_eq!(&result_lines[13], &"|######.|");
        assert_eq!(&result_lines[12], &"|.###...|");
        assert_eq!(&result_lines[11], &"|..#....|");
        assert_eq!(&result_lines[10], &"|.####..|");
        assert_eq!(&result_lines[9], &"|....##.|");
        assert_eq!(&result_lines[8], &"|....##.|");
        assert_eq!(&result_lines[7], &"|....#..|");
        assert_eq!(&result_lines[6], &"|..#.#..|");
        assert_eq!(&result_lines[5], &"|..#.#..|");
        assert_eq!(&result_lines[4], &"|#####..|");
        assert_eq!(&result_lines[3], &"|..###..|");
        assert_eq!(&result_lines[2], &"|...#...|");
        assert_eq!(&result_lines[1], &"|..####.|");
        assert_eq!(&result_lines[0], &"+-------+");
    }

    #[test]
    fn test_do_challenge() {
        let chamber = do_challenge(&INPUT, 2022);

        assert_eq!(chamber.lowest_empty_row(), 3068);
    }
}
