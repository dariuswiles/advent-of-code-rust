//! Advent of Code 2022 Day 11
//! https://adventofcode.com/2022/day/11
//!
//! Challenge part 2
//!
//! Simulates a number of monkeys passing objects between them according to rules defining the
//! priorities of the objects and which monkeys each object is passed to. Part 2 of the challenge
//! removes a division from the formulas that results in numbers too big to hold even in 64-bit
//! variables. The solution is to apply a modulus operation that keeps the numbers sufficiently
//! small without altering the outcome of the tests applied to the numbers.

use std::fs;

const INPUT_FILENAME: &str = "2022_day11_input.txt";
const ROUNDS: usize = 10000;

type WorryLevel = u64;
type OperandInt = u64;
type MonkeyId = u8;

#[derive(Debug, PartialEq)]
enum Operand {
    Old,
    Number(OperandInt),
}

#[derive(Debug, PartialEq)]
enum Operation {
    Add(Operand),
    Multiply(Operand),
}

impl Operation {
    /// Returns a new `Operation` created from the string passed. This must take the form:
    ///       Operation: new = old <operator> <operand>
    ///
    /// where <operator> is either '*' or '+', and <operand> is either a positive integer or the
    /// string "old".
    ///
    /// # Panics
    ///
    /// Panics if the input is empty or malformed.
    fn from_str(input: &str) -> Self {
        let tokens: Vec<&str> = input.trim().split(' ').collect();

        assert_eq!(tokens[0..4], ["Operation:", "new", "=", "old"]);

        let operand = if tokens[5] == "old" {
            Operand::Old
        } else {
            Operand::Number(tokens[5].parse().unwrap())
        };

        match tokens[4] {
            "+" => Operation::Add(operand),
            "*" => Operation::Multiply(operand),
            _ => {
                panic!("Unrecognized operator {}", tokens[4]);
            }
        }
    }
}

/// Holds the information required to perform the test to see which `Monkey` an item is thrown to.
/// The "worry level" of an item is checked to see if it is divisible by `divisible_by`. If so, the
/// item is passed to the `if_true` `Monkey`, or it is otherwise passed to the `if_false` `Monkey`.
#[derive(Debug, PartialEq)]
struct MonkeyTest {
    divisible_by: OperandInt,
    if_true: MonkeyId,
    if_false: MonkeyId,
}

