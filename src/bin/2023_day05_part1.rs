//! Advent of Code 2023 Day 05
//! https://adventofcode.com/2023/day/5
//!
//! Challenge part 1
//!
//! The challenge input is a series of mappings between various garden-related things, such as seeds
//! and soil. Each mapping defines how source numbers are converted to destination numbers, and is
//! expressed as the start of the source and destination ranges, and the length of the range.
//!
//! The challenge requires taking the numbers of a small number of seeds and converting them
//! through a series of mappings to find their final values, the smallest of which is the
//! challenge answer.

use std::collections::HashMap;
use std::fs;
use std::str::Lines;

const INPUT_FILENAME: &str = "2023_day05_input.txt";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum DataType {
    Fertilizer,
    Humidity,
    Light,
    Location,
    Seed,
    Soil,
    Temperature,
    Water,
}

impl DataType {
    /// Returns the enumerated value corresponding to the string passed.
    ///
    /// # Panics
    ///
    /// Panics if the string does not represent a `DataType`.
    fn from_str(s: &str) -> Self {
        match s.trim() {
            "fertilizer" => Self::Fertilizer,
            "humidity" => Self::Humidity,
            "light" => Self::Light,
            "location" => Self::Location,
            "seed" => Self::Seed,
            "soil" => Self::Soil,
            "temperature" => Self::Temperature,
            "water" => Self::Water,
            _ => {
                panic!("Unrecognized DataType");
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct DataRange {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl DataRange {
    fn from_str(s: &str) -> Self {
        let nums: Vec<_> = s.split(' ').collect();
        assert_eq!(
            3,
            nums.len(),
            "Could not find exactly 3 numbers in range: {s}"
        );

        Self {
            destination_range_start: u64::from_str_radix(nums[0], 10).unwrap(),
            source_range_start: u64::from_str_radix(nums[1], 10).unwrap(),
            range_length: u64::from_str_radix(nums[2], 10).unwrap(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Map {
    source_type: DataType,
    destination_type: DataType,
    ranges: Vec<DataRange>,
}

impl Map {
    /// Returns a `Map` created from the string passed. The first line declares the `DataType`s.
    /// Each subsequent line consists of three numbers separated by a single space which declaring:
    /// the start of the destination range, the start of the source range, and the length of the
    /// range. This continues until a blank line or the end of the `Lines` object passed.
    ///
    /// Example input:
    /// ```text
    /// seed-to-soil map:
    /// 50 98 2
    /// 52 50 48
    ///
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the string is malformed.
    fn from_lines(input_lines: &mut Lines) -> Option<Self> {
        let source_type;
        let destination_type;

        match input_lines.next() {
            Some(line) => {
                (source_type, destination_type) = parse_map_type(&line);
            }
            None => {
                return None;
            }
        }

        let mut ranges = Vec::new();

        for line in input_lines {
            if line == "" {
                break;
            }

            ranges.push(DataRange::from_str(line));
        }

        Some(Self {
            source_type,
            destination_type,
            ranges,
        })
    }

    /// Looks up the value `v` to see if it falls within any ranges defined in this `Map`. If it
    /// does, its corresponding mapped value is returned. This is based on applying `v`'s offset
    /// from `source_range_start` to `destination_range_start` for the matching range. For example,
    /// if the source start is 10, the destination start is 20, and `v` is 12, the result will be
    /// 22. If `v` does not fall within a range, the return value is the same as `v`.
    fn lookup(&self, v: u64) -> u64 {
        for r in &self.ranges {
            if v >= r.source_range_start && v < r.source_range_start + r.range_length {
                return v - r.source_range_start + r.destination_range_start;
            }
        }

        v
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    println!(
        "The points total of all scratch cards is {}",
        do_challenge(&input)
    );
}

/// Maps each of the seeds listed in the first line of input to its associated "Location" value.
/// This is found by mapping a seed value through each of the maps, as described by the challenge.
/// The Location with the smallest id is returned as the challenge answer.
fn do_challenge(input: &str) -> u64 {
    let (seeds, maps) = parse_input(input);
    let mut least_location_value = u64::MAX;

    for s in seeds {
        least_location_value = least_location_value.min(do_full_mapping(&maps, s));
    }

    least_location_value
}

/// Converts `input` into a tuple consisting of: a `Vec` of seed values; and a `HashMap` that maps
/// each `DataType` to a `Map` that converts source values of this `DataType` to a different
/// `DataType`.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> (Vec<u64>, HashMap<DataType, Map>) {
    let mut lines = input.lines();
    let seeds = parse_seeds(&lines.next().unwrap());
    assert_eq!(
        Some(""),
        lines.next(),
        "The list of seeds must be followed by a blank line"
    );

    let mut maps = HashMap::new();
    loop {
        match Map::from_lines(&mut lines) {
            Some(map) => {
                maps.insert(map.source_type, map);
            }
            None => {
                break;
            }
        }
    }

    (seeds, maps)
}

/// Parses a string containing the "seeds" line of the challenge input, and returns a `Vec`
/// containing the numeric equivalents of the seed numbers provided in the given string. Input is
/// of the form:
/// `seeds: 79 14 55 13`
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_seeds(s: &str) -> Vec<u64> {
    s.strip_prefix("seeds: ")
        .expect("Expected 'seeds' prefix not found in seed list: '{}'")
        .split(' ')
        .map(|n| u64::from_str_radix(n, 10).unwrap())
        .collect()
}

/// Converts a string specifying the type of map into enums containing the source and destination
/// types (in this order). Input should be of the format:
/// ```text
/// seed-to-soil map:
/// ```
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_map_type(s: &str) -> (DataType, DataType) {
    let tokens: Vec<_> = s
        .strip_suffix(" map:")
        .expect("Expected 'map' suffix not found in map type definition: '{}'")
        .split("-to-")
        .collect();

    (DataType::from_str(tokens[0]), DataType::from_str(tokens[1]))
}

/// Maps the given `seed` through mappings in `maps`, from source to destination `DataType`s until
/// the "Location" DataType is reached, and returns the "Location" value.
fn do_full_mapping(maps: &HashMap<DataType, Map>, seed: u64) -> u64 {
    let mut current_data_type = DataType::Seed;
    let mut current_value = seed;

    while let Some(map) = maps.get(&current_data_type) {
        current_value = map.lookup(current_value);
        current_data_type = map.destination_type;
    }

    current_value
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    const TEST_INPUT_SEED_MAP: &str = "\
seed-to-soil map:
50 98 2
52 50 48

";

    #[test]
    fn test_parse_seeds() {
        assert_eq!(vec![11, 22, 33], parse_seeds("seeds: 11 22 33"));
    }

    #[test]
    fn test_datatype_from_str() {
        assert_eq!(DataType::Seed, DataType::from_str("seed"));
        assert_eq!(DataType::Fertilizer, DataType::from_str("fertilizer"));
    }

    #[test]
    #[should_panic]
    fn test_datatype_from_str_invalid() {
        DataType::from_str("invalid");
    }

    #[test]
    fn test_parse_map_type() {
        assert_eq!(
            (DataType::Humidity, DataType::Location),
            parse_map_type("humidity-to-location map:")
        );
    }

    #[test]
    fn test_map_from_str() {
        let m = Map::from_lines(&mut TEST_INPUT_SEED_MAP.lines()).unwrap();

        assert_eq!(
            Map {
                source_type: DataType::Seed,
                destination_type: DataType::Soil,
                ranges: vec![
                    DataRange {
                        destination_range_start: 50,
                        source_range_start: 98,
                        range_length: 2,
                    },
                    DataRange {
                        destination_range_start: 52,
                        source_range_start: 50,
                        range_length: 48,
                    },
                ],
            },
            m
        );
    }

    #[test]
    fn test_lookup() {
        let m = Map::from_lines(&mut TEST_INPUT_SEED_MAP.lines()).unwrap();

        assert_eq!(50, m.lookup(98));
        assert_eq!(51, m.lookup(99));
        assert_eq!(55, m.lookup(53));
        assert_eq!(10, m.lookup(10));
    }

    #[test]
    fn test_parse_input() {
        let (seeds, maps) = parse_input(&TEST_INPUT);

        assert_eq!(vec![79, 14, 55, 13], seeds);

        assert_eq!(
            Some(&Map {
                source_type: DataType::Seed,
                destination_type: DataType::Soil,
                ranges: vec![
                    DataRange {
                        destination_range_start: 50,
                        source_range_start: 98,
                        range_length: 2,
                    },
                    DataRange {
                        destination_range_start: 52,
                        source_range_start: 50,
                        range_length: 48,
                    },
                ],
            }),
            maps.get(&DataType::Seed)
        );

        assert_eq!(
            Some(&Map {
                source_type: DataType::Soil,
                destination_type: DataType::Fertilizer,
                ranges: vec![
                    DataRange {
                        destination_range_start: 0,
                        source_range_start: 15,
                        range_length: 37,
                    },
                    DataRange {
                        destination_range_start: 37,
                        source_range_start: 52,
                        range_length: 2,
                    },
                    DataRange {
                        destination_range_start: 39,
                        source_range_start: 0,
                        range_length: 15,
                    },
                ],
            }),
            maps.get(&DataType::Soil)
        );

        assert_eq!(
            Some(&Map {
                source_type: DataType::Fertilizer,
                destination_type: DataType::Water,
                ranges: vec![
                    DataRange {
                        destination_range_start: 49,
                        source_range_start: 53,
                        range_length: 8,
                    },
                    DataRange {
                        destination_range_start: 0,
                        source_range_start: 11,
                        range_length: 42,
                    },
                    DataRange {
                        destination_range_start: 42,
                        source_range_start: 0,
                        range_length: 7,
                    },
                    DataRange {
                        destination_range_start: 57,
                        source_range_start: 7,
                        range_length: 4,
                    },
                ],
            }),
            maps.get(&DataType::Fertilizer)
        );

        assert_eq!(
            Some(&Map {
                source_type: DataType::Water,
                destination_type: DataType::Light,
                ranges: vec![
                    DataRange {
                        destination_range_start: 88,
                        source_range_start: 18,
                        range_length: 7,
                    },
                    DataRange {
                        destination_range_start: 18,
                        source_range_start: 25,
                        range_length: 70,
                    },
                ],
            }),
            maps.get(&DataType::Water)
        );

        assert_eq!(
            Some(&Map {
                source_type: DataType::Light,
                destination_type: DataType::Temperature,
                ranges: vec![
                    DataRange {
                        destination_range_start: 45,
                        source_range_start: 77,
                        range_length: 23,
                    },
                    DataRange {
                        destination_range_start: 81,
                        source_range_start: 45,
                        range_length: 19,
                    },
                    DataRange {
                        destination_range_start: 68,
                        source_range_start: 64,
                        range_length: 13,
                    },
                ],
            }),
            maps.get(&DataType::Light)
        );

        assert_eq!(
            Some(&Map {
                source_type: DataType::Temperature,
                destination_type: DataType::Humidity,
                ranges: vec![
                    DataRange {
                        destination_range_start: 0,
                        source_range_start: 69,
                        range_length: 1,
                    },
                    DataRange {
                        destination_range_start: 1,
                        source_range_start: 0,
                        range_length: 69,
                    },
                ],
            }),
            maps.get(&DataType::Temperature)
        );

        assert_eq!(
            Some(&Map {
                source_type: DataType::Humidity,
                destination_type: DataType::Location,
                ranges: vec![
                    DataRange {
                        destination_range_start: 60,
                        source_range_start: 56,
                        range_length: 37,
                    },
                    DataRange {
                        destination_range_start: 56,
                        source_range_start: 93,
                        range_length: 4,
                    },
                ],
            }),
            maps.get(&DataType::Humidity)
        );
    }

    #[test]
    fn test_do_full_mapping() {
        let (_, maps) = parse_input(&TEST_INPUT);

        // assert_eq!(vec![79, 14, 55, 13], seeds);

        assert_eq!(82, do_full_mapping(&maps, 79));
        assert_eq!(43, do_full_mapping(&maps, 14));
        assert_eq!(86, do_full_mapping(&maps, 55));
        assert_eq!(35, do_full_mapping(&maps, 13));
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(35, do_challenge(&TEST_INPUT));
    }
}
