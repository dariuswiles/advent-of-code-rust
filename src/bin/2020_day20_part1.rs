//! Advent of Code 2020 Day 20
//! https://adventofcode.com/2020/day/20
//!
//! Challenge part 1
//!
//! Given an input file containing square 2D tiles, rotate and/or flip the tiles to determine which
//! have identical borders. This is analogous to fitting pieces of a jigsaw puzzle together. When
//! complete, multiply the ids of the four corner tiles together to determine the answer to the
//! challenge.

use std::collections::HashMap;
use std::fs;

const INPUT_FILENAME: &str = "2020_day20_input.txt";
const TILE_SIZE: usize = 10;
const TILE_INPUT_KEYWORD: &str = "Tile "; // The string immediately preceding the tile id
const TOP: usize = 0;
const RIGHT: usize = 1;
const BOTTOM: usize = 2;
const LEFT: usize = 3;

type Id = u16;

/// A `Tile` stores a single tile, which is a square with a pre-determined, constant length. For
/// efficient searching of matching tiles the borders of the tile are stored in `borders`, and
/// reversed (flipped) versions in `borders_flipped`. Borders are stored in the order: top, right,
/// bottom, left. Borders are stored in a clockwise direction, e.g., left-to-right for the top
/// border and right-to-left for the bottom border. This makes comparisons easier when the tile is
/// rotated.
#[derive(Clone, Debug, PartialEq)]
struct Tile {
    id: Id,
    cells: Vec<String>,
    borders: [String; 4],
    borders_flipped: [String; 4],
}

impl Tile {
    fn from_string(input: &str) -> Self {
        let mut lines = input.lines();
        let id_line = lines.next().unwrap();

        if !id_line.starts_with(TILE_INPUT_KEYWORD) {
            panic!("Tile input does not contain expected starting keyword");
        }

        let id = id_line
            .trim_start_matches(TILE_INPUT_KEYWORD)
            .trim_end_matches(':')
            .parse()
            .unwrap();

        let mut cells = Vec::new();
        let mut lines_read = 0;

        loop {
            if let Some(line) = lines.next() {
                if line == "" {
                    if lines_read == TILE_SIZE {
                        break;
                    } else {
                        panic!("Input contained a tile with an unexpected number of rows");
                    }
                }

                if line.len() != TILE_SIZE {
                    panic!("Input contained a tile row with an unexpected number of columns");
                }

                cells.push(line.to_owned());
                lines_read += 1;
            } else {
                break;
            }
        }

        let mut left = String::new();
        let mut right = String::new();

        for rows in &cells {
            let mut r_chars = rows.chars();
            left.insert(0, r_chars.next().unwrap());
            right.push(r_chars.last().unwrap());
        }

        let top: String = cells[TILE_SIZE - 1].chars().rev().collect();

        let borders = [cells[0].to_owned(), right, top, left];
        let borders_flipped: [String; 4] = [
            borders[TOP].chars().rev().collect(),
            borders[RIGHT].chars().rev().collect(),
            borders[BOTTOM].chars().rev().collect(),
            borders[LEFT].chars().rev().collect(),
        ];

        Self {
            id,
            cells,
            borders,
            borders_flipped,
        }
    }

    /// Attempts to match all four borders of the current object with non-flipped and flipped
    /// borders of the given Tile. Returns `None` if there are no matches between the two tiles,
    /// otherwise a triplet containing:
    ///     - the border of `self` that matches.
    ///     - the border of `other` that matches.
    ///     - a bool that is true iff the match requires one of the tiles to be flipped.
    ///
    /// NOTE The algorithm used assumes that no tile borders are palindromes, as this requires
    ///      more sophisticated logic that allows tile flips to be optional. An example of a
    ///      palindromic border, that cannot be handled by this code, is "###....###".
    fn find_matching_border(&self, other: &Tile) -> Option<(usize, usize, bool)> {
        for self_border_idx in 0..4 {
            for other_border_idx in 0..4 {
                // For the borders of two tiles to match, one must be a reversed version of the
                // other, e.g., "####......" matches "......####". If a match like this is found,
                // it is the simple case where neither of the tiles needs to be flipped.
                if self.borders[self_border_idx] == other.borders_flipped[other_border_idx] {
                    // println!("\tMatched tile {} border {} with tile {} border {}",
                    //     self.id, self_border_idx, other.id, other_border_idx
                    // );

                    return Some((self_border_idx, other_border_idx, false));
                }

                // As above, but this time look for *identical* borders. These still match, but
                // only if one of the tiles is flipped.
                if self.borders[self_border_idx] == other.borders[other_border_idx] {
                    // println!("\tMatched tile {} border {} with *flipped* tile {} border {}",
                    //     self.id, self_border_idx, other.id, other_border_idx
                    // );

                    return Some((self_border_idx, other_border_idx, true));
                }
            }
        }

        None
    }
}

