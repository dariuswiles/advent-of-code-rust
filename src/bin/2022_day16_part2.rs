//! Advent of Code 2022 Day 16
//! https://adventofcode.com/2022/day/16
//!
//! Challenge part 2
//!
//! Determine the maximum amount of fluid that can flow through a system compromising individual
//! valves defined in the input file. The challenge consists of moving between valves and opening
//! them in the optimal manner, bearing in mind valves have different flow rates.
//!
//! Part 2 of the challenge adds a second entity which can move and open valves, increasing the
//! complexity of finding the best solution.
//
// The current code takes about 2 minutes to run, so could benefit from optimization. For example,
// a long move to a valve with a low flow could be rejected if a high flow valve is much closer.

use std::collections::{HashMap, HashSet};
use std::fs;

const INPUT_FILENAME: &str = "2022_day16_input.txt";
const INPUT_TOKEN_VALVE: &str = "Valve ";
const INPUT_TOKEN_FLOW_RATE: &str = " has flow rate=";
const INPUT_TOKEN_TUNNEL: &str = "; tunnel leads to valve ";
const INPUT_TOKEN_TUNNELS: &str = "; tunnels lead to valves ";
const TIME_LIMIT: u8 = 26; // In minutes

type FlowRateType = u32;
type Distance = u8;

/// Holds information relating to a `Valve`, composed of its identifier (which should be two
/// characters), its flow rate and a vector of other `Valve`s that can be reached directly from
/// this `Valve` via tunnels.
#[derive(Clone, Debug, PartialEq)]
struct Valve<'a> {
    identifier: &'a str,
    rate: FlowRateType,
    connected_valves: HashSet<&'a str>,
}

/// Parses a line in the format specified in the challenge (see example below), and returns the
/// data it contains as a new `Valve`. The input should be one of the following forms, depending
/// on the number of connecting tunnels:
///     Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
///     Valve HH has flow rate=22; tunnel leads to valve GG
///
/// # Panics
///
/// Panics if the input is not in the expected form (or is an empty string).
fn parse_line(input: &str) -> Valve {
    let identifier_onwards = input.strip_prefix(INPUT_TOKEN_VALVE).unwrap();

    let (identifier, flow_rate_onwards) = identifier_onwards
        .split_once(INPUT_TOKEN_FLOW_RATE)
        .unwrap();

    let mut tunnel_clause = flow_rate_onwards.split_once(INPUT_TOKEN_TUNNEL);
    if tunnel_clause.is_none() {
        tunnel_clause = flow_rate_onwards.split_once(INPUT_TOKEN_TUNNELS);
    }
    let (flow_rate, connected_valves_group) = tunnel_clause.unwrap();

    let connected_valves = connected_valves_group.split(", ").collect();

    Valve {
        identifier,
        rate: flow_rate.parse().unwrap(),
        connected_valves,
    }
}

/// Parses all lines in `input` into a `HashMap` of `Valve`s which is then returned. Empty lines
/// are skipped.
///
/// # Panics
///
/// Panics if the input is not in the expected form.
fn parse_lines(input: &str) -> HashMap<&str, Valve> {
    let mut valves = HashMap::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let v = parse_line(line);
        valves.insert(v.identifier, v);
    }

    valves
}

/// A lookup table indexed with pairs of `Valve` identifiers whose associated value is the
/// shortest distance between those pairs.
struct ValveDistances<'a> {
    map: HashMap<(&'a str, &'a str), Distance>,
}

impl<'a> ValveDistances<'a> {
    /// Creates a `ValveDistances` object which maps every possible pair of `Valve`s with the
    /// shortest distance between them.
    fn generate_valve_distance_lookup_table(
        valves: &'a HashMap<&str, Valve>,
    ) -> ValveDistances<'a> {
        let mut valve_distances: HashMap<(&str, &str), Distance> = HashMap::new();

