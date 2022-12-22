//! Advent of Code 2022 Day 05
//! https://adventofcode.com/2022/day/5
//!
//! Challenge part 1
//!
//! Reads an input file containing two sections. The first is the initial state of a number of
//! stacks of crates. The second is a list of instructions, one per line, moving crates between
//! stacks. Crates are moved between stacks per the instructions, yielding the challenge answer
//! which is a list of the crates on top of each of the stacks.

use std::fs;

const INPUT_FILENAME: &str = "2022_day05_input.txt";

type Crate = char;
type Stack = Vec<Crate>;

/// Holds stacks of crates. Each stack begins at the crate at ground level. The first stack is
/// never used so that the stacks `Vec` index matches the stack numbering used in the challenge,
/// where the first stack is #1.
#[derive(Clone, Debug, PartialEq)]
struct Stacks {
    stacks: Vec<Stack>,
}

impl Stacks {
    /// Takes a multi-line string containing the initial layout of crates on stacks. Example:
    ///     [D]
    /// [N] [C]
    /// [Z] [M] [P]
    ///  1   2   3
    ///
    ///
    /// # Panics
    ///
    /// Panics if the input is malformed.
    fn new(input: &str) -> Self {
        let mut rows = Vec::new();

        for line in input.lines() {
            if line == "" {
                break;
            }

            let mut row = Vec::new();
            for i in (0..line.len()).step_by(4) {
                let slice = &line[i..i + 3];

                if slice == "   " {
                    row.push(None);
                } else if slice.starts_with(' ') {
                    break; // The row containing column ids is unneeded and not parsed
                } else if slice.starts_with('[') {
                    assert!(slice.ends_with(']'));

                    let stack_crate = slice.chars().nth(1).unwrap();
                    row.push(Some(stack_crate));
                } else {
                    panic!("    Unrecognized input: '{}'", slice);
                }
            }

            if row.len() > 0 {
                rows.push(row);
            }
        }

        // At this point `rows` contains a representation of the input data. The following code
        // changes this to a column-based representation.

        let num_columns = rows[0].len();
        assert!(rows.iter().all(|r| r.len() == num_columns)); // Check all rows are same length

        let mut stacks = Vec::new();
        stacks.push(Vec::new()); // Add an unused column "0" so column numbering begins at 1.

        for c in 0..num_columns {
            let mut stack = Vec::new();
            for r in (0..rows.len()).rev() {
                if let Some(sc) = rows[r][c] {
                    stack.push(sc);
                }
            }
            stacks.push(stack);
        }

        Self { stacks }
    }

    /// Transfers the crate on top of stack `from` to the top of stack `to`.
    ///
    /// # Panics
    ///
    /// Panics if `from` is empty or if `from` or `to` are out of bounds.
    fn move_crate(&mut self, from: usize, to: usize) {
        assert!(to != 0);
        let stack_crate = self.stacks[from].pop().unwrap();
        self.stacks[to].push(stack_crate);
    }

    /// Transfers `count` crates from the top of stack `from` to the top of stack `to`. Crates are
    /// moved one at a time, so will end up stacked in the reverse of their initial order.
    ///
    /// # Panics
    ///
    /// Panics if `from` does not have `count` crates, or if `from` or `to` are out of bounds.
    fn move_crates(&mut self, m: &Move) {
        assert!(m.to_stack != 0);
        for _ in 0..m.num_crates {
            self.move_crate(m.from_stack, m.to_stack);
        }
    }

