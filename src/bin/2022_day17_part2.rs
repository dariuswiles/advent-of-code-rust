//! Advent of Code 2022 Day 17
//! https://adventofcode.com/2022/day/17
//!
//! Challenge part 2
//!
//! Determine the height of a stack of differently shaped rocks that are pushed left and right as
//! they fall before coming to rest. Part 2 of the challenge massively increases the number of
//! rocks that need to be simulated, requiring code that differs significantly from part 1. It is
//! based on finding a repeating pattern that forms because the output is based on the two inputs
//! which are endlessly cycled through.

use std::collections::HashSet;
use std::fmt::{self, Display};
use std::fs;
use std::iter::Iterator;
use std::ops::RangeInclusive;

type WidthType = u8;
type HeightType = usize;
type RowChar = [char; CHAMBER_WIDTH as usize];

const INPUT_FILENAME: &str = "2022_day17_input.txt";
const CHAMBER_WIDTH: WidthType = 7;
const ROCK_SHAPE_COUNT: usize = 5;
const REPETITIONS: usize = 1_000_000_000_000;

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

const ROCK_SHAPE_ORDER: [RockShape; ROCK_SHAPE_COUNT] = [
    RockShape::HorizontalLine,
    RockShape::Plus,
    RockShape::L,
    RockShape::VerticalLine,
    RockShape::Square,
];

#[derive(Clone, Copy, Debug, PartialEq)]
enum RockShape {
    HorizontalLine,
    Plus,
    L,
    VerticalLine,
    Square,
}

impl RockShape {
    /// Returns the `RockShape` at the given `rock_id` index. The modulus of the index is taken
    /// such that RockShape::HorizontalLine is return for indexes 0, 5, 10, 15, etc.
    fn lookup(rock_id: usize) -> Self {
        ROCK_SHAPE_ORDER[rock_id % ROCK_SHAPE_COUNT]
    }
}

/// An infinite iterator of jet directions created from a string representation (as specified in
/// the challenge).
struct JetIterator {
    jets: Vec<char>,
    jet_index: usize,
}

impl JetIterator {
    /// Returns a new infinite iterator over all jet directions in `jets_str`, starting with the
    /// first direction in the string.
    fn new(jets_str: &str) -> Self {
        let jets: Vec<char> = jets_str.trim().chars().collect();
        let len = jets.len();
        Self {
            jets,
            jet_index: len - 1,
        }
    }
}

impl Iterator for JetIterator {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.jet_index = (self.jet_index + 1) % self.jets.len();

        Some(self.jets[self.jet_index])
    }
}

/// Holds the contents of a single row of the cavern and the ids of the `Rock`s that have at least
/// one cell in this row.
#[derive(Clone, Debug, PartialEq)]
struct Row {
    contents: RowChar,
    rock_ids: HashSet<usize>,
}

impl Row {
    /// Returns a new `Row` containing the `Vec` of `char`s passed in `row_chars` and an empty
    /// `HashSet` to hold the ids of the `Rock`s that have at least one cell in this row.
    fn new(row_chars: RowChar) -> Self {
        Self {
            contents: row_chars,
            rock_ids: HashSet::new(),
        }
    }
}

/// Holds the empty space and at-rest rocks in the `Chamber`'s cavern. The rows are indexed with
/// the lowest empty row being index 0.
#[derive(Clone)]
struct Chamber {
    cavern: Vec<Row>,
}

