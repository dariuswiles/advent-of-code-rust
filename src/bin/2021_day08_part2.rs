//! Advent of Code 2021 Day 08
//! https://adventofcode.com/2021/day/8
//!
//! Challenge part 2
//!
//! Deduce which wires connect to which segments of several 7-digit displays. Use this knowledge to
//! determine the current readout on these displays and sum the numbers shown on all the displays
//! provided in the input file to determine the challenge answer.

use std::collections::{ HashMap, HashSet };
use std::fs;

const INPUT_FILENAME: &str = "2021_day08_input.txt";

const SEGMENT_PATTERNS: [&str; 10] = [
    &"abcefg",   // Digit 0,  6 segments
    &"cf",       // Digit 1,  2 segments
    &"acdeg",    // Digit 2,  5 segments
    &"acdfg",    // Digit 3,  5 segments
    &"bcdf",     // Digit 4,  4 segments
    &"abdfg",    // Digit 5,  5 segments
    &"abdefg",   // Digit 6,  6 segments
    &"acf",      // Digit 7,  3 segments
    &"abcdefg",  // Digit 8,  7 segments
    &"abcdfg",   // Digit 9,  6 segments
];


#[derive(Debug, PartialEq)]
struct ActiveWireSet {
    wires: HashSet<char>,
}

impl ActiveWireSet {
    fn new(input: &str) -> Self {
        let mut wires = HashSet::new();

        for c in input.chars() {
            wires.insert(c.clone());
        }
        Self { wires }
    }
}


/// Parses an input string consisting of a series of 10 blocks of segment letters, delimited by
/// spaces, then a pipe separator, then a further 4 blocks of segment letters. Returns a Vec
/// containing one element per line as a pair. The left side of the pair contains the 10 blocks,
/// and the right side the 4 blocks. The blocks of letters are represented as sets.
///
/// # Panics
///
/// Panics if the input string is malformed.
fn parse_input(input: &str) -> Vec<(Vec<ActiveWireSet>, Vec<ActiveWireSet>)> {
    let mut output = Vec::new();

    for line in input.lines() {
        if line == "" {
            continue;
        }

        let left_right: Vec<&str> = line.split(" | ").collect();
        if left_right.len() != 2 {
            panic!("Malformed input in: {}", line);
        }

        let left: Vec<ActiveWireSet> =
            left_right[0]
            .split(' ')
            .map(|s| ActiveWireSet::new(s))
            .collect();

        if left.len() != 10 {
            panic!("Malformed input with left segments in: {}", line);
        }

        let right: Vec<ActiveWireSet> =
            left_right[1]
            .split(' ')
            .map(|s| ActiveWireSet::new(s))
            .collect();

        if right.len() != 4 {
            panic!("Malformed input with right segments in: {}", line);
        }

        output.push((left, right));
    }
    output
}


/// Deduces the wire for segment 'a' by removing both elements from the set that has 2 active wires
/// from the set that has 3.
///
/// # Panics
///
/// Panics if the input sets don't contain 2 elements and 3 elements respectively, or if the
/// result contains more than one element (indicating that the contents of the sets passed is
/// incorrect).
fn deduce_wire_a(two_active: &ActiveWireSet, three_active: &ActiveWireSet) -> char {
    assert_eq!(two_active.wires.len(), 2);
    assert_eq!(three_active.wires.len(), 3);

    let result: HashSet<char> = three_active.wires.difference(&two_active.wires).cloned()
        .collect();
    assert_eq!(result.len(), 1);

    result.iter().next().cloned().unwrap()
}


/// Deduces the wire for segment 'd' by finding the one common wire between the set with 4 active
/// wires and the three sets that have 5.
///
/// # Panics
///
/// Panics if the input sets don't contain 4 elements and 5 elements respectively, or if the
/// result contains more than one element (indicating that the contents of the sets passed is
/// incorrect).
fn deduce_wire_d(
    four_active: &ActiveWireSet,
    five_active_1: &ActiveWireSet,
    five_active_2: &ActiveWireSet,
    five_active_3: &ActiveWireSet,
) -> char {
    assert_eq!(four_active.wires.len(), 4);
    assert_eq!(five_active_1.wires.len(), 5);
    assert_eq!(five_active_2.wires.len(), 5);
    assert_eq!(five_active_3.wires.len(), 5);

    let result: HashSet<char> =
        four_active.wires
        .intersection(&five_active_1.wires).cloned().collect::<HashSet<char>>()
        .intersection(&five_active_2.wires).cloned().collect::<HashSet<char>>()
        .intersection(&five_active_3.wires).cloned().collect();

    assert_eq!(result.len(), 1);

    result.iter().next().cloned().unwrap()
}


