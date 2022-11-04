//! Advent of Code 2020 Day 16
//! https://adventofcode.com/2020/day/16
//!
//! Challenge part 2
//!
//! Parse data from an input file representing ticket fields and associated valid ranges, my
//! ticket, and "nearby" tickets. Determine "nearby" tickets that have invalid fields and discard
//! those tickets in their entirety. Analyze the remaining tickets to determine the mapping between
//! the named ticket fields provided in the input, and ticket data. Then return the elements of my
//! ticket in the manner required by the challenge.

use std::collections::HashSet;
use std::fs;
use std::ops::RangeInclusive;
use std::str::Lines;

const INPUT_FILENAME: &str = "2020_day16_input.txt";
const YOUR_TICKET_TITLE: &str = "your ticket:";
const NEARBY_TICKETS_TITLE: &str = "nearby tickets:";

type Ticket = Vec<u32>;

#[derive(Debug, PartialEq)]
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
                panic!(
                    "Missing colon separating name from ranges in string: '{}'",
                    line
                );
            }
            let name = name_then_ranges[0].to_string();

            let tokens: Vec<&str> = name_then_ranges[1].split(" or ").collect();
            if tokens.len() != 2 {
                panic!("Malformed ranges in string: '{}'", line);
            }

            let range0: Vec<u32> = tokens[0].split('-').map(|n| n.parse().unwrap()).collect();
            let range1: Vec<u32> = tokens[1].split('-').map(|n| n.parse().unwrap()).collect();

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

/// Return an updated version of the `ChallengeData` passed in where all invalid 'nearby' tickets
/// have been removed.
fn discard_invalid_tickets(data: &mut ChallengeData) {
    let mut valid_tickets = Vec::new();
    let all_ranges = data.aggregate_ranges();

    for ticket in &data.nearby_tickets {
        let mut valid = true;
        for val in ticket {
            if !all_ranges.contains(val) {
                valid = false;
                // println!("Invalid field value {} in ticket: {:?}", val, ticket);
            }
        }

        if valid {
            valid_tickets.push(ticket.clone());
        }
    }
    data.nearby_tickets = valid_tickets;
}

/// Compares all values column index `column` in the 'nearby' tickets data to field definitions in
/// `data` and returns a vector of only the definitions whose allowed ranges are valid for all
/// values. For example, if a field has ranges 0..3 and 7..9 and the 'nearby' ticket values are
/// 8, 3 and 1, the field will be included in the vector returned.
fn map_one_ticket_field(data: &ChallengeData, column: usize) -> Vec<&TicketField> {
    // println!("Attempting to map column {} in 'nearby' tickets to a field definition.", column);
    let mut possibilities = Vec::new();
    for field in &data.field_definitions {
        possibilities.push(field);
    }

    for ticket in &data.nearby_tickets {
        let mut remaining_possibilities = Vec::new();
        let ticket_val = ticket[column];
        // println!("Checking ticket value {}", ticket_val);

        for p in &possibilities {
            // print!("\tChecking against possibility: {:?}. ", p);
            if p.range0.contains(&ticket_val) || p.range1.contains(&ticket_val) {
                // println!("\tStill a possibility.");
                remaining_possibilities.push(*p);
                // } else {
                // println!("\tNot possible - discarding.");
            }
        }

        possibilities = remaining_possibilities;
        if possibilities.len() == 1 {
            break;
        }
    }
    possibilities
}

