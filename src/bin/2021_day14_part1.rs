//! Advent of Code 2021 Day 14
//! https://adventofcode.com/2021/day/14
//!
//! Challenge part 1
//!
//! Reads a string and a set of transformation rules from an input file, repeatedly applies the
//! rules and outputs an answer based on the final string.

use std::collections::HashMap;
use std::fs;
use std::str::Lines;

const INPUT_FILENAME: &str = "2021_day14_input.txt";
const ITERATIONS: usize = 10;

type Rule = [char; 2];


/// A `RuleSet` is a set of transformation rules.
#[derive(Clone, Debug, Eq, PartialEq)]
struct RuleSet {
    rules: HashMap<Rule, char>,
}

impl RuleSet {
    /// Returns a new `RuleSet` created from an input string containing an arbitrary number of
    /// lines containing insertion rules.
    ///
    /// # Panics
    ///
    /// Panics if the input is malformed.
    fn new(lines: &mut Lines) -> Self {
        let mut rules = HashMap::new();

        for line in lines {
            let line_split: Vec<&str> = line.split(" -> ").collect();
            if line_split.len() != 2 {
                panic!("Malformed insertion rule : {}", line);
            }

            assert_eq!(line_split[1].chars().collect::<Vec<char>>().len(), 1);

            let rule_chars = line_split[0].chars().collect::<Vec<char>>();
            let rule: Rule = [rule_chars[0], rule_chars[1]];

            rules.insert(
                    rule,
                    line_split[1].chars().next().unwrap()
            );
        }
        Self { rules }
    }


    /// Applies the rules in this `RuleSet` to the string passed and returns a modified `String`
    /// with all the matching rules applied.
    fn apply_rules(&self, s: &str) -> String {
        let mut output = Vec::new();
        let s_chars: Vec<char> = s.chars().collect();

        for p in s_chars.as_slice().windows(2) {
            let p_rule = [p[0], p[1]];
            output.push(p[0]);

            if let Some(r) = self.rules.get(&p_rule) {
                output.push(*r);
            }
        }

        output.push(s_chars[s.len() - 1]);
        output.iter().collect()
    }


    /// Returns the result of applying this `RuleSet` to the given string `iterations` times.
    fn apply_rules_repeatedly(&self, s: &str, iterations: usize) -> String {
        let mut current_string = s.to_string();
        for _ in 0..iterations {
            current_string = self.apply_rules(&current_string);
        }
        current_string
    }
}


/// Returns a `HashMap` containing the frequency of every `char` in the input string.
fn count_letter_frequencies(s: &str) -> HashMap<char, u32> {
    let mut count = HashMap::new();

    for c in s.chars() {
        *count.entry(c).or_insert(0) += 1;
    }
    count
}


/// Parses a string consisting of lines of comma separated coordinates, then a blank line, then
/// lines with fold information. Returns a `Grid` containing dots at the coordinates, and a `Vec`
/// containing the individual `Fold` instructions.
fn parse_input(input: &str) -> (&str, RuleSet) {
    let mut line = input.lines();
    let template = line.next().unwrap();

    assert_eq!(line.next().unwrap().len(), 0);

    let ruleset = RuleSet::new(&mut line);

    (template, ruleset)
}


fn main() {
    let input_file =
        fs::read_to_string(INPUT_FILENAME)
            .expect("Error reading input file");

    let (template, ruleset) = parse_input(&input_file);
    let result = ruleset.apply_rules_repeatedly(template, ITERATIONS);
    let frequencies = count_letter_frequencies(&result);

    println!("The frequency of the most common letter in the output minus the least common is {}",
        frequencies.values().max().unwrap() - frequencies.values().min().unwrap()
    );
}


// Test using data from the examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;

    #[test]
    fn test_parse_input() {
        let (template, ruleset) = parse_input(&TEST_INPUT);

        assert_eq!(template, "NNCB");
        assert_eq!(ruleset.rules[(&['C', 'H'])], 'B');
        assert_eq!(ruleset.rules[(&['H', 'H'])], 'N');
        assert_eq!(ruleset.rules[(&['C', 'B'])], 'H');
        assert_eq!(ruleset.rules[(&['N', 'H'])], 'C');
        assert_eq!(ruleset.rules[(&['H', 'B'])], 'C');
        assert_eq!(ruleset.rules[(&['H', 'C'])], 'B');
        assert_eq!(ruleset.rules[(&['H', 'N'])], 'C');
        assert_eq!(ruleset.rules[(&['N', 'N'])], 'C');
        assert_eq!(ruleset.rules[(&['B', 'H'])], 'H');
        assert_eq!(ruleset.rules[(&['N', 'C'])], 'B');
        assert_eq!(ruleset.rules[(&['N', 'B'])], 'B');
        assert_eq!(ruleset.rules[(&['B', 'N'])], 'B');
        assert_eq!(ruleset.rules[(&['B', 'B'])], 'N');
        assert_eq!(ruleset.rules[(&['B', 'C'])], 'B');
        assert_eq!(ruleset.rules[(&['C', 'C'])], 'N');
        assert_eq!(ruleset.rules[(&['C', 'N'])], 'C');
    }

    #[test]
    fn test_apply_rules() {
        let (template, ruleset) = parse_input(&TEST_INPUT);
        let output1 = ruleset.apply_rules(template);
        assert_eq!(output1, "NCNBCHB".to_string());

        let output2 = ruleset.apply_rules(&output1);
        assert_eq!(output2, "NBCCNBBBCBHCB".to_string());

        let output3 = ruleset.apply_rules(&output2);
        assert_eq!(output3, "NBBBCNCCNBBNBNBBCHBHHBCHB".to_string());

        let output4 = ruleset.apply_rules(&output3);
        assert_eq!(output4, "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".to_string());

        let output5 = ruleset.apply_rules(&output4);
        assert_eq!(output5.len(), 97);

        let output6 = ruleset.apply_rules(&output5);
        let output7 = ruleset.apply_rules(&output6);
        let output8 = ruleset.apply_rules(&output7);
        let output9 = ruleset.apply_rules(&output8);
        let output10 = ruleset.apply_rules(&output9);

        let frequencies = count_letter_frequencies(&output10);
        assert_eq!(frequencies[&'B'], 1749);
        assert_eq!(frequencies[&'C'], 298);
        assert_eq!(frequencies[&'H'], 161);
        assert_eq!(frequencies[&'N'], 865);

        assert_eq!(frequencies.values().max().unwrap() - frequencies.values().min().unwrap(),
            1588
        );
    }

    #[test]
    fn test_apply_rules_repeatedly() {
        let (template, ruleset) = parse_input(&TEST_INPUT);
        let output = ruleset.apply_rules_repeatedly(template, ITERATIONS);
        let frequencies = count_letter_frequencies(&output);

        assert_eq!(frequencies[&'B'], 1749);
        assert_eq!(frequencies[&'C'], 298);
        assert_eq!(frequencies[&'H'], 161);
        assert_eq!(frequencies[&'N'], 865);

        assert_eq!(frequencies.values().max().unwrap() - frequencies.values().min().unwrap(),
            1588
        );
    }
}
