//! Advent of Code 2020 Day 19
//! https://adventofcode.com/2020/day/19
//!
//! Challenge part 2
//!
//! Parse a set of rules that define whether a string is valid, then validate all the strings in
//! the input file against these rules. Part 2 adds recursive rules.


use std::collections::HashMap;
use std::fs;
use std::iter;

const INPUT_FILENAME: &str = "2020_day19_input.txt";
const MAX_RECURSION_LEVEL: u8 = 4;
const EMPTY_ARRAY: [Id; 0] = [];

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Rule {
    Choice(Box<Rule>, Box<Rule>),
    ChoiceRecursive(Box<Rule>, Box<Rule>),
    Text(String),
    List(Vec<Id>),
}

type RuleSet = HashMap<Id, Rule>;
type Id = u32;


fn build_ruleset(lines: &Vec<&str>) -> RuleSet {
    let mut ruleset = HashMap::new();

    for line in lines.iter() {
        let new_rule;

//         println!("Ruleset processing line\t{}", &line);

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

            new_rule = Rule::Choice(
                Box::new(Rule::List(left)),
                Box::new(Rule::List(right)),
            );
        } else {
            let mut child_rules = Vec::new();
            for cr in id_and_rule[1].split(' ') {
                child_rules.push(cr.parse().unwrap());
            }

            new_rule = Rule::List(child_rules);
        }

//         println!("Adding new rule\t{:?} - {:?}", id_and_rule[0].parse::<u32>().unwrap(), &new_rule);
        ruleset.insert(id_and_rule[0].parse().unwrap(), new_rule);
    }
    ruleset
}


/// Part 2 of the challenge requires two changes to the ruleset given in the input file. Rather
/// than create a modified version of the given input file, this function makes the two changes
/// to the `ruleset` passed.
fn patch_ruleset_for_part2(ruleset: &mut RuleSet) {
    ruleset.insert(8, Rule::ChoiceRecursive(
                Box::new(Rule::List(vec![42])),
                Box::new(Rule::List(vec![42, 8])),
    ));

    ruleset.insert(11, Rule::ChoiceRecursive(
                Box::new(Rule::List(vec![42, 31])),
                Box::new(Rule::List(vec![42, 11, 31])),
    ));
}


/// Validates a ruleset `List`, which is a vector of rules, all of which must be met in the order
/// they appear. If any rule does not match, 0 is immediately returned to indicate the List doesn't
/// match. Otherwise, the number of characters in `msg` that are matched by all the rules is
/// returned.
fn validate_list(ruleset: &RuleSet, msg: &str, child_rules: &Vec<Id>, recurse: &HashMap<Id, usize>)
    -> usize
{
    let mut matched_so_far = 0;
    for cr in child_rules {
        let matched = validate_message(ruleset, &msg[matched_so_far..], *cr, recurse);
        if matched == 0 {
            return 0;
        } else {
            matched_so_far += matched;
        }
    }

    return matched_so_far;
}


