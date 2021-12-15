//! Advent of Code 2021 Day 12
//! https://adventofcode.com/2021/day/12
//!
//! Challenge part 2
//!
//! Traverse a cave system and determine the number of valid paths through it. Part 2 of the
//! challenge allows a single small cave to be visited twice instead of just once.

use std::collections::{ HashMap, HashSet };
use std::fs;

const INPUT_FILENAME: &str = "2021_day12_input.txt";

#[derive(Clone, Debug, Eq, PartialEq)]
struct Cave<'a> {
    name: &'a str,
    big: bool,
    connections: HashSet<&'a str>,
}

impl<'a> Cave<'a> {
    fn new(name: &'a str, connection: &'a str) -> Self {
        Cave {
            name,
            big: name.chars().fold(true, |acc, c| acc && c.is_uppercase()),
            connections: vec![connection].iter().cloned().collect(),
        }
    }
}


/// Converts the input into a `HashMap` of `Cave`s indexed by the `Cave` name.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> HashMap<String, Cave> {
    let mut caves: HashMap<String, Cave> = HashMap::new();

    for line in input.lines() {
        if line == "" {
            continue;
        }

        let end_points: Vec<&str> = line.split('-').collect();
        if end_points.len() != 2 {
            println!("Malformed input in data: {}", &line);
        }

        if let Some(cave) = caves.get_mut(end_points[0]) {
            cave.connections.insert(end_points[1]);
        } else {
            caves.insert(end_points[0].to_string(), Cave::new(end_points[0], &end_points[1]));
        }
    }
    caves
}


/// Takes a `HashMap` of `Cave`s and modifies it to add the reverse connections. For example, if
/// the `HashMap` contains `Cave` 'A' that connects to cave b, modifies cave b to include a
/// connection back to cave A. This makes it easier to exhaustively try all possible routes
/// through the caves. Reverse connections are not created for the "start" and "end" caves.
fn add_reverse_connections(caves: &mut HashMap<String, Cave>) {
// fn add_reverse_connections<'a>(caves: &'a mut HashMap<&'a str, Cave>) {
    for (_, cave) in caves.clone().iter() {
        if cave.name != "start" {
            for conn_end in &cave.connections {
                if let Some(ce) = caves.get_mut(&conn_end.to_string()) {
                    ce.connections.insert(cave.name);
                } else {
                    caves.insert(conn_end.to_string(), Cave::new(conn_end, cave.name));
                }
            }
        }
    }
}


/// Converts a `Vec` of `Cave`s to a comma-separated string of their names.
fn convert_cave_list_to_string(path: &Vec<&Cave>) -> String {
    path.iter().map(|c| c.name).collect::<Vec<&str>>().join(",")
}


/// Recursive part of `walk_paths` that should only be called from there. It walks all paths
/// between `Cave`s, avoiding small `Cave`s that have already been visited (as indicated by their
/// presence in `path`), starting with `current_cave`. A path terminates when there are no further
/// `Cave`s that can be visited, or the "end" `Cave` is reached. In the former case, the unfinished
/// path is discarded. The return value is a `Vec` containing all the paths found from this call
/// to this function.
///
/// Part 2 of the challenge allows a single small `Cave` to be visited twice. The name of this
/// `Cave` is preselected and passed in `small_cave_twice`.
fn walk_paths_int<'a>(
    caves: &'a HashMap<String, Cave>,
    path: &Vec<&'a Cave>,
    current_cave: &'a Cave,
    small_cave_twice: &str,
) -> Vec<Vec<&'a Cave<'a>>> {
    let mut this_path: Vec<&Cave> = path.to_vec();
    this_path.push(current_cave);

    if current_cave.name == "end" {
        return vec![this_path];
    }

    let mut completed_paths = Vec::new();
    for next_cave_name in &current_cave.connections {
        let next_cave: &Cave = &caves[&next_cave_name.to_string()];

        // Can only visit small caves once.
        if !next_cave.big {
            let next_cave_visits = this_path.iter().filter(|c| c == &&next_cave).count();

            if (next_cave_visits == 1 && next_cave.name != small_cave_twice) ||
                next_cave_visits == 2 {
                continue;
            }
        }

        let mut paths = walk_paths_int(caves, &this_path, &next_cave, small_cave_twice);
        if !paths.is_empty() {
            completed_paths.append(&mut paths);
        }
    }

    completed_paths
}


/// Walks all paths between `Cave`s and returns a sorted `Vec` of strings indicating every valid
/// path.
fn walk_paths(caves: &HashMap<String, Cave>) -> Vec<String> {
    let mut results = Vec::new();

    for c in caves.values() {
        if c.big || c.name == "start" || c.name == "end" {
            continue;
        }
        let paths = walk_paths_int(caves, &Vec::new(), &caves["start"], &c.name);

        for p in paths {
            results.push(convert_cave_list_to_string(&p));
        }
    }

    results.sort_unstable();
    results.dedup();
    results
}


