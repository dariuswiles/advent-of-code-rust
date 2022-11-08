//! Advent of Code 2021 Day 19
//! https://adventofcode.com/2021/day/19
//!
//! Challenge part 1
//!
//! Determine the number of unique beacons from an input file representing overlapping beacon
//! locations taken from scanners that are in unknown positions and orientations relative to each
//! other.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::Lines;

const INPUT_FILENAME: &str = "2021_day19_input.txt";
const SCANNER_INPUT_START_END: &str = "---";
const SCANNER_INPUT_KEYWORD: &str = "scanner";
const MATCH_THRESHOLD: usize = 12;

type PositionInt = i32;

/// Holds a location in 3D space as x, y and z coordinates. Coordinates can be negative.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: PositionInt,
    y: PositionInt,
    z: PositionInt,
}

impl Position {
    /// Returns a new `Position` created from an input string containing three comma-separated
    /// values.
    ///
    /// # Panics
    ///
    /// Panics if the input is malformed.
    fn new(s: &str) -> Self {
        let tokens: Vec<&str> = s.split(',').collect();

        if tokens.len() != 3 {
            panic!("Cannot create a Position from string '{}'", s);
        }

        Self {
            x: PositionInt::from_str_radix(tokens[0], 10).unwrap(),
            y: PositionInt::from_str_radix(tokens[1], 10).unwrap(),
            z: PositionInt::from_str_radix(tokens[2], 10).unwrap(),
        }
    }

    /// Returns the result of rotating the given position around its x-axis `rotations` times.
    fn rotate_around_x_axis(&self, rotations: u8) -> Self {
        let (mut y, mut z) = (self.y, self.z);

        for _ in 0..rotations {
            let y_save = y;
            y = z;
            z = -y_save;
        }

        Self { x: self.x, y, z }
    }

    /// Returns the result of rotating the given position around its y-axis `rotations` times.
    fn rotate_around_y_axis(&self, rotations: u8) -> Self {
        let (mut x, mut z) = (self.x, self.z);

        for _ in 0..rotations {
            let x_save = x;
            x = z;
            z = -x_save;
        }
        Self { x, y: self.y, z }
    }

    /// Returns the result of rotating the given position to a given `face` and then rotating it
    /// around its y-axis `rotations` times. `face` is in the range 0 to 5 inclusive and
    /// represents:
    ///     0. The original facing.
    ///     1. One rotation around the x-axis.
    ///     2. Two rotations around the x-axis.
    ///     3. Three rotations around the x-axis.
    ///     4. One rotation around the y-axis, then one rotation around the x-axis.
    ///     5. Three rotations around the y-axis, then one rotation around the x-axis.
    ///
    /// Calling this function 24 times on the same position with face values between 1 and 6
    /// (inclusive), and final rotations between 0 and 3 (inclusive) will yield every orientation
    /// that needs to be considered.
    fn orient(&self, face: u8, rotations: u8) -> Self {
        match face {
            0 => {
                return Self::rotate_around_y_axis(self, rotations);
            }
            1 => {
                let xr = Self::rotate_around_x_axis(self, 1);
                return Self::rotate_around_y_axis(&xr, rotations);
            }
            2 => {
                let xr = Self::rotate_around_x_axis(self, 2);
                return Self::rotate_around_y_axis(&xr, rotations);
            }
            3 => {
                let xr = Self::rotate_around_x_axis(self, 3);
                return Self::rotate_around_y_axis(&xr, rotations);
            }
            4 => {
                let yr = Self::rotate_around_y_axis(self, 1);
                let yxr = Self::rotate_around_x_axis(&yr, 1);
                return Self::rotate_around_y_axis(&yxr, rotations);
            }
            5 => {
                let yr = Self::rotate_around_y_axis(self, 3);
                let yxr = Self::rotate_around_x_axis(&yr, 1);
                return Self::rotate_around_y_axis(&yxr, rotations);
            }
            _ => panic!("reorient called with invalid face '{}'", face),
        }
    }

