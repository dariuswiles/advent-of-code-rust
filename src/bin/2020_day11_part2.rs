//! Advent of Code 2020 Day 11
//! https://adventofcode.com/2020/day/11
//!
//! Challenge part 2
//!
//! Repeatedly apply a set of rules to a seating plan until it remains the same for two iterations.
//! Part 2 changes the rules from looking at adjacent seats to looking in each direction until a
//! seat is found, and examining that. It also increases the number of visible occupied seats in
//! one of the rules.

use std::fs;

const INPUT_FILENAME: &str = "2020_day11_input.txt";

#[derive(Clone, Copy, Debug, PartialEq)]
enum SeatState {
    Empty,
    Occupied,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Floor,
    Seat(SeatState),
}

/// A structure to store and manipulate a grid of seats. The top-left seat has co-ordinates
/// row = 0 and col = 0.
#[derive(Debug)]
struct SeatingGrid {
    seats: Vec<Vec<Cell>>,
}

impl Clone for SeatingGrid {
    fn clone(&self) -> Self {
        let mut new_seats = Vec::new();

        for row in &self.seats {
            new_seats.push(row.clone());
        }

        Self { seats: new_seats }
    }
}

impl PartialEq for SeatingGrid {
    fn eq(&self, other: &Self) -> bool {
        if self.seats.len() != other.seats.len() {
            return false;
        }

        for r in 0..self.seats.len() {
            if self.seats[r] != other.seats[r] {
                return false;
            }
        }
        true
    }
}

impl SeatingGrid {
    fn from_str(input: &str) -> Self {
        let mut grid = Vec::new();

        for line in input.lines() {
            if line.len() == 0 {
                continue;
            }

            let mut row = Vec::new();
            for c in line.chars() {
                match c {
                    '.' => {
                        row.push(Cell::Floor);
                    }
                    'L' => {
                        row.push(Cell::Seat(SeatState::Empty));
                    }
                    '#' => {
                        row.push(Cell::Seat(SeatState::Occupied));
                    }
                    _ => {
                        panic!("Error: unexpected character in input: '{}'", c);
                    }
                }
            }
            grid.push(row);
        }
        Self { seats: grid }
    }

    /// Returns how many of the 8 seats visible from the given seat are occupied. "Visible"
    /// involves looking in each direction until reaching a seat (whether empty or occupied), or
    /// the edge of the grid.
    fn occupied_visible_seats(&self, row: usize, col: usize) -> u32 {
        let mut occupied_total = 0;
        let row_range = 0..self.seats.len() as i32;
        let col_range = 0..self.seats[0].len() as i32;

        for row_delta in -1..=1 {
            // println!("row_delta = {}", row_delta);

            for col_delta in -1..=1 {
                // println!("\tcol_delta = {}", col_delta);

                if (row_delta == 0) && (col_delta == 0) {
                    continue;
                }

                let mut r = row as i32;
                let mut c = col as i32;
                loop {
                    r += row_delta;
                    c += col_delta;

                    // println!("\t\tChecking ({},{})", r, c);

                    if !(row_range.contains(&r) && col_range.contains(&c)) {
                        break;
                    }

                    match self.seats[r as usize][c as usize] {
                        Cell::Floor => {}
                        Cell::Seat(SeatState::Empty) => {
                            // println!("\t\t\tFound empty seat at ({},{}). ", r, c);
                            break;
                        }
                        Cell::Seat(SeatState::Occupied) => {
                            occupied_total += 1;
                            // print!("\t\t\tFound occupied seat at ({},{}). ", r, c);
                            // println!("Total now {}", occupied_total);
                            break;
                        }
                    }
                }
            }
        }
        occupied_total
    }

    /// Returns the number of occupied seats in all cells of the seating plan.
    fn count_occupied_seats(&self) -> u32 {
        let mut occupied_total = 0;
        for r in 0..self.seats.len() {
            for c in 0..self.seats[0].len() {
                if let Cell::Seat(SeatState::Occupied) = self.seats[r][c] {
                    occupied_total += 1;
                }
            }
        }
        occupied_total
    }

