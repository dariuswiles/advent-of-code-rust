//! Advent of Code 2020 Day 05
//! https://adventofcode.com/2020/day/5
//!
//! Challenge part 1
//!
//! Read seating information from the input file, calculate the `seat ID` for each input row, and
//! display the highest `seat ID`.

use std::fs;

const INPUT_FILENAME: &str = "2020_day05_input.txt";

#[warn(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
struct Seat {
    row: u16,
    column: u16,
    seat_id: u16,
}

// Returns the `row`, `column` and `seat ID` of a seat as a tuple, given the 10 digit binary
// partitioning code for the seat.
fn find_seat(line_num: usize, line: &str) -> Seat {
    let mut row_limit_front = 0;
    let mut row_limit_back = 127;
    for c in line[..7].chars() {
        let mid = (row_limit_front + row_limit_back + 1) / 2;

        match c {
            'F' => {
                row_limit_back = mid - 1;
                // print!("Front half of current range chosen. ");
            }
            'B' => {
                row_limit_front = mid;
                // print!("Rear half of current range chosen. ");
            }
            _ => {
                panic!(
                    "Unrecognized character in row selection of line {}",
                    &line_num
                );
            }
        }
        // println!("Row seating range is now {} to {}", row_limit_front, row_limit_back);
    }

    let mut col_limit_left = 0;
    let mut col_limit_right = 7;
    for c in line[7..].chars() {
        let mid = (col_limit_left + col_limit_right + 1) / 2;

        match c {
            'L' => {
                col_limit_right = mid - 1;
                // print!("Left half of current range chosen. ");
            }
            'R' => {
                col_limit_left = mid;
                // print!("Right half of current range chosen. ");
            }
            _ => {
                panic!(
                    "Unrecognized character in column selection of line {}",
                    &line_num
                );
            }
        }
        // println!("Column seating range is now {} to {}", col_limit_right, col_limit_right);
    }

    let seat_id = row_limit_front * 8 + col_limit_left;
    // println!("Seat is in row {} and col {} with seat id {}", row_limit_front, col_limit_left,
    //     seat_id
    // );

    Seat {
        row: row_limit_front,
        column: col_limit_left,
        seat_id: seat_id,
    }
}

fn find_highest_seat_id(input: &str) -> u16 {
    let mut highest_seat_id = 0;

    for (line_num, line) in input.lines().enumerate() {
        if line == "" {
            continue;
        };

        let seat = find_seat(line_num, &line);

        if seat.seat_id > highest_seat_id {
            highest_seat_id = seat.seat_id;
            // println!("highest_seat_id is now {}", highest_seat_id);
        }
    }

    highest_seat_id
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let highest_seat_id = find_highest_seat_id(&input);
    println!(
        "The highest seat ID present in the input data is {}",
        highest_seat_id
    );
}

// Test data is from the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_0() {
        assert_eq!(
            find_seat(0, "FBFBBFFRLR"),
            Seat {
                row: 44,
                column: 5,
                seat_id: 357
            }
        );
    }

    #[test]
    fn test_data_1() {
        assert_eq!(
            find_seat(0, "BFFFBBFRRR"),
            Seat {
                row: 70,
                column: 7,
                seat_id: 567
            }
        );
    }

    #[test]
    fn test_data_2() {
        assert_eq!(
            find_seat(0, "FFFBBBFRRR"),
            Seat {
                row: 14,
                column: 7,
                seat_id: 119
            }
        );
    }

    #[test]
    fn test_data_3() {
        assert_eq!(
            find_seat(0, "BBFFBBFRLL"),
            Seat {
                row: 102,
                column: 4,
                seat_id: 820
            }
        );
    }
}
