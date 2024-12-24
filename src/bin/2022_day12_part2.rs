//! Advent of Code 2022 Day 12
//! https://adventofcode.com/2022/day/12
//!
//! Challenge part 2
//!
//! Finds the shortest path through the given heightmap of mountainous terrain from any position
//! at the lowest height to a given end position.

use std::cmp::min;
use std::collections::HashSet;
use std::fmt::{self, Display};
use std::fs;

const INPUT_FILENAME: &str = "2022_day12_input.txt";

/// A position expressed as `x` and `y` coordinates. The top-left position is x = 0, y = 0.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

/// A representation of a map of the heights of mountainous terrain in a 2D grid. The heights are
/// stored in `map` as a letter between 'a' and 'z', where 'a' is the lowest terrain height.
/// In addition to the `map` itself, the start `Position`, the end `Position`, and the map's
/// `width` and `height` are also stored.
#[derive(Debug, PartialEq)]
struct Heightmap {
    map: Vec<char>,
    width: usize,
    height: usize,
    start: Position,
    end: Position,
}

impl Heightmap {
    /// Creates and returns a new `Heightmap` based on the given input string. In addition to the
    /// `map` itself, the `Position`s of the start and end, and the map's `width` and `height` are
    /// also stored.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     - the input contains unexpected characters;
    ///     - non-empty lines are not all the same length;
    ///     - the input does not contain a start location (denoted with 'S'); or
    ///     - the input does not contain an end location (denoted with 'E').
    ///
    /// Specifying multiple start or end locations is invalid, but does not result in a panic.
    /// Instead, the last encountered position of each is used.
    fn from_str(input: &str) -> Self {
        let mut map = Vec::new();
        let mut widths = Vec::new();
        let mut start = None;
        let mut end = None;

        let mut height = 0;
        for line in input.lines() {
            if !line.is_empty() {
                let mut row: Vec<char> = line.chars().collect();

                if let Some(start_column) = row.iter().position(|&c| c == 'S') {
                    start = Some(Position {
                        x: start_column,
                        y: height,
                    });
                    row[start_column] = 'a';
                }

                if let Some(end_column) = row.iter().position(|&c| c == 'E') {
                    end = Some(Position {
                        x: end_column,
                        y: height,
                    });
                    row[end_column] = 'z';
                }

                widths.push(row.len());
                map.append(&mut row);
                height += 1;
            }
        }

        let width = widths[0];
        assert!(
            widths.iter().all(|w| w == &width),
            "Error: all lines of input must be the same length"
        );

        assert!(
            map.iter().all(|c| c.is_ascii_lowercase()),
            "Error: invalid character found in input"
        );

        Self {
            map,
            width,
            height,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }
}

/// Displays this `Heightmap` in a similar format to that used by the challenge. The exception is
/// that the start and end locations are not marked.
impl Display for Heightmap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rows_as_chars: Vec<&[char]> = self.map.chunks(self.width).collect();
        let rows_as_strings: Vec<String> =
            rows_as_chars.iter().map(|r| r.iter().collect()).collect();

        writeln!(f, "{}", rows_as_strings.join("\n"))
    }
}

/// A single cell in a `FlatMap` that indicates whether travel is possible to adjacent cells in
/// cardinal directions.
#[derive(Debug, PartialEq)]
struct FlatMapCell {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}

/// Represents a heightmap as a 2D grid of `FlatMapCell`s, a start `Position`, an end `Position',
/// and the map's `width` and `height`. Each `FlatMapCell` indicates in which cardinal directions
/// travel is possible.
#[derive(Debug, PartialEq)]
struct FlatMap {
    flat_map: Vec<FlatMapCell>,
    width: usize,
    height: usize,
    start: Position,
    end: Position,
}

impl FlatMap {
    fn new(hm: &Heightmap) -> Self {
        let mut flat_map = Vec::new();

        for row in 0..hm.height {
            for column in 0..hm.width {
                let cell_index = row * hm.width + column;
                let cell_height = hm.map[cell_index];
                let mut north = false;
                let mut east = false;
                let mut south = false;
                let mut west = false;

                if row > 0 {
                    north =
                        (cell_height as u8 + 1) >= (hm.map[(row - 1) * hm.width + column] as u8);
                }

                if column < hm.width - 1 {
                    east = (cell_height as u8 + 1) >= (hm.map[row * hm.width + column + 1] as u8);
                }

                if row < hm.height - 1 {
                    south =
                        (cell_height as u8 + 1) >= (hm.map[(row + 1) * hm.width + column] as u8);
                }

                if column > 0 {
                    west = (cell_height as u8 + 1) >= (hm.map[row * hm.width + column - 1] as u8);
                }

                flat_map.push(FlatMapCell {
                    north,
                    east,
                    south,
                    west,
                });
            }
        }

        Self {
            flat_map,
            width: hm.width,
            height: hm.height,
            start: hm.start,
            end: hm.end,
        }
    }
}

