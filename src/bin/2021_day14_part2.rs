//! Advent of Code 2021 Day 14
//! https://adventofcode.com/2021/day/14
//!
//! Challenge part 2
//!
//! Read a string and a set of transformation rules from an input file, repeatedly apply the
//! rules defined in the challenge and output an answer based on the final string. Part 2 increases
//! the number of required iterations.

use std::collections::HashMap;
use std::fs;
use std::str::Lines;

const INPUT_FILENAME: &str = "2021_day14_input.txt";
const ITERATIONS: usize = 40;

type Pair = [char; 2];

/// A `RuleSet` is a set of transformation rules.
#[derive(Clone, Debug, Eq, PartialEq)]
struct RuleSet {
    rules: HashMap<Pair, char>,
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
            let rule: Pair = [rule_chars[0], rule_chars[1]];

            rules.insert(rule, line_split[1].chars().next().unwrap());
        }
        Self { rules }
    }
}

/// Stores the number of occurrences of each distinct pair of `char`.
#[derive(Clone, Debug, Eq, PartialEq)]
struct PairTally {
    template: String,
    pairs: HashMap<Pair, u64>,
}

impl PairTally {
    /// Returns a new `PairTally` from a string by looking at each overlapping pair of `char`s.
    fn new(template: &str) -> Self {
        let mut pairs = HashMap::new();

        let template_chars: Vec<char> = template.chars().collect();

        for p in template_chars.as_slice().windows(2) {
            let counter = pairs.entry([p[0], p[1]]).or_insert(0);
            *counter += 1;
        }

        Self {
            template: template.to_string(),
            pairs,
        }
    }

    /// Applies the rules in the `RuleSet` passed to the pairs of `char`s in this object.
    fn apply_rules(&mut self, rules: &RuleSet) {
        let mut new_pairs = HashMap::new();

        for (pair, count) in &self.pairs {
            let char_to_insert: char = rules.rules[pair];
            add(&mut new_pairs, &[pair[0], char_to_insert], *count);
            add(&mut new_pairs, &[char_to_insert, pair[1]], *count);
        }

        self.pairs = new_pairs;
    }

    /// Applies the given `RuleSet` to the data in this object `iterations` times.
    fn apply_rules_repeatedly(&mut self, rules: &RuleSet, iterations: usize) {
        for _ in 0..iterations {
            self.apply_rules(rules);
        }
    }

    /// Returns a `HashMap` containing the frequency of every `char` in this object.
    fn letter_frequencies(&self) -> HashMap<char, u64> {
        let mut freq = HashMap::new();

        for (pair, count) in &self.pairs {
            *freq.entry(pair[0]).or_insert(0) += count;
            *freq.entry(pair[1]).or_insert(0) += count;
        }

        // Every char in the string is double counted as it appears in exactly two pairs, except
        // the first and last chars in the original `template` string, that only appear once. Add
        // these two chars in so every char is double counted.
        *freq
            .entry(self.template.chars().next().unwrap())
            .or_insert(0) += 1;
        *freq
            .entry(self.template.chars().last().unwrap())
            .or_insert(0) += 1;

        // Halve the frequency of each char to correct for the double counting.
        for (_, count) in freq.iter_mut() {
            *count /= 2;
        }

        freq
    }
}

fn add(hm: &mut HashMap<Pair, u64>, pair: &Pair, inc: u64) {
    let counter = hm.entry(*pair).or_insert(0);
    *counter += inc;
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
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let (template, ruleset) = parse_input(&input_file);
    let mut pt = PairTally::new(template);
    pt.apply_rules_repeatedly(&ruleset, ITERATIONS);
    let frequencies = pt.letter_frequencies();

    println!(
        "The frequency of the most common letter in the output minus the least common is {}",
        frequencies.values().max().unwrap() - frequencies.values().min().unwrap()
    );
}

