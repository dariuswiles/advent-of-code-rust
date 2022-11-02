//! Advent of Code 2020 Day 07
//! https://adventofcode.com/2020/day/7
//!
//! Challenge part 2
//!
//! Determine the total number of bags that need to be carried in the bag color posed in the
//! challenge. As bags can contain other bags which in turn can contain bags, the solution contains
//! a recursive algorithm.

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

            return b;
        } else {
            // println!("No bag '{}' exists, so adding with BagId {}", &name, self.bags.len());
            self.bags.push(Bag {
                name: name.to_owned(),
            });
            return self.bags.len() - 1;
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
            outer_bag: outer_bag,
            inner_bags: inner_bags,
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
        let new_rule = parse_rule(&line, &mut ruleset.bags);
        ruleset.add_rule(new_rule);
    }

    ruleset
}

/// Returns the number of bags that must be contained within the give `outer_bagid`. For example,
/// if bag A must contain 3 bag Bs, and each bag B must contain 2 bag Cs, 3x2 = 6 is returned.
/// Note that the result does not include the containing bag.
fn must_contain_bag_total(rs: &Ruleset, outer_bagid: &BagId) -> u32 {
    // println!("Calculating contents of BagId: {:?}", outer_bagid);

    for r in &rs.rules {
        if r.outer_bag != *outer_bagid {
            continue;
        }

        let num_inner_bags = r.inner_bags.len();
        if num_inner_bags == 0 {
            // println!("BagId {}: This bag contains no other bags. Returning 1.", outer_bagid);
            return 1;
        }

        let mut total = 1;
        for b in &r.inner_bags {
            // println!("BagId {}: Recursively finding total for BagId {}.", outer_bagid, b.0);
            total += b.1 * (must_contain_bag_total(rs, &b.0) + 0);
            // println!("BagId {}: Returned from recursion and total is now {}.", outer_bagid, total);
        }
        // println!("BagId {}: Returning a total of {} other bags.", outer_bagid, total);
        return total;
    }

    panic!("No rule found for bag with BagId {}", outer_bagid);
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let ruleset = parse_rules(&input);

    let target_bag_id = ruleset.bags.get_bag_id(CHALLENGE_BAG).unwrap();
    let total_bags = must_contain_bag_total(&ruleset, &target_bag_id) - 1;

    println!(
        "Number of bags the given bag needs to contain is {}",
        total_bags
    );
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_RULES_0: &str = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const TEST_RULES_1: &str = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn set_0() {
        let ruleset = parse_rules(&TEST_RULES_0);
        println!("{:#?}", &ruleset);
        let target_bag_id = ruleset.bags.get_bag_id("shiny gold").unwrap();
        let total = must_contain_bag_total(&ruleset, &target_bag_id) - 1;
        assert_eq!(total, 32);
    }

    #[test]
    fn set_1() {
        let ruleset = parse_rules(&TEST_RULES_1);
        let target_bag_id = ruleset.bags.get_bag_id("shiny gold").unwrap();
        let total = must_contain_bag_total(&ruleset, &target_bag_id) - 1;
        assert_eq!(total, 126);
    }
}