    /// Returns a string containing the letter of the crate at the top of each stack, as required
    /// by part 1 of the challenge.
    ///
    /// # Panics
    ///
    /// Panics if any of the stacks are empty.
    //
    // Stack 0 is not included because it is unused. It is only present so that the indexing of
    // other stacks begins at 1, as required by the challenge.
    fn top_crates_to_string(&self) -> String {
        self.stacks[1..]
            .iter()
            .map(|s| *s.last().unwrap())
            .collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Move {
    num_crates: usize,
    from_stack: usize,
    to_stack: usize,
}

impl Move {
    /// Creates a new `Move` object by parsing the string passed which must be of the form:
    /// move 1 from 2 to 1
    /// where the first number is the number of crates to move, and the other numbers are the
    /// stacks to move the crates from and to respectively.
    ///
    /// # Panics
    ///
    /// Panics if the input is malformed.
    fn new(input: &str) -> Self {
        let mut tokens = input.split(" ");

        assert_eq!(tokens.next(), Some("move"));
        let num_crates = usize::from_str_radix(tokens.next().unwrap(), 10).unwrap();
        assert_eq!(tokens.next(), Some("from"));
        let from_stack = usize::from_str_radix(tokens.next().unwrap(), 10).unwrap();
        assert_eq!(tokens.next(), Some("to"));
        let to_stack = usize::from_str_radix(tokens.next().unwrap(), 10).unwrap();

        Self {
            num_crates,
            from_stack,
            to_stack,
        }
    }
}

/// Converts the input string passed into a `Vec` of `Move` objects.
fn parse_moves(input: &str) -> Vec<Move> {
    let mut moves = Vec::new();
    for line in input.lines() {
        if line != "" {
            moves.push(Move::new(line));
        }
    }
    moves
}

/// Converts a string containing the entire input file into its representation of the initial
/// state of the crates as the first value of a pair, and the requested moves as the second.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> (Stacks, Vec<Move>) {
    let part: Vec<&str> = input.split("\n\n").collect();

    (Stacks::new(part[0]), parse_moves(part[1]))
}

/// Executes all the crate movements in `moves` by modififying the crates in the `stacks` object
/// passed.
fn make_moves(stacks: &mut Stacks, moves: &Vec<Move>) {
    for m in moves {
        stacks.move_crates(&m);
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    let (mut stacks, moves) = parse_input(&input);
    make_moves(&mut stacks, &moves);

    println!("The challenge answer is {}", stacks.top_crates_to_string());
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = concat!(
        "    [D]    \n",
        "[N] [C]    \n",
        "[Z] [M] [P]\n",
        " 1   2   3 \n",
        "\n",
        "move 1 from 2 to 1\n",
        "move 3 from 1 to 3\n",
        "move 2 from 2 to 1\n",
        "move 1 from 1 to 2\n",
    );

    #[test]
    fn test_input_parsing() {
        let stacks = Stacks::new(&TEST_INPUT);

        assert_eq!(
            stacks,
            Stacks {
                stacks: vec![vec![], vec!['Z', 'N',], vec!['M', 'C', 'D'], vec!['P'],]
            }
        );
    }

    #[test]
    #[should_panic]
    fn test_different_row_lengths() {
        Stacks::new(concat!(
            "    [D]    \n",
            "[N] [C]\n", // short row
            "[Z] [M] [P]\n",
            " 1   2   3 \n",
            "\n",
            "move 1 from 2 to 1\n"
        ));
    }

    #[test]
    fn test_create_move() {
        let m = Move::new("move 4 from 3 to 7");

        assert_eq!(
            m,
            Move {
                num_crates: 4,
                from_stack: 3,
                to_stack: 7,
            }
        );
    }

    #[test]
    fn test_parse_moves() {
        let m = parse_moves("move 4 from 3 to 7\nmove 1 from 4 to 9");

        assert_eq!(m.len(), 2);
        assert_eq!(
            m[0],
            Move {
                num_crates: 4,
                from_stack: 3,
                to_stack: 7,
            }
        );
        assert_eq!(
            m[1],
            Move {
                num_crates: 1,
                from_stack: 4,
                to_stack: 9,
            }
        );
    }

    #[test]
    fn test_parse_input() {
        let (stacks, moves) = parse_input(&TEST_INPUT);

        assert_eq!(
            stacks,
            Stacks {
                stacks: vec![vec![], vec!['Z', 'N',], vec!['M', 'C', 'D'], vec!['P'],]
            }
        );
        assert_eq!(
            moves,
            vec![
                Move {
                    num_crates: 1,
                    from_stack: 2,
                    to_stack: 1,
                },
                Move {
                    num_crates: 3,
                    from_stack: 1,
                    to_stack: 3,
                },
                Move {
                    num_crates: 2,
                    from_stack: 2,
                    to_stack: 1,
                },
                Move {
                    num_crates: 1,
                    from_stack: 1,
                    to_stack: 2,
                },
            ]
        );
    }

    #[test]
    fn test_move_crate() {
        let (mut stacks, _moves) = parse_input(&TEST_INPUT);

        stacks.move_crate(1, 3);
        stacks.move_crate(1, 3);
        assert_eq!(
            stacks,
            Stacks {
                stacks: vec![vec![], vec![], vec!['M', 'C', 'D'], vec!['P', 'N', 'Z'],]
            }
        );

        stacks.move_crate(3, 2);
        stacks.move_crate(3, 1);
        stacks.move_crate(2, 1);
        stacks.move_crate(2, 1);
        assert_eq!(
            stacks,
            Stacks {
                stacks: vec![vec![], vec!['N', 'Z', 'D'], vec!['M', 'C'], vec!['P'],]
            }
        );
    }

    #[test]
    fn test_move_crates() {
        let (mut stacks, _moves) = parse_input(&TEST_INPUT);

        stacks.move_crates(&Move {
            num_crates: 2,
            from_stack: 1,
            to_stack: 3,
        });
        assert_eq!(
            stacks,
            Stacks {
                stacks: vec![vec![], vec![], vec!['M', 'C', 'D'], vec!['P', 'N', 'Z'],]
            }
        );

        stacks.move_crates(&Move {
            num_crates: 1,
            from_stack: 3,
            to_stack: 2,
        });
        stacks.move_crates(&Move {
            num_crates: 1,
            from_stack: 3,
            to_stack: 1,
        });
        stacks.move_crates(&Move {
            num_crates: 2,
            from_stack: 2,
            to_stack: 1,
        });
        assert_eq!(
            stacks,
            Stacks {
                stacks: vec![vec![], vec!['N', 'Z', 'D'], vec!['M', 'C'], vec!['P'],]
            }
        );
    }

    #[test]
    fn test_make_moves() {
        let (mut stacks, moves) = parse_input(&TEST_INPUT);
        make_moves(&mut stacks, &moves);
        assert_eq!(
            stacks,
            Stacks {
                stacks: vec![vec![], vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z'],]
            }
        );
    }

    #[test]
    fn test_top_crates_to_string() {
        let (mut stacks, moves) = parse_input(&TEST_INPUT);
        make_moves(&mut stacks, &moves);
        assert_eq!(stacks.top_crates_to_string(), "CMZ");
    }
}
