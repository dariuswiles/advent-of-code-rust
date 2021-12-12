//! Advent of Code 2021 Day 08
//! https://adventofcode.com/2021/day/8
//!
//! Challenge part 1
//!
//! Count how many times the digits 1, 4, 7 and 8 appear in the display output provided in the
//! input file, e.g., the sets of data provided to the right of the pipe. These digits are easy
//! to deduce because each requires a unique number of display segments to be active, namely
//! 2, 4, 3 and 7 respectively.

use std::collections::{ HashSet };
use std::fs;

const INPUT_FILENAME: &str = "2021_day08_input.txt";

#[derive(Debug, PartialEq)]
struct ActiveWireSet {
    wires: HashSet<char>,
}

impl ActiveWireSet {
    fn new(input: &str) -> Self {
        let mut wires = HashSet::new();

        for c in input.chars() {
            wires.insert(c.clone());
        }
        Self { wires }
    }
}


/// Parses an input string consisting of a series of 10 blocks of segment letters, delimited by
/// spaces, then a pipe separator, then a further 4 blocks of segment letters. Returns a Vec
/// containing one element per line as a pair. The left side of the pair contains the 10 blocks,
/// and the right side the 4 blocks. The blocks of letters are represented as sets.
///
/// # Panics
///
/// Panics if the input string is malformed.
fn parse_input(input: &str) -> Vec<(Vec<ActiveWireSet>, Vec<ActiveWireSet>)> {
    let mut output = Vec::new();

    for line in input.lines() {
        if line == "" {
            continue;
        }

        let left_right: Vec<&str> = line.split(" | ").collect();
        if left_right.len() != 2 {
            panic!("Malformed input in: {}", line);
        }

        let left: Vec<ActiveWireSet> =
            left_right[0]
            .split(' ')
            .map(|s| ActiveWireSet::new(s))
            .collect();

        if left.len() != 10 {
            panic!("Malformed input with left segments in: {}", line);
        }

        let right: Vec<ActiveWireSet> =
            left_right[1]
            .split(' ')
            .map(|s| ActiveWireSet::new(s))
            .collect();

        if right.len() != 4 {
            panic!("Malformed input with right segments in: {}", line);
        }

        output.push((left, right));
    }
    output
}


/// Return the number of occurrences of wire sets that contain exactly 2, 3, 4 or 7 active wires.
fn count_easy_lengths(sets: &Vec<ActiveWireSet>) -> usize {
    let mut total = 0;

    for s in sets {
        total +=
            match s.wires.len() {
                2 => { 1 }
                3 => { 1 }
                4 => { 1 }
                7 => { 1 }
                _ => { 0 }
            };
    }
    total
}


/// Count the number of occurrences of wire sets that contain exactly 2, 3, 4 or 7 active wires in
/// the right hand side of all input lines.
fn count_all_easy_lengths(wire_sets: &Vec<(Vec<ActiveWireSet>, Vec<ActiveWireSet>)>
) -> usize {
    let mut total = 0;

    for rhs in wire_sets {
        total += count_easy_lengths(&rhs.1);
    }

    total
}


fn main() {
    let input_file =
        fs::read_to_string(INPUT_FILENAME)
            .expect("Error reading input file");

        let wire_sets = parse_input(&input_file);

    println!("The digits 1, 4, 7 and 8 occur {} times in the right hand side of the input",
        count_all_easy_lengths(&wire_sets)
    );
}


// Test using data from the examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

    #[test]
    fn parse_test_input() {
        let wire_sets = parse_input(&TEST_INPUT);

        assert_eq!(wire_sets[0].0[0], ActiveWireSet::new("be"));
        assert_eq!(wire_sets[0].0[4], ActiveWireSet::new("cgeb"));
        assert_eq!(wire_sets[0].0[9], ActiveWireSet::new("edb"));
        assert_eq!(wire_sets[1].1[1], ActiveWireSet::new("cgb"));
        assert_eq!(wire_sets[9].0[4], ActiveWireSet::new("gf"));
        assert_eq!(wire_sets[9].1[2], ActiveWireSet::new("fg"));
    }

    #[test]
    fn test_count_easy_lengths() {
        let wire_sets = parse_input(&TEST_INPUT);

        assert_eq!(count_easy_lengths(&wire_sets[0].1), 2);
        assert_eq!(count_easy_lengths(&wire_sets[1].1), 3);
        assert_eq!(count_easy_lengths(&wire_sets[2].1), 3);
        assert_eq!(count_easy_lengths(&wire_sets[7].1), 1);
        assert_eq!(count_easy_lengths(&wire_sets[9].1), 2);
    }

    #[test]
    fn test_count_all_easy_lengths() {
        let wire_sets = parse_input(&TEST_INPUT);

        assert_eq!(count_all_easy_lengths(&wire_sets), 26);
    }
}
