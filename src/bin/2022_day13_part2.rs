//! Advent of Code 2022 Day 13
//! https://adventofcode.com/2022/day/13
//!
//! Challenge part 2
//!
//! Sort an input file of packets based on ordering rules described in the challenge.

use std::cmp::Ordering;
use std::fs;

const INPUT_FILENAME: &str = "2022_day13_input.txt";

type Int = u8;

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

                    let int_tmp =
                        Int::from_str_radix(&char_digits.iter().collect::<String>(), 10).unwrap();

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

/// Parses the input and returns its `ListElement`s representation in a `Vec`.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> Vec<ListElement> {
    let mut list_elements = Vec::new();

    for line in input.lines() {
        if line.len() > 0 {
            list_elements.push(ListElement::parse_str(line));
        }
    }

    list_elements
}

/// Compares the ordering of the two 'left' and 'right' `ListElement`s passed, as per the challenge
/// rules.
fn compare_packets(left: &ListElement, right: &ListElement) -> Ordering {
    if let ListElement::Integer(left_integer) = left {
        if let ListElement::Integer(right_integer) = right {
            if left_integer < right_integer {
                return Ordering::Less;
            } else if left_integer > right_integer {
                return Ordering::Greater;
            } else {
                return Ordering::Equal;
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
        let pair_ordering = compare_packets(&left_elements[index], &right_elements[index]);
        if pair_ordering != Ordering::Equal {
            return pair_ordering;
        }
    }

    // If the lists are the same length, all data passed is identical, so return `None` to
    // indicate this.
    if left_length == right_length {
        return Ordering::Equal;
    }

    // As per the challenge rules, the pairs are ordered correctly if the 'left' list is shorter,
    // and are not ordered correctly otherwise.
    if left_length < right_length {
        return Ordering::Less;
    } else {
        return Ordering::Greater;
    }
}

/// Append the two divider packets required by the challenge to the `Vec` of `ListElement`s
/// passed.
fn add_divider_packets(packets: &mut Vec<ListElement>) {
    packets.push(ListElement::List(vec![ListElement::List(vec![
        ListElement::Integer(2),
    ])]));

    packets.push(ListElement::List(vec![ListElement::List(vec![
        ListElement::Integer(6),
    ])]));
}

/// Sort all packets based on the ordering defined in the challenge.
fn sort_packets(packets: &mut Vec<ListElement>) {
    packets.sort_unstable_by(|a, b| compare_packets(&a, &b));
}

/// Returns the index of `packet` in `packets`, or `None` if it is not found. The first index is 0,
/// which is the Rust standard, so the caller may need to add one to be consistent with the
/// challenge.
fn find_packet(packet: &ListElement, packets: &Vec<ListElement>) -> Option<usize> {
    for (index, list_element) in packets.iter().enumerate() {
        if compare_packets(packet, list_element) == Ordering::Equal {
            return Some(index);
        }
    }
    None
}

/// Adds the divider packets to the `Vec` of packets passed, sorts all packets, finds the indexes
/// of the divider packets and returns their product.
///
/// # Panics
///
/// Panics if any of the pairs are identical.
//
// The challenge numbers indexes starting at 1 rather than 0, but this is taken into account.
fn do_challenge(packets: &mut Vec<ListElement>) -> usize {
    add_divider_packets(packets);
    sort_packets(packets);

    let first = find_packet(
        &ListElement::List(vec![ListElement::List(vec![ListElement::Integer(2)])]),
        packets,
    )
    .unwrap();

    let second = find_packet(
        &ListElement::List(vec![ListElement::List(vec![ListElement::Integer(6)])]),
        packets,
    )
    .unwrap();

    (first + 1) * (second + 1)
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    let mut packets = parse_input(&input_file);
    println!("The challenge answer is {}", do_challenge(&mut packets));
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
            ListElement::List(vec![ListElement::Integer(11), ListElement::Integer(0),])
        );
    }

    #[test]
    fn test_parse_str_1() {
        assert_eq!(
            ListElement::parse_str(&"[[1],[2,3,4]]"),
            ListElement::List(vec![
                ListElement::List(vec![ListElement::Integer(1),]),
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

        assert_eq!(
            result[0],
            ListElement::List(vec![
                ListElement::Integer(1),
                ListElement::Integer(1),
                ListElement::Integer(3),
                ListElement::Integer(1),
                ListElement::Integer(1),
            ]),
        );

        assert_eq!(
            result[1],
            ListElement::List(vec![
                ListElement::Integer(1),
                ListElement::Integer(1),
                ListElement::Integer(5),
                ListElement::Integer(1),
                ListElement::Integer(1),
            ]),
        );

        assert_eq!(
            result[2],
            ListElement::List(vec![
                ListElement::List(vec![ListElement::Integer(1),]),
                ListElement::List(vec![
                    ListElement::Integer(2),
                    ListElement::Integer(3),
                    ListElement::Integer(4),
                ]),
            ]),
        );

        assert_eq!(
            result[3],
            ListElement::List(vec![
                ListElement::List(vec![ListElement::Integer(1),]),
                ListElement::Integer(4),
            ]),
        );

        assert_eq!(result[4], ListElement::List(vec![ListElement::Integer(9),]),);

        assert_eq!(
            result[5],
            ListElement::List(vec![ListElement::List(vec![
                ListElement::Integer(8),
                ListElement::Integer(7),
                ListElement::Integer(6),
            ]),]),
        );

        assert_eq!(
            result[6],
            ListElement::List(vec![
                ListElement::List(vec![ListElement::Integer(4), ListElement::Integer(4),]),
                ListElement::Integer(4),
                ListElement::Integer(4),
            ]),
        );

        assert_eq!(
            result[7],
            ListElement::List(vec![
                ListElement::List(vec![ListElement::Integer(4), ListElement::Integer(4),]),
                ListElement::Integer(4),
                ListElement::Integer(4),
                ListElement::Integer(4),
            ]),
        );

        assert_eq!(
            result[8],
            ListElement::List(vec![
                ListElement::Integer(7),
                ListElement::Integer(7),
                ListElement::Integer(7),
                ListElement::Integer(7),
            ]),
        );

        assert_eq!(
            result[9],
            ListElement::List(vec![
                ListElement::Integer(7),
                ListElement::Integer(7),
                ListElement::Integer(7),
            ]),
        );

        assert_eq!(result[10], ListElement::List(vec![]),);

        assert_eq!(
            result[11],
            ListElement::List(vec![ListElement::Integer(3),]),
        );

        assert_eq!(
            result[12],
            ListElement::List(vec![ListElement::List(vec![ListElement::List(vec![]),]),]),
        );

        assert_eq!(
            result[13],
            ListElement::List(vec![ListElement::List(vec![]),]),
        );

        assert_eq!(
            result[14],
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

        assert_eq!(
            result[15],
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
    fn check_compare_packets() {
        let packets = parse_input(&TEST_INPUT);
        assert_eq!(compare_packets(&packets[0], &packets[1]), Ordering::Less);
        assert_eq!(compare_packets(&packets[2], &packets[3]), Ordering::Less);
        assert_eq!(compare_packets(&packets[4], &packets[5]), Ordering::Greater);
        assert_eq!(compare_packets(&packets[6], &packets[7]), Ordering::Less);
        assert_eq!(compare_packets(&packets[8], &packets[9]), Ordering::Greater);
        assert_eq!(compare_packets(&packets[10], &packets[11]), Ordering::Less);
        assert_eq!(
            compare_packets(&packets[12], &packets[13]),
            Ordering::Greater
        );
        assert_eq!(
            compare_packets(&packets[14], &packets[15]),
            Ordering::Greater
        );
    }

    #[test]
    fn test_add_divider_packets() {
        let mut packets = parse_input(&TEST_INPUT);
        assert_eq!(packets.len(), 16);
        add_divider_packets(&mut packets);
        assert_eq!(packets.len(), 18);

        assert_eq!(
            packets[16],
            ListElement::List(vec![ListElement::List(vec![ListElement::Integer(2),]),])
        );

        assert_eq!(
            packets[17],
            ListElement::List(vec![ListElement::List(vec![ListElement::Integer(6),]),])
        );
    }

    #[test]
    fn test_sort_packets() {
        let mut packets = parse_input(&TEST_INPUT);
        sort_packets(&mut packets);

        assert_eq!(
            packets,
            vec![
                ListElement::List(vec![]),
                ListElement::List(vec![ListElement::List(vec![]),]),
                ListElement::List(vec![ListElement::List(vec![ListElement::List(vec![]),]),]),
                ListElement::List(vec![
                    ListElement::Integer(1),
                    ListElement::Integer(1),
                    ListElement::Integer(3),
                    ListElement::Integer(1),
                    ListElement::Integer(1),
                ]),
                ListElement::List(vec![
                    ListElement::Integer(1),
                    ListElement::Integer(1),
                    ListElement::Integer(5),
                    ListElement::Integer(1),
                    ListElement::Integer(1),
                ]),
                ListElement::List(vec![
                    ListElement::List(vec![ListElement::Integer(1),]),
                    ListElement::List(vec![
                        ListElement::Integer(2),
                        ListElement::Integer(3),
                        ListElement::Integer(4),
                    ]),
                ]),
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
                ListElement::List(vec![
                    ListElement::List(vec![ListElement::Integer(1),]),
                    ListElement::Integer(4),
                ]),
                ListElement::List(vec![ListElement::Integer(3),]),
                ListElement::List(vec![
                    ListElement::List(vec![ListElement::Integer(4), ListElement::Integer(4),]),
                    ListElement::Integer(4),
                    ListElement::Integer(4),
                ]),
                ListElement::List(vec![
                    ListElement::List(vec![ListElement::Integer(4), ListElement::Integer(4),]),
                    ListElement::Integer(4),
                    ListElement::Integer(4),
                    ListElement::Integer(4),
                ]),
                ListElement::List(vec![
                    ListElement::Integer(7),
                    ListElement::Integer(7),
                    ListElement::Integer(7),
                ]),
                ListElement::List(vec![
                    ListElement::Integer(7),
                    ListElement::Integer(7),
                    ListElement::Integer(7),
                    ListElement::Integer(7),
                ]),
                ListElement::List(vec![ListElement::List(vec![
                    ListElement::Integer(8),
                    ListElement::Integer(7),
                    ListElement::Integer(6),
                ]),]),
                ListElement::List(vec![ListElement::Integer(9),]),
            ]
        );
    }

    #[test]
    fn test_do_challenge() {
        let mut packets = parse_input(&TEST_INPUT);
        assert_eq!(do_challenge(&mut packets), 140);
    }
}
