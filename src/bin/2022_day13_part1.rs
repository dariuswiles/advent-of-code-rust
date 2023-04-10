//! Advent of Code 2022 Day 13
//! https://adventofcode.com/2022/day/13
//!
//! Challenge part 1
//!
//! Compare pairs of packets to determine if each pair is in the correct order based on rules
//! described in the challenge. Sum the indexes of correctly ordered pairs to generate the
//! challenge answer.

use std::fs;

const INPUT_FILENAME: &str = "2022_day13_input.txt";

type Int = u8;
type Pairs = Vec<(ListElement, ListElement)>;

/// A `ListElement` contains either an individual number or a `Vec` of zero or more `ListElement`s.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum ListElement {
    Integer(Int),
    List(Vec<ListElement>),
}

impl ListElement {
    /// Convert the passed string into `ListElement`s.
    fn parse_str(input: &str) -> Self {
        let input_chars: Vec<char> = input.chars().collect();
        assert_eq!(input_chars[0], '[');

        let slice = &mut &input_chars[1..];
        let result = Self::parse_element_recurse(slice);

        result
    }

    /// Internal function that parses a slice of `char`s representing the input string into a
    /// `ListElement` representation. The slice passed is modified to keep track of the input that
    /// has been processed so far.
    ///
    /// # Panics
    ///
    /// Panics if the input contains an invalid character or is malformed in certain ways. However,
    /// many malformed inputs are accepted if the problems are not too bad, e.g., ",," is treated
    /// as ",".
    fn parse_element_recurse(ic: &mut &[char]) -> Self {
        let mut elements = Vec::new();

        loop {
            match ic[0] {
                ']' => {
                    *ic = &mut &ic[1..];
                    break;
                }
                '[' => {
                    *ic = &mut &ic[1..];
                    let sublist = ListElement::parse_element_recurse(ic);
                    elements.push(sublist);
                }
                '0'..='9' => {
                    let mut char_digits = Vec::new();

                    while ic[0].is_digit(10) {
                        char_digits.push(ic[0]);
                        *ic = &mut &ic[1..];
                    }

                    let int_tmp = Int::from_str_radix(&char_digits.iter().collect::<String>(), 10)
                        .unwrap();

                    elements.push(ListElement::Integer(int_tmp));
                }
                ',' => {
                    *ic = &mut &ic[1..];
                }
                _ => {
                    panic!("Unrecognized character '{}' in input", ic[0]);
                }
            }

            if ic.len() == 0 {
                panic!("The input contains unbalanced start and end list tags");
            }
        }

        ListElement::List(elements)
    }
}

/// Parses the input as sets of 3 lines. The first and second each contain a `ListElement`, which
/// the challenge refers to as "Left" and "Right". The third is a blank line.
///
/// Returns a `Pairs` object which is a `Vec` of pairs of `ListElement`s.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> Pairs {
    let mut pairs = Vec::new();

    let mut left = None;

    for (line_number, line) in input.lines().enumerate() {
        match line_number % 3 {
            0 => {
                left = Some(ListElement::parse_str(line));
            }
            1 => {
                pairs.push((left.unwrap(), ListElement::parse_str(line)));
                left = None;
            }
            2 => {
                assert!(line.is_empty(), "Blank line between pairs in input was not found");
            }
            _ => {
            }
        }
    }

    pairs
}