/// Deduces the wire for segment 'g' by finding the three common wires between the sets with 5
/// active wires, and then removing wires 'a' and 'd', which must be known.
///
/// # Panics
///
/// Panics if the input sets don't 5 elements, if wire 'a' or 'd' aren't common to these sets, or
/// if the result contains more than one element (indicating that the contents of the sets passed
/// is incorrect).
fn deduce_wire_g(
    five_active_1: &ActiveWireSet,
    five_active_2: &ActiveWireSet,
    five_active_3: &ActiveWireSet,
    wire_a: &char,
    wire_d: &char,
) -> char {
    assert_eq!(five_active_1.wires.len(), 5);
    assert_eq!(five_active_2.wires.len(), 5);
    assert_eq!(five_active_3.wires.len(), 5);

    let mut result: HashSet<char> =
        five_active_1.wires
        .intersection(&five_active_2.wires).cloned().collect::<HashSet<char>>()
        .intersection(&five_active_3.wires).cloned().collect();

    assert!(result.remove(wire_a));
    assert!(result.remove(wire_d));
    assert_eq!(result.len(), 1);

    result.iter().next().cloned().unwrap()
}


/// Deduces the wire for segment 'b' by removing known wire 'd' from the set containing 4 active
/// wires, and then removing the wires in the set containing 2 active wires.
///
/// # Panics
///
/// Panics if the input sets don't 4 and 2 elements respectively, if wire 'd' isn't in the
/// resulting set, or if the result contains more than one element (indicating that the contents of
/// the sets passed is incorrect).
fn deduce_wire_b(
    two_active: &ActiveWireSet,
    four_active: &ActiveWireSet,
    wire_d: &char,
) -> char {
    assert_eq!(two_active.wires.len(), 2);
    assert_eq!(four_active.wires.len(), 4);

    let mut four_cloned = four_active.wires.clone();
    assert!(four_cloned.remove(wire_d));

    let result: HashSet<char> = four_cloned.difference(&two_active.wires).cloned().collect();
    assert_eq!(result.len(), 1);

    result.iter().next().cloned().unwrap()
}


/// Deduces the wire for segment 'f' by finding the three common wires between the sets with 6
/// active wires, and then removing wires 'a', 'b' and 'g', which must already be known.
///
/// # Panics
///
/// Panics if the input sets don't 6 elements, if any of wires 'a', 'b' or 'g' aren't in the
/// resulting set, or if the result contains more than one element (indicating that the contents of
/// the sets passed is incorrect).
fn deduce_wire_f(
    six_active_1: &ActiveWireSet,
    six_active_2: &ActiveWireSet,
    six_active_3: &ActiveWireSet,
    wire_a: &char,
    wire_b: &char,
    wire_g: &char,
) -> char {
    assert_eq!(six_active_1.wires.len(), 6);
    assert_eq!(six_active_2.wires.len(), 6);
    assert_eq!(six_active_3.wires.len(), 6);

    let mut result: HashSet<char> =
        six_active_1.wires
        .intersection(&six_active_2.wires).cloned().collect::<HashSet<char>>()
        .intersection(&six_active_3.wires).cloned().collect();

    assert!(result.remove(wire_a));
    assert!(result.remove(wire_b));
    assert!(result.remove(wire_g));
    assert_eq!(result.len(), 1);

    result.iter().next().cloned().unwrap()
}



/// Deduces the wire for segment 'c' by removing the known 'f' wire from the set that has 2 active
/// wires.
///
/// # Panics
///
/// Panics if the input set doesn't contain 2 elements, or if the result contains more than one
/// element (indicating that the contents of the sets passed is incorrect).
fn deduce_wire_c(two_active: &ActiveWireSet, wire_f: &char) -> char {
    assert_eq!(two_active.wires.len(), 2);

    let mut two_cloned = two_active.wires.clone();
    assert!(two_cloned.remove(wire_f));

    two_cloned.iter().next().cloned().unwrap()
}