fn main() {
    let input_file =
        fs::read_to_string(INPUT_FILENAME)
            .expect("Error reading input file");

    let mut caves = parse_input(&input_file);
    add_reverse_connections(&mut caves);

    println!("There are {} paths through the cave system", walk_paths(&caves).len());
}


// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str =
r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;

    const TEST_INPUT_2: &str =
r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#;

    const TEST_INPUT_3: &str =
r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#;

    #[test]
    fn create_caves() {
        let cave1 = Cave::new(&"AA", "bb");
        assert_eq!(cave1.name, "AA");
        assert_eq!(cave1.big, true);
        assert_eq!(cave1.connections, vec!["bb"].iter().cloned().collect());

        let cave2 = Cave::new(&"bb", "CC");
        assert_eq!(cave2.name, "bb");
        assert_eq!(cave2.big, false);
        assert_eq!(cave2.connections, vec!["CC"].iter().cloned().collect());
    }

    #[test]
    fn parse_test_input() {
        let caves = parse_input(&TEST_INPUT_1);

        let start_cave = &caves["start"];
        assert_eq!(start_cave.name, "start");
        assert_eq!(start_cave.big, false);
        assert_eq!(start_cave.connections, vec!["A", "b"].iter().cloned().collect());

        let cave_a = &caves["A"];
        assert_eq!(cave_a.name, "A");
        assert_eq!(cave_a.big, true);
        assert_eq!(cave_a.connections, vec!["b", "c", "end"].iter().cloned().collect());
    }

    #[test]
    fn test_reverse_connections() {
        let mut caves = parse_input(&TEST_INPUT_1);
        add_reverse_connections(&mut caves);

        assert_eq!(caves["start"].connections, vec!["A", "b"].iter().cloned().collect());
        assert_eq!(caves["A"].connections, vec!["b", "c", "end"].iter().cloned().collect());
        assert_eq!(caves["b"].connections, vec!["A", "d", "end"].iter().cloned().collect());
        assert_eq!(caves["c"].connections, vec!["A"].iter().cloned().collect());
        assert_eq!(caves["d"].connections, vec!["b"].iter().cloned().collect());
    }

    #[test]
    fn test_convert_cave_list_to_string() {
        let mut caves = parse_input(&TEST_INPUT_1);
        add_reverse_connections(&mut caves);
        let path: Vec<&Cave> = vec![&caves["start"], &caves["b"], &caves["A"], &caves["end"]];

        assert_eq!(convert_cave_list_to_string(&path), "start,b,A,end");
    }

    #[test]
    fn test_walk_paths_1() {
        let mut caves = parse_input(&TEST_INPUT_1);
        add_reverse_connections(&mut caves);
        assert_eq!(walk_paths(&caves),
            vec!["start,A,b,A,b,A,c,A,end",
                    "start,A,b,A,b,A,end",
                    "start,A,b,A,b,end",
                    "start,A,b,A,c,A,b,A,end",
                    "start,A,b,A,c,A,b,end",
                    "start,A,b,A,c,A,c,A,end",
                    "start,A,b,A,c,A,end",
                    "start,A,b,A,end",
                    "start,A,b,d,b,A,c,A,end",
                    "start,A,b,d,b,A,end",
                    "start,A,b,d,b,end",
                    "start,A,b,end",
                    "start,A,c,A,b,A,b,A,end",
                    "start,A,c,A,b,A,b,end",
                    "start,A,c,A,b,A,c,A,end",
                    "start,A,c,A,b,A,end",
                    "start,A,c,A,b,d,b,A,end",
                    "start,A,c,A,b,d,b,end",
                    "start,A,c,A,b,end",
                    "start,A,c,A,c,A,b,A,end",
                    "start,A,c,A,c,A,b,end",
                    "start,A,c,A,c,A,end",
                    "start,A,c,A,end",
                    "start,A,end",
                    "start,b,A,b,A,c,A,end",
                    "start,b,A,b,A,end",
                    "start,b,A,b,end",
                    "start,b,A,c,A,b,A,end",
                    "start,b,A,c,A,b,end",
                    "start,b,A,c,A,c,A,end",
                    "start,b,A,c,A,end",
                    "start,b,A,end",
                    "start,b,d,b,A,c,A,end",
                    "start,b,d,b,A,end",
                    "start,b,d,b,end",
                    "start,b,end",
            ]
        );
    }

    #[test]
    fn test_walk_paths_2() {
        let mut caves = parse_input(&TEST_INPUT_2);
        add_reverse_connections(&mut caves);
        assert_eq!(walk_paths(&caves).len(), 103);
    }

    #[test]
    fn test_walk_paths_3() {
        let mut caves = parse_input(&TEST_INPUT_3);
        add_reverse_connections(&mut caves);
        assert_eq!(walk_paths(&caves).len(), 3509);
    }
}
