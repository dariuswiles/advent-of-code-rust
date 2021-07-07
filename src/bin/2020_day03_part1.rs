//! Advent of Code 2020 Day 03
//! https://adventofcode.com/2020/day/3
//!
//! Challenge part 1
//!
//! The input file is a grid of open terrain (character `.`) and trees (character `#`). This code
//! starts at the top-left of the map and counts the number of trees encountered when moving down
//! one line and right 3 positions per move. The map is tiled horizontally, so going beyond the
//! right edge of the input data is effectively the same as wrapping round to the left edge.

use std::fs;

const INPUT_FILENAME: &str = "2020_day03_input.txt";
const TREE: &str = "#";

const MOVE_PATTERN: Pattern = Pattern { right: 3, down: 1 };

/// A movement pattern expressed as how many positions `down` and `right` constitute one move.
#[derive(Clone, Copy, Debug)]
struct Pattern {
    right: usize,
    down: usize,
}


/// Returns the number of trees hit when the given pattern is taken through the map provided in
/// `input`.
fn tree_hits_for_pattern(input: &str, p: &Pattern) -> u32 {
//     println!("Calculating total trees hit for movement pattern {:#?}", &p);

    let mut trees_hit = 0;

    let mut y_pos: usize = 0;
    for (line_num, line) in input.lines().enumerate() {
        if line_num == 0 {
//         println!("Skipping first line");
            continue;
        }

        if line_num % p.down != 0 {
//             println!("Skipping line {} as it doesn't match the `down` value of this pattern",
//                 line_num
//             );
            continue;
        }

//        println!("Terrain for line #{} is {}", line_num, line);

        y_pos += p.right;

        // If the horizontal position moves outside the right edge of the map, wrap it to the
        // corresponding position on the left edge.
        let y_pos_wrapped = y_pos % line.len();

        let terrain = line.get(y_pos_wrapped..y_pos_wrapped+1).unwrap();
//         println!("\tTerrain at y_pos={} is '{}'", y_pos, terrain);

        if terrain == TREE {
            trees_hit += 1;
//             println!("\tHit a tree.");
        }
    }

//         println!("{} trees hit", trees_hit);
    trees_hit
}


fn main() {
    let input =
        fs::read_to_string(INPUT_FILENAME)
            .expect("Error reading input file");

    println!("{} trees hit", tree_hits_for_pattern(&input, &MOVE_PATTERN));
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_0: &str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn success() {
        assert_eq!(tree_hits_for_pattern(INPUT_0, &MOVE_PATTERN), 7);
    }
}