// Test using data from the examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
NNCB

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
CN -> C";

    #[test]
    fn test_parse_input() {
        let (template, ruleset) = parse_input(TEST_INPUT);

        assert_eq!(template, "NNCB");
        assert_eq!(ruleset.rules[&['C', 'H']], 'B');
        assert_eq!(ruleset.rules[&['H', 'H']], 'N');
        assert_eq!(ruleset.rules[&['C', 'B']], 'H');
        assert_eq!(ruleset.rules[&['N', 'H']], 'C');
        assert_eq!(ruleset.rules[&['H', 'B']], 'C');
        assert_eq!(ruleset.rules[&['H', 'C']], 'B');
        assert_eq!(ruleset.rules[&['H', 'N']], 'C');
        assert_eq!(ruleset.rules[&['N', 'N']], 'C');
        assert_eq!(ruleset.rules[&['B', 'H']], 'H');
        assert_eq!(ruleset.rules[&['N', 'C']], 'B');
        assert_eq!(ruleset.rules[&['N', 'B']], 'B');
        assert_eq!(ruleset.rules[&['B', 'N']], 'B');
        assert_eq!(ruleset.rules[&['B', 'B']], 'N');
        assert_eq!(ruleset.rules[&['B', 'C']], 'B');
        assert_eq!(ruleset.rules[&['C', 'C']], 'N');
        assert_eq!(ruleset.rules[&['C', 'N']], 'C');
    }

    #[test]
    fn test_create_pairs() {
        let (template, _ruleset) = parse_input(TEST_INPUT);
        let pt = PairTally::new(template);

        // Expecting NNCB
        assert_eq!(pt.pairs[&['N', 'N']], 1);
        assert_eq!(pt.pairs[&['N', 'C']], 1);
        assert_eq!(pt.pairs[&['C', 'B']], 1);
    }

    #[test]
    fn test_apply_rules() {
        let (template, ruleset) = parse_input(TEST_INPUT);
        let mut pt = PairTally::new(template);
        pt.apply_rules(&ruleset);

        // Expecting NCNB CHB
        assert_eq!(pt.pairs[&['N', 'C']], 1);
        assert_eq!(pt.pairs[&['C', 'N']], 1);
        assert_eq!(pt.pairs[&['N', 'B']], 1);
        assert_eq!(pt.pairs[&['B', 'C']], 1);
        assert_eq!(pt.pairs[&['C', 'H']], 1);
        assert_eq!(pt.pairs[&['H', 'B']], 1);
    }

    #[test]
    fn test_apply_rules_repeatedly_1() {
        let (template, ruleset) = parse_input(TEST_INPUT);
        let mut pt = PairTally::new(template);
        pt.apply_rules_repeatedly(&ruleset, 1);

        // Expecting NCNB CHB
        assert_eq!(pt.pairs[&['N', 'C']], 1);
        assert_eq!(pt.pairs[&['C', 'N']], 1);
        assert_eq!(pt.pairs[&['N', 'B']], 1);
        assert_eq!(pt.pairs[&['B', 'C']], 1);
        assert_eq!(pt.pairs[&['C', 'H']], 1);
        assert_eq!(pt.pairs[&['H', 'B']], 1);
    }

    #[test]
    fn test_apply_rules_repeatedly_2() {
        let (template, ruleset) = parse_input(TEST_INPUT);
        let mut pt = PairTally::new(template);
        pt.apply_rules_repeatedly(&ruleset, 2);

        // Expecting: NBCC NBBB CBHCB
        assert_eq!(pt.pairs[&['B', 'B']], 2);
        assert_eq!(pt.pairs[&['B', 'C']], 2);
        assert_eq!(pt.pairs[&['B', 'H']], 1);
        assert_eq!(pt.pairs[&['C', 'B']], 2);
        assert_eq!(pt.pairs[&['C', 'C']], 1);
        assert_eq!(pt.pairs[&['C', 'N']], 1);
        assert_eq!(pt.pairs[&['N', 'B']], 2);
        assert_eq!(pt.pairs[&['H', 'C']], 1);
    }

    #[test]
    fn test_apply_rules_repeatedly_3() {
        let (template, ruleset) = parse_input(TEST_INPUT);
        let mut pt = PairTally::new(template);
        pt.apply_rules_repeatedly(&ruleset, 3);

        // Expecting: NBBB CNCC NBBN BNBB CHBH HBCH B
        assert_eq!(pt.pairs[&['B', 'B']], 4);
        assert_eq!(pt.pairs[&['B', 'C']], 3);
        assert_eq!(pt.pairs[&['B', 'H']], 1);
        assert_eq!(pt.pairs[&['B', 'N']], 2);
        assert_eq!(pt.pairs[&['C', 'C']], 1);
        assert_eq!(pt.pairs[&['C', 'H']], 2);
        assert_eq!(pt.pairs[&['C', 'N']], 2);
        assert_eq!(pt.pairs[&['H', 'B']], 3);
        assert_eq!(pt.pairs[&['H', 'H']], 1);
        assert_eq!(pt.pairs[&['N', 'B']], 4);
        assert_eq!(pt.pairs[&['N', 'C']], 1);
    }

    #[test]
    fn test_apply_rules_repeatedly_4() {
        let (template, ruleset) = parse_input(TEST_INPUT);
        let mut pt = PairTally::new(template);
        pt.apply_rules_repeatedly(&ruleset, 4);

        // Expecting: NBBN BNBB CCNB CNCC NBBN BBNB BBNB BNBB CBHC BHHN HCBB CBHC B
        assert_eq!(pt.pairs[&['B', 'B']], 9);
        assert_eq!(pt.pairs[&['B', 'C']], 4);
        assert_eq!(pt.pairs[&['B', 'H']], 3);
        assert_eq!(pt.pairs[&['B', 'N']], 6);
        assert_eq!(pt.pairs[&['C', 'B']], 5);
        assert_eq!(pt.pairs[&['C', 'C']], 2);
        assert_eq!(pt.pairs[&['C', 'N']], 3);
        assert_eq!(pt.pairs[&['H', 'C']], 3);
        assert_eq!(pt.pairs[&['H', 'H']], 1);
        assert_eq!(pt.pairs[&['H', 'N']], 1);
        assert_eq!(pt.pairs[&['N', 'B']], 9);
        assert_eq!(pt.pairs[&['N', 'C']], 1);
        assert_eq!(pt.pairs[&['N', 'H']], 1);
    }

    #[test]
    fn frequency_for_4() {
        let (template, ruleset) = parse_input(TEST_INPUT);
        let mut pt = PairTally::new(template);
        pt.apply_rules_repeatedly(&ruleset, 4);
        let freq = pt.letter_frequencies();

        // Expecting: NBBN BNBB CCNB CNCC NBBN BBNB BBNB BNBB CBHC BHHN HCBB CBHC B
        assert_eq!(freq[&'B'], 23);
        assert_eq!(freq[&'C'], 10);
        assert_eq!(freq[&'H'], 5);
        assert_eq!(freq[&'N'], 11);
    }

    #[test]
    fn test_apply_rules_repeatedly_5() {
        let (template, ruleset) = parse_input(TEST_INPUT);
        let mut pt = PairTally::new(template);
        pt.apply_rules_repeatedly(&ruleset, 5);
        let freq = pt.letter_frequencies();

        assert_eq!(freq.values().sum::<u64>(), 97);
    }

    #[test]
    fn frequency_for_10() {
        let (template, ruleset) = parse_input(TEST_INPUT);
        let mut pt = PairTally::new(template);
        pt.apply_rules_repeatedly(&ruleset, 10);
        let frequencies = pt.letter_frequencies();

        assert_eq!(frequencies[&'B'], 1749);
        assert_eq!(frequencies[&'C'], 298);
        assert_eq!(frequencies[&'H'], 161);
        assert_eq!(frequencies[&'N'], 865);

        assert_eq!(
            frequencies.values().max().unwrap() - frequencies.values().min().unwrap(),
            1588
        );
    }

    #[test]
    fn frequency_for_40() {
        let (template, ruleset) = parse_input(TEST_INPUT);
        let mut pt = PairTally::new(template);
        pt.apply_rules_repeatedly(&ruleset, 40);
        let frequencies = pt.letter_frequencies();

        assert_eq!(frequencies[&'B'], 2192039569602);
        assert_eq!(frequencies[&'H'], 3849876073);

        assert_eq!(
            frequencies.values().max().unwrap() - frequencies.values().min().unwrap(),
            2188189693529
        );
    }
}