/// Deduces the wire for segment 'e' by removing all the other known wires from the entire set of
/// wires.
///
/// # Panics
///
/// Panics if the result contains more than one element (indicating that the input passed is
/// incorrect).
fn deduce_wire_e(wire_a: &char, wire_b: &char, wire_c: &char, wire_d: &char, wire_f: &char,
    wire_g: &char,
) -> char {
    let mut wires = HashSet::new();

    for c in "abcdefg".chars() {
        wires.insert(c);
    }

    assert!(wires.remove(wire_a));
    assert!(wires.remove(wire_b));
    assert!(wires.remove(wire_c));
    assert!(wires.remove(wire_d));
    assert!(wires.remove(wire_f));
    assert!(wires.remove(wire_g));

    assert_eq!(wires.len(), 1);
    wires.iter().next().cloned().unwrap()
}


/// Uses a process of deduction to determine the correlation between a wire and a segment in the
/// display, based on the given wire sets. The output is a mapping of wire label to segment label,
/// e.g., wire 'a' connects to segment 'b'.
fn deduce_all_wires(wire_sets: &Vec<ActiveWireSet>) -> HashMap<char, char> {
    let mut two_active = &ActiveWireSet { wires: HashSet::new() };
    let mut three_active = &ActiveWireSet { wires: HashSet::new() };
    let mut four_active = &ActiveWireSet { wires: HashSet::new() };
    let mut five_active = Vec::new(); // 3 sets
    let mut six_active = Vec::new(); // 3 sets

    for ws in wire_sets {
        match ws.wires.len() {
            2 => { two_active = ws; }
            3 => { three_active = ws; }
            4 => { four_active = ws; }
            5 => { five_active.push(ws); }
            6 => { six_active.push(ws); }
            7 => { /* Do nothing as the wire set with 7 wires contains each wire */ }
            _ => { panic!("Input contains a set that has only one, or more than 7, wires."); }
        }
    }

    let mut s2w = HashMap::new();  // Key is segment, value is wire
    s2w.insert('a', deduce_wire_a(&two_active, &three_active));
    s2w.insert('d',
        deduce_wire_d(&four_active, &five_active[0], &five_active[1], &five_active[2])
    );
    s2w.insert('g',
        deduce_wire_g(&five_active[0], &five_active[1], &five_active[2], &s2w[&'a'], &s2w[&'d'])
    );
    s2w.insert('b', deduce_wire_b(&two_active, &four_active, &s2w[&'d']));
    s2w.insert('f', deduce_wire_f(&six_active[0], &six_active[1], &six_active[2], &s2w[&'a'],
            &s2w[&'b'], &s2w[&'g']
        )
    );
    s2w.insert('c', deduce_wire_c(&two_active, &s2w[&'f']));
    s2w.insert('e',
        deduce_wire_e(&s2w[&'a'], &s2w[&'b'], &s2w[&'c'], &s2w[&'d'], &s2w[&'f'], &s2w[&'g'])
    );


    let mut w2s= HashMap::new();  // Key is wire, value is segment

    for (w, s) in s2w.iter() {
        w2s.insert(*s, *w);
    }

    w2s
}


/// Takes a set of wires and a mapping of wires to segments, and returns a `String` containing the
/// segments that the wires correspond to. The `char`s in the return value are sorted.
fn wire_set_to_segment_set(map: &HashMap<char, char>, wire_set: &ActiveWireSet) -> String {
    let mut segments = Vec::new();

    for w in &wire_set.wires {
        segments.push(map.get(w).unwrap());
    }

    segments.sort_unstable();
    segments.iter().cloned().collect()
}


/// Takes a set of wires, maps them to display segments using `map`, and determines which display
/// digit this corresponds to. For example, the set "feagb" could map to segments, "acdeg", which
/// is the digit 2.
///
/// # Panics
///
/// Panics if the input does not map to a digit.
fn wire_set_to_digit(map: &HashMap<char, char>, wire_set: &ActiveWireSet) -> u8 {
    let wires_len = wire_set.wires.len();
    match wires_len {
        2 => { return 1; }
        3 => { return 7; }
        4 => { return 4; }
        7 => { return 8; }
        _ => { }
    }

    let segments = wire_set_to_segment_set(map, wire_set);

    SEGMENT_PATTERNS.iter().position(|sp| sp == &&segments).unwrap() as u8
}


