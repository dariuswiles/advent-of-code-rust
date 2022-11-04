//! Advent of Code 2020 Day 24
//! https://adventofcode.com/2020/day/24
//!
//! Challenge part 2
//!
//! Flip hexagonal tiles within a large grid of tiles following instructions in the input file.
//! Perform multiple rounds of tile flipping following the rules, then count the number of flipped
//! tiles to answer the challenge.
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

/// Return how many of the tiles in the six adjacent to the tile at `p` are flipped.
fn count_adjacent_flipped(grid: &FlippedTileGrid, p: &Position) -> u8 {
    let mut count = 0;

    if grid.contains(&Position { x: p.x - 2, y: p.y }) {
        count += 1;
    }
    if grid.contains(&Position { x: p.x + 2, y: p.y }) {
        count += 1;
    }
    if grid.contains(&Position {
        x: p.x - 1,
        y: p.y - 1,
    }) {
        count += 1;
    }
    if grid.contains(&Position {
        x: p.x - 1,
        y: p.y + 1,
    }) {
        count += 1;
    }
    if grid.contains(&Position {
        x: p.x + 1,
        y: p.y - 1,
    }) {
        count += 1;
    }
    if grid.contains(&Position {
        x: p.x + 1,
        y: p.y + 1,
    }) {
        count += 1;
    }

    count
}

/// Examine every tile to see if it should be flipped according to the following challenge rules:
///     - a flipped tile with zero, or more than 2, flipped tiles immediately adjacent to it is
///       unflipped.
///     - Any unflipped tile with exactly 2 flipped tiles immediately adjacent to it is flipped.
fn perform_day_flip(grid: &mut FlippedTileGrid) {
    let flipped_list_x = grid.iter().map(|Position { x, y: _ }| x);
    let flipped_min_x = flipped_list_x.clone().min().unwrap();
    let flipped_max_x = flipped_list_x.max().unwrap();

    let flipped_list_y = grid.iter().map(|Position { x: _, y }| y);
    let flipped_min_y = flipped_list_y.clone().min().unwrap();
    let flipped_max_y = flipped_list_y.max().unwrap();

    // println!("x ranges from {} to {} and y ranges from {} to {}", flipped_min_x, flipped_max_x,
    //     flipped_min_y, flipped_max_y
    // );

    let mut flip = Vec::new();
    let mut unflip = Vec::new();
    for y in flipped_min_y - 2..=flipped_max_y + 2 {
        for x in flipped_min_x - 2..=flipped_max_x + 2 {
            // Coordinates are only valid if both `x` and `y` are odd, or both are even.
            if (x + y) % 2 != 0 {
                continue;
            }

            let p = Position { x, y };
            let adjacent_flipped = count_adjacent_flipped(grid, &p);

            if grid.contains(&p) {
                if (adjacent_flipped == 0) || (adjacent_flipped > 2) {
                    unflip.push(p);
                }
            } else {
                if adjacent_flipped == 2 {
                    flip.push(p);
                }
            }
        }
    }

    // println!("Unflipped (white) tiles to flip to black: {:?}", &flip);
    for f in flip {
        grid.insert(f);
    }

    // println!("Flipped (black) tiles to unflip to white: {:?}", &unflip);
    for uf in unflip {
        grid.remove(&uf);
    }
}