/// Compares the ordering of the 'left' and 'right' `ListElement`s passed, as per the challenge
/// rules. Returns 'Some(true)' if ordering is correct, 'Some(false)' if incorrect, and `None` if
/// the two parameters passed are identical.
fn is_order_correct(left: &ListElement, right: &ListElement) -> Option<bool> {
    if let ListElement::Integer(left_integer) = left {
        if let ListElement::Integer(right_integer) = right {
            if left_integer != right_integer {
                return Some(left_integer < right_integer);
            } else {
                return None;
            }
        }
    }

    // At least one of 'left' or 'right' is an `ElementList`, but both need to be treated as if
    // they are `ElementList`s so they can be compared, as described the challenge rules. This is
    // done by converting an `Integer` into a new `Vec` with it as the only element.
    let left_elements;
    let right_elements;

    match left {
        ListElement::Integer(int) => {
            left_elements = vec![ListElement::Integer(*int)];
        }
        ListElement::List(list) => {
            left_elements = list.clone();
        }
    }

    match right {
        ListElement::Integer(int) => {
            right_elements = vec![ListElement::Integer(*int)];
        }
        ListElement::List(list) => {
            right_elements = list.clone();
        }
    }

    let left_length = left_elements.len();
    let right_length = right_elements.len();
    let shortest = usize::min(left_length, right_length);

    for index in 0..shortest {
        let pair_ordering_correct = is_order_correct(&left_elements[index], &right_elements[index]);
        if pair_ordering_correct.is_some() {
            return pair_ordering_correct;
        }
    }

    // If the lists are the same length, all data passed is identical, so return `None` to
    // indicate this.
    if left_length == right_length {
        return None;
    }

    // As per the challenge rules, the pairs are ordered correctly if the 'left' list is shorter,
    // and are not ordered correctly otherwise.
    return Some(left_length < right_length);
}

/// Iterates through all pairs of packets passed to determine which pairs are in the correct order.
/// Returns a challenge answer that is the sum of the indexes of all correctly ordered pairs.
///
/// # Panics
///
/// Panics if any of the pairs are identical.
//
// The challenge orders pairs starting at 1 rather than 0, so this is taken into account.
fn check_order_of_all_pairs(pairs: &Pairs) -> usize {
    let mut challenge_total = 0;

    for (index, pair) in pairs.iter().enumerate() {
        if is_order_correct(&pair.0, &pair.1).expect("Error: a pair of packets was identical") {
            challenge_total += index + 1;
        }
    }

    challenge_total
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    let pairs = parse_input(&input_file);
    println!("The challenge answer is {}", check_order_of_all_pairs(&pairs));
}

