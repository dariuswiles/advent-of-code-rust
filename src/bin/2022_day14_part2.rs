//! Advent of Code 2022 Day 14
//! https://adventofcode.com/2022/day/14
//!
//! Challenge part 1
//!
//! Creates a 2D grid of rock based on the input data. Then models sand falling into the grid from
//! the top following the rules specified in the challenge to determine how much sand falls before
//! it backs up to the cell where sand enters the grid. The amount of sand is the challenge answer.

use std::collections::HashMap;
use std::fmt::{self, Display};
use std::fs;
use std::ops::RangeInclusive;

const INPUT_FILENAME: &str = "2022_day14_input.txt";
const INPUT_SEPARATOR: &str = " -> ";
const SAND_PRODUCTION_POINT: Point = Point { x: 500, y: 0 };

type Axis = u16;

/// Possible contents of a cell. The default is `Air`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum Cell {
    #[default]
    Air,
    Rock,
    Sand,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: Axis,
    y: Axis,
}

impl Point {
    /// Returns a `Point` created from the string passed. The string must be a pair of comma-
    /// separated integers, e.g., "500,0".
    ///
    /// # Panics
    ///
    /// Panics if the input string is not in this format.
    fn from_str(s: &str) -> Self {
        let tokens: Vec<&str> = s.split(',').collect();
        assert_eq!(tokens.len(), 2, "Error during parsing of x,y pair in input");

        let x = tokens[0].parse().unwrap();
        let y = tokens[1].parse().unwrap();

        Self { x, y }
    }
}

/// Maps `Point`s to their associated `Cell` contents. Records the `floor` which runs horizontally
/// two rows belong the lowest row containing rock (lowest means the furthest down, which will have
/// the highest integer row number). Its value will not be correct until all rock has been placed.
struct Grid {
    cells: HashMap<Point, Cell>,
    floor: Axis,
}

impl Grid {
    /// Returns a new empty `Grid`.
    fn new() -> Self {
        Self {
            cells: HashMap::new(),
            floor: 0,
        }
    }

    /// Returns a new `Grid` containing rocks at the cell positions given in the input string
    /// passed.
    fn from_input_str(input: &str) -> Grid {
        let mut grid = Grid::new();

        for line in input.lines() {
            if line.is_empty() {
                continue;
            }

            let mut p_previous = None;
            for p_str in line.split(INPUT_SEPARATOR) {
                let p = Point::from_str(p_str);

                if let Some(prev) = p_previous {
                    grid.add_line(&prev, &p, Cell::Rock);
                }
                p_previous = Some(p);
            }
        }
        grid
    }

    fn get(&self, p: &Point) -> Cell {
        *self.cells.get(p).unwrap_or(&Cell::Air)
    }

    fn set(&mut self, p: Point, value: Cell) {
        self.cells.insert(p, value);

        if value == Cell::Rock {
            self.floor = Axis::max(self.floor, p.y + 2);
        }
    }

    /// Returns an inclusive range over the x-coordinates of all `Cell`s defined in this object, or
    /// `None` if no cells have yet been defined.
    fn range_x(&self) -> Option<RangeInclusive<Axis>> {
        if self.cells.is_empty() {
            return None;
        }

        Some(RangeInclusive::new(
            self.cells.keys().map(|p| p.x).min().unwrap(),
            self.cells.keys().map(|p| p.x).max().unwrap(),
        ))
    }

    /// Returns an inclusive range over the y-coordinates of all `Cell`s defined in this object, or
    /// `None` if no cells have yet been defined.
    fn range_y(&self) -> Option<RangeInclusive<Axis>> {
        if self.cells.is_empty() {
            return None;
        }

        Some(RangeInclusive::new(
            self.cells.keys().map(|p| p.y).min().unwrap(),
            self.cells.keys().map(|p| p.y).max().unwrap(),
        ))
    }

