//! Advent of Code 2020 Day 17
//! https://adventofcode.com/2020/day/17
//!
//! Challenge part 1
//!
//! Update a grid of active and inactive cubes following the rules in the challenge. Count the
//! number of active cubes after 6 iterations of the rules to get the answer.

use std::collections::HashSet;
use std::fmt;
use std::fs;

const INPUT_FILENAME: &str = "2020_day17_input.txt";
const STATE_ACTIVE: char = '#';
const STATE_INACTIVE: char = '.';

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, Debug, Default)]
struct CubeGrid {
    active_cubes: HashSet<Position>
}

impl PartialEq for CubeGrid {
    fn eq(&self, other: &Self) -> bool {
        if self.active_cubes.len() != other.active_cubes.len() {
            return false;
        }

        for cube in &self.active_cubes {
            if other.active_cubes.get(&cube) == None {
                return false;
            }
        }
        true
    }
}

impl Eq for CubeGrid {}

impl fmt::Display for CubeGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let active_box = self.get_grid_limits();
        writeln!(f, "Top-left corner is x={} and y={}\n", active_box.0.x, active_box.0.y).unwrap();

        for z in active_box.0.z..=active_box.1.z {
            writeln!(f, "z={}", z).unwrap();
            for y in active_box.0.y..=active_box.1.y {
                let mut output = Vec::new();
                for x in active_box.0.x..=active_box.1.x {
                    if self.active_cubes.contains(&Position { x: x, y: y, z: z }) {
                        output.push(STATE_ACTIVE);
                    } else {
                        output.push(STATE_INACTIVE);
                    }
                }
                writeln!(f, "{}", output.iter().collect::<String>()).unwrap();
            }
            writeln!(f, "").unwrap();
        }
        writeln!(f, "")
    }
}


impl CubeGrid {
    // Create a new `CubeGrid` from a string representing a 2D grid of cube states. `z`=0 for all
    // cubes.
    fn from_str(layers: &[&str]) -> Self {
        let layer_count = layers.len() as i32;
        let layer_start = -layer_count / 2;
        let grid_length = layers[0].lines().next().unwrap().len() as i32;
        let grid_start = -(grid_length as f64 / 2.0) as i32;
        let mut active_cubes = HashSet::new();

        let mut z = layer_start;
        for s in layers {
            let mut x = grid_start;
            let mut y = grid_start;

            for line in s.lines() {
                if line == "" {
                    continue;
                }

                for c in line.chars() {
                    if c == STATE_ACTIVE {
                        active_cubes.insert(Position {x, y, z} );
                    }
                    x += 1;
                }

                x = grid_start;
                y += 1;
            }

            z += 1;
        }

        CubeGrid { active_cubes: active_cubes }
    }

    // Returns a tuple containing two `Position`s. The first contains the minimum `x`, `y` and `z`
    // values across all active cubes. The second is similar but contains maximum values. A cube
    // with two opposite corners having these positions will encompass all active cubes.
    fn get_grid_limits(&self) -> (Position, Position) {
        let mut x_min = i32::MAX;
        let mut x_max = i32::MIN;
        let mut y_min = i32::MAX;
        let mut y_max = i32::MIN;
        let mut z_min = i32::MAX;
        let mut z_max = i32::MIN;

        for p in &self.active_cubes {
            x_min = i32::min(x_min, p.x);
            x_max = i32::max(x_max, p.x);
            y_min = i32::min(y_min, p.y);
            y_max = i32::max(y_max, p.y);
            z_min = i32::min(z_min, p.z);
            z_max = i32::max(z_max, p.z);
        }

        ((Position { x: x_min, y: y_min, z: z_min }), (Position { x: x_max, y: y_max, z: z_max }))
    }