/// Returns the shortest path between the `start` `Position` passed as a parameter, and the `end`
/// `Position` in the given `FlatMap`. Returns `None` if there is no path between the `start` and
/// `end` `Position`s`.
//
// The shortest path is found by noting all cells adjacent to `start` that are reachable in one
// move. All cells reachable from *this* set can, by definition, be reached in two moves. This
// process is repeated until the set of reachable cells includes the `end` `Position`.
fn find_shortest_path(fm: &FlatMap, start: &Position) -> Option<usize> {
    let mut turn = 0;
    let mut visited = Vec::new();
    let mut visited_last_turn = HashSet::new();

    // Mark all locations as unvisited (`None`) except the start `Position`.
    visited.resize_with(fm.width * fm.height, Default::default);
    visited[start.y * fm.width + start.x] = Some(0);

    // Add the start `Position` to the set of locations visited last turn so that it is used as the
    // starting point for the shortest path search.
    visited_last_turn.insert(*start);

    while !visited_last_turn.contains(&fm.end) {
        turn += 1;
        let mut visited_this_turn = HashSet::new();

        for vlt in visited_last_turn {
            let cell_index = vlt.y * fm.width + vlt.x;
            let flat_map_details = &fm.flat_map[cell_index];

            if flat_map_details.north {
                let adjacent_position = Position {
                    x: vlt.x,
                    y: vlt.y - 1,
                };

                if visited[adjacent_position.y * fm.width + adjacent_position.x].is_none() {
                    visited[adjacent_position.y * fm.width + adjacent_position.x] = Some(turn);
                    visited_this_turn.insert(adjacent_position);
                }
            }

            if flat_map_details.east {
                let adjacent_position = Position {
                    x: vlt.x + 1,
                    y: vlt.y,
                };

                if visited[adjacent_position.y * fm.width + adjacent_position.x].is_none() {
                    visited[adjacent_position.y * fm.width + adjacent_position.x] = Some(turn);
                    visited_this_turn.insert(adjacent_position);
                }
            }

            if flat_map_details.south {
                let adjacent_position = Position {
                    x: vlt.x,
                    y: vlt.y + 1,
                };
                if visited[adjacent_position.y * fm.width + adjacent_position.x].is_none() {
                    visited[adjacent_position.y * fm.width + adjacent_position.x] = Some(turn);
                    visited_this_turn.insert(adjacent_position);
                }
            }

            if flat_map_details.west {
                let adjacent_position = Position {
                    x: vlt.x - 1,
                    y: vlt.y,
                };
                if visited[adjacent_position.y * fm.width + adjacent_position.x].is_none() {
                    visited[adjacent_position.y * fm.width + adjacent_position.x] = Some(turn);
                    visited_this_turn.insert(adjacent_position);
                }
            }
        }

        if visited_this_turn.is_empty() {
            return None;
        }

        visited_last_turn = visited_this_turn;
    }

    Some(turn)
}