    /// Creates a line of the given type of `Cell` in `self`, from the `start` point to the `end`
    /// inclusive. The line must be either exactly horizontal or exactly vertical. `start` and
    /// `end` can be specified in either order.
    fn add_line(&mut self, start: &Point, end: &Point, value: Cell) {
        if start.x == end.x {
            let y_min = Axis::min(start.y, end.y);
            let y_max = Axis::max(start.y, end.y);

            for y in y_min..=y_max {
                self.set(Point { x: start.x, y }, value);
            }
        } else if start.y == end.y {
            let x_min = Axis::min(start.x, end.x);
            let x_max = Axis::max(start.x, end.x);

            for x in x_min..=x_max {
                self.set(Point { x, y: start.y }, value);
            }
        } else {
            panic!("Error: lines cannot be diagonal.");
        }
    }
}

/// Displays this `Grid` in the format used by the challenge.
impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..=*(self
            .range_y()
            .expect("Error: cannot display an empty Grid")
            .end())
        {
            let mut contents = String::new();

            for column in self.range_x().unwrap() {
                match self.get(&Point { x: column, y: row }) {
                    Cell::Air => {
                        if (Point { x: column, y: row }) == SAND_PRODUCTION_POINT {
                            contents.push('+');
                        } else {
                            contents.push('.');
                        }
                    }
                    Cell::Rock => {
                        contents.push('#');
                    }
                    Cell::Sand => {
                        contents.push('o');
                    }
                }
            }
            let result = writeln!(f, "{}", contents);
            result?;
        }

        Ok(())
    }
}

/// Modifies `grid` to simulate the introduction of one cell of sand at `SAND_PRODUCTION_POINT`.
/// As per the challenge rules, the introduced sand tries to move by following the rules in the
/// order below.
///     - It falls one cell down if that cell is empty.
///     - It falls one cell down and to the left if that cell is empty.
///     - It falls one cell down and to the right if that cell is empty.
///     - It comes to rest at its present location.
///
/// Returns `true` if the sand falls before coming to rest within the grid, and `false` if the
/// sand fails to move beyond its starting cell.
fn drop_one_sand(grid: &mut Grid) -> bool {
    let mut position = SAND_PRODUCTION_POINT;

    loop {
        let mut next_position = position;
        next_position.y += 1;

        if next_position.y == grid.floor {
            break;
        }

        if grid.get(&next_position) == Cell::Air {
            position = next_position;
            continue;
        }

        next_position.x -= 1;
        if grid.get(&next_position) == Cell::Air {
            position = next_position;
            continue;
        }

        next_position.x += 2;
        if grid.get(&next_position) == Cell::Air {
            position = next_position;
            continue;
        }

        break;
    }

    grid.set(position, Cell::Sand);

    position != SAND_PRODUCTION_POINT
}

