//! Advent of Code 2023 Day 08
//! https://adventofcode.com/2023/day/8
//!
//! Challenge part 1
//!
//! The input contains instructions of the form of left/right directions, and a network of nodes.
//! Each node has a label and points to a "left" node and a "right" node. The challenge states the
//! nodes to start and end at, and the challenge is to determine the number of steps required to
//! travel between these two nodes by following the directions.

use std::collections::HashMap;
use std::fs;

const INPUT_FILENAME: &str = "2023_day08_input.txt";
const START_NODE: &str = "AAA";
const END_NODE: &str = "ZZZ";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Node<'a> {
    label: &'a str,
    left: &'a str,
    right: &'a str,
}

impl<'a> Node<'a> {
    /// Creates and returns a new `Node` based on the input string provided which contains the
    /// `Node`'s label, and left and right instructions in this order in the following format:
    /// `AAA = (BBB, CCC)`
    ///
    /// # Panics
    ///
    /// Panics if the string passed is malformed.
    fn from_str(s: &'a str) -> Self {
        let (label, choices) = s
            .split_once(" = ")
            .expect("A node definition must contain an equals sign");

        let (left, right) = choices
            .strip_prefix('(')
            .expect("Node definition choices must start with a '('")
            .strip_suffix(')')
            .expect("Node definition choices must end with a ')'")
            .split_once(", ")
            .expect("Node definition choices must be separated with a comma");

        assert_eq!(
            3,
            label.len(),
            "A node label must be exactly three characters in length"
        );
        assert_eq!(
            3,
            left.len(),
            "A node label must be exactly three characters in length"
        );
        assert_eq!(
            3,
            right.len(),
            "A node label must be exactly three characters in length"
        );

        Self { label, left, right }
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    println!(
        "The number of steps to get from the start node to the end node is {}",
        do_challenge(&input)
    );
}

/// Returns the number of steps required to get from the start node to the end node.
fn do_challenge(input: &str) -> u64 {
    let (instructions, nodes) = parse_input(&input);

    follow_instructions(instructions, nodes)
}

/// Parses the input into a string slice containing the instructions, and `HashMap` of `Node`s
/// representing the rest of the input. These are returned in a tuple in this order.
///
/// # Panics
///
/// Panics if the string passed is malformed.
fn parse_input(input: &str) -> (&str, HashMap<&str, Node>) {
    let mut lines = input.lines();
    let instructions = lines.next().expect("Input string contains no data");
    assert_eq!(
        Some(""),
        lines.next(),
        "The line of instructions must be followed by a blank line"
    );

    let mut nodes = HashMap::new();
    for line in lines {
        let node = Node::from_str(&line);
        nodes.insert(node.label, node);
    }

    (instructions, nodes)
}

/// Follows the instructions to traverse the network of nodes starting at the node labelled `AAA`.
/// Returns the number of steps required to get from the start node to the end node.
///
/// # Panics
///
/// Panics if the string of instructions contains anything other than `L` or `R`.
/// Panics if a node points to another node that does not exist.
fn follow_instructions(instructions: &str, nodes: HashMap<&str, Node>) -> u64 {
    let mut steps = 0;
    let mut current_node = START_NODE;
    let mut directions = instructions.chars().cycle();

    for dir in directions {
        if current_node == END_NODE {
            break;
        }

        match dir {
            'L' => {
                current_node = nodes
                    .get(current_node)
                    .expect("Could not find a node labelled '{current_node}'")
                    .left;
                steps += 1;
            }
            'R' => {
                current_node = nodes
                    .get(current_node)
                    .expect("Could not find a node labelled '{current_node}'")
                    .right;
                steps += 1;
            }
            _ => {
                panic!("Instructions must be 'L' or 'R', but found '{dir}'");
            }
        }
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_0: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    const TEST_INPUT_1: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    #[test]
    fn test_node_from_str() {
        assert_eq!(
            Node {
                label: &"AAA",
                left: &"BBB",
                right: &"CCC",
            },
            Node::from_str("AAA = (BBB, CCC)")
        );
    }

    #[test]
    #[should_panic]
    fn test_node_from_str_malformed() {
        Node::from_str("AAA = (BB, CCC)");
    }

    #[test]
    fn test_parse_input_0() {
        let (instructions, nodes) = parse_input(&TEST_INPUT_0);

        assert_eq!("RL", instructions);
        assert_eq!(7, nodes.len());
        assert_eq!(
            Some(&Node {
                label: &"AAA",
                left: &"BBB",
                right: &"CCC",
            }),
            nodes.get(&"AAA")
        );
        assert_eq!(
            Some(&Node {
                label: &"BBB",
                left: &"DDD",
                right: &"EEE",
            }),
            nodes.get(&"BBB")
        );
        assert_eq!(
            Some(&Node {
                label: &"CCC",
                left: &"ZZZ",
                right: &"GGG",
            }),
            nodes.get(&"CCC")
        );
        assert_eq!(
            Some(&Node {
                label: &"DDD",
                left: &"DDD",
                right: &"DDD",
            }),
            nodes.get(&"DDD")
        );
        assert_eq!(
            Some(&Node {
                label: &"EEE",
                left: &"EEE",
                right: &"EEE",
            }),
            nodes.get(&"EEE")
        );
        assert_eq!(
            Some(&Node {
                label: &"GGG",
                left: &"GGG",
                right: &"GGG",
            }),
            nodes.get(&"GGG")
        );
        assert_eq!(
            Some(&Node {
                label: &"ZZZ",
                left: &"ZZZ",
                right: &"ZZZ",
            }),
            nodes.get(&"ZZZ")
        );
    }

    #[test]
    fn test_parse_input_1() {
        let (instructions, nodes) = parse_input(&TEST_INPUT_1);

        assert_eq!("LLR", instructions);
        assert_eq!(3, nodes.len());
        assert_eq!(
            Some(&Node {
                label: &"AAA",
                left: &"BBB",
                right: &"BBB",
            }),
            nodes.get(&"AAA")
        );
        assert_eq!(
            Some(&Node {
                label: &"BBB",
                left: &"AAA",
                right: &"ZZZ",
            }),
            nodes.get(&"BBB")
        );
        assert_eq!(
            Some(&Node {
                label: &"ZZZ",
                left: &"ZZZ",
                right: &"ZZZ",
            }),
            nodes.get(&"ZZZ")
        );
    }

    #[test]
    fn test_do_challenge_0() {
        assert_eq!(2, do_challenge(&TEST_INPUT_0));
    }

    #[test]
    fn test_do_challenge_1() {
        assert_eq!(6, do_challenge(&TEST_INPUT_1));
    }
}