/// Find the shortest path between every cell with height 'a' and the `end` `Position`. The
/// shortest of these is the hiking trail that is the answer to part 2 of the challenge.
fn find_shortest_hiking_trail(hm: &Heightmap, fm: &FlatMap) -> usize {
    let mut shortest_so_far = usize::MAX;

    for row in 0..hm.height {
        for column in 0..hm.width {
            let cell_index = row * fm.width + column;

            if hm.map[cell_index] == 'a' {
                if let Some(path_length) = find_shortest_path(fm, &Position { x: column, y: row }) {
                    shortest_so_far = min(shortest_so_far, path_length);
                }
            }
        }
    }

    shortest_so_far
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    let hm = Heightmap::from_str(&input);
    let fm = FlatMap::new(&hm);

    println!(
        "The shortest path from start to finish is {}",
        find_shortest_hiking_trail(&hm, &fm)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test_create_heightmap() {
        let hm = Heightmap::from_str(&TEST_INPUT);

        assert_eq!(
            hm,
            Heightmap {
                map: "aabqponmabcryxxlaccszzxkacctuvwjabdefghi".chars().collect(),
                width: 8,
                height: 5,
                start: Position { x: 0, y: 0 },
                end: Position { x: 5, y: 2 },
            }
        );
    }

    #[test]
    fn test_heightmap_display() {
        let hm = Heightmap::from_str(&TEST_INPUT);

        assert_eq!(
            hm.to_string(),
            "aabqponm\nabcryxxl\naccszzxk\nacctuvwj\nabdefghi\n"
        );
    }

    #[test]
    fn test_create_flatmap() {
        let hm = Heightmap::from_str(&TEST_INPUT);

        assert_eq!(
            FlatMap::new(&hm),
            FlatMap {
                flat_map: vec![
                    // Row #0
                    FlatMapCell {
                        north: false,
                        east: true,
                        south: true,
                        west: false,
                    },
                    FlatMapCell {
                        north: false,
                        east: true,
                        south: true,
                        west: true,
                    },
                    FlatMapCell {
                        north: false,
                        east: false,
                        south: true,
                        west: true,
                    },
                    FlatMapCell {
                        north: false,
                        east: true,
                        south: true,
                        west: true,
                    },
                    FlatMapCell {
                        north: false,
                        east: true,
                        south: false,
                        west: true,
                    },
                    FlatMapCell {
                        north: false,
                        east: true,
                        south: false,
                        west: true,
                    },
                    FlatMapCell {
                        north: false,
                        east: true,
                        south: false,
                        west: true,
                    },
                    FlatMapCell {
                        north: false,
                        east: false,
                        south: true,
                        west: true,
                    },
                    // Row #1
                    FlatMapCell {
                        north: true,
                        east: true,
                        south: true,
                        west: false,
                    },
                    FlatMapCell {
                        north: true,
                        east: true,
                        south: true,
                        west: true,
                    },
                    FlatMapCell {
                        north: true,
                        east: false,
                        south: true,
                        west: true,
                    },
                    FlatMapCell {
                        north: true,
                        east: false,
                        south: true,
                        west: true,
                    },
                    FlatMapCell {
                        north: true,
                        east: true,
                        south: true,
                        west: true,
                    },
                    FlatMapCell {
                        north: true,
                        east: true,
                        south: false,
                        west: true,
                    },
                    FlatMapCell {
                        north: true,
                        east: true,
                        south: true,
                        west: true,
                    },
                    FlatMapCell {
                        north: true,
                        east: false,
                        south: true,
                        west: false,
                    },
                    // Row #2
                    FlatMapCell {
                        north: true,
                        east: false,
                        south: true,
                        west: false,
                    },
                    FlatMapCell {
                        north: true,
                        east: true,
                        south: true,
                        west: true,
                    },
                    FlatMapCell {
                        north: true,
                        east: false,
                        south: true,
                        west: true,
                    },
                    FlatMapCell {
                        north: true,
                        east: false,
                        south: true,
                        west: true,
                    },
                    FlatMapCell {
                        north: true,
                        east: true,
                        south: true,
                        west: true,
                    },
                    FlatMapCell {
                        north: true,
                        east: true,
                        south: true,
                        west: true,
                    },
                    FlatMapCell {
                        north: true,
                        east: true,
                        south: true,
                        west: false,
                    },
                    FlatMapCell {
                        north: true,
                        east: false,
                        south: true,
                        west: false,
                    },
                    // Row #3
                    FlatMapCell {
                        north: true,
                        east: false,
                        south: true,
                        west: false,
                    },
                    FlatMapCell {
                        north: true,
                        east: true,
                        south: true,
                        west: true,
                    },
                    FlatMapCell {
                        north: true,
                        east: false,
                        south: true,
                        west: true,
                    },
                    FlatMapCell {
                        north: true,
                        east: true,
                        south: true,
                        west: true,
                    },
                    FlatMapCell {
                        north: false,
                        east: true,
                        south: true,
                        west: true,
                    },
                    FlatMapCell {
                        north: false,
                        east: true,
                        south: true,
                        west: true,
                    },
                    FlatMapCell {
                        north: true,
                        east: true,
                        south: true,
                        west: true,
                    },
                    FlatMapCell {
                        north: true,
                        east: false,
                        south: true,
                        west: false,
                    },
                    // Row #4
                    FlatMapCell {
                        north: true,
                        east: true,
                        south: false,
                        west: false,
                    },
                    FlatMapCell {
                        north: true,
                        east: false,
                        south: false,
                        west: true,
                    },
                    FlatMapCell {
                        north: true,
                        east: true,
                        south: false,
                        west: true,
                    },
                    FlatMapCell {
                        north: false,
                        east: true,
                        south: false,
                        west: true,
                    },
                    FlatMapCell {
                        north: false,
                        east: true,
                        south: false,
                        west: true,
                    },
                    FlatMapCell {
                        north: false,
                        east: true,
                        south: false,
                        west: true,
                    },
                    FlatMapCell {
                        north: false,
                        east: true,
                        south: false,
                        west: true,
                    },
                    FlatMapCell {
                        north: true,
                        east: false,
                        south: false,
                        west: true,
                    },
                ],
                width: 8,
                height: 5,
                start: Position { x: 0, y: 0 },
                end: Position { x: 5, y: 2 },
            }
        );
    }

    #[test]
    fn test_find_shortest_path() {
        let hm = Heightmap::from_str(&TEST_INPUT);
        let fm = FlatMap::new(&hm);

        assert_eq!(find_shortest_path(&fm, &fm.start), Some(31));
    }

    #[test]
    fn test_find_hiking_trail() {
        let hm = Heightmap::from_str(&TEST_INPUT);
        let fm = FlatMap::new(&hm);

        assert_eq!(find_shortest_hiking_trail(&hm, &fm), 29);
    }
}
