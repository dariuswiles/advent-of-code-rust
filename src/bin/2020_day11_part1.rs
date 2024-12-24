//! Advent of Code 2020 Day 11
//! https://adventofcode.com/2020/day/11
//!
//! Challenge part 1
//!
//! Repeatedly apply a set of rules to a seating plan until it remains the same for two iterations.

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
            if line.is_empty() {
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

    /// Returns how many of the 8 seats adjacent to the given seat are occupied.
    fn occupied_adjacent_seats(&self, row: usize, col: usize) -> u32 {
        let top = if row > 0 { row - 1 } else { row };
        let left = if col > 0 { col - 1 } else { col };
        let bottom = if row < self.seats.len() - 1 {
            row + 1
        } else {
            row
        };
        let right = if col < self.seats[0].len() - 1 {
            col + 1
        } else {
            col
        };

        let mut occupied_total = 0;
        for r in top..=bottom {
            for c in left..=right {
                if (r == row) && (c == col) {
                    continue;
                }

                if let Cell::Seat(SeatState::Occupied) = self.seats[r][c] {
                    occupied_total += 1;
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
                        if self.occupied_adjacent_seats(r, c) == 0 {
                            new_grid.seats[r][c] = Cell::Seat(SeatState::Occupied);
                        }
                    }
                    Cell::Seat(SeatState::Occupied) => {
                        if self.occupied_adjacent_seats(r, c) >= 4 {
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
#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##";

    const TEST_INPUT_3: &str = "\
#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##";

    const TEST_INPUT_4: &str = "\
#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##";

    const TEST_INPUT_5: &str = "\
#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##";

    #[test]
    fn test_0() {
        let sg = SeatingGrid::from_str(TEST_INPUT_0);

        assert_eq!(sg.seats[0][0], Cell::Seat(SeatState::Empty));
        assert_eq!(sg.seats[0][4], Cell::Floor);
        assert_eq!(sg.seats[6][1], Cell::Floor);
    }

    #[test]
    fn test_1() {
        let sg = SeatingGrid::from_str(TEST_INPUT_0);
        let sg_round_1 = sg.apply_rules_once();

        let sg_expected_1 = SeatingGrid::from_str(TEST_INPUT_1);
        assert_eq!(&sg_round_1, &sg_expected_1);
    }

    #[test]
    fn test_2() {
        let sg = SeatingGrid::from_str(TEST_INPUT_0);
        let sg_round_2 = sg.apply_rules_once().apply_rules_once();

        let sg_expected_2 = SeatingGrid::from_str(TEST_INPUT_2);
        assert_eq!(&sg_round_2, &sg_expected_2);
    }

    #[test]
    fn test_3() {
        let sg = SeatingGrid::from_str(TEST_INPUT_0);
        let sg_round_3 = sg.apply_rules_once().apply_rules_once().apply_rules_once();

        let sg_expected_3 = SeatingGrid::from_str(TEST_INPUT_3);
        assert_eq!(&sg_round_3, &sg_expected_3);
    }

    #[test]
    fn test_4() {
        let sg = SeatingGrid::from_str(TEST_INPUT_0);
        let sg_round_4 = sg
            .apply_rules_once()
            .apply_rules_once()
            .apply_rules_once()
            .apply_rules_once();

        let sg_expected_4 = SeatingGrid::from_str(TEST_INPUT_4);
        assert_eq!(&sg_round_4, &sg_expected_4);
    }

    #[test]
    fn test_5() {
        let sg = SeatingGrid::from_str(TEST_INPUT_0);
        let sg_round_5 = sg
            .apply_rules_once()
            .apply_rules_once()
            .apply_rules_once()
            .apply_rules_once()
            .apply_rules_once();

        let sg_expected_5 = SeatingGrid::from_str(TEST_INPUT_5);
        assert_eq!(&sg_round_5, &sg_expected_5);
    }

    #[test]
    fn test_challenge() {
        let mut sg = SeatingGrid::from_str(TEST_INPUT_0);
        let sg_challenge = sg.apply_rules_until_stable();

        assert_eq!(sg_challenge, 37);
    }

    #[test]
    fn seating_grid_clone_and_eq() {
        let sg1 = SeatingGrid::from_str(TEST_INPUT_0);
        let sg2 = sg1.clone();

        assert_eq!(&sg1, &sg2);
    }

    #[test]
    fn seating_grid_clone_and_ne() {
        let sg1 = SeatingGrid::from_str(TEST_INPUT_0);
        let mut sg2 = sg1.clone();

        sg2.seats[2][2] = Cell::Seat(SeatState::Occupied);
        assert_ne!(&sg1, &sg2);
    }

    #[test]
    fn seating_grid_count_occupied() {
        let sg = SeatingGrid::from_str(TEST_INPUT_3);

        assert_eq!(sg.occupied_adjacent_seats(1, 6), 2);
        assert_eq!(sg.occupied_adjacent_seats(5, 9), 1);
        assert_eq!(sg.occupied_adjacent_seats(9, 0), 1);
        assert_eq!(sg.occupied_adjacent_seats(0, 4), 3);
    }

    #[test]
    fn test_count_occupied_seats_0() {
        let sg = SeatingGrid::from_str(TEST_INPUT_0);
        assert_eq!(sg.count_occupied_seats(), 0);
    }

    #[test]
    fn test_count_occupied_seats_1() {
        let sg = SeatingGrid::from_str(TEST_INPUT_1);
        assert_eq!(sg.count_occupied_seats(), 71);
    }

    #[test]
    fn test_count_occupied_seats_2() {
        let sg = SeatingGrid::from_str(TEST_INPUT_2);
        assert_eq!(sg.count_occupied_seats(), 20);
    }

    #[test]
    fn test_count_occupied_seats_3() {
        let sg = SeatingGrid::from_str(TEST_INPUT_3);
        assert_eq!(sg.count_occupied_seats(), 51);
    }

    #[test]
    fn test_count_occupied_seats_4() {
        let sg = SeatingGrid::from_str(TEST_INPUT_4);
        assert_eq!(sg.count_occupied_seats(), 30);
    }
}