/// The rule with id `rule_id` is looked up in `ruleset`, and is evaluated based on its type. If it
/// matches the leftmost character or characters in `msg`, the number of characters matched is
/// returned. If the rule doesn't match, 0 is returned.
fn validate_message(ruleset: &RuleSet, msg: &str, rule_id: Id, recursion: &HashMap<Id, usize>)
    -> usize
{
    let rule = &ruleset[&rule_id];

    match rule {
        Rule::Choice(left, right) => {
            if let Rule::List(left_rules) = &**left {
                let left_result = validate_list(ruleset, msg, &left_rules, recursion);
                if left_result != 0 {
                    return left_result;
                }
            } else {
                panic!("Unexpected rule type found on left side of rule {}", rule_id);
            }

            if let Rule::List(right_rules) = &**right {
                return validate_list(ruleset, msg, &right_rules, recursion);
            } else {
                panic!("Unexpected rule type found on right side of rule {}", rule_id);
            }
        }
        Rule::ChoiceRecursive(left, right) => {
            // NOTE This code is a partial implementation that only works in specific cases, namely
            //      that the left choice of the rule is the same as the right choice except that
            //      the recursive term is omitted. For example, "8: 42 | 42 8" is acceptable
            //      because the left choice is "42 8" without the "8".

            let mut left_choice: Vec<Id>;

            if let Rule::List(left_rules) = &**left {
                left_choice = left_rules.iter().cloned().collect();
            } else {
                panic!("Unexpected rule type found on left side of rule {}", rule_id);
            }

            if let Rule::List(right_rules) = &**right {

                let recursion_position = right_rules.iter().position(|&r| r == rule_id).unwrap();

                let before_recursion: &[Id] = &right_rules[..recursion_position];

                let after_recursion;
                if right_rules.len() > recursion_position + 1 {
                    after_recursion = &right_rules[recursion_position+1..];
                } else {
                    after_recursion = &EMPTY_ARRAY;
                }

                let recursion_level = *recursion.get(&rule_id).expect(&format!(
                    "Recursive rule id {} needs an associated recursion level to be passed",
                    rule_id
                ));

                let mut new_list: Vec<Id> = iter::repeat(before_recursion)
                    .take(recursion_level as usize)
                    .collect::<Vec<&[Id]>>()
                    .concat()
                    .to_vec();

                new_list.append(&mut left_choice);

                new_list.append(&mut iter::repeat(after_recursion)
                    .take(recursion_level as usize)
                    .collect::<Vec<&[Id]>>()
                    .concat()
                    .to_vec());

//                 println!("Rule id {}: Checking for matches with generated recursive rule {:?}",
//                     rule_id, &new_list);

                validate_list(ruleset, msg, &new_list, recursion)

            } else {
                panic!("Unexpected rule type found on right side of rule {}", rule_id);
            }
        }
        Rule::Text(s) => {
            if msg.starts_with(s) {
                return s.len();
            } else {
                return 0;
            }
        }
        Rule::List(child_rules) => {
            validate_list(ruleset, msg, child_rules, recursion)
        }
    }
}


/// Determines if `msg` matches any rules in `ruleset` and returns the result.
fn is_message_valid(ruleset: &RuleSet, msg: &str) -> bool {
    if msg.len() == 0 {
        return false;
    }

    let mut recursion = HashMap::new();

    for (rule_id, rule) in ruleset.iter() {
        if let Rule::ChoiceRecursive(..) = rule {
            recursion.insert(*rule_id, 0);
        }
    }

    let mut recursion_rule_ids: Vec<Id> = recursion.keys().cloned().collect();
    recursion_rule_ids.sort_unstable();

    // Search for a permutation of rules that match the text of message `msg`. Permutations are
    // constructed by cycling through recursion levels for each of the recursive rules. For
    // example, if rules 8 and 11 are recursive, try both rules without recursion, then rule 8 with
    // one level while rule 11 is still none, then rule 8 with two levels, etc. The maximum
    // recursion level is defined in MAX_RECURSION_LEVEL.
    let mut complete = false;
    while !complete {
//         println!("Validate message using recursion values of {:?}", &recursion);

        if validate_message(ruleset, msg, 0, &recursion) == msg.len() {
            return true;
        }

        complete = true;
        for rid in &recursion_rule_ids {
            let recursion_value = recursion[&rid];

            if recursion_value < MAX_RECURSION_LEVEL as usize {
                recursion.insert(*rid, recursion_value + 1);
                complete = false;
                break;
            } else {
                recursion.insert(*rid, 0);
            }
        }
    }

    false
}


/// Parse the given `input` and return a tuple containing its `RuleSet` and list of messages that
/// need to be validated.
fn parse_input(input: &str) -> (RuleSet, Vec<&str>) {
    let mut input_lines = input.lines();
    let mut rules_input = Vec::new();

    for line in &mut input_lines {
        if line.len() == 0 {
            break;
        }
        rules_input.push(line);
    }

    let ruleset = build_ruleset(&rules_input);

    let mut messages = Vec::new();
    for line in &mut input_lines {
        if line == "" {
            continue;
        }

        messages.push(line);
    }

    (ruleset, messages)
}


fn verify_messages(ruleset: &RuleSet, messages: Vec<&str>) -> u32 {
    let mut valid_messages = 0;
    for msg in messages.iter() {
        if is_message_valid(&ruleset, &msg) {
//             println!("Valid message '{}'", &msg);
            valid_messages += 1;
        } else {
//             println!("Invalid message '{}'", &msg);
        }
    }

    valid_messages
}