    /// Returns how many of the 26 cubes adjacent to the given cube are active.
    fn active_adjacent_cubes(&self, p: &Position) -> u32 {
        let mut active_total = 0;
        for z in p.z-1..=p.z+1 {
            for y in p.y-1..=p.y+1 {
                for x in p.x-1..=p.x+1 {
                    if !((x == p.x) && (y == p.y) && (z == p.z)) {
                        if self.active_cubes.contains(&Position {x: x, y: y, z: z}) {
                            active_total += 1;
                        }
                    }
                }
            }
        }

        active_total
    }

    fn cycle_state_once(&mut self) {
        // Bounding box containing all active cubes.
        let active_box = self.get_grid_limits();

//         println!("Bounding box == {:?}", &active_box);

        let mut new_state = HashSet::new();
        for z in active_box.0.z-1..=active_box.1.z+1 {
            for y in active_box.0.y-1..=active_box.1.y+1 {
                for x in active_box.0.x-1..=active_box.1.x+1 {
                    let p = Position {x: x, y: y, z: z};
                    let currently_active = self.active_cubes.contains(&p);
                    let active_adjacent = self.active_adjacent_cubes(&p);

//                     println!("{:?} is active == {}. Active adjacent = {}", &p, currently_active,
//                         active_adjacent);

                    if currently_active {
                        if (active_adjacent == 2) || (active_adjacent == 3) {
//                             println!("{:?} is active and remains so.", &p);
                            new_state.insert(p);
                        }
                    } else {
                        if active_adjacent == 3 {
//                             println!("{:?} is inactive but becomes active.", &p);
                            new_state.insert(p);
                        }
                    }
                }
            }
        }
        self.active_cubes = new_state;
    }

    fn cycle_states(&mut self, rounds: u16) {
        for _ in 0..rounds {
            self.cycle_state_once();
        }
    }
}


fn main() {
    let input_file =
        fs::read_to_string(INPUT_FILENAME)
            .expect("Error reading input file");

    let mut grid = CubeGrid::from_str(&[&input_file]);
    grid.cycle_states(6);

    println!("The answer to the challenge is {:?}", grid.active_cubes.len());
}


// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: [&str; 1] = ["\
..#..
...#.
.###.
....."];

    const TEST_INPUT_ROUND_1: [&str; 3] = ["\
#..
..#
.#.",
"\
#.#
.##
.#.",
"\
#..
..#
.#."];

    const TEST_INPUT_ROUND_2: [&str; 5] = ["\
.....
.....
..#..
.....
.....",
"\
..#..
.#..#
....#
.#...
.....",
"\
##...
##...
#....
....#
.###.",
"\
..#..
.#..#
....#
.#...
.....",
"\
.....
.....
..#..
.....
....."];

    const TEST_INPUT_ROUND_3: [&str; 5] = ["\
.......
.......
..##...
..###..
.......
.......
.......",
"\
..#....
...#...
#......
.....##
.#...#.
..#.#..
...#...",
"\
...#...
.......
#......
.......
.....##
.##.#..
...#...",
"\
..#....
...#...
#......
.....##
.#...#.
..#.#..
...#...",
"\
.......
.......
..##...
..###..
.......
.......
......."];


    #[test]
    fn initialize_grid() {
        let grid = CubeGrid::from_str(&TEST_INPUT);
        println!("Result\n{}", &grid);

        assert_eq!(grid.active_cubes.len(), 5);
        assert!(grid.active_cubes.contains(&Position {x: -1, y: 0, z: 0}));
        assert!(grid.active_cubes.contains(&Position {x: 0, y: -2, z: 0}));
        assert!(grid.active_cubes.contains(&Position {x: 0, y: 0, z: 0}));
        assert!(grid.active_cubes.contains(&Position {x: 1, y: -1, z: 0}));
        assert!(grid.active_cubes.contains(&Position {x: 1, y: 0, z: 0}));
    }

    #[test]
    fn initialize_grid_1() {
        let grid = CubeGrid::from_str(&TEST_INPUT_ROUND_1);
        println!("Result\n{}", &grid);

        assert_eq!(grid.active_cubes.len(), 11);
        assert!(grid.active_cubes.contains(&Position { x: -1, y: -1, z: -1 }));
        assert!(grid.active_cubes.contains(&Position { x: 1, y: 0, z: -1 }));
        assert!(grid.active_cubes.contains(&Position { x: 0, y: 1, z: -1 }));
        assert!(grid.active_cubes.contains(&Position { x: -1, y: -1, z: 0 }));
        assert!(grid.active_cubes.contains(&Position { x: 1, y: -1, z: 0 }));
        assert!(grid.active_cubes.contains(&Position { x: 0, y: 0, z: 0 }));
        assert!(grid.active_cubes.contains(&Position { x: 1, y: 0, z: 0 }));
        assert!(grid.active_cubes.contains(&Position { x: 0, y: 1, z: 0 }));
        assert!(grid.active_cubes.contains(&Position { x: -1, y: -1, z: 1 }));
        assert!(grid.active_cubes.contains(&Position { x: 1, y: 0, z: 1 }));
        assert!(grid.active_cubes.contains(&Position { x: 0, y: 1, z: 1 }));
    }

    #[test]
    fn grid_round_1() {
        let mut grid = CubeGrid::from_str(&TEST_INPUT);
        grid.cycle_state_once();

        let expected = CubeGrid::from_str(&TEST_INPUT_ROUND_1);

        assert_eq!(grid, expected);
    }

    #[test]
    fn grid_round_2() {
        let mut grid = CubeGrid::from_str(&TEST_INPUT);
        grid.cycle_states(2);

        let expected = CubeGrid::from_str(&TEST_INPUT_ROUND_2);

        println!("Result\n{}", &grid);
        println!("Expected\n{}", &expected);

        assert_eq!(grid, expected);
    }

    #[test]
    fn grid_round_3() {
        let mut grid = CubeGrid::from_str(&TEST_INPUT);
        grid.cycle_states(3);

        let expected = CubeGrid::from_str(&TEST_INPUT_ROUND_3);

        println!("Result\n{}", &grid);
        println!("Expected\n{}", &expected);

        assert_eq!(grid, expected);
    }

    #[test]
    fn grid_round_6() {
        let mut grid = CubeGrid::from_str(&TEST_INPUT);
        grid.cycle_states(6);

        assert_eq!(grid.active_cubes.len(), 112);
    }

    #[test]
    fn grid_eq_trait() {
        let grid0 = CubeGrid::from_str(&TEST_INPUT);
        let mut h = HashSet::new();

        println!("Result\n{}", &grid0);

        h.insert(Position {x: -1, y: 0, z: 0});
        h.insert(Position {x: 0, y: -2, z: 0});
        h.insert(Position {x: 0, y: 0, z: 0});
        h.insert(Position {x: 1, y: -1, z: 0});
        h.insert(Position {x: 1, y: 0, z: 0});

        assert_eq!(grid0, CubeGrid { active_cubes: h } );
    }

    #[test]
    fn grid_eq_trait_ne_diff_position() {
        let grid0 = CubeGrid::from_str(&TEST_INPUT);
        let mut h = HashSet::new();

        h.insert(Position {x: -1, y: 1, z: 0});
        h.insert(Position {x: 0, y: -1, z: 0});
        h.insert(Position {x: 0, y: 1, z: 0});
        h.insert(Position {x: 1, y: 0, z: 0});
        h.insert(Position {x: 1, y: 1, z: 999});

        assert_ne!(grid0, CubeGrid { active_cubes: h } );
    }

    #[test]
    fn grid_eq_trait_ne_shorter() {
        let grid0 = CubeGrid::from_str(&TEST_INPUT);
        let mut h = HashSet::new();

        h.insert(Position {x: -1, y: 1, z: 0});
        h.insert(Position {x: 0, y: -1, z: 0});
        h.insert(Position {x: 0, y: 1, z: 0});
        h.insert(Position {x: 1, y: 0, z: 0});

        assert_ne!(grid0, CubeGrid { active_cubes: h } );
    }
}