impl Display for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cavern_row_count = self.cavern.len();

        let top_rows = if cavern_row_count > 20 {
            &self.cavern[cavern_row_count - 20..]
        } else {
            &self.cavern[..]
        };

        for row in top_rows.iter().rev() {
            _ = writeln!(f, "|{}|", row.contents.iter().collect::<String>());
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
    fn put_rock(&mut self, rock_id: usize, left_edge: WidthType, bottom_edge: HeightType) {
        let rock = RockShape::lookup(rock_id);

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
                self.cavern
                    .push(Row::new(['.', '.', '.', '.', '.', '.', '.']));
            }
        }

        for (y, rock_row) in rock_cells.iter().enumerate() {
            for (x, cell) in rock_row.chars().enumerate() {
                if cell == '#' {
                    self.cavern[y + bottom_edge].contents[x + left_edge as usize] = cell;
                    self.cavern[y + bottom_edge].rock_ids.insert(rock_id);
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
            if y + bottom_edge >= chamber_height {
                break;
            }

            for (x, cell) in rock_row.chars().enumerate() {
                let offset_x = x + left_edge as usize;
                if offset_x >= CHAMBER_WIDTH as usize {
                    return true;
                }

                if cell == '#' && self.cavern[y + bottom_edge].contents[offset_x] == '#' {
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

        for row_index in (0..top_row).rev() {
            if self.cavern[row_index].rock_ids.is_empty() {
                return row_index + 1;
            }
        }

        top_row
    }

    /// Returns the highest row containing a part of `rock_id`, or `None` if `rock_id` is not
    /// found.
    fn highest_row_for_rock(&self, rock_id: usize) -> Option<usize> {
        (0..self.cavern.len())
            .rev()
            .find(|&row| self.cavern[row].rock_ids.contains(&rock_id))
    }
}

/// Holds the shape, horizontal offset and bottom row of a falling rock. 'horizontal_offset' is
/// specified as the column that the leftmost part of the rock occupies. 'bottom row' is the row
/// occupied by the lowest part of the rock (i.e., nearest the floor).
#[derive(Clone, Debug, PartialEq)]
struct FallingRock {
    rock_id: usize,
    shape: RockShape,
    left_edge: WidthType,
    bottom_edge: HeightType,
}

impl FallingRock {
    /// Creates a new falling rock of the given shape whose lowest part (meaning nearest the floor)
    /// is 'bottom_edge'. As per the challenge, the left edge of the rock begins two units in from
    /// the `Chamber`'s left wall.
    fn new(rock_id: usize, shape: RockShape, bottom_edge: HeightType) -> Self {
        Self {
            rock_id,
            shape,
            left_edge: 2,
            bottom_edge,
        }
    }

    /// Moves this `FallingRock` object one unit to the left, providing this does not result in
    /// colliding with an existing rock in the 'chamber', or the chamber's left wall. If there is a
    /// collision, make no changes to the position of this `FallingRock`.
    fn move_left(&mut self, chamber: &Chamber) {
        if self.left_edge > 0 && !chamber.overlaps(self.shape, self.left_edge - 1, self.bottom_edge)
        {
            self.left_edge -= 1;
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

        if self.left_edge as usize + shape_width < CHAMBER_WIDTH as usize
            && !chamber.overlaps(self.shape, self.left_edge + 1, self.bottom_edge)
        {
            self.left_edge += 1;
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

    /// Places this rock within the given `chamber`.
    fn place(self, chamber: &mut Chamber) {
        chamber.put_rock(self.rock_id, self.left_edge, self.bottom_edge);
    }
}

/// Models the movement of the `FallingRock` as it gets pushed horizontally by the jets and falls
/// due to gravity until it comes to rest on rocks that have already settled in the `Chamber` or
/// on the chamber's floor. 'chamber' is updated with the final resting place of the rock, and the
/// rock object passed is consumed by this operation. The `jets` object is modified as jet data is
/// read from it, as required by the challenge.
///
/// Returns the row the bottom edge of the rock came to rest in.
fn land_one_rock(chamber: &mut Chamber, mut rock: FallingRock, jets: &mut JetIterator) -> usize {
    loop {
        match jets.next().unwrap() {
            '<' => {
                rock.move_left(chamber);
            }
            '>' => {
                rock.move_right(chamber);
            }
            _ => {
                panic!("Unexpected character found in input");
            }
        }

        if !rock.move_down(chamber) {
            let bottom_edge = rock.bottom_edge;
            rock.place(chamber);
            return bottom_edge;
        }
    }
}

/// Models the fall of each rock defined in the challenge until each comes to rest. Rocks are given
/// an incrementing identifier, where the first rock to fall (a horizontal line), is 0. `count`
/// is the total number of rocks to model, so 1 models a single falling rock.
///
/// The return value contains a `Vec` of the rows in `chamber` that had horizontal line rocks
/// added as part of the additions of rocks.
fn land_multiple_rocks(chamber: &mut Chamber, count: usize, jets: &mut JetIterator) -> Vec<usize> {
    let mut horizontal_line_row_ids = Vec::new();
    for rock_id in 0..count {
        let lowest_empty_row = chamber.lowest_empty_row();
        let rock_shape = RockShape::lookup(rock_id);
        let falling_rock = FallingRock::new(rock_id, rock_shape, lowest_empty_row + 3);

        let bottom_edge = land_one_rock(chamber, falling_rock, jets);

        if rock_shape == RockShape::HorizontalLine {
            horizontal_line_row_ids.push(bottom_edge);
        }
    }

    horizontal_line_row_ids
}

/// Looks for repeating blocks of rows in the `Chamber` passed, starting with the first row (i.e.,
/// the bottom row). `horizontal_line_row_ids` are the indexes of the rows containing horizontal
/// line shapes. This is needed as only repeating patterns with a horizontal row as their lowest
/// shape is searched for. For performance reasons, only blocks up to `window` rows are searched
/// for.
///
/// If a repeating block is found, its inclusive row range (inclusive) is returned,
/// e.g., a return value of 5..=9 means rows 5, 6, 7, 8 and 9 are identical to rows 10, 11, 12, 13
/// and 14 respectively.
///
/// # Panics
///
/// Panics if no repeating pattern is found.
fn find_repeating_pattern(
    chamber: &Chamber,
    horizontal_line_row_ids: &Vec<usize>,
    window: usize,
) -> RangeInclusive<usize> {
    let mut match_original = None;
    let mut match_dupe = None;
    let top_row = chamber.cavern.len();
    'outer: for earlier in horizontal_line_row_ids {
        let mut matching_rows = 0;
        for later in earlier + 1..usize::min(window * 3, top_row) {
            if chamber.cavern[earlier + matching_rows].contents == chamber.cavern[later].contents {
                matching_rows += 1;

                if matching_rows == window {
                    match_original = Some(*earlier);
                    match_dupe = Some(later - window);
                    break 'outer;
                }
            } else {
                matching_rows = 0;
            }
        }
    }

    if match_original.is_none() {
        panic!("Failed to find a match :(");
    }

    match_original.unwrap()..=match_dupe.unwrap()
}

/// Returns the id of the horizontal rock that's on row `row_num`. This is determined by looking in
/// `horizontal_line_row_ids`, which contains the row number that every horizontal rock came to
/// rest within.
///
/// # Panics
///
/// Panics if the given row contains no horizontal rock or `row_num` is outside the range of
/// `horizontal_line_row_ids`.
fn get_rock_id(horizontal_line_row_ids: &[usize], row_num: usize) -> usize {
    horizontal_line_row_ids
        .binary_search(&row_num)
        .expect("Internal error: cannot find a horizontal rock on row {row_num}")
        * ROCK_SHAPE_COUNT
}

/// Creates a new `Chamber` and models `count` `Rock`s falling and coming to rest in its cavern.
/// `input` is the single line of characters representing the configuration of jets that push the
/// rocks horizontally as they fall.
///
/// Returns the total height of the stack of `Rock`s after `count` `Rock`s have fallen.
///
/// # Panics
///
/// Panics if a repeating pattern of `Rock`s cannot be found in the `Chamber` as this is necessary
/// to generate an answer for part 2 in a reasonable time.
//
// Modelling the number of falling `Rock`s required by part 2 of the challenge would take too long,
// so a repeating block of `Rock`s is searched for and then used to mathematically determine the
// height.
fn do_challenge(input: &str, count: usize) -> usize {
    let mut chamber = Chamber::new();
    let mut jets = JetIterator::new(input);

    // The maximum number of rocks that need simulating for a pattern to develop.
    let cycle_period = jets.jets.len() * ROCK_SHAPE_ORDER.len();

    // Simulate enough falling rocks for a pattern to develop.
    let horizontal_line_row_ids = land_multiple_rocks(&mut chamber, cycle_period, &mut jets);

    // Look for repeating patterns in the first rows of settled `Rock`s.
    let repeating_range = find_repeating_pattern(&chamber, &horizontal_line_row_ids, cycle_period);

    // The rock ids of the rocks at the start and end of the repeating pattern.
    let repeat_rock_id_start = get_rock_id(&horizontal_line_row_ids, *repeating_range.start());
    let repeat_rock_id_end = get_rock_id(&horizontal_line_row_ids, *repeating_range.end() + 1);

    // The size of the repeating pattern in both rows and rocks.
    let repeat_size_rows = repeating_range.end() + 1 - repeating_range.start();
    let repeat_size_rocks = repeat_rock_id_end - repeat_rock_id_start;

    // The number of times the repeating block can be repeated in its entirety before we reach the
    // desired number of simulated rocks.
    let number_of_repeats = (count - repeat_rock_id_start) / repeat_size_rocks;

    // It is likely that there will be some remaining rocks that need to be modelled, after the
    // last full repeating block. For example, if `count` is 100 and the repeating pattern begins
    // on rock 12 and is 10 rocks long, the last repeating block ends with rock 92. There will
    // therefore be 7 rocks left to simulate, i.e., rocks 93 to 99 inclusive (remembering that
    // rock ids begin at 0, so rocks 0..=99 are being simulated).
    //
    // The "-1" in the formula is because rock identifiers begin at 0.
    let rocks_in_partial_cycle =
        count - 1 - repeat_rock_id_start - number_of_repeats * repeat_size_rocks;

    // The rock in the repeating block that corresponds to the last rock in `count`. For example,
    // rock id 19 could represent rock id 99.
    let final_rock_id = repeat_rock_id_start + rocks_in_partial_cycle;

    // The highest row of the rock described above.
    let highest_row_of_final_rock = chamber.highest_row_for_rock(final_rock_id).unwrap();

    // The number of rows created by the rocks that are at the very top of the stack of rocks,
    // above the rocks added as part of the repeating block.
    let partial_cycle_row_count = highest_row_of_final_rock - repeating_range.start();

    // The answer is the sum of:
    //   - the rows created by rocks at the bottom of the stack, before the repeating pattern
    //     starts;
    //   - the rows created by rocks in the repeating pattern, which form the bulk of the answer;
    //   - the rows created by rocks at the top of the stack, above the repeating pattern.
    repeating_range.start() + 1 + repeat_size_rows * number_of_repeats + partial_cycle_row_count
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    let answer = do_challenge(input_file.trim(), REPETITIONS);

    println!("The number of rows in the cavern containing rocks is {answer}",);
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_rocks_cycling() {
        assert_eq!(RockShape::lookup(0), (RockShape::HorizontalLine));
        assert_eq!(RockShape::lookup(1), (RockShape::Plus));
        assert_eq!(RockShape::lookup(2), (RockShape::L));
        assert_eq!(RockShape::lookup(3), (RockShape::VerticalLine));
        assert_eq!(RockShape::lookup(4), (RockShape::Square));
        assert_eq!(RockShape::lookup(5), (RockShape::HorizontalLine));
        assert_eq!(RockShape::lookup(6), (RockShape::Plus));
        assert_eq!(RockShape::lookup(7), (RockShape::L));
    }

    #[test]
    fn test_input_cycling() {
        let mut input = JetIterator::new(INPUT);

        assert_eq!(input.next(), Some('>'));
        assert_eq!(input.nth(37), Some('>')); // Get penultimate char in INPUT
        assert_eq!(input.next(), Some('>')); // Get last char in INPUT
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
        chamber.put_rock(2 /* RockShape::L */, 3, 0);
        let result = format!("{}", chamber);
        let result_lines: Vec<_> = result.lines().rev().collect();
        assert_eq!(&result_lines[3], &"|.....#.|");
        assert_eq!(&result_lines[2], &"|.....#.|");
        assert_eq!(&result_lines[1], &"|...###.|");
        assert_eq!(&result_lines[0], &"+-------+");

        assert_eq!(chamber.cavern[2].rock_ids, HashSet::from([2]));
        assert_eq!(chamber.cavern[1].rock_ids, HashSet::from([2]));
        assert_eq!(chamber.cavern[0].rock_ids, HashSet::from([2]));
    }

    #[test]
    fn test_lowest_empty_row() {
        let mut chamber = Chamber::new();
        assert_eq!(chamber.lowest_empty_row(), 0);

        chamber.put_rock(2 /* RockShape::L */, 3, 0);
        assert_eq!(chamber.lowest_empty_row(), 3);
    }

    #[test]
    fn test_overlaps() {
        let empty_chamber = Chamber::new();
        assert!(!empty_chamber.overlaps(RockShape::Plus, 2, 3));

        let mut chamber_vertical_rock = Chamber::new();
        chamber_vertical_rock.put_rock(3 /* RockShape::VerticalLine */, 4, 0);

        println!("{chamber_vertical_rock}");

        assert!(!chamber_vertical_rock.overlaps(RockShape::Plus, 0, 0));
        assert!(!chamber_vertical_rock.overlaps(RockShape::Plus, 1, 0));
        assert!(chamber_vertical_rock.overlaps(RockShape::Plus, 2, 0));
        assert!(chamber_vertical_rock.overlaps(RockShape::Plus, 2, 2));
        assert!(!chamber_vertical_rock.overlaps(RockShape::Plus, 2, 3));
    }

    #[test]
    fn test_move_left() {
        let chamber = Chamber::new();
        let mut falling_rock = FallingRock::new(1, RockShape::Plus, 2);

        falling_rock.move_left(&chamber);
        assert_eq!(
            falling_rock,
            FallingRock {
                rock_id: 1,
                shape: RockShape::Plus,
                left_edge: 1,
                bottom_edge: 2,
            }
        );

        falling_rock.move_left(&chamber);
        assert_eq!(
            falling_rock,
            FallingRock {
                rock_id: 1,
                shape: RockShape::Plus,
                left_edge: 0,
                bottom_edge: 2,
            }
        );

        falling_rock.move_left(&chamber);
        assert_eq!(
            falling_rock,
            FallingRock {
                rock_id: 1,
                shape: RockShape::Plus,
                left_edge: 0,
                bottom_edge: 2,
            }
        );
    }

    #[test]
    fn test_move_right() {
        let chamber = Chamber::new();
        let mut falling_rock = FallingRock::new(1, RockShape::Plus, 3);

        falling_rock.move_right(&chamber);
        assert_eq!(
            falling_rock,
            FallingRock {
                rock_id: 1,
                shape: RockShape::Plus,
                left_edge: 3,
                bottom_edge: 3,
            }
        );

        falling_rock.move_right(&chamber);
        assert_eq!(
            falling_rock,
            FallingRock {
                rock_id: 1,
                shape: RockShape::Plus,
                left_edge: 4,
                bottom_edge: 3,
            }
        );

        falling_rock.move_right(&chamber);
        assert_eq!(
            falling_rock,
            FallingRock {
                rock_id: 1,
                shape: RockShape::Plus,
                left_edge: 4,
                bottom_edge: 3,
            }
        );
    }

    #[test]
    fn test_move_down() {
        let mut chamber = Chamber::new();
        let mut falling_rock_0 = FallingRock::new(2, RockShape::L, 2);

        assert!(falling_rock_0.move_down(&chamber));
        assert_eq!(
            falling_rock_0,
            FallingRock {
                rock_id: 2,
                shape: RockShape::L,
                left_edge: 2,
                bottom_edge: 1,
            }
        );

        assert!(falling_rock_0.move_down(&chamber));
        assert_eq!(
            falling_rock_0,
            FallingRock {
                rock_id: 2,
                shape: RockShape::L,
                left_edge: 2,
                bottom_edge: 0,
            }
        );

        assert!(!falling_rock_0.move_down(&chamber));
        assert_eq!(
            falling_rock_0,
            FallingRock {
                rock_id: 2,
                shape: RockShape::L,
                left_edge: 2,
                bottom_edge: 0,
            }
        );

        falling_rock_0.place(&mut chamber);
        let mut falling_rock_1 = FallingRock::new(1, RockShape::Plus, 4);

        assert!(falling_rock_1.move_down(&chamber));
        assert_eq!(
            falling_rock_1,
            FallingRock {
                rock_id: 1,
                shape: RockShape::Plus,
                left_edge: 2,
                bottom_edge: 3,
            }
        );

        assert!(falling_rock_1.move_down(&chamber));
        assert_eq!(
            falling_rock_1,
            FallingRock {
                rock_id: 1,
                shape: RockShape::Plus,
                left_edge: 2,
                bottom_edge: 2,
            }
        );

        assert!(!falling_rock_1.move_down(&chamber));
        assert_eq!(
            falling_rock_1,
            FallingRock {
                rock_id: 1,
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
        let mut jets = JetIterator::new(INPUT);
        let mut chamber = Chamber::new();
        let falling_rock_0 = FallingRock::new(0, RockShape::HorizontalLine, 3);

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

        assert_eq!(chamber.cavern[0].rock_ids, HashSet::from([0]));

        let falling_rock_1 = FallingRock::new(1, RockShape::Plus, 4);
        land_one_rock(&mut chamber, falling_rock_1, &mut jets);
        let result_t2 = format!("{}", chamber);
        let result_lines_t2: Vec<_> = result_t2.lines().rev().collect();
        assert_eq!(&result_lines_t2[4], &"|...#...|");
        assert_eq!(&result_lines_t2[3], &"|..###..|");
        assert_eq!(&result_lines_t2[2], &"|...#...|");
        assert_eq!(&result_lines_t2[1], &"|..####.|");
        assert_eq!(&result_lines_t2[0], &"+-------+");

        assert_eq!(chamber.cavern[3].rock_ids, HashSet::from([1]));
        assert_eq!(chamber.cavern[2].rock_ids, HashSet::from([1]));
        assert_eq!(chamber.cavern[1].rock_ids, HashSet::from([1]));
        assert_eq!(chamber.cavern[0].rock_ids, HashSet::from([0]));
    }

    #[test]
    fn test_land_multiple_rocks_2() {
        let mut jets = JetIterator::new(INPUT);
        let mut chamber = Chamber::new();

        land_multiple_rocks(&mut chamber, 2, &mut jets);

        let result = format!("{}", chamber);
        let result_lines: Vec<_> = result.lines().rev().collect();

        assert_eq!(&result_lines[4], &"|...#...|");
        assert_eq!(&result_lines[3], &"|..###..|");
        assert_eq!(&result_lines[2], &"|...#...|");
        assert_eq!(&result_lines[1], &"|..####.|");
        assert_eq!(&result_lines[0], &"+-------+");

        assert_eq!(chamber.cavern[3].rock_ids, HashSet::from([1]));
        assert_eq!(chamber.cavern[2].rock_ids, HashSet::from([1]));
        assert_eq!(chamber.cavern[1].rock_ids, HashSet::from([1]));
        assert_eq!(chamber.cavern[0].rock_ids, HashSet::from([0]));
    }

    #[test]
    fn test_land_multiple_rocks_3() {
        let mut jets = JetIterator::new(INPUT);
        let mut chamber = Chamber::new();
        land_multiple_rocks(&mut chamber, 3, &mut jets);

        let result = format!("{}", chamber);
        let result_lines: Vec<_> = result.lines().rev().collect();

        assert_eq!(&result_lines[6], &"|..#....|");
        assert_eq!(&result_lines[5], &"|..#....|");
        assert_eq!(&result_lines[4], &"|####...|");
        assert_eq!(&result_lines[3], &"|..###..|");
        assert_eq!(&result_lines[2], &"|...#...|");
        assert_eq!(&result_lines[1], &"|..####.|");
        assert_eq!(&result_lines[0], &"+-------+");

        assert_eq!(chamber.cavern[5].rock_ids, HashSet::from([2]));
        assert_eq!(chamber.cavern[4].rock_ids, HashSet::from([2]));
        assert_eq!(chamber.cavern[3].rock_ids, HashSet::from([1, 2]));
        assert_eq!(chamber.cavern[2].rock_ids, HashSet::from([1]));
        assert_eq!(chamber.cavern[1].rock_ids, HashSet::from([1]));
        assert_eq!(chamber.cavern[0].rock_ids, HashSet::from([0]));
    }

    #[test]
    fn test_land_multiple_rocks_4() {
        let mut jets = JetIterator::new(INPUT);
        let mut chamber = Chamber::new();
        land_multiple_rocks(&mut chamber, 4, &mut jets);

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

        assert_eq!(chamber.cavern[6].rock_ids, HashSet::from([3]));
        assert_eq!(chamber.cavern[5].rock_ids, HashSet::from([2, 3]));
        assert_eq!(chamber.cavern[4].rock_ids, HashSet::from([2, 3]));
        assert_eq!(chamber.cavern[3].rock_ids, HashSet::from([1, 2, 3]));
        assert_eq!(chamber.cavern[2].rock_ids, HashSet::from([1]));
        assert_eq!(chamber.cavern[1].rock_ids, HashSet::from([1]));
        assert_eq!(chamber.cavern[0].rock_ids, HashSet::from([0]));
    }

    #[test]
    fn test_land_multiple_rocks_5() {
        let mut jets = JetIterator::new(INPUT);
        let mut chamber = Chamber::new();
        land_multiple_rocks(&mut chamber, 5, &mut jets);

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

        assert_eq!(chamber.cavern[8].rock_ids, HashSet::from([4]));
        assert_eq!(chamber.cavern[7].rock_ids, HashSet::from([4]));
        assert_eq!(chamber.cavern[6].rock_ids, HashSet::from([3]));
        assert_eq!(chamber.cavern[5].rock_ids, HashSet::from([2, 3]));
        assert_eq!(chamber.cavern[4].rock_ids, HashSet::from([2, 3]));
        assert_eq!(chamber.cavern[3].rock_ids, HashSet::from([1, 2, 3]));
        assert_eq!(chamber.cavern[2].rock_ids, HashSet::from([1]));
        assert_eq!(chamber.cavern[1].rock_ids, HashSet::from([1]));
        assert_eq!(chamber.cavern[0].rock_ids, HashSet::from([0]));
    }

    #[test]
    fn test_land_multiple_rocks_6() {
        let mut jets = JetIterator::new(INPUT);
        let mut chamber = Chamber::new();
        land_multiple_rocks(&mut chamber, 6, &mut jets);

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

        assert_eq!(chamber.cavern[9].rock_ids, HashSet::from([5]));
        assert_eq!(chamber.cavern[8].rock_ids, HashSet::from([4]));
        assert_eq!(chamber.cavern[7].rock_ids, HashSet::from([4]));
        assert_eq!(chamber.cavern[6].rock_ids, HashSet::from([3]));
        assert_eq!(chamber.cavern[5].rock_ids, HashSet::from([2, 3]));
        assert_eq!(chamber.cavern[4].rock_ids, HashSet::from([2, 3]));
        assert_eq!(chamber.cavern[3].rock_ids, HashSet::from([1, 2, 3]));
        assert_eq!(chamber.cavern[2].rock_ids, HashSet::from([1]));
        assert_eq!(chamber.cavern[1].rock_ids, HashSet::from([1]));
        assert_eq!(chamber.cavern[0].rock_ids, HashSet::from([0]));
    }

    #[test]
    fn test_land_multiple_rocks_7() {
        let mut jets = JetIterator::new(INPUT);
        let mut chamber = Chamber::new();
        land_multiple_rocks(&mut chamber, 7, &mut jets);

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

        assert_eq!(chamber.cavern[12].rock_ids, HashSet::from([6]));
        assert_eq!(chamber.cavern[11].rock_ids, HashSet::from([6]));
        assert_eq!(chamber.cavern[10].rock_ids, HashSet::from([6]));
        assert_eq!(chamber.cavern[9].rock_ids, HashSet::from([5]));
        assert_eq!(chamber.cavern[8].rock_ids, HashSet::from([4]));
        assert_eq!(chamber.cavern[7].rock_ids, HashSet::from([4]));
        assert_eq!(chamber.cavern[6].rock_ids, HashSet::from([3]));
        assert_eq!(chamber.cavern[5].rock_ids, HashSet::from([2, 3]));
        assert_eq!(chamber.cavern[4].rock_ids, HashSet::from([2, 3]));
        assert_eq!(chamber.cavern[3].rock_ids, HashSet::from([1, 2, 3]));
        assert_eq!(chamber.cavern[2].rock_ids, HashSet::from([1]));
        assert_eq!(chamber.cavern[1].rock_ids, HashSet::from([1]));
        assert_eq!(chamber.cavern[0].rock_ids, HashSet::from([0]));
    }

    #[test]
    fn test_land_multiple_rocks_8() {
        let mut jets = JetIterator::new(INPUT);
        let mut chamber = Chamber::new();
        land_multiple_rocks(&mut chamber, 8, &mut jets);

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

        assert_eq!(chamber.cavern[14].rock_ids, HashSet::from([7]));
        assert_eq!(chamber.cavern[13].rock_ids, HashSet::from([7]));
        assert_eq!(chamber.cavern[12].rock_ids, HashSet::from([6, 7]));
        assert_eq!(chamber.cavern[11].rock_ids, HashSet::from([6]));
        assert_eq!(chamber.cavern[10].rock_ids, HashSet::from([6]));
        assert_eq!(chamber.cavern[9].rock_ids, HashSet::from([5]));
        assert_eq!(chamber.cavern[8].rock_ids, HashSet::from([4]));
        assert_eq!(chamber.cavern[7].rock_ids, HashSet::from([4]));
        assert_eq!(chamber.cavern[6].rock_ids, HashSet::from([3]));
        assert_eq!(chamber.cavern[5].rock_ids, HashSet::from([2, 3]));
        assert_eq!(chamber.cavern[4].rock_ids, HashSet::from([2, 3]));
        assert_eq!(chamber.cavern[3].rock_ids, HashSet::from([1, 2, 3]));
        assert_eq!(chamber.cavern[2].rock_ids, HashSet::from([1]));
        assert_eq!(chamber.cavern[1].rock_ids, HashSet::from([1]));
        assert_eq!(chamber.cavern[0].rock_ids, HashSet::from([0]));
    }

    #[test]
    fn test_land_multiple_rocks_9() {
        let mut jets = JetIterator::new(INPUT);
        let mut chamber = Chamber::new();
        land_multiple_rocks(&mut chamber, 9, &mut jets);

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

        assert_eq!(chamber.cavern[16].rock_ids, HashSet::from([8]));
        assert_eq!(chamber.cavern[15].rock_ids, HashSet::from([8]));
        assert_eq!(chamber.cavern[14].rock_ids, HashSet::from([7, 8]));
        assert_eq!(chamber.cavern[13].rock_ids, HashSet::from([7, 8]));
        assert_eq!(chamber.cavern[12].rock_ids, HashSet::from([6, 7]));
        assert_eq!(chamber.cavern[11].rock_ids, HashSet::from([6]));
        assert_eq!(chamber.cavern[10].rock_ids, HashSet::from([6]));
        assert_eq!(chamber.cavern[9].rock_ids, HashSet::from([5]));
        assert_eq!(chamber.cavern[8].rock_ids, HashSet::from([4]));
        assert_eq!(chamber.cavern[7].rock_ids, HashSet::from([4]));
        assert_eq!(chamber.cavern[6].rock_ids, HashSet::from([3]));
        assert_eq!(chamber.cavern[5].rock_ids, HashSet::from([2, 3]));
        assert_eq!(chamber.cavern[4].rock_ids, HashSet::from([2, 3]));
        assert_eq!(chamber.cavern[3].rock_ids, HashSet::from([1, 2, 3]));
        assert_eq!(chamber.cavern[2].rock_ids, HashSet::from([1]));
        assert_eq!(chamber.cavern[1].rock_ids, HashSet::from([1]));
        assert_eq!(chamber.cavern[0].rock_ids, HashSet::from([0]));
    }

    #[test]
    fn test_land_multiple_rocks_10() {
        let mut jets = JetIterator::new(INPUT);
        let mut chamber = Chamber::new();

        let horizontal_line_row_ids = land_multiple_rocks(&mut chamber, 10, &mut jets);
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

        assert_eq!(chamber.cavern[16].rock_ids, HashSet::from([8]));
        assert_eq!(chamber.cavern[15].rock_ids, HashSet::from([8]));
        assert_eq!(chamber.cavern[14].rock_ids, HashSet::from([7, 8]));
        assert_eq!(chamber.cavern[13].rock_ids, HashSet::from([7, 8, 9]));
        assert_eq!(chamber.cavern[12].rock_ids, HashSet::from([6, 7, 9]));
        assert_eq!(chamber.cavern[11].rock_ids, HashSet::from([6]));
        assert_eq!(chamber.cavern[10].rock_ids, HashSet::from([6]));
        assert_eq!(chamber.cavern[9].rock_ids, HashSet::from([5]));
        assert_eq!(chamber.cavern[8].rock_ids, HashSet::from([4]));
        assert_eq!(chamber.cavern[7].rock_ids, HashSet::from([4]));
        assert_eq!(chamber.cavern[6].rock_ids, HashSet::from([3]));
        assert_eq!(chamber.cavern[5].rock_ids, HashSet::from([2, 3]));
        assert_eq!(chamber.cavern[4].rock_ids, HashSet::from([2, 3]));
        assert_eq!(chamber.cavern[3].rock_ids, HashSet::from([1, 2, 3]));
        assert_eq!(chamber.cavern[2].rock_ids, HashSet::from([1]));
        assert_eq!(chamber.cavern[1].rock_ids, HashSet::from([1]));
        assert_eq!(chamber.cavern[0].rock_ids, HashSet::from([0]));

        assert_eq!(horizontal_line_row_ids, vec![0, 9]);
    }

    #[test]
    fn test_get_rock_id() {
        let horizontal_line_row_ids = vec![0, 9, 17, 20, 25, 36];
        assert_eq!(get_rock_id(&horizontal_line_row_ids, 25), 20);
        assert_eq!(get_rock_id(&horizontal_line_row_ids, 36), 25);
    }

    #[test]
    #[should_panic]
    fn test_get_rock_id_panic() {
        let horizontal_line_row_ids = vec![0, 9, 17, 20, 25, 36];
        assert_eq!(get_rock_id(&horizontal_line_row_ids, 26), 4);
    }

    #[test]
    fn test_highest_row_for_rock() {
        let mut jets = JetIterator::new(INPUT);
        let mut chamber = Chamber::new();

        land_multiple_rocks(&mut chamber, 10, &mut jets);

        assert_eq!(chamber.highest_row_for_rock(0), Some(0));
        assert_eq!(chamber.highest_row_for_rock(1), Some(3));
        assert_eq!(chamber.highest_row_for_rock(2), Some(5));
        assert_eq!(chamber.highest_row_for_rock(3), Some(6));
        assert_eq!(chamber.highest_row_for_rock(4), Some(8));
        assert_eq!(chamber.highest_row_for_rock(5), Some(9));
        assert_eq!(chamber.highest_row_for_rock(6), Some(12));
        assert_eq!(chamber.highest_row_for_rock(7), Some(14));
        assert_eq!(chamber.highest_row_for_rock(8), Some(16));
        assert_eq!(chamber.highest_row_for_rock(9), Some(13));
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(do_challenge(INPUT, 1_000_000_000_000), 1514285714288);
    }
}