        for v in valves.values() {
            let mut d = 0u8;
            let mut visited = HashSet::new();
            let mut leading_edge = HashSet::new();

            leading_edge.insert(v.identifier);
            visited.insert(v.identifier);
            loop {
                let mut new_leading_edge = HashSet::new();

                for edge in &leading_edge {
                    new_leading_edge.extend(valves.get(edge).unwrap().connected_valves.clone());
                }

                new_leading_edge = &new_leading_edge - &visited;
                visited.extend(&new_leading_edge.clone());

                if new_leading_edge.is_empty() {
                    break;
                }

                d += 1;
                for nle in &new_leading_edge {
                    if v.identifier < nle {
                        valve_distances.insert((v.identifier, nle), d);
                    }
                }

                leading_edge = new_leading_edge;
            }

            leading_edge.insert(v.identifier);
        }

        Self {
            map: valve_distances,
        }
    }

    /// Returns the shortest `Distance` between the two `Valve` identifiers passed.
    ///
    /// # Panics
    ///
    /// Panics if one or both identifiers are invalid.
    fn distance(&self, a: &str, b: &str) -> Distance {
        if a == b {
            return 0;
        }

        let lower = String::min(a.to_string(), b.to_string());
        let upper = String::max(a.to_string(), b.to_string());

        *self.map.get(&(&lower, &upper)).unwrap()
    }
}

/// Holds the state of an entity consisting of their current 'destination' `Valve` and the last
/// minute when they will be busy moving there and opening the `Valve`. They will be free to make
/// their next move on minute 'busy_until' + 1. An entity is either us or our elephant helper
/// introduced in Part 2 of the challenge.
#[derive(Clone, Debug, PartialEq)]
struct EntityState<'a> {
    destination: &'a str,
    busy_until: u8,
}

/// Returns a `HashMap` that maps each `Valve` whose identifier is passed in 'closed_valve_ids' to
/// the amount of liquid that will flow if we move from 'current_location' to the `Valve` and
/// open it. Theses scores are intended to help algorithms decide which of the closed valves to
/// open. `current_time` indicates the number of minutes that have passed so far, where a value of
/// 0 indicates the initial state with no actions taken and 29 means one more move or open can be
/// performed (though it's too late to make a difference to the result).
fn score_valves<'a>(
    current_location: &'a str,
    current_time: u8,
    valves: &HashMap<&str, Valve>,
    valve_distances: &ValveDistances,
    closed_valve_ids: &HashSet<&'a str>,
) -> HashMap<&'a str, FlowRateType> {
    let mut scored_valves = HashMap::new();

    for v in closed_valve_ids {
        let distance_to_valve = valve_distances.distance(current_location, v);
        if current_time + distance_to_valve + 1 >= TIME_LIMIT {
            continue;
        }

        let total_flow_contribution =
            (TIME_LIMIT - current_time - distance_to_valve) as FlowRateType * valves[v].rate;

        scored_valves.insert(*v, total_flow_contribution);
    }

    scored_valves
}

/// Investigates which of the currently closed `Valve`s the entity that is first available should
/// move to and open. This is decided by trying all the possibilities and picking the one that
/// results in the highest flow. It doesn't matter which entities are passed in `entity0` versus
/// `entity1`.
/// Returns the maximum flow that can be achieved from the starting conditions passed.
fn make_move(
    entity0: &EntityState,
    entity1: &EntityState,
    valves: &HashMap<&str, Valve>,
    valve_distances: &ValveDistances,
    closed_valve_ids: &HashSet<&str>,
    total_flow: FlowRateType,
) -> FlowRateType {
    let time = u8::min(entity0.busy_until, entity1.busy_until) + 1;

    // Is there enough time to move to a closed valve and open it such that it will increase
    // the total flow before 'TIME_LIMIT' minutes?
    if time >= TIME_LIMIT - 2 {
        return total_flow;
    }

    let decision_entity;
    let other_entity;
    if time - 1 == entity0.busy_until {
        decision_entity = entity0;
        other_entity = entity1;
    } else {
        decision_entity = entity1;
        other_entity = entity0;
    }

    let choices = score_valves(
        decision_entity.destination,
        time,
        valves,
        valve_distances,
        closed_valve_ids,
    );

    let mut results = Vec::new();
    for (choice_valve_id, choice_flow_rate) in choices {
        results.push(make_move(
            &EntityState {
                destination: choice_valve_id,
                busy_until: time
                    + valve_distances.distance(decision_entity.destination, choice_valve_id),
            },
            other_entity,
            valves,
            valve_distances,
            &(closed_valve_ids - &HashSet::from([choice_valve_id])),
            total_flow + choice_flow_rate,
        ));
    }

    if !results.is_empty() {
        return *results.iter().max().unwrap();
    }

    total_flow
}