/// Returns the field definition associated with each column of data in the 'nearby' tickets. The
/// return vector lists the definitions in the same order as the columns of data.
///
/// # Panics
///
/// Panics if every column cannot be uniquely mapped to a definition.
//
// TODO
// Although the following code works for the example and my test input, it only finds a solution
// if at least one column has only one possible field on each loop iteration. Situations can occur
// where this is not the case but a solution can still be found. This happens when a field
// definition is only listed in one column's remaining possibilities, but no column has only one
// possibility. For example:
// Column 0 possibilities: class, row
// Column 1 possibilities: duration, row
// Column 2 possibilities: duration, row, train
//
// All columns have multiple possibilities, but it can be seen that 'class' only appears once, for
// column 0, and 'train' for column 2. Thus, a solution can be found. The following code could be
// enhanced to perform this check before giving up and panicking.
fn map_all_ticket_fields(data: &ChallengeData) -> Vec<&TicketField> {
    let mut possibilities = Vec::new();
    let num_of_fields = data.field_definitions.len();

    for col in 0..num_of_fields {
        possibilities.push(map_one_ticket_field(&data, col));
    }

    let mut column_verified = Vec::new();
    column_verified.resize(num_of_fields, false);

    let mut verified_columns_total = usize::MAX;
    loop {
        // println!("column_verified at loop start {:#?}", column_verified);

        for col in 0..num_of_fields {
            // Skip columns that already have mappings.
            if column_verified[col] {
                continue;
            }

            // If previous iterations of this loop have eliminated all but one possibility for this
            // column, update the state to indicate this. More importantly, remove this field from
            // the possibilities for all *other* columns.
            if possibilities[col].len() == 1 {
                column_verified[col] = true;

                for other_col in 0..num_of_fields {
                    if (other_col == col) || (column_verified[other_col]) {
                        continue;
                    }

                    if let Some(idx_to_remove) = possibilities[other_col]
                        .iter()
                        .position(|&i| i == possibilities[col][0])
                    {
                        possibilities[other_col].remove(idx_to_remove);
                    }
                }
            }
        }

        let new_verified_columns_total = column_verified.iter().filter(|&n| *n).count();
        if new_verified_columns_total == verified_columns_total {
            panic!("Cannot uniquely map every column of data in 'nearby' tickets to a field defn");
        } else if new_verified_columns_total == num_of_fields {
            break;
        } else {
            verified_columns_total = new_verified_columns_total;
        }
    }

    possibilities.iter().map(|v| v[0]).collect()
}

fn perform_work(input: &str) -> u64 {
    let mut data = ChallengeData::from_string(input);
    discard_invalid_tickets(&mut data);

    let mapping = map_all_ticket_fields(&data);

    let mut answer = 1;

    let mapping_length = mapping.len();
    for i in 0..mapping_length {
        if mapping[i].name.starts_with("departure") {
            answer *= data.my_ticket[i] as u64;
        }
    }

    answer
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

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

    const TEST_INPUT_1: &str = "\
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

    #[test]
    fn test_game_init_and_aggregation() {
        let data = ChallengeData::from_string(&TEST_INPUT_0);

        println!("{:#?}", data);

        let all_ranges = data.aggregate_ranges();

        assert_eq!(all_ranges.len(), 48);

        for c in &[
            1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
            27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
            49, 50,
        ] {
            if !all_ranges.contains(c) {
                panic!("Aggregate range should contain {} but does not.", c);
            }
        }
    }

    #[test]
    fn test_ticket_discard() {
        let mut data = ChallengeData::from_string(&TEST_INPUT_0);
        discard_invalid_tickets(&mut data);

        assert_eq!(data.nearby_tickets, vec![vec![7, 3, 47]]);
    }

    #[test]
    fn partially_determine_field_mapping() {
        let mut data = ChallengeData::from_string(&TEST_INPUT_1);
        discard_invalid_tickets(&mut data);

        let mut results = Vec::new();
        let num_of_fields = data.field_definitions.len();
        for col in 0..num_of_fields {
            results.push(map_one_ticket_field(&data, col));
        }

        assert_eq!(
            results[0],
            vec![&TicketField {
                name: "row".to_string(),
                range0: 0..=5,
                range1: 8..=19,
            },]
        );

        assert_eq!(
            results[1],
            vec![
                &TicketField {
                    name: "class".to_string(),
                    range0: 0..=1,
                    range1: 4..=19,
                },
                &TicketField {
                    name: "row".to_string(),
                    range0: 0..=5,
                    range1: 8..=19,
                },
            ]
        );

        assert_eq!(
            results[2],
            vec![
                &TicketField {
                    name: "class".to_string(),
                    range0: 0..=1,
                    range1: 4..=19,
                },
                &TicketField {
                    name: "row".to_string(),
                    range0: 0..=5,
                    range1: 8..=19,
                },
                &TicketField {
                    name: "seat".to_string(),
                    range0: 0..=13,
                    range1: 16..=19,
                },
            ]
        );
    }
}