impl MonkeyTest {
    /// Returns a new `MonkeyTest` created from the string passed. This must be a 3-line string of
    /// the form:
    ///       Test: divisible by <positive integer>
    ///         If true: throw to monkey <MonkeyId>
    ///         If false: throw to monkey <MonkeyId>
    ///
    /// # Panics
    ///
    /// Panics if the input is empty or malformed.
    fn from_str(lines: &[&str]) -> Self {
        let definition = lines[0]
            .trim()
            .strip_prefix("Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();

        let true_id = lines[1]
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        let false_id = lines[2]
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        Self {
            divisible_by: definition,
            if_true: true_id,
            if_false: false_id,
        }
    }

    /// Returns a Boolean indicating if `worry_level` is divisable by the `divisible_by` value that
    /// is part of this `MonkeyTest`.
    fn is_divisable(&self, worry_level: WorryLevel) -> bool {
        worry_level % self.divisible_by == 0
    }
}

/// Holds information relating to a single monkey.
#[derive(Debug, PartialEq)]
struct Monkey {
    id: MonkeyId,
    items: Vec<WorryLevel>,
    operation: Operation,
    test: MonkeyTest,
    num_inspections: usize,
}

impl Monkey {
    /// Creates a new `Monkey` by parsing the string passed and returns it. The input must be of
    /// the form:
    ///     Monkey 0:
    ///       Starting items: 79, 98
    ///       Operation: new = old * 19
    ///       Test: divisible by 23
    ///         If true: throw to monkey 2
    ///         If false: throw to monkey 3
    ///
    /// # Panics
    ///
    /// Panics if the input is empty or malformed.
    fn from_str(input: &str) -> Self {
        let mut lines: Vec<&str> = input.lines().collect();

        while lines[0].is_empty() {
            lines.remove(0);
        }

        assert_eq!(
            lines.len(),
            6,
            "Monkey creation failed because input is not a 6-line block"
        );

        let id_string: &str = lines[0]
            .trim()
            .strip_prefix("Monkey ")
            .unwrap()
            .strip_suffix(':')
            .unwrap();
        let id = id_string.parse().unwrap();

        let mut items = Vec::new();
        let items_string: Vec<&str> = lines[1]
            .trim()
            .strip_prefix("Starting items: ")
            .unwrap()
            .split(", ")
            .collect();

        for item in items_string {
            items.push(item.parse().unwrap());
        }

        Monkey {
            id,
            items,
            operation: Operation::from_str(lines[2]),
            test: MonkeyTest::from_str(&lines[3..6]),
            num_inspections: 0,
        }
    }
}

/// The entire group of `Monkey`s, where the `Vec` index is each `Monkey`'s Id. The `modulus` is
/// added for part 2 of the challenge and is applied to the `WorryLevel`s to keep them from
/// becoming too big to fit in 64-bit variables.
#[derive(Debug, PartialEq)]
struct MonkeyGroup {
    monkeys: Vec<Monkey>,
    modulus: WorryLevel,
}

impl MonkeyGroup {
    /// Creates a new `Vec` of `Monkey`s by parsing the string passed and returns it. See `Monkey`
    /// for details of the expected format. Each `Monkey` needs to be separated with a blank line.
    ///
    /// The `modulus` is new for part 2 and is applied to the `WorryLevel` to prevent it becoming
    /// too big to store in a 64-bit variable. It is created by multiplying the `divisible_by`
    /// values for each of the `Monkey`s in this `MonkeyGroup`. It is safe to apply this modulus
    /// because it does not change the outcome of the tests that divide the `WorryLevel` by one of
    /// the `divisible_by` values.
    ///
    /// # Panics
    ///
    /// Panics if the input is empty or malformed.
    fn from_str(input: &str) -> Self {
        let monkey_blocks: Vec<&str> = input.split("\n\n").collect();
        let mut monkeys = Vec::new();

        for monkey_block in monkey_blocks {
            monkeys.push(Monkey::from_str(monkey_block));
        }

        let modulus = monkeys.iter().map(|m| m.test.divisible_by).product();

        Self { monkeys, modulus }
    }

    /// The monkey identified by `monkey_id` throws each of its items, starting at the item at the
    /// start of the `Monkey`'s item vector. This monkey`s item `list` is then replaced with an
    /// empty `Vec`, and the `item` vectors of two monkies receiving items is appended with the
    /// items thrown to them. The total number of items this monkey inspected is updated to keep
    /// the running total needed to calculate the challenge answer.
    fn inspect_and_throw_items(&mut self, monkey_id: MonkeyId) {
        let mut true_item_queue = Vec::new();
        let mut false_item_queue = Vec::new();

        self.monkeys[monkey_id as usize].num_inspections +=
            self.monkeys[monkey_id as usize].items.len();

        for item_worry_level in &self.monkeys[monkey_id as usize].items {
            let new_worry_level = inspect_item(
                *item_worry_level,
                &self.monkeys[monkey_id as usize].operation,
                self.modulus,
            );

            if self.monkeys[monkey_id as usize]
                .test
                .is_divisable(new_worry_level)
            {
                true_item_queue.push(new_worry_level);
            } else {
                false_item_queue.push(new_worry_level);
            }
        }

        self.monkeys[monkey_id as usize].items = Vec::new();

        let true_monkey_id = self.monkeys[monkey_id as usize].test.if_true as usize;
        self.monkeys[true_monkey_id]
            .items
            .append(&mut true_item_queue);

        let false_monkey_id = self.monkeys[monkey_id as usize].test.if_false as usize;
        self.monkeys[false_monkey_id]
            .items
            .append(&mut false_item_queue);
    }

    /// Simulates one round of item throwing, defined as allowing each monkey to throw all its
    /// items in turn, starting with `Monkey` 0.
    fn play_one_round(&mut self) {
        for m in 0..self.monkeys.len() {
            self.inspect_and_throw_items(m as MonkeyId);
        }
    }

    /// Simulates the given number of `rounds` of item throwing.
    fn play_rounds(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.play_one_round();
        }
    }
}

