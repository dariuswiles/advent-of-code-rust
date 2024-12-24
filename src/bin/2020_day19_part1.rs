//! Advent of Code 2020 Day 19
//! https://adventofcode.com/2020/day/19
//!
//! Challenge part 1
//!
//! Parse a set of rules that define whether a string is valid, then validate all the strings in
//! the input file against these rules.

use std::collections::HashMap;
use std::fs;

const INPUT_FILENAME: &str = "2020_day19_input.txt";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Rule {
    Choice(Box<Rule>, Box<Rule>),
    Text(String),
    List(Vec<Id>),
}

type RuleSet = HashMap<Id, Rule>;
type Id = u32;

fn build_ruleset(lines: &[&str]) -> RuleSet {
    let mut ruleset = HashMap::new();

    for line in lines.iter() {
        let new_rule;

        // println!("Ruleset processing line\t{}", &line);

        let id_and_rule: Vec<&str> = line.split(": ").collect();

        if id_and_rule[1].starts_with('"') {
            new_rule = Rule::Text(id_and_rule[1].trim_matches('"').to_string())
        } else if id_and_rule[1].contains('|') {
            let mut left = Vec::new();
            let mut right = Vec::new();

            for cr in id_and_rule[1].split(' ') {
                if cr.starts_with('|') {
                    left = right;
                    right = Vec::new();
                } else {
                    right.push(cr.parse().unwrap());
                }
            }

            new_rule = Rule::Choice(Box::new(Rule::List(left)), Box::new(Rule::List(right)));
        } else {
            let mut child_rules = Vec::new();
            for cr in id_and_rule[1].split(' ') {
                child_rules.push(cr.parse().unwrap());
            }

            new_rule = Rule::List(child_rules);
        }

        // println!("Adding new rule\t{:?} - {:?}", id_and_rule[0].parse::<u32>().unwrap(), &new_rule);
        ruleset.insert(id_and_rule[0].parse().unwrap(), new_rule);
    }
    ruleset
}

/// Validates a ruleset `List`, which is a vector of rules, all of which must be met in the order
/// they appear. If any rule does not match, 0 is immediately returned to indicate the List doesn't
/// match. Otherwise, the number of characters in `msg` that are matched by all the rules is
/// returned.
fn validate_list(ruleset: &RuleSet, msg: &str, child_rules: &Vec<Id>) -> usize {
    let mut matched_so_far = 0;
    for cr in child_rules {
        let matched = validate_message(ruleset, &msg[matched_so_far..], *cr);
        if matched == 0 {
            return 0;
        } else {
            matched_so_far += matched;
        }
    }

    matched_so_far
}

/// The rule with id `rule_id` is looked up in `ruleset`, and is evaluated based on its type. If it
/// matches the leftmost character or characters in `msg`, the number of characters matched is
/// returned. If the rule doesn't match, 0 is returned.
fn validate_message(ruleset: &RuleSet, msg: &str, rule_id: Id) -> usize {
    let rule = &ruleset[&rule_id];

    match rule {
        Rule::Choice(left, right) => {
            if let Rule::List(left_rules) = &**left {
                let left_result = validate_list(ruleset, msg, left_rules);
                if left_result != 0 {
                    return left_result;
                }
            } else {
                panic!(
                    "Unexpected rule type found on left side of rule {}",
                    rule_id
                );
            }

            if let Rule::List(right_rules) = &**right {
                validate_list(ruleset, msg, right_rules)
            } else {
                panic!(
                    "Unexpected rule type found on right side of rule {}",
                    rule_id
                );
            }
        }
        Rule::Text(s) => {
            if msg.starts_with(s) {
                s.len()
            } else {
                0
            }
        }
        Rule::List(child_rules) => validate_list(ruleset, msg, child_rules),
    }
}

/// Determines if `msg` matches any rules in `ruleset` and returns the result.
fn is_message_valid(ruleset: &RuleSet, msg: &str) -> bool {
    if msg.is_empty() {
        return false;
    }

    let is_valid = validate_message(ruleset, msg, 0);

    is_valid == msg.len()
}

fn parse_rules_and_verify_messages(input: &str) -> u32 {
    let mut input_lines = input.lines();
    let mut rules_input = Vec::new();

    for line in &mut input_lines {
        if line.is_empty() {
            break;
        }
        rules_input.push(line);
    }
    let ruleset = build_ruleset(&rules_input);
    // println!("Ruleset:\n{:#?}", &ruleset);

    let mut valid_messages = 0;
    for line in &mut input_lines {
        if is_message_valid(&ruleset, line) {
            // println!("Valid message '{}'", &line);
            valid_messages += 1;
        } else {
            // println!("Invalid message '{}'", &line);
        }
    }

    valid_messages
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let answer = parse_rules_and_verify_messages(&input_file);
    println!("{} messages are valid", answer);
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_0: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

    #[test]
    fn validate_test_input() {
        assert_eq!(parse_rules_and_verify_messages(TEST_INPUT_0), 2);
    }

    #[test]
    fn rule_creation() {
        let mut rules_input = Vec::new();

        for line in &mut TEST_INPUT_0.lines() {
            if line.is_empty() {
                break;
            }
            rules_input.push(line);
        }
        let ruleset = build_ruleset(&rules_input);

        assert_eq!(ruleset[&0], Rule::List(vec![4, 1, 5]));
        assert_eq!(
            ruleset[&1],
            Rule::Choice(
                Box::new(Rule::List(vec![2, 3])),
                Box::new(Rule::List(vec![3, 2]))
            )
        );
        assert_eq!(
            ruleset[&2],
            Rule::Choice(
                Box::new(Rule::List(vec![4, 4])),
                Box::new(Rule::List(vec![5, 5]))
            )
        );
        assert_eq!(
            ruleset[&3],
            Rule::Choice(
                Box::new(Rule::List(vec![4, 5])),
                Box::new(Rule::List(vec![5, 4]))
            )
        );
        assert_eq!(ruleset[&4], Rule::Text("a".to_string()));
        assert_eq!(ruleset[&5], Rule::Text("b".to_string()));
    }

    #[test]
    fn validate_text() {
        let mut ruleset = HashMap::new();
        ruleset.insert(0, Rule::Text("c".to_string()));

        assert!(is_message_valid(&ruleset, "c"));
        assert!(!is_message_valid(&ruleset, "x"));
        assert!(!is_message_valid(&ruleset, "cc"));
        assert!(!is_message_valid(&ruleset, ""));
    }

    #[test]
    fn validate_list() {
        let mut ruleset = HashMap::new();
        ruleset.insert(0, Rule::List(vec![1, 2, 1]));
        ruleset.insert(1, Rule::Text("c".to_string()));
        ruleset.insert(2, Rule::Text("d".to_string()));

        assert!(is_message_valid(&ruleset, "cdc"));
        assert!(!is_message_valid(&ruleset, "cdd"));
        assert!(!is_message_valid(&ruleset, "ccc"));
        assert!(!is_message_valid(&ruleset, "cdcc"));
        assert!(!is_message_valid(&ruleset, "ccdc"));
        assert!(!is_message_valid(&ruleset, ""));
    }
}
