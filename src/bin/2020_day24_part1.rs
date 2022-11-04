//! Advent of Code 2020 Day 24
//! https://adventofcode.com/2020/day/24
//!
//! Challenge part 1
//!
//! Flip hexagonal tiles within a large grid of tiles following instructions in the input file, and
//! count the number of flipped tiles to answer the challenge.
//
// The grid of hexagons is represented with a 2D coordinate system where the reference tile in the
// middle is (0, 0), east is +x and north is +y. Moves east and west are x+2 and x-2
// respectively. Other moves are a combination of 1 unit of move in both x and y directions, e.g.,
// north-east is x+1 and y+1. Some grid positions are invalid as tile locations, e.g., x=0, y=1,
// and are not used.

use std::collections::HashSet;
use std::fs;

const INPUT_FILENAME: &str = "2020_day24_input.txt";

type FlippedTileGrid = HashSet<Position>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i16,
    y: i16,
}

fn parse_input(input: &str) -> FlippedTileGrid {
    let mut grid = FlippedTileGrid::new();

    for line in input.lines() {
        if line == "" {
            continue;
        }

        flip_tile(&mut grid, &parse_one_line(&line));
    }
    grid
}

/// Read one line of input, representing one set of moves, and return the position of the resultant
/// tile.
fn parse_one_line(line: &str) -> Position {
    let mut x = 0;
    let mut y = 0;
    let mut index = 0;
    let input: Vec<char> = line.chars().collect();
    let input_len = input.len();

    while index < input_len {
        match input[index] {
            'n' => {
                y += 1;
                match input[index + 1] {
                    'e' => {
                        x += 1;
                    }
                    'w' => {
                        x -= 1;
                    }
                    _ => {
                        panic!("Unrecognized input after 'n' of '{}'", &input[index + 1]);
                    }
                }
                index += 2;
            }
            's' => {
                y -= 1;
                match input[index + 1] {
                    'e' => {
                        x += 1;
                    }
                    'w' => {
                        x -= 1;
                    }
                    _ => {
                        panic!("Unrecognized input after 's' of '{}'", &input[index + 1]);
                    }
                }
                index += 2;
            }
            'e' => {
                x += 2;
                index += 1;
            }
            'w' => {
                x -= 2;
                index += 1;
            }
            _ => {
                panic!("Unrecognized input '{}'", &input[index]);
            }
        }
    }

    // println!("parse_one_line returning position ({}, {})", x, y);
    Position { x, y }
}

/// Flips the tile at position `pos` within `grid`. If the tile is already present in `grid`,
/// this flip will return it to its starting orientation, and it is therefore removed from `grid`.
fn flip_tile(grid: &mut FlippedTileGrid, pos: &Position) {
    if let Some(_) = grid.get(pos) {
        grid.remove(pos);
    } else {
        grid.insert(*pos);
    }
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let grid = parse_input(&input_file);

    println!("Challenge answer is {}", grid.len());
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn test_parse_one_line() {
        assert_eq!(Position { x: 1, y: -1 }, parse_one_line(&"esew"));
        assert_eq!(Position { x: 0, y: 0 }, parse_one_line(&"nwwswee"));

        assert_eq!(
            Position { x: -4, y: -2 },
            parse_one_line(&"sesenwnenenewseeswwswswwnenewsewsw")
        );

        assert_eq!(
            Position { x: -1, y: 3 },
            parse_one_line(&"neeenesenwnwwswnenewnwwsewnenwseswesw")
        );
    }

    #[test]
    fn test_parse_file() {
        let grid = parse_input(&TEST_INPUT);

        assert_eq!(10, grid.len());
    }
}