/// Returns a new `WorryLevel` calculated by modifying the `worry_level` passed with the
/// `operation` passed.
fn inspect_item(worry_level: WorryLevel, operation: &Operation, modulus: WorryLevel) -> WorryLevel {
    let new_worry_level;

    match operation {
        Operation::Add(operand) => match operand {
            Operand::Old => {
                new_worry_level = worry_level + worry_level;
            }
            Operand::Number(n) => {
                new_worry_level = worry_level + n;
            }
        },
        Operation::Multiply(operand) => match operand {
            Operand::Old => {
                new_worry_level = worry_level * worry_level;
            }
            Operand::Number(n) => {
                new_worry_level = worry_level * n;
            }
        },
    }

    new_worry_level % (modulus)
}

/// Parses `input` into a `MonkeyGroup`, then simulates 20 rounds of item throwing. The number of
/// times each `Monkey` has inspected items is collated, and the highest two are multiplied to
/// get the challenge answer.
fn do_challenge(input: &str) -> usize {
    let mut group = MonkeyGroup::from_str(input);
    group.play_rounds(ROUNDS);

    let mut inspection_totals = Vec::new();

    for m in group.monkeys {
        inspection_totals.push(m.num_inspections);
    }

    inspection_totals.sort_unstable();
    inspection_totals.reverse();

    inspection_totals[0] * inspection_totals[1]
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    println!("The challenge answer is {}", do_challenge(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn test_parse_one_monkey() {
        assert_eq!(
            Monkey::from_str(TEST_INPUT.split("\n\n").collect::<Vec<&str>>()[0]),
            Monkey {
                id: 0,
                items: Vec::from([79, 98]),
                operation: Operation::Multiply(Operand::Number(19)),
                test: MonkeyTest {
                    divisible_by: 23,
                    if_true: 2,
                    if_false: 3
                },
                num_inspections: 0,
            }
        );
    }

    #[test]
    fn test_monkeygroup() {
        assert_eq!(
            MonkeyGroup::from_str(TEST_INPUT).monkeys,
            vec![
                Monkey {
                    id: 0,
                    items: Vec::from([79, 98]),
                    operation: Operation::Multiply(Operand::Number(19)),
                    test: MonkeyTest {
                        divisible_by: 23,
                        if_true: 2,
                        if_false: 3
                    },
                    num_inspections: 0,
                },
                Monkey {
                    id: 1,
                    items: Vec::from([54, 65, 75, 74]),
                    operation: Operation::Add(Operand::Number(6)),
                    test: MonkeyTest {
                        divisible_by: 19,
                        if_true: 2,
                        if_false: 0
                    },
                    num_inspections: 0,
                },
                Monkey {
                    id: 2,
                    items: Vec::from([79, 60, 97]),
                    operation: Operation::Multiply(Operand::Old),
                    test: MonkeyTest {
                        divisible_by: 13,
                        if_true: 1,
                        if_false: 3
                    },
                    num_inspections: 0,
                },
                Monkey {
                    id: 3,
                    items: Vec::from([74]),
                    operation: Operation::Add(Operand::Number(3)),
                    test: MonkeyTest {
                        divisible_by: 17,
                        if_true: 0,
                        if_false: 1
                    },
                    num_inspections: 0,
                },
            ]
        );
        assert_eq!(MonkeyGroup::from_str(TEST_INPUT).modulus, 23 * 19 * 13 * 17);
    }

    #[test]
    fn test_play_one_round() {
        let mut group = MonkeyGroup::from_str(TEST_INPUT);

        group.play_one_round();
        assert_eq!(group.monkeys[0].num_inspections, 2);
        assert_eq!(group.monkeys[1].num_inspections, 4);
        assert_eq!(group.monkeys[2].num_inspections, 3);
        assert_eq!(group.monkeys[3].num_inspections, 6);
    }

    #[test]
    fn play_rounds() {
        let mut group = MonkeyGroup::from_str(TEST_INPUT);

        group.play_rounds(20);
        assert_eq!(group.monkeys[0].num_inspections, 99);
        assert_eq!(group.monkeys[1].num_inspections, 97);
        assert_eq!(group.monkeys[2].num_inspections, 8);
        assert_eq!(group.monkeys[3].num_inspections, 103);
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(do_challenge(TEST_INPUT), 2713310158);
    }
}