fn perform_multiple_day_flips(grid: &mut FlippedTileGrid, days: usize) {
    for _ in 0..days {
        perform_day_flip(grid);
    }
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let mut grid = parse_input(&input_file);

    perform_multiple_day_flips(&mut grid, 100);

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
        assert_eq!(
            Position { x: -3, y: -3 },
            parse_one_line(&"seswneswswsenwwnwse")
        );
        assert_eq!(
            Position { x: 2, y: 2 },
            parse_one_line(&"nwnwneseeswswnenewneswwnewseswneseene")
        );
        assert_eq!(
            Position { x: 0, y: 2 },
            parse_one_line(&"swweswneswnenwsewnwneneseenw")
        );
        assert_eq!(
            Position { x: -2, y: 0 },
            parse_one_line(&"eesenwseswswnenwswnwnwsewwnwsene")
        );
        assert_eq!(
            Position { x: -1, y: 3 },
            parse_one_line(&"sewnenenenesenwsewnenwwwse")
        );
        assert_eq!(
            Position { x: -4, y: 0 },
            parse_one_line(&"wenwwweseeeweswwwnwwe")
        );
        assert_eq!(
            Position { x: -1, y: 1 },
            parse_one_line(&"wsweesenenewnwwnwsenewsenwwsesesenwne")
        );
        assert_eq!(
            Position { x: -3, y: -1 },
            parse_one_line(&"neeswseenwwswnwswswnw")
        );
        assert_eq!(
            Position { x: -2, y: 2 },
            parse_one_line(&"nenwswwsewswnenenewsenwsenwnesesenew")
        );
        assert_eq!(
            Position { x: -2, y: 2 },
            parse_one_line(&"enewnwewneswsewnwswenweswnenwsenwsw")
        );
        assert_eq!(
            Position { x: 3, y: 3 },
            parse_one_line(&"sweneswneswneneenwnewenewwneswswnese")
        );
        assert_eq!(
            Position { x: -2, y: 0 },
            parse_one_line(&"swwesenesewenwneswnwwneseswwne")
        );
        assert_eq!(
            Position { x: 2, y: -2 },
            parse_one_line(&"enesenwswwswneneswsenwnewswseenwsese")
        );
        assert_eq!(
            Position { x: 0, y: 0 },
            parse_one_line(&"wnwnesenesenenwwnenwsewesewsesesew")
        );
        assert_eq!(
            Position { x: 0, y: 2 },
            parse_one_line(&"nenewswnwewswnenesenwnesewesw")
        );
        assert_eq!(
            Position { x: 2, y: 2 },
            parse_one_line(&"eneswnwswnwsenenwnwnwwseeswneewsenese")
        );
        assert_eq!(
            Position { x: 4, y: 0 },
            parse_one_line(&"neswnwewnwnwseenwseesewsenwsweewe")
        );
        assert_eq!(
            Position { x: -3, y: 1 },
            parse_one_line(&"wseweeenwnesenwwwswnew")
        );
    }

    #[test]
    fn test_parse_file() {
        let grid = parse_input(&TEST_INPUT);

        assert_eq!(10, grid.len());
    }

    #[test]
    fn test_day_flip() {
        let mut grid = parse_input(&TEST_INPUT);

        perform_day_flip(&mut grid);
        assert_eq!(15, grid.len());

        perform_day_flip(&mut grid);
        assert_eq!(12, grid.len());

        perform_day_flip(&mut grid);
        assert_eq!(25, grid.len());

        perform_day_flip(&mut grid);
        assert_eq!(14, grid.len());

        perform_day_flip(&mut grid);
        assert_eq!(23, grid.len());

        perform_day_flip(&mut grid);
        assert_eq!(28, grid.len());

        perform_day_flip(&mut grid);
        assert_eq!(41, grid.len());

        perform_day_flip(&mut grid);
        assert_eq!(37, grid.len());

        perform_day_flip(&mut grid);
        assert_eq!(49, grid.len());

        perform_day_flip(&mut grid);
        assert_eq!(37, grid.len());
    }

    #[test]
    fn test_perform_multiple_day_flips() {
        let mut grid = parse_input(&TEST_INPUT);

        perform_multiple_day_flips(&mut grid, 10);
        assert_eq!(37, grid.len());

        perform_multiple_day_flips(&mut grid, 10);
        assert_eq!(132, grid.len());

        perform_multiple_day_flips(&mut grid, 10);
        assert_eq!(259, grid.len());

        perform_multiple_day_flips(&mut grid, 10);
        assert_eq!(406, grid.len());

        perform_multiple_day_flips(&mut grid, 10);
        assert_eq!(566, grid.len());

        perform_multiple_day_flips(&mut grid, 10);
        assert_eq!(788, grid.len());

        perform_multiple_day_flips(&mut grid, 10);
        assert_eq!(1106, grid.len());

        perform_multiple_day_flips(&mut grid, 10);
        assert_eq!(1373, grid.len());

        perform_multiple_day_flips(&mut grid, 10);
        assert_eq!(1844, grid.len());

        perform_multiple_day_flips(&mut grid, 10);
        assert_eq!(2208, grid.len());
    }
}
