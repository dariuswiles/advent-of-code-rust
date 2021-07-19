//! Advent of Code 2020 Day 16
//! https://adventofcode.com/2020/day/16
//!
//! Challenge part 1
//!
//! Sum all invalid fields of "nearby" tickets. A field is invalid if its value is outside the
//! range of every field.

use std::collections::HashSet;
use std::fs;
use std::ops::RangeInclusive;
use std::str::Lines;

const INPUT_FILENAME: &str = "2020_day16_input.txt";
const YOUR_TICKET_TITLE: &str = "your ticket:";
const NEARBY_TICKETS_TITLE: &str = "nearby tickets:";

type Ticket = Vec<u32>;

#[derive(Debug)]
struct TicketField {
    name: String,
    range0: RangeInclusive<u32>,
    range1: RangeInclusive<u32>,
}

#[derive(Debug)]
struct ChallengeData {
    field_definitions: Vec<TicketField>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl ChallengeData {
    /// Create and return a new `ChallengeData` object containing all data from the string passed.
    /// The data is grouped into three sections: field definitions, data for my ticket, and data
    /// for nearby tickets. Each is parsed and stored separately.
    fn from_string(s: &str) -> Self {
        let mut input_lines = s.lines();

        Self {
            field_definitions: Self::parse_field_definitions(&mut input_lines),
            my_ticket: Self::parse_my_ticket(&mut input_lines),
            nearby_tickets: Self::parse_nearby_tickets(&mut input_lines),
        }
    }

    fn parse_field_definitions(input_lines: &mut Lines) -> Vec<TicketField> {
        let mut defns = Vec::new();

        for line in input_lines {
            if line == "" {
                break;
            }

            let name_then_ranges: Vec<&str> = line.split(": ").collect();
            if name_then_ranges.len() != 2 {
                panic!(format!("Missing colon separating name from ranges in string: '{}'", line));
            }
            let name = name_then_ranges[0].to_string();

            let tokens: Vec<&str> = name_then_ranges[1].split(" or ").collect();
            if tokens.len() != 2 {
                panic!(format!("Malformed ranges in string: '{}'", line));
            }

            let range0: Vec<u32> = tokens[0].split('-')
                .map(|n| n.parse().unwrap())
                .collect();
            let range1: Vec<u32> = tokens[1].split('-')
                .map(|n| n.parse().unwrap())
                .collect();

            defns.push(TicketField {
                name: name,
                range0: range0[0]..=range0[1],
                range1: range1[0]..=range1[1],
            });

        }

        defns
    }

    fn parse_my_ticket(input_lines: &mut Lines) -> Ticket {
        if input_lines.next().unwrap() != YOUR_TICKET_TITLE {
            panic!("Did not find 'your ticket' section of input file where expected");
        }

        let my_ticket = input_lines.next().unwrap();

        if input_lines.next().unwrap() != "" {
            panic!("The 'your ticket' section should end with a blank line, but none was found.");
        }

        my_ticket.split(',').map(|n| n.parse().unwrap()).collect()
    }

    fn parse_nearby_tickets(input_lines: &mut Lines) -> Vec<Ticket> {
        let mut tickets = Vec::new();

        if input_lines.next().unwrap() != NEARBY_TICKETS_TITLE {
            panic!("Did not find 'nearby tickets' section of input file where expected");
        }

        for line in input_lines {
            tickets.push(line.split(',').map(|n| n.parse().unwrap()).collect());
        }

        tickets
    }

    /// Return a `HashSet` containing the superset of all ranges in this object. For example, if
    /// Self contains ranges 1-3 and 9-10, the `HashSet` returned will contain 1, 2, 3, 9 and 10.
    fn aggregate_ranges(&self) -> HashSet<u32> {
        let mut agg = HashSet::new();

        for field in &self.field_definitions {
            for r in field.range0.clone() {
                agg.insert(r);
            }

            for r in field.range1.clone() {
                agg.insert(r);
            }
        }

        agg
    }
}


/// Return the sum of all values of all nearby tickets that are not in the superset of all
/// allowed ticket field ranges. This is the answer required by part 1 of this challenge.
fn perform_work(input: &str) -> u32 {
    let mut answer = 0;

    let data = ChallengeData::from_string(&input);
    let all_ranges = data.aggregate_ranges();

    for ticket in data.nearby_tickets {
        for val in &ticket {
            if !all_ranges.contains(val) {
                answer += val;
            }
        }
    }

    answer
}


fn main() {
    let input_file =
        fs::read_to_string(INPUT_FILENAME)
            .expect("Error reading input file");

    let answer = perform_work(&input_file);
    println!("The answer to the challenge is {:?}", answer);
}


// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_0: &str = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn test_game_init_and_aggregation() {
        let data = ChallengeData::from_string(&TEST_INPUT_0);

        println!("{:#?}", data);

        let all_ranges = data.aggregate_ranges();

        assert_eq!(all_ranges.len(), 48);

        for c in &[1,2,3,5,6,7,8,9,10,11,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,
            31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50
        ] {
            if !all_ranges.contains(c) {
                panic!(format!("Aggregate range should contain {} but does not.", c));
            }
        }
    }

    #[test]
    fn test_game_full() {
        let answer = perform_work(&TEST_INPUT_0);

        assert_eq!(answer, 71);
    }
}