    /// Returns a new object representing the vector to move from `other` to `self`.
    fn minus(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    /// Returns a new object representing the addition of `self` and `other`.
    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

/// Holds data relating to a scanner. When a scanner is created this is relative only to the
/// scanner, but once the scanner's absolute position and orientation is determined relative to a
/// reference scanner, the absolute positions of the beacons can also be stored.
#[derive(Clone, Debug, PartialEq)]
struct Scanner {
    id: usize,
    rel_beacons: HashSet<Position>,
    abs_position: Option<Position>,
    abs_beacons: Option<HashSet<Position>>,
}

impl Scanner {
    /// Returns a new `Scanner` from `input`. If no input is found, returns None. Modifies `input`
    /// such that it points to the next unread line of input.
    ///
    /// # Panics
    ///
    /// Panics if the input is malformed.
    fn new(input: &mut Lines) -> Option<Self> {
        let id;

        if let Some(line) = input.next() {
            let tokens: Vec<&str> = line.split(' ').collect();
            if tokens.len() >= 4
                && tokens[0] == SCANNER_INPUT_START_END
                && tokens[1] == SCANNER_INPUT_KEYWORD
                && tokens[3] == SCANNER_INPUT_START_END
            {
                id = usize::from_str_radix(tokens[2], 10).unwrap();
            } else {
                panic!("Expecting scanner header in input, but found {}", line);
            }
        } else {
            return None;
        }

        let mut rel_beacons = HashSet::new();

        while let Some(line) = input.next() {
            if line == "" {
                if rel_beacons.len() > 0 {
                    break;
                } else {
                    panic!(
                        "Did not find any beacon coordinates in input for scanner {}",
                        id
                    );
                }
            }

            rel_beacons.insert(Position::new(line));
        }
        Some(Self {
            id,
            rel_beacons,
            abs_position: None,
            abs_beacons: None,
        })
    }

    /// Searches for an overlap between the beacons of this `Scanner` which must have known,
    /// absolute coordinates, and the beacons of the `other_scanner` passed. The latter's beacons'
    /// coordinates are relative to that scanner, so they are tried in all possible  orientations
    /// to look for at least MATCH_THRESHOLD beacons that both scanners can see.
    ///
    /// If such a match is found, returns the absolute position of `other_scanner` and its beacons
    /// as a tuple. Otherwise, returns None.
    ///
    /// #Panics
    ///
    /// Panics if this scanner does not have absolute coordinates for its beacons.
    /// Panics if `other_scanner` already has absolute coordinates for its beacons.
    //
    // The code generates the 24 possible sets of positions for `other_scanner`'s beacons. The
    // absolute position of every known beacon (from this scanner), is paired with every possible
    // relative beacon position in the sets to give candidate absolute positions for
    // `other_scanner`. If any candidate position is seen the threshold number of times during
    // this analysis, it's a match.
    fn find_overlap(&self, other_scanner: &Self) -> Option<(Position, HashSet<Position>)> {
        assert!(self.abs_beacons.is_some());
        assert!(other_scanner.abs_beacons.is_none());

        let other_beacon_sets = other_scanner.all_beacon_orientations();

        for obs in other_beacon_sets.iter() {
            // Possible absolute positions for `other_scanner`
            let mut candidate_pos_count: HashMap<Position, usize> = HashMap::new();

            for this_beacon in self.abs_beacons.as_ref().unwrap().iter() {
                for other_beacon in obs.iter() {
                    let count = candidate_pos_count
                        .entry(this_beacon.minus(other_beacon))
                        .or_insert(0);
                    *count += 1;
                }
            }

            let threshold_met: Vec<(&Position, &usize)> = candidate_pos_count
                .iter()
                .filter(|(_, &cnt)| cnt >= MATCH_THRESHOLD)
                .collect();

            match threshold_met.len() {
                1 => {
                    // The set of beacons in `obs` are the correct orientation because we know at
                    // least MATCH_THRESHOLD are in the same position as beacons in known,
                    // absolute positions. As we also now know the absolute position of
                    // `other_scanner`, translate the `obs` beacons to their absolute positions.
                    // This is done for all beacons, even those that don't match beacons from this
                    // scanner, as they may be needed for future overlap checking.

                    let other_scanner_position = threshold_met[0].0;
                    let mut absolute_beacon_positions = HashSet::new();

                    for b in obs {
                        absolute_beacon_positions.insert(b.add(&other_scanner_position));
                    }

                    return Some((*other_scanner_position, absolute_beacon_positions));
                }
                2 => {
                    panic!("find_overlap found multiple candidate positions for scanner");
                }
                _ => {}
            }
        }
        None
    }

    /// Returns a vector containing 24 sets of `Position`s of this object's beacons, where each set
    /// represents one possible orientation of this scanner. This function must only be called if
    /// this object does not already have an absolute set of positions for its beacons.
    ///
    /// # Panics
    ///
    /// Panics if this object already has an absolute set of positions for its beacons, i.e.,
    /// the `abs_beacons` field is not `None`.
    fn all_beacon_orientations(&self) -> Vec<HashSet<Position>> {
        assert!(self.abs_beacons.is_none());
        let mut beacon_sets = Vec::new();

        for face in 0..6 {
            for rotation in 0..4 {
                let mut bs = HashSet::new();

                for beacon in &self.rel_beacons {
                    bs.insert(beacon.orient(face, rotation));
                }

                beacon_sets.push(bs);
            }
        }
        beacon_sets
    }
}

/// Determines the absolute positions of all scanners and beacons, and updates `scanners` with
/// this information.
fn fix_all_scanner_positions(scanners: &mut Vec<Scanner>) {
    let scanners_len = scanners.len();
    let mut scanners_to_do: HashSet<_> = (0..scanners_len).collect();

    scanners[0].abs_beacons = Some(scanners[0].rel_beacons.clone());
    scanners[0].abs_position = Some(Position::new("0,0,0"));

    while scanners_to_do.len() > 0 {
        for known_idx in scanners_to_do.clone() {
            if scanners[known_idx].abs_beacons.is_none() {
                continue;
            }

            scanners_to_do.remove(&known_idx);

            for current_scanner_idx in 1..scanners_len {
                let current_scanner = &scanners[current_scanner_idx];

                if current_scanner.abs_beacons.is_some() {
                    continue;
                }
                // println!("Looking for an overlap between scanners {} and {}", known_idx,
                //     current_scanner_idx
                // );

                if let Some((overlap_scanner_position, overlap_scanner_beacons)) =
                    scanners[known_idx].find_overlap(current_scanner)
                {
                    // println!("    Match found. Scanner {} is at {:?}", current_scanner_idx,
                    //     overlap_scanner_position
                    // );
                    scanners[current_scanner_idx].abs_position = Some(overlap_scanner_position);
                    scanners[current_scanner_idx].abs_beacons = Some(overlap_scanner_beacons);
                }
            }
        }
    }
}

/// Returns a `HashSet` containing the absolute `Position`s of all beacons in `scanners`.
///
/// # Panics
///
/// Panics if any `scanner` does not have absolute positions for its beacons.
fn all_beacon_positions(scanners: &Vec<Scanner>) -> HashSet<Position> {
    scanners.iter().fold(HashSet::new(), |b, s| {
        b.union(&s.abs_beacons.as_ref().unwrap()).cloned().collect()
    })
}

/// Returns the `input` as a vector of `Scanner`s, each containing the set of beacons provided in
/// the input.
fn parse_input(input: &str) -> Vec<Scanner> {
    let mut input_lines = input.lines();
    let mut scanners = Vec::new();

    while let Some(scanner) = Scanner::new(&mut input_lines) {
        scanners.push(scanner);
    }

    scanners
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let mut scanners = parse_input(&input_file);

    fix_all_scanner_positions(&mut scanners);
    let result_beacon_set = all_beacon_positions(&scanners);

    println!("There are {} unique beacons", result_beacon_set.len());
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
";

    const TEST_SINGLE_SCANNER: &str = "\
--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7
";

    #[test]
    fn create_position() {
        assert_eq!(
            Position::new("11,-22,-33"),
            Position {
                x: 11,
                y: -22,
                z: -33
            }
        );
    }

    #[test]
    fn create_single_scanner() {
        let scanner = Scanner::new(&mut TEST_SINGLE_SCANNER.lines()).unwrap();

        assert_eq!(scanner.id, 0);
        assert_eq!(scanner.rel_beacons.len(), 6);
        assert!(scanner.rel_beacons.get(&Position::new("-1,-1,1")).is_some());
        assert!(scanner.rel_beacons.get(&Position::new("-2,-2,2")).is_some());
        assert!(scanner.rel_beacons.get(&Position::new("-3,-3,3")).is_some());
        assert!(scanner.rel_beacons.get(&Position::new("-2,-3,1")).is_some());
        assert!(scanner.rel_beacons.get(&Position::new("5,6,-4")).is_some());
        assert!(scanner.rel_beacons.get(&Position::new("8,0,7")).is_some());
        assert!(scanner.rel_beacons.get(&Position::new("1,1,1")).is_none());
    }

    #[test]
    fn create_multiple_scanners() {
        let scanners = parse_input(&TEST_INPUT);

        assert_eq!(scanners.len(), 5);
        assert_eq!(scanners[0].id, 0);
        assert_eq!(scanners[4].id, 4);
        assert_eq!(scanners[0].rel_beacons.len(), 25);
        assert_eq!(scanners[4].rel_beacons.len(), 26);
        assert!(scanners[0]
            .rel_beacons
            .get(&Position::new("-345,-311,381"))
            .is_some());
        assert!(scanners[1]
            .rel_beacons
            .get(&Position::new("-345,-311,381"))
            .is_none());
        assert!(scanners[1]
            .rel_beacons
            .get(&Position::new("686,422,578"))
            .is_some());
        assert!(scanners[1]
            .rel_beacons
            .get(&Position::new("553,889,-390"))
            .is_some());
        assert!(scanners[2]
            .rel_beacons
            .get(&Position::new("-675,-892,-343"))
            .is_some());
        assert!(scanners[2]
            .rel_beacons
            .get(&Position::new("697,-426,-610"))
            .is_some());
        assert!(scanners[3]
            .rel_beacons
            .get(&Position::new("-500,565,-823"))
            .is_some());
        assert!(scanners[3]
            .rel_beacons
            .get(&Position::new("595,780,-596"))
            .is_some());
        assert!(scanners[4]
            .rel_beacons
            .get(&Position::new("30,-46,-14"))
            .is_some());
    }

    #[test]
    fn test_rotate_around_x_axis() {
        let original = Position::new("5,6,-4");
        assert_eq!(original.rotate_around_x_axis(0), original);
        assert_eq!(original.rotate_around_x_axis(1), Position::new("5,-4,-6"));
        assert_eq!(original.rotate_around_x_axis(2), Position::new("5,-6,4"));
        assert_eq!(original.rotate_around_x_axis(3), Position::new("5,4,6"));
    }

    #[test]
    fn test_orient() {
        let original = Position::new("8,0,7");
        let mut results = HashSet::new();

        for face in 0..6 {
            for rotation in 0..4 {
                assert!(results.insert(Position::orient(&original, face, rotation)));
            }
        }

        assert!(results.get(&Position::new("8,0,7")).is_some());
        assert!(results.get(&Position::new("-8,-7,0")).is_some());
        assert!(results.get(&Position::new("-7,0,8")).is_some());
        assert!(results.get(&Position::new("7,0,8")).is_some());
        assert!(results.get(&Position::new("0,7,-8")).is_some());
    }

    #[test]
    fn test_minus() {
        assert_eq!(
            Position::new("8,0,7").minus(&Position::new("8,-4,9")),
            Position::new("0,4,-2")
        );
    }

    #[test]
    fn test_all_scanner0_orientations() {
        let scanners = parse_input(&TEST_INPUT);
        let results: HashSet<Position> = scanners[0]
            .all_beacon_orientations()
            .iter()
            .cloned()
            .flatten()
            .collect();

        assert!(results.get(&Position::new("-618,-824,-621")).is_some());
        assert!(results.get(&Position::new("-537,-823,-458")).is_some());
        assert!(results.get(&Position::new("-447,-329,318")).is_some());
        assert!(results.get(&Position::new("404,-588,-901")).is_some());
        assert!(results.get(&Position::new("544,-627,-890")).is_some());
        assert!(results.get(&Position::new("528,-643,409")).is_some());
        assert!(results.get(&Position::new("-661,-816,-575")).is_some());
        assert!(results.get(&Position::new("390,-675,-793")).is_some());
        assert!(results.get(&Position::new("423,-701,434")).is_some());
        assert!(results.get(&Position::new("-345,-311,381")).is_some());
        assert!(results.get(&Position::new("459,-707,401")).is_some());
        assert!(results.get(&Position::new("-485,-357,347")).is_some());
    }

    #[test]
    fn test_all_scanner1_orientations() {
        let scanners = parse_input(&TEST_INPUT);
        let results: HashSet<Position> = scanners[1]
            .all_beacon_orientations()
            .iter()
            .cloned()
            .flatten()
            .collect();

        assert!(results.get(&Position::new("686,422,578")).is_some());
        assert!(results.get(&Position::new("605,423,415")).is_some());
        assert!(results.get(&Position::new("515,917,-361")).is_some());
        assert!(results.get(&Position::new("-336,658,858")).is_some());
        assert!(results.get(&Position::new("-476,619,847")).is_some());
        assert!(results.get(&Position::new("-460,603,-452")).is_some());
        assert!(results.get(&Position::new("729,430,532")).is_some());
        assert!(results.get(&Position::new("-322,571,750")).is_some());
        assert!(results.get(&Position::new("-355,545,-477")).is_some());
        assert!(results.get(&Position::new("413,935,-424")).is_some());
        assert!(results.get(&Position::new("-391,539,-444")).is_some());
        assert!(results.get(&Position::new("553,889,-390")).is_some());
    }

    #[test]
    fn test_find_overlap_0_1() {
        let mut scanners = parse_input(&TEST_INPUT);
        scanners[0].abs_beacons = Some(scanners[0].rel_beacons.clone());

        let overlap_result = scanners[0].find_overlap(&scanners[1]).unwrap();
        let (overlap_position, results) = overlap_result;

        assert_eq!(overlap_position, Position::new("68,-1246,-43"));
        assert!(results.get(&Position::new("-618,-824,-621")).is_some());
        assert!(results.get(&Position::new("-537,-823,-458")).is_some());
        assert!(results.get(&Position::new("-447,-329,318")).is_some());
        assert!(results.get(&Position::new("404,-588,-901")).is_some());
        assert!(results.get(&Position::new("544,-627,-890")).is_some());
        assert!(results.get(&Position::new("528,-643,409")).is_some());
        assert!(results.get(&Position::new("-661,-816,-575")).is_some());
        assert!(results.get(&Position::new("390,-675,-793")).is_some());
        assert!(results.get(&Position::new("423,-701,434")).is_some());
        assert!(results.get(&Position::new("-345,-311,381")).is_some());
        assert!(results.get(&Position::new("459,-707,401")).is_some());
        assert!(results.get(&Position::new("-485,-357,347")).is_some());
    }

    #[test]
    fn test_find_overlap_0_1_4() {
        let mut scanners = parse_input(&TEST_INPUT);
        scanners[0].abs_beacons = Some(scanners[0].rel_beacons.clone());

        let result_0_1 = scanners[0].find_overlap(&scanners[1]).unwrap();

        scanners[1].abs_position = Some(result_0_1.0);
        scanners[1].abs_beacons = Some(result_0_1.1);

        let overlap_result_1_4 = scanners[1].find_overlap(&scanners[4]).unwrap();
        let (overlap_position_4, result_1_4) = overlap_result_1_4;

        assert_eq!(overlap_position_4, Position::new("-20,-1133,1061"));
        assert!(result_1_4.get(&Position::new("459,-707,401")).is_some());
        assert!(result_1_4.get(&Position::new("-739,-1745,668")).is_some());
        assert!(result_1_4.get(&Position::new("-485,-357,347")).is_some());
        assert!(result_1_4.get(&Position::new("432,-2009,850")).is_some());
        assert!(result_1_4.get(&Position::new("528,-643,409")).is_some());
        assert!(result_1_4.get(&Position::new("423,-701,434")).is_some());
        assert!(result_1_4.get(&Position::new("-345,-311,381")).is_some());
        assert!(result_1_4.get(&Position::new("408,-1815,803")).is_some());
        assert!(result_1_4.get(&Position::new("534,-1912,768")).is_some());
        assert!(result_1_4.get(&Position::new("-687,-1600,576")).is_some());
        assert!(result_1_4.get(&Position::new("-447,-329,318")).is_some());
        assert!(result_1_4.get(&Position::new("-635,-1737,486")).is_some());
    }

    #[test]
    fn test_fix_all() {
        let mut scanners = parse_input(&TEST_INPUT);

        fix_all_scanner_positions(&mut scanners);

        assert_eq!(scanners[0].abs_position, Some(Position::new("0,0,0")));
        assert_eq!(
            scanners[1].abs_position,
            Some(Position::new("68,-1246,-43"))
        );
        assert_eq!(
            scanners[2].abs_position,
            Some(Position::new("1105,-1205,1229"))
        );
        assert_eq!(
            scanners[3].abs_position,
            Some(Position::new("-92,-2380,-20"))
        );
        assert_eq!(
            scanners[4].abs_position,
            Some(Position::new("-20,-1133,1061"))
        );
    }

    const EXPECTED_ABSOLUTE_BEACON_POSITIONS: [Position; 79] = [
        Position {
            x: -892,
            y: 524,
            z: 684,
        },
        Position {
            x: -876,
            y: 649,
            z: 763,
        },
        Position {
            x: -838,
            y: 591,
            z: 734,
        },
        Position {
            x: -789,
            y: 900,
            z: -551,
        },
        Position {
            x: -739,
            y: -1745,
            z: 668,
        },
        Position {
            x: -706,
            y: -3180,
            z: -659,
        },
        Position {
            x: -697,
            y: -3072,
            z: -689,
        },
        Position {
            x: -689,
            y: 845,
            z: -530,
        },
        Position {
            x: -687,
            y: -1600,
            z: 576,
        },
        Position {
            x: -661,
            y: -816,
            z: -575,
        },
        Position {
            x: -654,
            y: -3158,
            z: -753,
        },
        Position {
            x: -635,
            y: -1737,
            z: 486,
        },
        Position {
            x: -631,
            y: -672,
            z: 1502,
        },
        Position {
            x: -624,
            y: -1620,
            z: 1868,
        },
        Position {
            x: -620,
            y: -3212,
            z: 371,
        },
        Position {
            x: -618,
            y: -824,
            z: -621,
        },
        Position {
            x: -612,
            y: -1695,
            z: 1788,
        },
        Position {
            x: -601,
            y: -1648,
            z: -643,
        },
        Position {
            x: -584,
            y: 868,
            z: -557,
        },
        Position {
            x: -537,
            y: -823,
            z: -458,
        },
        Position {
            x: -532,
            y: -1715,
            z: 1894,
        },
        Position {
            x: -518,
            y: -1681,
            z: -600,
        },
        Position {
            x: -499,
            y: -1607,
            z: -770,
        },
        Position {
            x: -485,
            y: -357,
            z: 347,
        },
        Position {
            x: -470,
            y: -3283,
            z: 303,
        },
        Position {
            x: -456,
            y: -621,
            z: 1527,
        },
        Position {
            x: -447,
            y: -329,
            z: 318,
        },
        Position {
            x: -430,
            y: -3130,
            z: 366,
        },
        Position {
            x: -413,
            y: -627,
            z: 1469,
        },
        Position {
            x: -345,
            y: -311,
            z: 381,
        },
        Position {
            x: -36,
            y: -1284,
            z: 1171,
        },
        Position {
            x: -27,
            y: -1108,
            z: -65,
        },
        Position {
            x: 7,
            y: -33,
            z: -71,
        },
        Position {
            x: 12,
            y: -2351,
            z: -103,
        },
        Position {
            x: 26,
            y: -1119,
            z: 1091,
        },
        Position {
            x: 346,
            y: -2985,
            z: 342,
        },
        Position {
            x: 366,
            y: -3059,
            z: 397,
        },
        Position {
            x: 377,
            y: -2827,
            z: 367,
        },
        Position {
            x: 390,
            y: -675,
            z: -793,
        },
        Position {
            x: 396,
            y: -1931,
            z: -563,
        },
        Position {
            x: 404,
            y: -588,
            z: -901,
        },
        Position {
            x: 408,
            y: -1815,
            z: 803,
        },
        Position {
            x: 423,
            y: -701,
            z: 434,
        },
        Position {
            x: 432,
            y: -2009,
            z: 850,
        },
        Position {
            x: 443,
            y: 580,
            z: 662,
        },
        Position {
            x: 455,
            y: 729,
            z: 728,
        },
        Position {
            x: 456,
            y: -540,
            z: 1869,
        },
        Position {
            x: 459,
            y: -707,
            z: 401,
        },
        Position {
            x: 465,
            y: -695,
            z: 1988,
        },
        Position {
            x: 474,
            y: 580,
            z: 667,
        },
        Position {
            x: 496,
            y: -1584,
            z: 1900,
        },
        Position {
            x: 497,
            y: -1838,
            z: -617,
        },
        Position {
            x: 527,
            y: -524,
            z: 1933,
        },
        Position {
            x: 528,
            y: -643,
            z: 409,
        },
        Position {
            x: 534,
            y: -1912,
            z: 768,
        },
        Position {
            x: 544,
            y: -627,
            z: -890,
        },
        Position {
            x: 553,
            y: 345,
            z: -567,
        },
        Position {
            x: 564,
            y: 392,
            z: -477,
        },
        Position {
            x: 568,
            y: -2007,
            z: -577,
        },
        Position {
            x: 605,
            y: -1665,
            z: 1952,
        },
        Position {
            x: 612,
            y: -1593,
            z: 1893,
        },
        Position {
            x: 630,
            y: 319,
            z: -379,
        },
        Position {
            x: 686,
            y: -3108,
            z: -505,
        },
        Position {
            x: 776,
            y: -3184,
            z: -501,
        },
        Position {
            x: 846,
            y: -3110,
            z: -434,
        },
        Position {
            x: 1135,
            y: -1161,
            z: 1235,
        },
        Position {
            x: 1243,
            y: -1093,
            z: 1063,
        },
        Position {
            x: 1660,
            y: -552,
            z: 429,
        },
        Position {
            x: 1693,
            y: -557,
            z: 386,
        },
        Position {
            x: 1735,
            y: -437,
            z: 1738,
        },
        Position {
            x: 1749,
            y: -1800,
            z: 1813,
        },
        Position {
            x: 1772,
            y: -405,
            z: 1572,
        },
        Position {
            x: 1776,
            y: -675,
            z: 371,
        },
        Position {
            x: 1779,
            y: -442,
            z: 1789,
        },
        Position {
            x: 1780,
            y: -1548,
            z: 337,
        },
        Position {
            x: 1786,
            y: -1538,
            z: 337,
        },
        Position {
            x: 1847,
            y: -1591,
            z: 415,
        },
        Position {
            x: 1889,
            y: -1729,
            z: 1762,
        },
        Position {
            x: 1994,
            y: -1805,
            z: 1792,
        },
    ];

    #[test]
    fn test_all_beacon_positions() {
        let expected_beacons: HashSet<Position> = EXPECTED_ABSOLUTE_BEACON_POSITIONS
            .to_vec()
            .iter()
            .cloned()
            .collect();

        let mut scanners = parse_input(&TEST_INPUT);

        fix_all_scanner_positions(&mut scanners);
        let result_beacon_set = all_beacon_positions(&scanners);

        assert_eq!(result_beacon_set.len(), 79);
        assert_eq!(result_beacon_set, expected_beacons);
    }
}
