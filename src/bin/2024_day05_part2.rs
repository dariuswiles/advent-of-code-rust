//! Advent of Code 2024 Day 05
//! https://adventofcode.com/2024/day/5
//!
//! Challenge part 2
//!
//! Given a set of rules restricting which order pairs of pages must appear in a sequence of pages,
//! determines which sequences of pages meet the rules. The challenge answer is then the sum of the
//! middle pages of each of the invalid sequences after they have been corrected to follow all
//! rules. Valid sequences are simply ignored.

use std::collections::{HashMap, HashSet};
use std::fs;

const INPUT_FILENAME: &str = "2024_day05_input.txt";

type Rules = HashMap<u8, HashSet<u8>>;
type PageUpdate = Vec<u8>;

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    println!(
        "The sum of all corrected invalid page update sequences is {}",
        do_challenge(&input)
    );
}

/// Returns the sum of the results of checking the validity of each sequence of page updates. Each
/// check returns the middle page number, so summing these page numbers gives the challenge answer.
fn do_challenge(input: &str) -> u32 {
    let (rules, page_updates) = parse_input(input);

    page_updates
        .iter()
        .filter_map(|pu| check_page_updates(&rules, pu))
        .sum()
}

/// Returns the first section of input as `Rules` and the second section as a `Vec` of
/// `PageUpdate`s. The former maps a page number to the set of all page numbers that must appear
/// after it.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> (Rules, Vec<PageUpdate>) {
    let mut rules: Rules = HashMap::new();
    let mut lines = input.lines();
    // while let Some(line) = lines.next() {
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let (earlier_str, later_str) = line
            .split_once('|')
            .expect("Each rule must contain a '|' character");

        let earlier: u8 = earlier_str
            .parse()
            .expect("Rule contains invalid page identifier '{earlier_str}'");

        let later: u8 = later_str
            .parse()
            .expect("Rule contains invalid page identifier '{later_str}'");

        match rules.get_mut(&earlier) {
            None => {
                rules.insert(earlier, HashSet::from([later]));
            }
            Some(later_pages) => {
                later_pages.insert(later);
            }
        }
    }

    let mut page_updates = Vec::new();
    for line in lines {
        if !line.is_empty() {
            page_updates.push(line.split(',').map(|n| n.parse::<u8>().unwrap()).collect());
        }
    }

    (rules, page_updates)
}

/// Checks the validity of the `page_updates` sequence against `Rules`. A valid sequence is one
/// where every `Rule` is followed, i.e., where every pair of pages that comprise a rule and which
/// are in the sequence are in the order mandated by the rule. As per part 2 of the challenge,
/// valid page updates are ignored and invalid updates are corrected by reordering them until they
/// meet all the rules.
///
/// The return value is:
///     - `None` for valid `page_updates`;
///     - the middle page value of invalid `page_updates` after being corrected to follow all rules.
fn check_page_updates(rules: &Rules, page_updates: &PageUpdate) -> Option<u32> {
    let mut i = 1;
    let mut pages = page_updates.clone(); // Only required to keep compiler happy
    let mut modified_pages = page_updates.clone();
    let mut original_data_is_valid = true;

    while i < page_updates.len() {
        pages = modified_pages.clone();

        let (page, preceding_pages) = &pages[..=i].split_last().unwrap();

        if let Some(rule) = rules.get(page) {
            for (pp_index, pp) in preceding_pages.iter().enumerate() {
                if rule.contains(pp) {
                    modified_pages = pages.clone();
                    modified_pages.swap(pp_index, i);
                    i = 0;
                    original_data_is_valid = false;
                    break;
                }
            }
        }

        i += 1;
    }

    if original_data_is_valid {
        None
    } else {
        Some((pages[(page_updates.len() - 1) / 2]).into())
    }
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn test_parse_input() {
        let (rules, page_updates) = parse_input(TEST_INPUT);

        assert_eq!(6, rules.len());
        assert_eq!(Some(&HashSet::from([53, 13, 61, 29])), rules.get(&47));
        assert_eq!(
            Some(&HashSet::from([13, 61, 47, 29, 53, 75])),
            rules.get(&97)
        );
        assert_eq!(Some(&HashSet::from([29, 53, 47, 61, 13])), rules.get(&75));
        assert_eq!(Some(&HashSet::from([13, 53, 29])), rules.get(&61));
        assert_eq!(Some(&HashSet::from([13])), rules.get(&29));
        assert_eq!(Some(&HashSet::from([29, 13])), rules.get(&53));

        assert_eq!(6, page_updates.len());
        assert_eq!(vec![75, 47, 61, 53, 29], page_updates[0]);
        assert_eq!(vec![97, 61, 53, 29, 13], page_updates[1]);
        assert_eq!(vec![75, 29, 13], page_updates[2]);
        assert_eq!(vec![75, 97, 47, 61, 53], page_updates[3]);
        assert_eq!(vec![61, 13, 29], page_updates[4]);
        assert_eq!(vec![97, 13, 75, 29, 47], page_updates[5]);
    }

    #[test]
    fn test_check_page_updates() {
        let (rules, page_updates) = parse_input(TEST_INPUT);

        assert_eq!(None, check_page_updates(&rules, &page_updates[0]));
        assert_eq!(None, check_page_updates(&rules, &page_updates[1]));
        assert_eq!(None, check_page_updates(&rules, &page_updates[2]));
        assert_eq!(Some(47), check_page_updates(&rules, &page_updates[3]));
        assert_eq!(Some(29), check_page_updates(&rules, &page_updates[4]));
        assert_eq!(Some(47), check_page_updates(&rules, &page_updates[5]));
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(do_challenge(TEST_INPUT), 123);
    }
}