fn parse_input(input: &str) -> Vec<Tile> {
    // println!("parse_input called with data \n{}", &input);
    let lines: Vec<&str> = input.lines().collect();

    let mut tiles = Vec::new();
    let mut tile_start = 0;
    for i in 0..lines.len() {
        if lines[i].starts_with(TILE_INPUT_KEYWORD) {
            tile_start = i;
            // println!("tile_start = {}", tile_start);
        }

        if lines[i] == "" {
            let tile_block = lines[tile_start..i].join("\n");
            // println!("parse_input about to call from_string with data\n{:#?}", &tile_block);

            tiles.push(Tile::from_string(&tile_block));
        }
    }

    if tile_start + TILE_SIZE + 1 == lines.len() {
        let tile_block = lines[tile_start..lines.len()].join("\n");
        // println!("parse_input calling from_string with final block of data\n{:#?}", &tile_block);

        tiles.push(Tile::from_string(&tile_block));
    }

    // println!("parse_input returning\n{:#?}", &tiles);
    tiles
}

fn find_tile_matches(tiles: &Vec<Tile>) -> HashMap<Id, Vec<Id>> {
    let mut matches = HashMap::new();

    let tiles_count = tiles.len();

    for t0 in 0..tiles_count {
        for t1 in 0..tiles_count {
            if tiles[t0].id == tiles[t1].id {
                continue;
            }

            if let Some(_) = tiles[t0].find_matching_border(&tiles[t1]) {
                matches
                    .entry(tiles[t0].id)
                    .or_insert_with(Vec::new)
                    .push(tiles[t1].id);
                continue;
            }
        }
    }

    matches
}

/// Load tiles from input file, find matching borders and return the product of the ids of the four
/// corner tiles.
fn do_challenge(input: &str) -> u64 {
    let tiles = parse_input(input);

    let tile_matches = find_tile_matches(&tiles);
    // println!("List of all matching borders\n{:?}", &tile_matches);

    let corners: HashMap<_, _> = tile_matches.iter().filter(|tm| tm.1.len() == 2).collect();

    if corners.len() != 4 {
        panic!(
            "Expecting four corners to be found, but found {}",
            corners.len()
        );
    }

    corners.iter().fold(1, |acc, x| acc * **x.0 as u64)
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let answer = do_challenge(&input_file);
    println!("The product of the ids of the corner tiles is {}", answer);
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    const TEST_SINGLE_TILE: &str = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###";

    const TEST_TWO_TILES: &str = "\
Tile 5555:
..##......
#.........
#.........
#.........
#.........
#.........
#.........
#.........
#.........
..#.......

Tile 7777:
##########
#.........
#.........
#.........
#.........
#.........
#.........
#.........
#.........
......##..";

    #[test]
    fn solve_test_puzzle() {
        let answer = do_challenge(&TEST_INPUT);
        assert_eq!(answer, 20899048083289u64);
    }

    #[test]
    fn tile_creation() {
        let tile = Tile::from_string(TEST_SINGLE_TILE);

        assert_eq!(tile.cells.len(), TILE_SIZE);
        assert_eq!(tile.cells[0].len(), TILE_SIZE);
        assert_eq!(tile.cells[0], "..##.#..#.");
        assert_eq!(tile.cells[9], "..###..###");
        assert_eq!(tile.cells[9].len(), TILE_SIZE);

        assert_eq!(tile.borders[0], "..##.#..#.");
        assert_eq!(tile.borders[1], "...#.##..#");
        assert_eq!(tile.borders[2], "###..###..");
        assert_eq!(tile.borders[3], ".#..#####.");

        assert_eq!(tile.borders_flipped[0], ".#..#.##..");
        assert_eq!(tile.borders_flipped[1], "#..##.#...");
        assert_eq!(tile.borders_flipped[2], "..###..###");
        assert_eq!(tile.borders_flipped[3], ".#####..#.");
    }

    #[test]
    fn parse_one_tile() {
        let tile = parse_input(TEST_SINGLE_TILE);
        assert_eq!(tile[0].cells.len(), TILE_SIZE);
        assert_eq!(tile[0].cells[0].len(), TILE_SIZE);
        assert_eq!(tile[0].cells[0], "..##.#..#.");
        assert_eq!(tile[0].borders[1], "...#.##..#");
    }

    #[test]
    fn match_two_tiles() {
        let tiles = parse_input(TEST_TWO_TILES);

        let matches = find_tile_matches(&tiles);

        println!("find_tile_matches returned\n{:?}", matches);
        assert_eq!(matches[&5555], vec![7777]);
    }
}
