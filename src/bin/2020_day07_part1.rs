//! Advent of Code 2020 Day 07
//! https://adventofcode.com/2020/day/7
//!
//! Challenge part 1
//!
//! Determine the number of different bag colors that can contain the bag color posed in the
//! challenge.

use std::collections::HashSet;
use std::fs;

const INPUT_FILENAME: &str = "2020_day07_input.txt";
const CHALLENGE_BAG: &str = "shiny gold"; // Name of bag needed for the challenge answer.

/// A single bag.
#[derive(Debug, PartialEq)]
struct Bag {
    name: String,
}

type BagId = usize;

/// A collection of bags, each with a unique index.
#[derive(Debug)]
struct Bags {
    bags: Vec<Bag>,
}

impl Bags {
    fn new() -> Self {
        Self { bags: Vec::new() }
    }

    fn add_bag_id(&mut self, name: &str) -> BagId {
        if let Some(b) = self.bags.iter().position(|b| b.name == name) {
            // println!("Lookup of bag '{}' found existing bag with BagId {}", &name, b);

            b
        } else {
            // println!("No bag '{}' exists, so adding with BagId {}", &name, self.bags.len());
            self.bags.push(Bag {
                name: name.to_owned(),
            });
            self.bags.len() - 1
        }
    }

    fn get_bag_id(&self, name: &str) -> Option<BagId> {
        // println!("Finding `BagId` for bag name '{}'", name);

        self.bags.iter().position(|b| b.name == name)
    }
}

/// A `Rule` maps a containing `outer_bag` to the zero or more `inner_bags` it contains. Each
/// inner bag is a tuple of the `BagId` of the inner bag and a count of the number of those bags
/// that need to be present.
#[derive(Debug)]
struct Rule {
    outer_bag: BagId,
    inner_bags: Vec<(BagId, u32)>,
}

impl Rule {
    fn new(outer_bag: BagId, inner_bags: Vec<(BagId, u32)>) -> Self {
        Self {
            outer_bag,
            inner_bags,
        }
    }
}

/// A `Ruleset` holds zero or more `Rules`.
#[derive(Debug)]
struct Ruleset {
    rules: Vec<Rule>,
    bags: Bags,
}

impl Ruleset {
    fn new() -> Self {
        Self {
            rules: Vec::new(),
            bags: Bags::new(),
        }
    }

    fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule)
    }
}

/// Returns the given string with either " bags " or " bag" removed from its end.
///
/// # Panics
/// Panics if the given string contains neither of the expected suffixes.
fn strip_bag_suffix(s: &str) -> &str {
    if let Some(stripped) = s.strip_suffix(" bags") {
        stripped
    } else {
        s.strip_suffix(" bag").unwrap()
    }
}

fn parse_rule(line: &str, bags: &mut Bags) -> Rule {
    // println!("parse_rule parsing input line: {}", line);

    let outside_inside: Vec<&str> = line.split(" bags contain ").collect();
    // println!("Outside: '{}'", outside_inside[0]);
    // println!("Inside: '{}'", outside_inside[1]);
    let outside_bag_id = bags.add_bag_id(outside_inside[0]);

    let inside: Vec<&str> = outside_inside[1]
        .strip_suffix('.')
        .unwrap()
        .split(", ")
        .collect();
    // println!("Inside tokenized: '{:?}'", inside);

    let mut inside_bags = Vec::new();
    for b in inside {
        // println!("Examining `inside` string: '{:?}'", b);

        if b == "no other bags" {
            // println!("Leaf rule");
            break;
        } else {
            let inside_split: Vec<&str> = b.splitn(2, ' ').collect();
            // println!("Bag '{}', count = '{}'", inside_split[1], inside_split[0]);

            let bag_id = bags.add_bag_id(strip_bag_suffix(inside_split[1]));

            inside_bags.push((bag_id, inside_split[0].parse::<u32>().unwrap()));
        }
    }

    // println!("Returning: {:?} = {:?}", outside_bag_id, inside_bags);
    Rule::new(outside_bag_id, inside_bags)
}

fn parse_rules(input: &str) -> Ruleset {
    let mut ruleset = Ruleset::new();

    for line in input.lines() {
        let new_rule = parse_rule(line, &mut ruleset.bags);
        ruleset.add_rule(new_rule);
    }

    ruleset
}

/// Return the set of `BagId`s of all bags that can contain `target_bag_name`.
fn outer_bag_options(rs: &Ruleset, target_bag_name: &str) -> HashSet<BagId> {
    let target_bag_id = rs.bags.get_bag_id(target_bag_name).unwrap();

    // println!("Target bag: name = {}, BagId = {}", target_bag_name, target_bag_id);

    let mut matching_outer_bags = HashSet::new();
    let mut bags_to_check = Vec::new();

    bags_to_check.push(target_bag_id);

    while let Some(b) = bags_to_check.pop() {
        // println!("Looking for outer bags that can directly contain bag {}", &b);

        // If Bag `b` has already been examined, i.e., it is already in `matching_outer_bags`, skip
        // the rest of this loop and move on to the next `bag_to_check`.
        if matching_outer_bags.contains(&b) {
            // println!("Skipping, as this bag has already been examined.");
            matching_outer_bags.insert(b);
            continue;
        }

        // Bag `b` has not previously been examined, so look for it in the `inner_bags` fields of
        // all rules in the ruleset.
        for r in &rs.rules {
            let matching_bag = r
                .inner_bags
                .iter()
                .position(|(bag_id, _count)| *bag_id == b);

            if matching_bag.is_some() {
                // println!("Bag {} can contain bag {}", &r.outer_bag, &b);
                bags_to_check.push(r.outer_bag);
            }
        }

        matching_outer_bags.insert(b);
    }

    matching_outer_bags.remove(&target_bag_id);
    matching_outer_bags
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let ruleset = parse_rules(&input);

    let obo = outer_bag_options(&ruleset, CHALLENGE_BAG);

    println!("Number of outer bag options is {}", obo.len());
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_RULES: &str = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn set_0() {
        let ruleset = parse_rules(TEST_RULES);
        let obo = outer_bag_options(&ruleset, "shiny gold");
        assert_eq!(obo.len(), 4);
    }
}