/// Deduces the wire to segment mapping for every line of the input file, uses this to determine
/// the output digits (provided as the right-hand side of the input), and sums them to produce
/// the challenge answer.
fn sum_all_output_digits(wire_sets: &Vec<(Vec<ActiveWireSet>, Vec<ActiveWireSet>)>) -> u64 {
    let mut total = 0;
    for ws in wire_sets {
        let map = deduce_all_wires(&ws.0);

        let mut subtotal = 0;
        for output in &ws.1 {
            subtotal = subtotal * 10 + wire_set_to_digit(&map, &output) as u64;
        }

        total += subtotal;
    }

    total
}


fn main() {
    let input_file =
        fs::read_to_string(INPUT_FILENAME)
            .expect("Error reading input file");

    let wire_sets = parse_input(&input_file);
    println!("The sum of all output digits is {}", sum_all_output_digits(&wire_sets));
}


// Test using data from the examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_ONE_LINE: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    const TEST_INPUT: &str =
r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

    #[test]
    fn parse_test_input() {
        let wire_sets = parse_input(&TEST_INPUT);

        assert_eq!(wire_sets[0].0[0], ActiveWireSet::new("be"));
        assert_eq!(wire_sets[0].0[4], ActiveWireSet::new("cgeb"));
        assert_eq!(wire_sets[0].0[9], ActiveWireSet::new("edb"));
        assert_eq!(wire_sets[1].1[1], ActiveWireSet::new("cgb"));
        assert_eq!(wire_sets[9].0[4], ActiveWireSet::new("gf"));
        assert_eq!(wire_sets[9].1[2], ActiveWireSet::new("fg"));
    }

    #[test]
    fn test_deduce_wire_a() {
        assert_eq!(deduce_wire_a(&ActiveWireSet::new("be"), &ActiveWireSet::new("edb")), 'd');
    }

    #[test]
    fn test_deduce_wire_d() {
        assert_eq!(deduce_wire_d(
                &ActiveWireSet::new("cgeb"),
                &ActiveWireSet::new("fdcge"),
                &ActiveWireSet::new("fecdb"),
                &ActiveWireSet::new("fabcd")
            ), 'c'
        );
    }

    #[test]
    fn test_deduce_wire_g() {
        assert_eq!(deduce_wire_g(
                &ActiveWireSet::new("fdcge"),
                &ActiveWireSet::new("fecdb"),
                &ActiveWireSet::new("fabcd"),
                &'d',
                &'c',
            ), 'f'
        );
    }

    #[test]
    fn test_deduce_wire_b() {
        assert_eq!(deduce_wire_b(
                &ActiveWireSet::new("be"),
                &ActiveWireSet::new("cgeb"),
                &'c'
            ), 'g'
        );
    }

    #[test]
    fn test_deduce_wire_f() {
        assert_eq!(deduce_wire_f(
                &ActiveWireSet::new("cbdgef"),
                &ActiveWireSet::new("fgaecd"),
                &ActiveWireSet::new("agebfd"),
                &'d',
                &'g',
                &'f',
            ), 'e'
        );
    }

    #[test]
    fn test_deduce_wire_c() {
        assert_eq!(deduce_wire_c(
                &ActiveWireSet::new("be"),
                &'e',
            ), 'b'
        );
    }

    #[test]
    fn test_deduce_wire_e() {
        assert_eq!(deduce_wire_e(
                &'d',
                &'c',
                &'f',
                &'g',
                &'e',
                &'b',
            ), 'a'
        );
    }

    #[test]
    fn test_deduce_all_wires() {
        let wire_sets = parse_input(&TEST_INPUT);
        let result = deduce_all_wires(&wire_sets[0].0);

        let mut expected = HashMap::new();
        expected.insert('a', 'e');
        expected.insert('b', 'c');
        expected.insert('c', 'd');
        expected.insert('d', 'a');
        expected.insert('e', 'f');
        expected.insert('f', 'g');
        expected.insert('g', 'b');

        assert_eq!(result, expected);
    }

    #[test]
    fn test_wire_set_to_segment_set() {
        let wire_sets = parse_input(&TEST_INPUT);
        let map = deduce_all_wires(&wire_sets[0].0);

        assert_eq!(wire_set_to_segment_set(&map, &wire_sets[0].1[0]), "abcdefg");
        assert_eq!(wire_set_to_segment_set(&map, &wire_sets[0].1[1]), "acdfg");
        assert_eq!(wire_set_to_segment_set(&map, &wire_sets[0].1[2]), "abcdfg");
        assert_eq!(wire_set_to_segment_set(&map, &wire_sets[0].1[3]), "bcdf");
    }

    #[test]
    fn test_wire_set_to_digit_one_liner() {
        let wire_sets = parse_input(&TEST_INPUT_ONE_LINE);
        let map = deduce_all_wires(&wire_sets[0].0);

        assert_eq!(wire_set_to_digit(&map, &wire_sets[0].1[0]), 5);
        assert_eq!(wire_set_to_digit(&map, &wire_sets[0].1[1]), 3);
        assert_eq!(wire_set_to_digit(&map, &wire_sets[0].1[2]), 5);
        assert_eq!(wire_set_to_digit(&map, &wire_sets[0].1[3]), 3);
    }

    #[test]
    fn test_wire_set_to_digit() {
        let wire_sets = parse_input(&TEST_INPUT);
        let map = deduce_all_wires(&wire_sets[0].0);

        assert_eq!(wire_set_to_digit(&map, &wire_sets[0].1[0]), 8);
        assert_eq!(wire_set_to_digit(&map, &wire_sets[0].1[1]), 3);
        assert_eq!(wire_set_to_digit(&map, &wire_sets[0].1[2]), 9);
        assert_eq!(wire_set_to_digit(&map, &wire_sets[0].1[3]), 4);
    }

    #[test]
    fn challenge_answer() {
        let wire_sets = parse_input(&TEST_INPUT);

        assert_eq!(sum_all_output_digits(&wire_sets), 61229);
    }

    #[test]
    fn basic_deductions() {
        assert_eq!(deduce_wire_a(&ActiveWireSet::new("cf"), &ActiveWireSet::new("acf")), 'a');
        assert_eq!(deduce_wire_d(
                &ActiveWireSet::new("bcdf"),
                &ActiveWireSet::new("acdeg"),
                &ActiveWireSet::new("acdfg"),
                &ActiveWireSet::new("abdfg"),
            ), 'd'
        );

        assert_eq!(deduce_wire_g(
                &ActiveWireSet::new("acdeg"),
                &ActiveWireSet::new("acdfg"),
                &ActiveWireSet::new("abdfg"),
                &'a',
                &'d',
            ), 'g'
        );

        assert_eq!(deduce_wire_b(&ActiveWireSet::new("cf"), &ActiveWireSet::new("bcdf"), &'d'),
            'b');

        assert_eq!(deduce_wire_f(
                &ActiveWireSet::new("abcefg"),
                &ActiveWireSet::new("abdefg"),
                &ActiveWireSet::new("abcdfg"),
                &'a',
                &'b',
                &'g',
            ), 'f'
        );

        assert_eq!(deduce_wire_c(&ActiveWireSet::new("cf"), &'f'), 'c');

        assert_eq!(deduce_wire_e(&'a', &'b', &'c', &'d', &'f', &'g'), 'e');

        let result = deduce_all_wires(&vec![
                ActiveWireSet::new("abcefg"),  // Digit 0,  6 segments
                ActiveWireSet::new("cf"),      // Digit 1,  2 segments
                ActiveWireSet::new("acdeg"),   // Digit 2,  5 segments
                ActiveWireSet::new("acdfg"),   // Digit 3,  5 segments
                ActiveWireSet::new("bcdf"),    // Digit 4,  4 segments
                ActiveWireSet::new("abdfg"),   // Digit 5,  5 segments
                ActiveWireSet::new("abdefg"),  // Digit 6,  6 segments
                ActiveWireSet::new("acf"),     // Digit 7,  3 segments
                ActiveWireSet::new("abcdefg"), // Digit 8,  7 segments
                ActiveWireSet::new("abcdfg"),  // Digit 9,  6 segments
            ]
        );

        assert_eq!(result.get(&'a').unwrap(), &'a');
        assert_eq!(result.get(&'b').unwrap(), &'b');
        assert_eq!(result.get(&'c').unwrap(), &'c');
        assert_eq!(result.get(&'d').unwrap(), &'d');
        assert_eq!(result.get(&'e').unwrap(), &'e');
        assert_eq!(result.get(&'f').unwrap(), &'f');
        assert_eq!(result.get(&'g').unwrap(), &'g');
    }
}