    /// Apply the rules specified in the challenge, which are:
    /// - If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes
    ///   occupied.
    /// - If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the
    ///   seat becomes empty.
    /// - Otherwise, the seat's state does not change.
    fn apply_rules_once(&self) -> SeatingGrid {
        let mut new_grid = self.clone();

        for r in 0..self.seats.len() {
            for c in 0..self.seats[r].len() {
                match self.seats[r][c] {
                    Cell::Floor => {}
                    Cell::Seat(SeatState::Empty) => {
                        if self.occupied_visible_seats(r, c) == 0 {
                            new_grid.seats[r][c] = Cell::Seat(SeatState::Occupied);
                        }
                    }
                    Cell::Seat(SeatState::Occupied) => {
                        if self.occupied_visible_seats(r, c) >= 5 {
                            new_grid.seats[r][c] = Cell::Seat(SeatState::Empty);
                        }
                    }
                }
            }
        }
        new_grid
    }

    /// Apply rules until they no longer result in any changes. Return the number of occupied seats
    /// in the final seating arrangement.
    fn apply_rules_until_stable(&mut self) -> u32 {
        loop {
            let updated_grid = self.apply_rules_once();

            if *self == updated_grid {
                return self.count_occupied_seats();
            }

            self.seats = updated_grid.seats;
        }
    }
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let mut sg = SeatingGrid::from_str(&input_file);
    let result = sg.apply_rules_until_stable();

    println!("The answer to the challenge is {}", result);
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_VISIBILITY_0: &str = "\
.......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....";

    const TEST_VISIBILITY_1: &str = "\
.............
.L.L.#.#.#.#.
.............";

    const TEST_VISIBILITY_2: &str = "\
.##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.";

    const TEST_INPUT_0: &str = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    const TEST_INPUT_1: &str = "\
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";

    const TEST_INPUT_2: &str = "\
#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#";

    const TEST_INPUT_3: &str = "\
#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#";

    const TEST_INPUT_4: &str = "\
#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#";

    const TEST_INPUT_5: &str = "\
#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#";

    const TEST_INPUT_6: &str = "\
#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#";

    #[test]
    fn test_0() {
        let sg = SeatingGrid::from_str(&TEST_INPUT_0);

        assert_eq!(sg.seats[0][0], Cell::Seat(SeatState::Empty));
        assert_eq!(sg.seats[0][4], Cell::Floor);
        assert_eq!(sg.seats[6][1], Cell::Floor);
    }

    #[test]
    fn test_1() {
        let sg = SeatingGrid::from_str(&TEST_INPUT_0);
        let sg_round_1 = sg.apply_rules_once();

        let sg_expected_1 = SeatingGrid::from_str(&TEST_INPUT_1);
        assert_eq!(&sg_round_1, &sg_expected_1);
    }

    #[test]
    fn test_2() {
        let sg = SeatingGrid::from_str(&TEST_INPUT_0);
        let sg_round_2 = sg.apply_rules_once().apply_rules_once();

        let sg_expected_2 = SeatingGrid::from_str(&TEST_INPUT_2);
        assert_eq!(&sg_round_2, &sg_expected_2);
    }

    #[test]
    fn test_3() {
        let sg = SeatingGrid::from_str(&TEST_INPUT_0);
        let sg_round_3 = sg.apply_rules_once().apply_rules_once().apply_rules_once();

        let sg_expected_3 = SeatingGrid::from_str(&TEST_INPUT_3);
        assert_eq!(&sg_round_3, &sg_expected_3);
    }

    #[test]
    fn test_4() {
        let sg = SeatingGrid::from_str(&TEST_INPUT_0);
        let sg_round_4 = sg
            .apply_rules_once()
            .apply_rules_once()
            .apply_rules_once()
            .apply_rules_once();

        let sg_expected_4 = SeatingGrid::from_str(&TEST_INPUT_4);
        assert_eq!(&sg_round_4, &sg_expected_4);
    }

    #[test]
    fn test_5() {
        let sg = SeatingGrid::from_str(&TEST_INPUT_0);
        let sg_round_5 = sg
            .apply_rules_once()
            .apply_rules_once()
            .apply_rules_once()
            .apply_rules_once()
            .apply_rules_once();

        let sg_expected_5 = SeatingGrid::from_str(&TEST_INPUT_5);
        assert_eq!(&sg_round_5, &sg_expected_5);
    }