/// Takes the input file, parses it into `Valve` objects, creates a lookup table with the distances
/// between the `Valve`s, and calls the logic that determines the most fluid that can be made to
/// flow by opening the `Valve`s in the optimal order. Returns the optimal result.
fn do_challenge(input: &str) -> FlowRateType {
    let valves = parse_lines(input);
    let valve_distance_lookup = ValveDistances::generate_valve_distance_lookup_table(&valves);

    let valves_with_non_zero_flow: HashSet<&str> = valves
        .iter()
        .filter_map(|(id, v)| if v.rate > 0 { Some(*id) } else { None })
        .collect();

    let us = EntityState {
        destination: "AA",
        busy_until: 0,
    };
    let elephant = EntityState {
        destination: "AA",
        busy_until: 0,
    };

    make_move(
        &us,
        &elephant,
        &valves,
        &valve_distance_lookup,
        &valves_with_non_zero_flow,
        0, // initial total flow
    )
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    println!(
        "The highest achievable flow is {}",
        do_challenge(&input_file)
    );
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"),
            Valve {
                identifier: "AA",
                rate: 0,
                connected_valves: HashSet::from(["DD", "II", "BB"]),
            }
        );
    }

    #[test]
    fn test_parse_lines() {
        let valves = parse_lines(TEST_INPUT);

        assert_eq!(valves.len(), 10);

        assert_eq!(
            valves.get("AA").unwrap(),
            &Valve {
                identifier: "AA",
                rate: 0,
                connected_valves: HashSet::from(["DD", "II", "BB"]),
            }
        );

        assert_eq!(
            valves.get("BB").unwrap(),
            &Valve {
                identifier: "BB",
                rate: 13,
                connected_valves: HashSet::from(["CC", "AA"]),
            }
        );

        assert_eq!(
            valves.get("CC").unwrap(),
            &Valve {
                identifier: "CC",
                rate: 2,
                connected_valves: HashSet::from(["DD", "BB"]),
            }
        );

        assert_eq!(
            valves.get("DD").unwrap(),
            &Valve {
                identifier: "DD",
                rate: 20,
                connected_valves: HashSet::from(["CC", "AA", "EE"]),
            }
        );

        assert_eq!(
            valves.get("EE").unwrap(),
            &Valve {
                identifier: "EE",
                rate: 3,
                connected_valves: HashSet::from(["FF", "DD"]),
            }
        );

        assert_eq!(
            valves.get("FF").unwrap(),
            &Valve {
                identifier: "FF",
                rate: 0,
                connected_valves: HashSet::from(["EE", "GG"]),
            }
        );

        assert_eq!(
            valves.get("GG").unwrap(),
            &Valve {
                identifier: "GG",
                rate: 0,
                connected_valves: HashSet::from(["FF", "HH"]),
            }
        );

        assert_eq!(
            valves.get("HH").unwrap(),
            &Valve {
                identifier: "HH",
                rate: 22,
                connected_valves: HashSet::from(["GG"]),
            }
        );

        assert_eq!(
            valves.get("II").unwrap(),
            &Valve {
                identifier: "II",
                rate: 0,
                connected_valves: HashSet::from(["AA", "JJ"]),
            }
        );

        assert_eq!(
            valves.get("JJ").unwrap(),
            &Valve {
                identifier: "JJ",
                rate: 21,
                connected_valves: HashSet::from(["II"]),
            }
        );
    }

    #[test]
    fn test_valve_distance_lookup_table() {
        let valves = parse_lines(TEST_INPUT);
        let valve_distance_lookup = ValveDistances::generate_valve_distance_lookup_table(&valves);

        assert_eq!(valve_distance_lookup.map.len(), 45);
        assert_eq!(
            valve_distance_lookup.map.get(&("AA", "BB")),
            Some(&(1 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("AA", "CC")),
            Some(&(2 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("AA", "DD")),
            Some(&(1 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("AA", "EE")),
            Some(&(2 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("AA", "FF")),
            Some(&(3 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("AA", "GG")),
            Some(&(4 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("AA", "HH")),
            Some(&(5 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("AA", "II")),
            Some(&(1 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("AA", "JJ")),
            Some(&(2 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("BB", "CC")),
            Some(&(1 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("BB", "DD")),
            Some(&(2 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("BB", "EE")),
            Some(&(3 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("BB", "FF")),
            Some(&(4 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("BB", "GG")),
            Some(&(5 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("BB", "HH")),
            Some(&(6 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("BB", "II")),
            Some(&(2 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("BB", "JJ")),
            Some(&(3 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("CC", "DD")),
            Some(&(1 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("CC", "EE")),
            Some(&(2 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("CC", "FF")),
            Some(&(3 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("CC", "GG")),
            Some(&(4 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("CC", "HH")),
            Some(&(5 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("CC", "II")),
            Some(&(3 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("CC", "JJ")),
            Some(&(4 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("DD", "EE")),
            Some(&(1 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("DD", "FF")),
            Some(&(2 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("DD", "GG")),
            Some(&(3 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("DD", "HH")),
            Some(&(4 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("DD", "II")),
            Some(&(2 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("DD", "JJ")),
            Some(&(3 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("EE", "FF")),
            Some(&(1 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("EE", "GG")),
            Some(&(2 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("EE", "HH")),
            Some(&(3 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("EE", "II")),
            Some(&(3 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("EE", "JJ")),
            Some(&(4 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("FF", "GG")),
            Some(&(1 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("FF", "HH")),
            Some(&(2 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("FF", "II")),
            Some(&(4 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("FF", "JJ")),
            Some(&(5 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("GG", "HH")),
            Some(&(1 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("GG", "II")),
            Some(&(5 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("GG", "JJ")),
            Some(&(6 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("HH", "II")),
            Some(&(6 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("HH", "JJ")),
            Some(&(7 as Distance))
        );
        assert_eq!(
            valve_distance_lookup.map.get(&("II", "JJ")),
            Some(&(1 as Distance))
        );
    }

    #[test]
    fn test_valve_distance_lookup() {
        let valves = parse_lines(TEST_INPUT);
        let valve_distance_lookup = ValveDistances::generate_valve_distance_lookup_table(&valves);

        // Sampled data as the previous test verifies the underlying data exhaustively.
        assert_eq!(valve_distance_lookup.distance("AA", "BB"), 1 as Distance);
        assert_eq!(valve_distance_lookup.distance("AA", "HH"), 5 as Distance);
        assert_eq!(valve_distance_lookup.distance("HH", "JJ"), 7 as Distance);
        assert_eq!(valve_distance_lookup.distance("FF", "HH"), 2 as Distance);
    }

    #[test]
    fn test_score_valves() {
        let valves = parse_lines(TEST_INPUT);
        let valve_distance_lookup = ValveDistances::generate_valve_distance_lookup_table(&valves);

        let result0 = score_valves(
            "AA",
            0,
            &valves,
            &valve_distance_lookup,
            &HashSet::from(["BB", "CC", "DD", "EE", "HH", "JJ"]),
        );

        assert_eq!(result0.len(), 6);
        assert_eq!(
            result0,
            HashMap::from([
                ("BB", 25 * 13),
                ("CC", 24 * 2),
                ("DD", 25 * 20),
                ("EE", 24 * 3),
                ("HH", 21 * 22),
                ("JJ", 24 * 21),
            ])
        );
    }

    #[test]
    fn test_make_move_0() {
        let valves = parse_lines(TEST_INPUT);
        let valve_distance_lookup = ValveDistances::generate_valve_distance_lookup_table(&valves);

        assert_eq!(
            make_move(
                &EntityState {
                    destination: "AA",
                    busy_until: 23
                },
                &EntityState {
                    destination: "AA",
                    busy_until: 23
                },
                &valves,
                &valve_distance_lookup,
                &HashSet::from(["BB", "CC", "DD", "EE", "HH", "JJ"]), // Valves that can be opened
                100,                                                  // Total flow
            ),
            100
        );
    }

    #[test]
    fn test_make_move_1() {
        let valves = parse_lines(TEST_INPUT);
        let valve_distance_lookup = ValveDistances::generate_valve_distance_lookup_table(&valves);

        assert_eq!(
            make_move(
                &EntityState {
                    destination: "AA",
                    busy_until: 22
                },
                &EntityState {
                    destination: "AA",
                    busy_until: 22
                },
                &valves,
                &valve_distance_lookup,
                &HashSet::from(["BB", "CC", "DD", "EE", "HH", "JJ"]), // Valves that can be opened
                50,                                                   // Total flow
            ),
            50 + 20 * 2 + 13 * 2
        );
    }

    #[test]
    fn test_make_move_2() {
        let valves = parse_lines(TEST_INPUT);
        let valve_distance_lookup = ValveDistances::generate_valve_distance_lookup_table(&valves);

        assert_eq!(
            make_move(
                &EntityState {
                    destination: "AA",
                    busy_until: 21
                },
                &EntityState {
                    destination: "AA",
                    busy_until: 21
                },
                &valves,
                &valve_distance_lookup,
                &HashSet::from(["BB", "CC", "DD", "EE", "HH", "JJ"]), // Valves that can be opened
                0,                                                    // Total flow
            ),
            21 * 2 + 20 * 3
        );
    }

    #[test]
    fn test_make_move_3() {
        let valves = parse_lines(TEST_INPUT);
        let valve_distance_lookup = ValveDistances::generate_valve_distance_lookup_table(&valves);

        assert_eq!(
            make_move(
                &EntityState {
                    destination: "AA",
                    busy_until: 20
                },
                &EntityState {
                    destination: "AA",
                    busy_until: 20
                },
                &valves,
                &valve_distance_lookup,
                &HashSet::from(["BB", "CC", "DD", "EE", "HH", "JJ"]), // Valves that can be opened
                0,                                                    // Total flow
            ),
            20 * 4 + 21 * 3 + 3 * 2
        );
    }

    #[test]
    fn test_make_move_4() {
        let valves = parse_lines(TEST_INPUT);
        let valve_distance_lookup = ValveDistances::generate_valve_distance_lookup_table(&valves);

        assert_eq!(
            make_move(
                &EntityState {
                    destination: "AA",
                    busy_until: 19
                },
                &EntityState {
                    destination: "AA",
                    busy_until: 19
                },
                &valves,
                &valve_distance_lookup,
                &HashSet::from(["BB", "CC", "DD", "EE", "HH", "JJ"]), // Valves that can be opened
                0,                                                    // Total flow
            ),
            20 * 5 + 21 * 4 + 13 * 2
        );
    }

    #[test]
    fn test_make_move() {
        let valves = parse_lines(TEST_INPUT);
        let valve_distance_lookup = ValveDistances::generate_valve_distance_lookup_table(&valves);

        assert_eq!(
            make_move(
                &EntityState {
                    destination: "AA",
                    busy_until: 0
                },
                &EntityState {
                    destination: "AA",
                    busy_until: 0
                },
                &valves,
                &valve_distance_lookup,
                &HashSet::from(["BB", "CC", "DD", "EE", "HH", "JJ"]), // Valves that can be opened
                0,                                                    // Total flow
            ),
            1707
        );
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(do_challenge(TEST_INPUT), 1707);
    }
}