// Test using data from the examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn test_parse_str_0() {
        assert_eq!(
            ListElement::parse_str(&"[11,0]"),
            ListElement::List(vec![
                    ListElement::Integer(11),
                    ListElement::Integer(0),
                ])
        );
    }

    #[test]
    fn test_parse_str_1() {
        assert_eq!(
            ListElement::parse_str(&"[[1],[2,3,4]]"),
            ListElement::List(vec![
                    ListElement::List(vec![
                            ListElement::Integer(1),
                    ]),
                    ListElement::List(vec![
                            ListElement::Integer(2),
                            ListElement::Integer(3),
                            ListElement::Integer(4),
                    ]),
            ]),
        );
    }

    #[test]
    #[should_panic]
    fn test_parse_str_bad_char() {
        ListElement::parse_str(&"[9,6,[2],a,5]");
    }

    #[test]
    #[should_panic]
    fn test_parse_str_unbalanced1() {
        ListElement::parse_str(&"[9,6,[2]");
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(&TEST_INPUT);

        assert_eq!(result[0].0,
            ListElement::List(vec![
                ListElement::Integer(1),
                ListElement::Integer(1),
                ListElement::Integer(3),
                ListElement::Integer(1),
                ListElement::Integer(1),
            ]),
        );

        assert_eq!(result[0].1,
            ListElement::List(vec![
                ListElement::Integer(1),
                ListElement::Integer(1),
                ListElement::Integer(5),
                ListElement::Integer(1),
                ListElement::Integer(1),
            ]),
        );

        assert_eq!(result[1].0,
            ListElement::List(vec![
                ListElement::List(vec![
                    ListElement::Integer(1),
                ]),
                ListElement::List(vec![
                    ListElement::Integer(2),
                    ListElement::Integer(3),
                    ListElement::Integer(4),
                ]),
            ]),
        );

        assert_eq!(result[1].1,
            ListElement::List(vec![
                ListElement::List(vec![
                    ListElement::Integer(1),
                ]),
                ListElement::Integer(4),
            ]),
        );

        assert_eq!(result[2].0,
            ListElement::List(vec![
                ListElement::Integer(9),
            ]),
        );

        assert_eq!(result[2].1,
            ListElement::List(vec![
                ListElement::List(vec![
                    ListElement::Integer(8),
                    ListElement::Integer(7),
                    ListElement::Integer(6),
                ]),
            ]),
        );

        assert_eq!(result[3].0,
            ListElement::List(vec![
                ListElement::List(vec![
                    ListElement::Integer(4),
                    ListElement::Integer(4),
                ]),
                ListElement::Integer(4),
                ListElement::Integer(4),
            ]),
        );

        assert_eq!(result[3].1,
            ListElement::List(vec![
                ListElement::List(vec![
                    ListElement::Integer(4),
                    ListElement::Integer(4),
                ]),
                ListElement::Integer(4),
                ListElement::Integer(4),
                ListElement::Integer(4),
            ]),
        );

        assert_eq!(result[4].0,
            ListElement::List(vec![
                ListElement::Integer(7),
                ListElement::Integer(7),
                ListElement::Integer(7),
                ListElement::Integer(7),
            ]),
        );

        assert_eq!(result[4].1,
            ListElement::List(vec![
                ListElement::Integer(7),
                ListElement::Integer(7),
                ListElement::Integer(7),
            ]),
        );

        assert_eq!(result[5].0,
            ListElement::List(vec![
            ]),
        );

        assert_eq!(result[5].1,
            ListElement::List(vec![
                ListElement::Integer(3),
            ]),
        );

        assert_eq!(result[6].0,
            ListElement::List(vec![
                ListElement::List(vec![
                    ListElement::List(vec![
                    ]),
                ]),
            ]),
        );

        assert_eq!(result[6].1,
            ListElement::List(vec![
                ListElement::List(vec![
                ]),
            ]),
        );

        assert_eq!(result[7].0,
            ListElement::List(vec![
                ListElement::Integer(1),
                ListElement::List(vec![
                    ListElement::Integer(2),
                    ListElement::List(vec![
                        ListElement::Integer(3),
                        ListElement::List(vec![
                            ListElement::Integer(4),
                            ListElement::List(vec![
                                ListElement::Integer(5),
                                ListElement::Integer(6),
                                ListElement::Integer(7),
                            ]),
                        ]),
                    ]),
                ]),
                ListElement::Integer(8),
                ListElement::Integer(9),
            ]),
        );

        assert_eq!(result[7].1,
            ListElement::List(vec![
                ListElement::Integer(1),
                ListElement::List(vec![
                    ListElement::Integer(2),
                    ListElement::List(vec![
                        ListElement::Integer(3),
                        ListElement::List(vec![
                            ListElement::Integer(4),
                            ListElement::List(vec![
                                ListElement::Integer(5),
                                ListElement::Integer(6),
                                ListElement::Integer(0),
                            ]),
                        ]),
                    ]),
                ]),
                ListElement::Integer(8),
                ListElement::Integer(9),
            ]),
        );
    }

    #[test]
    fn check_ordering() {
        let pairs = parse_input(&TEST_INPUT);
        assert_eq!(is_order_correct(&pairs[0].0, &pairs[0].1), Some(true));
        assert_eq!(is_order_correct(&pairs[1].0, &pairs[1].1), Some(true));
        assert_eq!(is_order_correct(&pairs[2].0, &pairs[2].1), Some(false));
        assert_eq!(is_order_correct(&pairs[3].0, &pairs[3].1), Some(true));
        assert_eq!(is_order_correct(&pairs[4].0, &pairs[4].1), Some(false));
        assert_eq!(is_order_correct(&pairs[5].0, &pairs[5].1), Some(true));
        assert_eq!(is_order_correct(&pairs[6].0, &pairs[6].1), Some(false));
        assert_eq!(is_order_correct(&pairs[7].0, &pairs[7].1), Some(false));
    }

    #[test]
    fn test_check_order_of_all_pairs() {
        let pairs = parse_input(&TEST_INPUT);
        assert_eq!(check_order_of_all_pairs(&pairs), 13);
    }
}
