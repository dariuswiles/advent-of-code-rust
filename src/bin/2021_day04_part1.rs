//! Advent of Code 2021 Day 04
//! https://adventofcode.com/2021/day/4
//!
//! Challenge part 1
//!
//! Read a sequence of bingo numbers and several bingo cards from an input file, determine which
//! card wins and output a challenge answer based on this.

use std::fs;

const INPUT_FILENAME: &str = "2021_day04_input.txt";
const BOARD_SIZE: usize = 5;

type BingoNum = u8;

/// A bingo board containing the numbers on the board and a separate indication of which have been
/// called so far.
#[derive(Debug, PartialEq)]
struct Board {
    cells: [[BingoNum; BOARD_SIZE]; BOARD_SIZE],
    marks: [[bool; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    /// Creates a new bingo `Board` from a slice that has exactly `BOARD_SIZE` lines. Bingo
    /// numbers must be space delimited. Multiple spaces are okay.
    ///
    /// # Panics
    ///
    /// Panics if input is not exactly `BOARD_SIZE` lines long.
    /// Panics if data contains any character other than spaces or digits.
    /// Panics if any number is larger than 255.
    fn new(input: &[&str]) -> Self {
        let mut cells = [[0; BOARD_SIZE]; BOARD_SIZE];

        if input.len() != BOARD_SIZE {
            panic!(
                "Malformed input. Every board must be {} rows long.",
                BOARD_SIZE
            );
        }

        for idx in 0..input.len() {
            let number_vector: Vec<BingoNum> = input[idx]
                .split(' ')
                .filter_map(|s| s.parse().ok())
                .collect();

            for (col_idx, data) in number_vector.iter().enumerate() {
                cells[idx][col_idx] = *data;
            }
        }

        Self {
            cells,
            marks: [[false; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    /// If this `Board` contains `num`, mark it as a called number. Return `true` if this number
    /// wins the game.
    fn mark_number(&mut self, num: BingoNum) -> bool {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if self.cells[row][col] == num {
                    self.marks[row][col] = true;
                    return self.check_for_win(row, col);
                }
            }
        }

        false
    }

    /// Returns `true` if marking the number at `row` and `col` completes a row or column, thus
    /// winnig the game.
    fn check_for_win(&self, row: usize, col: usize) -> bool {
        if self.marks[row].iter().all(|b| *b) {
            return true;
        }

        for r in 0..BOARD_SIZE {
            if !self.marks[r][col] {
                return false;
            }
        }

        true
    }

    /// Returns the score for the winning board, as per the challenge rules.
    fn calculate_score(&self, winning_number: BingoNum) -> u32 {
        let mut sum = 0;
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if !self.marks[row][col] {
                    sum += self.cells[row][col] as u32;
                }
            }
        }

        sum * winning_number as u32
    }
}

/// Parses a single line into a vector of called bingo numbers.
fn parse_called_numbers(input: &str) -> Vec<BingoNum> {
    let mut called_numbers = Vec::new();

    for num in input.split(',').map(|i| i.parse().unwrap()) {
        called_numbers.push(num);
    }

    called_numbers
}

/// Parses a string consisting of a line of comma separated called names, then multiple boards.
/// Each board must be preceded by a blank line and be exactly `BOARD_SIZE` rows in length.
fn parse_input(input: &str) -> (Vec<BingoNum>, Vec<Board>) {
    let lines = input.lines().collect::<Vec<&str>>();
    let lines_len = lines.len();

    let called_numbers = parse_called_numbers(lines[0]);

    let mut boards = Vec::new();
    let mut line_idx = 1;

    while line_idx < lines_len {
        if !lines[line_idx].is_empty() {
            panic!("Malformed input. Each board must be preceded by a blank line.");
        }
        line_idx += 1;

        if line_idx >= lines_len {
            break;
        }

        boards.push(Board::new(&lines[line_idx..line_idx + BOARD_SIZE]));
        line_idx += BOARD_SIZE;
    }

    (called_numbers, boards)
}

/// Marks `called_num` on all `boards` passed. Returns None if this does not lead to any wins, or
/// the winning board otherwise.
/// NOTE Returns as soon as a winning board is found, leaving the remaining boards unmarked. This
/// is okay for part 1 of the challenge.
fn mark_all_boards(boards: &mut [Board], called_num: BingoNum) -> Option<&Board> {
    for b in boards.iter_mut() {
        let a: &mut Board = b;
        if a.mark_number(called_num) {
            return Some(a);
        }
    }
    None
}

/// Iterate through all `called_numbers` until one of the `boards` wins.
fn mark_numbers_until_win(called_numbers: Vec<BingoNum>, boards: &mut [Board]) -> Option<u32> {
    for cn in called_numbers {
        if let Some(b) = mark_all_boards(boards, cn) {
            return Some(b.calculate_score(cn));
        }
    }
    None
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let (called_numbers, mut boards) = parse_input(&input_file);
    let answer = mark_numbers_until_win(called_numbers, &mut boards).unwrap();

    println!("The challenge answer is {}", answer);
}

// Test using data from the examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    const TEST_ONE_BOARD: &str = "\
22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19";

    #[test]
    fn test_parse_called_numbers() {
        let expected = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];

        let mut input = TEST_INPUT.lines();
        assert_eq!(parse_called_numbers(input.next().unwrap()), expected);
    }

    #[test]
    fn parse_board_new() {
        let expected = Board {
            cells: [
                [22, 13, 17, 11, 0],
                [8, 2, 23, 4, 24],
                [21, 9, 14, 16, 7],
                [6, 10, 3, 18, 5],
                [1, 12, 20, 15, 19],
            ],
            marks: [[false; BOARD_SIZE]; BOARD_SIZE],
        };

        assert_eq!(
            Board::new(&TEST_ONE_BOARD.lines().collect::<Vec<&str>>()[..]),
            expected
        );
    }

    #[test]
    fn test_parse_input() {
        let expected_called_numbers = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];

        let (called_numbers, boards) = parse_input(TEST_INPUT);

        assert_eq!(called_numbers, expected_called_numbers);
        assert_eq!(boards.len(), 3);
        assert_eq!(boards[0].cells[0], [22, 13, 17, 11, 0]);
        assert_eq!(boards[1].cells[1], [9, 18, 13, 17, 5]);
        assert_eq!(boards[2].cells[3], [22, 11, 13, 6, 5]);
    }

    #[test]
    fn test_mark_board() {
        let (_, mut boards) = parse_input(TEST_INPUT);

        assert!(!boards[0].mark_number(17));
        assert!(!boards[0].mark_number(3));
        assert!(!boards[0].mark_number(14));
        assert!(!boards[0].mark_number(20));
        assert!(boards[0].mark_number(23));
    }

    #[test]
    fn challenge_answer() {
        let (called_numbers, mut boards) = parse_input(TEST_INPUT);
        assert_eq!(
            mark_numbers_until_win(called_numbers, &mut boards),
            Some(4512)
        );
    }
}