fn do_challenge(input: &str) -> u32 {
    let (mut ruleset, messages) = parse_input(input);
    patch_ruleset_for_part2(&mut ruleset);
//     println!("Ruleset:\n{:?}", &ruleset);

    verify_messages(&ruleset, messages)
}


fn main() {
    let input_file =
        fs::read_to_string(INPUT_FILENAME)
            .expect("Error reading input file");

    let answer = do_challenge(&input_file);
    println!("{} messages are valid", answer);
}


// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_0: &str =
r#"0: 4 1 5
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


    const TEST_INPUT_1: &str =
r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;


    #[test]
    fn validate_test_input_0() {
        assert_eq!(do_challenge(&TEST_INPUT_0), 2);
    }

    #[test]
    fn rule_creation_0() {
        let mut rules_input = Vec::new();

        for line in &mut TEST_INPUT_0.lines() {
            if line.len() == 0 {
                break;
            }
            rules_input.push(line);
        }
        let ruleset = build_ruleset(&rules_input);

        assert_eq!(ruleset[&0], Rule::List(vec![4, 1, 5]));
        assert_eq!(ruleset[&1], Rule::Choice(Box::new(Rule::List(vec![2, 3])), Box::new(Rule::List(vec![3, 2]))));
        assert_eq!(ruleset[&2], Rule::Choice(Box::new(Rule::List(vec![4, 4])), Box::new(Rule::List(vec![5, 5]))));
        assert_eq!(ruleset[&3], Rule::Choice(Box::new(Rule::List(vec![4, 5])), Box::new(Rule::List(vec![5, 4]))));
        assert_eq!(ruleset[&4], Rule::Text("a".to_string()));
        assert_eq!(ruleset[&5], Rule::Text("b".to_string()));
    }

    #[test]
    fn validate_text() {
        let mut ruleset = HashMap::new();
        ruleset.insert(0, Rule::Text("c".to_string()));

        assert!(is_message_valid(&ruleset, &"c".to_string()));
        assert!(!is_message_valid(&ruleset, &"x".to_string()));
        assert!(!is_message_valid(&ruleset, &"cc".to_string()));
        assert!(!is_message_valid(&ruleset, &"".to_string()));
    }

    #[test]
    fn validate_list() {
        let mut ruleset = HashMap::new();
        ruleset.insert(0, Rule::List(vec![1, 2, 1]));
        ruleset.insert(1, Rule::Text("c".to_string()));
        ruleset.insert(2, Rule::Text("d".to_string()));

        assert!(is_message_valid(&ruleset, &"cdc".to_string()));
        assert!(!is_message_valid(&ruleset, &"cdd".to_string()));
        assert!(!is_message_valid(&ruleset, &"ccc".to_string()));
        assert!(!is_message_valid(&ruleset, &"cdcc".to_string()));
        assert!(!is_message_valid(&ruleset, &"ccdc".to_string()));
        assert!(!is_message_valid(&ruleset, &"".to_string()));
    }

    #[test]
    fn validate_recursive_list() {
        let mut ruleset = HashMap::new();
        ruleset.insert(0, Rule::ChoiceRecursive(
            Box::new(Rule::List(vec![1])),
            Box::new(Rule::List(vec![1, 0])),
        ));
        ruleset.insert(1, Rule::Text("e".to_string()));

        assert!(is_message_valid(&ruleset, &"e".to_string()));
        assert!(is_message_valid(&ruleset, &"ee".to_string()));
        assert!(is_message_valid(&ruleset, &"eee".to_string()));
    }

    #[test]
    fn full_test_no_recursive_rules() {
        let mut input = &TEST_INPUT_1;
        let (ruleset, messages) = parse_input(&mut input);
        let result = verify_messages(&ruleset, messages);

        assert_eq!(result, 3);
    }

    #[test]
    fn full_test_with_recursive_rules() {
        let mut input = &TEST_INPUT_1;
        let (mut ruleset, messages) = parse_input(&mut input);

        patch_ruleset_for_part2(&mut ruleset);
        println!("Ruleset:\n{:?}", &ruleset);

        let result = verify_messages(&ruleset, messages);

        assert_eq!(result, 12);
    }
}
