//! Advent of Code 2021 Day 22
//! https://adventofcode.com/2021/day/22
//!
//! Challenge part 1
//!
//! Update a 3D grid of cells that all start in an off state by following a list of rules that each
//! either turn a specified group of cells on or off. After following all the rules count the
//! number of cubes that are on. Part 1 of the challenge only considers a small region centered on
//! the origin.

use std::fs;
use std::ops::RangeInclusive;

const INPUT_FILENAME: &str = "2021_day22_input.txt";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum CellState {
    Off,
    On,
}

/// Holds the x, y and z ranges associated with a rule, and whether the rule is to switch cells on
/// or off.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Rule {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
    change_state_to: CellState,
}

impl Rule {
    /// If the x, y and z coordinates passed are all within the ranges of this `Rule`, return its
    /// `CellState`. If not, return `None`.
    fn changes_cell_state(&self, x: i32, y: i32, z: i32) -> Option<CellState> {
        if self.x.contains(&x) && self.y.contains(&y) && self.z.contains(&z) {
            return Some(self.change_state_to);
        }

        None
    }
}

/// Examines all `Rules` in the order they are passed to find the first that contains the given
/// cell (as specified in the `x`, `y` and `z` arguments). If a matching `Rule` is found, its
/// state is returned. If no rules match, the cell state of `Off` is returned as this is the
/// initial state of cells defined in the challenge.
///
/// Note: In most cases the `Rules` passed should be in the reverse order from that given in the
///       input file.
fn check_all_rules(x: i32, y: i32, z: i32, rules: &Vec<Rule>) -> CellState {
    for rule in rules {
        if let Some(cell_state) = rule.changes_cell_state(x, y, z) {
            return cell_state;
        }
    }

    CellState::Off
}

/// Count the number of cells marked as 'on' in the volume passed.
fn count_active_cells(
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
    z_range: RangeInclusive<i32>,
    rules: &Vec<Rule>,
) -> u32 {
    let mut result = 0;

    for x in x_range {
        for y in y_range.clone() {
            for z in z_range.clone() {
                if check_all_rules(x, y, z, rules) == CellState::On {
                    result += 1;
                }
            }
        }
    }

    result
}

/// Reads the list of rules in the string passed and returns a `Vec` containing a list of `Rule`
/// objects representing this data.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> Vec<Rule> {
    let mut rules = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split(" ").collect();
        if tokens.len() != 2 {
            panic!("The input file is malformed");
        }

        let change_state_to = match tokens[0] {
            "on" => CellState::On,
            "off" => CellState::Off,
            _ => {
                panic!("Input contains an unrecognized cell state.");
            }
        };

        let ranges: Vec<&str> = tokens[1].split(",").collect();
        if ranges.len() != 3 {
            panic!("A rule in the input file does not contain the 3 expected ranges");
        }

        let x_vec: Vec<&str> = ranges[0].strip_prefix("x=").unwrap().split("..").collect();
        let y_vec: Vec<&str> = ranges[1].strip_prefix("y=").unwrap().split("..").collect();
        let z_vec: Vec<&str> = ranges[2].strip_prefix("z=").unwrap().split("..").collect();

        rules.push(Rule {
            x: x_vec[0].parse().unwrap()..=x_vec[1].parse().unwrap(),
            y: y_vec[0].parse().unwrap()..=y_vec[1].parse().unwrap(),
            z: z_vec[0].parse().unwrap()..=z_vec[1].parse().unwrap(),
            change_state_to,
        });
    }

    rules
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let mut rules = parse_input(&input_file);
    rules.reverse();
    let answer = count_active_cells(-50..=50, -50..=50, -50..=50, &rules);

    println!("{} cells are in the 'on' state.", answer);
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_0: &str = "\
on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

    const TEST_INPUT_1: &str = "\
on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

    #[test]
    fn parse_test_input_0() {
        let rules = parse_input(TEST_INPUT_0);

        assert_eq!(
            rules[0],
            Rule {
                x: 10..=12,
                y: 10..=12,
                z: 10..=12,
                change_state_to: CellState::On
            }
        );
        assert_eq!(
            rules[1],
            Rule {
                x: 11..=13,
                y: 11..=13,
                z: 11..=13,
                change_state_to: CellState::On
            }
        );
        assert_eq!(
            rules[2],
            Rule {
                x: 9..=11,
                y: 9..=11,
                z: 9..=11,
                change_state_to: CellState::Off
            }
        );
        assert_eq!(
            rules[3],
            Rule {
                x: 10..=10,
                y: 10..=10,
                z: 10..=10,
                change_state_to: CellState::On
            }
        );
    }

    #[test]
    fn changes_cell_state_0() {
        let rule = Rule {
            x: 1..=3,
            y: 7..=9,
            z: -5..=-2,
            change_state_to: CellState::On,
        };
        assert_eq!(rule.changes_cell_state(1, 9, -4), Some(CellState::On));
    }

    #[test]
    fn changes_cell_state_1() {
        let rule = Rule {
            x: 1..=3,
            y: 7..=9,
            z: -5..=-2,
            change_state_to: CellState::On,
        };
        assert_eq!(rule.changes_cell_state(2, 8, -6), None);
    }

    #[test]
    fn test_check_all_rules() {
        let mut rules = parse_input(TEST_INPUT_0);
        rules.reverse();

        assert_eq!(check_all_rules(10, 10, 10, &rules), CellState::On); // Last rule in input
        assert_eq!(check_all_rules(10, 10, 11, &rules), CellState::Off); // Penultimate rule
        assert_eq!(check_all_rules(10, 10, 10, &rules), CellState::On); // Second rule in input
        assert_eq!(check_all_rules(10, 11, 12, &rules), CellState::On); // First rule in input
        assert_eq!(check_all_rules(9, 13, 9, &rules), CellState::Off); // No matching rule
    }

    #[test]
    fn test_count_active_cells_0() {
        let mut rules = parse_input(TEST_INPUT_0);
        rules.reverse();
        assert_eq!(count_active_cells(-50..=50, -50..=50, -50..=50, &rules), 39);
    }

    #[test]
    fn test_count_active_cells_1() {
        let mut rules = parse_input(TEST_INPUT_1);
        rules.reverse();
        assert_eq!(
            count_active_cells(-50..=50, -50..=50, -50..=50, &rules),
            590784
        );
    }
}