    #[test]
    fn test_6() {
        let sg = SeatingGrid::from_str(&TEST_INPUT_0);
        let sg_round_6 = sg
            .apply_rules_once()
            .apply_rules_once()
            .apply_rules_once()
            .apply_rules_once()
            .apply_rules_once()
            .apply_rules_once();

        let sg_expected_6 = SeatingGrid::from_str(&TEST_INPUT_6);
        assert_eq!(&sg_round_6, &sg_expected_6);
    }

    #[test]
    fn test_7() {
        let sg = SeatingGrid::from_str(&TEST_INPUT_0);
        let sg_round_7 = sg
            .apply_rules_once()
            .apply_rules_once()
            .apply_rules_once()
            .apply_rules_once()
            .apply_rules_once()
            .apply_rules_once()
            .apply_rules_once();

        // Round 7 is expected to be unchanged from round 6
        let sg_expected_6 = SeatingGrid::from_str(&TEST_INPUT_6);
        assert_eq!(&sg_round_7, &sg_expected_6);
    }

    #[test]
    fn test_challenge() {
        let mut sg = SeatingGrid::from_str(&TEST_INPUT_0);
        let sg_challenge = sg.apply_rules_until_stable();

        assert_eq!(sg_challenge, 26);
    }

    #[test]
    fn seating_grid_clone_and_eq() {
        let sg1 = SeatingGrid::from_str(&TEST_INPUT_0);
        let sg2 = sg1.clone();

        assert_eq!(&sg1, &sg2);
    }

    #[test]
    fn seating_grid_clone_and_ne() {
        let sg1 = SeatingGrid::from_str(&TEST_INPUT_0);
        let mut sg2 = sg1.clone();

        sg2.seats[2][2] = Cell::Seat(SeatState::Occupied);
        assert_ne!(&sg1, &sg2);
    }

    #[test]
    fn occupied_visibility_0() {
        let sg = SeatingGrid::from_str(&TEST_VISIBILITY_0);

        assert_eq!(sg.occupied_visible_seats(4, 3), 8);
    }

    #[test]
    fn occupied_visibility_1() {
        let sg = SeatingGrid::from_str(&TEST_VISIBILITY_1);

        assert_eq!(sg.occupied_visible_seats(1, 1), 0);
    }

    #[test]
    fn occupied_visibility_2() {
        let sg = SeatingGrid::from_str(&TEST_VISIBILITY_2);

        assert_eq!(sg.occupied_visible_seats(3, 3), 0);
    }

    #[test]
    fn seating_grid_count_occupied() {
        let sg = SeatingGrid::from_str(&TEST_INPUT_5);

        assert_eq!(sg.occupied_visible_seats(2, 5), 3);
        assert_eq!(sg.occupied_visible_seats(9, 6), 1);
        assert_eq!(sg.occupied_visible_seats(6, 7), 3);
    }

    #[test]
    fn test_count_occupied_seats_0() {
        let sg = SeatingGrid::from_str(&TEST_INPUT_0);
        assert_eq!(sg.count_occupied_seats(), 0);
    }

    #[test]
    fn test_count_occupied_seats_1() {
        let sg = SeatingGrid::from_str(&TEST_INPUT_1);
        assert_eq!(sg.count_occupied_seats(), 71);
    }

    #[test]
    fn test_count_occupied_seats_2() {
        let sg = SeatingGrid::from_str(&TEST_INPUT_2);
        assert_eq!(sg.count_occupied_seats(), 7);
    }

    #[test]
    fn test_count_occupied_seats_3() {
        let sg = SeatingGrid::from_str(&TEST_INPUT_3);
        assert_eq!(sg.count_occupied_seats(), 53);
    }

    #[test]
    fn test_count_occupied_seats_4() {
        let sg = SeatingGrid::from_str(&TEST_INPUT_4);
        assert_eq!(sg.count_occupied_seats(), 18);
    }

    #[test]
    fn test_count_occupied_seats_5() {
        let sg = SeatingGrid::from_str(&TEST_INPUT_5);
        assert_eq!(sg.count_occupied_seats(), 31);
    }
}