/// Repeatedly drops sand until a sand fails to move beyond its starting cell. Returns the number
/// of cells of sand that settle before this end state is reached.
fn drop_sand(grid: &mut Grid) -> usize {
    let mut settled_sand = 1;

    while drop_one_sand(grid) {
        settled_sand += 1;
    }

    settled_sand
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    let mut grid = Grid::from_input_str(&input_file);

    println!(
        "The number of cells of sand that come to rest is {}",
        drop_sand(&mut grid)
    );
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    /// Drops `count` cells of sand. See `drop_one_sand()` for parameters and return value.
    fn drop_multiple_sand(grid: &mut Grid, count: usize) -> bool {
        for _ in 0..count {
            if !drop_one_sand(grid) {
                return false;
            }
        }
        true
    }

    const TEST_INPUT: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    const EXPECTED_OUTPUT_SIMPLE: &str = "\
..+...
......
......
......
#....#
#.....
#.....
";

    const EXPECTED_OUTPUT_0: &str = "\
......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
........#.
#########.
";

    const EXPECTED_OUTPUT_TURN_1: &str = "\
......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
......o.#.
#########.
";

    const EXPECTED_OUTPUT_TURN_2: &str = "\
......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
.....oo.#.
#########.
";

    const EXPECTED_OUTPUT_TURN_5: &str = "\
......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
......o.#.
....oooo#.
#########.
";

    const EXPECTED_OUTPUT_TURN_22: &str = "\
......+...
..........
......o...
.....ooo..
....#ooo##
....#ooo#.
..###ooo#.
....oooo#.
...ooooo#.
#########.
";

    const EXPECTED_OUTPUT_TURN_24: &str = "\
......+...
..........
......o...
.....ooo..
....#ooo##
...o#ooo#.
..###ooo#.
....oooo#.
.o.ooooo#.
#########.
";

    // Edited from challenge to remove two-cell border on both left and right edges, and the row of
    // rock at the bottom edge.
    const EXPECTED_OUTPUT_PART_2: &str = "\
..........o..........
.........ooo.........
........ooooo........
.......ooooooo.......
......oo#ooo##o......
.....ooo#ooo#ooo.....
....oo###ooo#oooo....
...oooo.oooo#ooooo...
..oooooooooo#oooooo..
.ooo#########ooooooo.
ooooo.......ooooooooo
";

    #[test]
    fn test_set() {
        let mut grid = Grid::new();
        grid.set(Point { x: 498, y: 4 }, Cell::Rock);
        grid.set(Point { x: 498, y: 5 }, Cell::Rock);
        grid.set(Point { x: 498, y: 6 }, Cell::Rock);

        assert_eq!(grid.cells.len(), 3);
        assert_eq!(grid.get(&Point { x: 498, y: 4 }), Cell::Rock);
        assert_eq!(grid.get(&Point { x: 498, y: 5 }), Cell::Rock);
        assert_eq!(grid.get(&Point { x: 498, y: 6 }), Cell::Rock);
        assert_eq!(grid.get(&Point { x: 99, y: 9 }), Cell::Air);
    }

    #[test]
    fn test_grid_display_simple() {
        let mut grid = Grid::new();
        grid.set(Point { x: 498, y: 4 }, Cell::Rock);
        grid.set(Point { x: 498, y: 5 }, Cell::Rock);
        grid.set(Point { x: 498, y: 6 }, Cell::Rock);
        grid.set(Point { x: 503, y: 4 }, Cell::Rock);

        assert_eq!(&format!("{}", grid), EXPECTED_OUTPUT_SIMPLE);
    }

    #[test]
    fn test_add_line() {
        let mut grid = Grid::new();
        grid.add_line(&Point { x: 498, y: 4 }, &Point { x: 498, y: 6 }, Cell::Rock);
        grid.add_line(&Point { x: 498, y: 6 }, &Point { x: 496, y: 6 }, Cell::Rock);
        grid.add_line(&Point { x: 503, y: 4 }, &Point { x: 502, y: 4 }, Cell::Rock);
        grid.add_line(&Point { x: 502, y: 4 }, &Point { x: 502, y: 9 }, Cell::Rock);
        grid.add_line(&Point { x: 502, y: 9 }, &Point { x: 494, y: 9 }, Cell::Rock);

        assert_eq!(&format!("{}", grid), EXPECTED_OUTPUT_0);
    }

    #[test]
    fn test_input_parsing() {
        let grid = Grid::from_input_str(TEST_INPUT);

        assert_eq!(&format!("{}", grid), EXPECTED_OUTPUT_0);
    }

    #[test]
    fn test_drop_one_sand() {
        let mut grid = Grid::from_input_str(TEST_INPUT);

        assert!(drop_one_sand(&mut grid));
        assert_eq!(&format!("{}", grid), EXPECTED_OUTPUT_TURN_1);

        assert!(drop_one_sand(&mut grid));
        assert_eq!(&format!("{}", grid), EXPECTED_OUTPUT_TURN_2);

        assert!(drop_multiple_sand(&mut grid, 3));
        assert_eq!(&format!("{}", grid), EXPECTED_OUTPUT_TURN_5);

        assert!(drop_multiple_sand(&mut grid, 17));
        assert_eq!(&format!("{}", grid), EXPECTED_OUTPUT_TURN_22);

        assert!(drop_multiple_sand(&mut grid, 2));
        assert_eq!(&format!("{}", grid), EXPECTED_OUTPUT_TURN_24);
    }

    #[test]
    fn test_drop_sand() {
        let mut grid = Grid::from_input_str(TEST_INPUT);
        assert_eq!(drop_sand(&mut grid), 93);
        assert_eq!(&format!("{}", grid), EXPECTED_OUTPUT_PART_2);
    }
}
