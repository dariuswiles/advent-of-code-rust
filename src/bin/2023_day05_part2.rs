//! Advent of Code 2023 Day 05
//! https://adventofcode.com/2023/day/5
//!
//! Challenge part 2
//!
//! The challenge input is a series of mappings between various garden-related things, such as seeds
//! and soil. Each mapping defines how source numbers are converted to destination numbers, and is
//! expressed as the start of the source and destination ranges, and the length of the range.
//!
//! Part 2 of the challenge treats the seed numbers as a range. The challenge answer then requires
//! converting these ranges through a series of mappings to find their final values, the smallest of
//! which is the challenge answer.

use std::collections::HashMap;
use std::fs;
use std::ops::Range;
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
    source_range: Range<u64>,
    // source_range_start: u64,
    // range_length: u64,
}

impl DataRange {
    fn from_str(s: &str) -> Self {
        let nums: Vec<_> = s.split(' ').collect();
        assert_eq!(
            3,
            nums.len(),
            "Could not find exactly 3 numbers in range: {s}"
        );

        let source_range_start = nums[1].parse().unwrap();
        let range_length: u64 = nums[2].parse().unwrap();

        Self {
            destination_range_start: nums[0].parse().unwrap(),
            source_range: source_range_start..source_range_start + range_length,
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
                (source_type, destination_type) = parse_map_type(line);
            }
            None => {
                return None;
            }
        }

        let mut ranges = Vec::new();

        for line in input_lines {
            if line.is_empty() {
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

    /// Converts the `Vec` of `Range`s provided in `input` to a corresponding `Vec` of destination
    /// `Range`s that is returned. Input values that lie within a `input_range` of this object
    /// are offset by the difference between the `input_range_start` and `destination_range_start`.
    /// Input values that don't lie within a range are returned unchanged.
    ///
    ///  up the value `v` to see if it falls within any ranges defined in this `Map`. If it
    /// does, its corresponding mapped value is returned. This is based on applying `v`'s offset
    /// from for the matching range. For example,
    /// if the input start is 10, the destination start is 20, and `v` is 12, the result will be
    /// 22. If `v` does not fall within a range, the return value is the same as `v`.
    fn convert(&self, input: Vec<Range<u64>>) -> Vec<Range<u64>> {
        let mut unconverted_ranges = input.clone();
        let mut converted_ranges = Vec::new();

        for sr in &self.ranges {
            let offset: i64 = i64::try_from(sr.destination_range_start)
                .expect("Conversion error when calculating range offset")
                - i64::try_from(sr.source_range.start)
                    .expect("Conversion error when calculating range offset");

            let mut outside_of_range = Vec::new();

            while let Some(to_convert) = unconverted_ranges.pop() {
                let (mut unconverted, just_converted) = remove_range(&to_convert, &sr.source_range);

                outside_of_range.append(&mut unconverted);

                if let Some(just_converted_unwrapped) = just_converted {
                    converted_ranges.push(
                        (just_converted_unwrapped.start as i64 + offset) as u64
                            ..(i64::try_from(just_converted_unwrapped.end).unwrap() + offset)
                                as u64,
                    );
                }
            }

            unconverted_ranges = outside_of_range;
        }

        // `unconverted_ranges` now contains the parts of the input that are outside all ranges
        // contained in this `Map` object. The challenge rules state that no conversion is required
        // and these values should be passed through unchanged.
        converted_ranges.append(&mut unconverted_ranges);

        converted_ranges
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    println!("The lowest location value is {}", do_challenge(&input));
}

/// Maps each of the seed ranges listed in the first line of input to the associated "Location"
/// ranges. This is calculated by mapping every seed range through each of the maps in turn, as
/// described by the challenge. The Location with the smallest id is returned as the challenge
/// answer.
fn do_challenge(input: &str) -> u64 {
    let (seeds, maps) = parse_input(input);
    let locations = do_full_mapping(&maps, &seeds);

    locations
        .iter()
        .filter(|locations| !locations.is_empty())
        .fold(u64::MAX, |lowest, locations| lowest.min(locations.start))
}

/// Converts `input` into a tuple consisting of: a `Vec` of seed `Range`s; and a `HashMap` that maps
/// each `DataType` to a `Map` that converts source values of this `DataType` to the destination
/// `Range`s of a different `DataType`.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> (Vec<Range<u64>>, HashMap<DataType, Map>) {
    let mut lines = input.lines();
    let seeds = parse_seeds(lines.next().unwrap());
    assert_eq!(
        Some(""),
        lines.next(),
        "The list of seeds must be followed by a blank line"
    );

    let mut maps: HashMap<DataType, _> = HashMap::new();
    while let Some(map) = Map::from_lines(&mut lines) {
        maps.insert(map.source_type, map);
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
fn parse_seeds(s: &str) -> Vec<Range<u64>> {
    let tokens: Vec<&str> = s
        .strip_prefix("seeds: ")
        .expect("Expected 'seeds' prefix not found in seed list: '{}'")
        .split(' ')
        .collect();

    let mut result = Vec::new();
    for pair in tokens.chunks(2) {
        let range_start = pair[0].parse().unwrap();
        let range_length: u64 = pair[1].parse().unwrap();

        result.push(range_start..(range_start + range_length));
    }

    result
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

/// Maps the given `Range`s of one `seed` through mappings in `maps`, from source to destination
/// `DataType`s until the "Location" DataType is reached. Returns the resulting "Location" ranges.
fn do_full_mapping(maps: &HashMap<DataType, Map>, seeds: &[Range<u64>]) -> Vec<Range<u64>> {
    let mut current_data_type = DataType::Seed;
    let mut current_value = seeds.to_vec();

    while let Some(map) = maps.get(&current_data_type) {
        current_value = map.convert(current_value);
        current_data_type = map.destination_type;
    }

    current_value
}

/// Removes `Range` `r2` from `Range` `r1` and returns a tuple containing: a `Vec` of the parts of
/// `r1` that are not in `r2` (if any); and a `Range` containing the intersection of `r1` and `r2`
/// or `None` if the two do not overlap.
fn remove_range(r1: &Range<u64>, r2: &Range<u64>) -> (Vec<Range<u64>>, Option<Range<u64>>) {
    // No overlap
    if r1.start > r2.end || r1.end < r2.start {
        return (vec![r1.clone()], None);
    }

    // `r1` is a superset of `r2`
    if r1.start <= r2.start && r1.end >= r2.end {
        return (vec![r1.start..r2.start, r2.end..r1.end], Some(r2.clone()));
    }

    // `r1` is a subset of `r2`
    if r1.start >= r2.start && r1.end <= r2.end {
        return (vec![], Some(r1.clone()));
    }

    // The lower end of `r1` overlaps `r2`
    if r1.start > r2.start {
        #[allow(clippy::single_range_in_vec_init)]
        return (vec![r2.end..r1.end], Some(r1.start..r2.end));
    }

    // The upper end of `r1` overlaps `r2`
    #[allow(clippy::single_range_in_vec_init)]
    (vec![r1.start..r2.start], Some(r2.start..r1.end))
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
        assert_eq!(vec![11..21, 22..72], parse_seeds("seeds: 11 10 22 50"));
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
                        source_range: 98..100,
                    },
                    DataRange {
                        destination_range_start: 52,
                        source_range: 50..98,
                    },
                ],
            },
            m
        );
    }

    #[test]
    fn test_convert() {
        let m = Map::from_lines(&mut TEST_INPUT_SEED_MAP.lines()).unwrap();

        assert_eq!(
            Map {
                source_type: DataType::Seed,
                destination_type: DataType::Soil,
                ranges: vec![
                    DataRange {
                        destination_range_start: 50,
                        source_range: 98..100,
                    },
                    DataRange {
                        destination_range_start: 52,
                        source_range: 50..98,
                    },
                ],
            },
            m
        );
    }

    #[test]
    fn test_parse_input() {
        let (seeds, maps) = parse_input(TEST_INPUT);

        assert_eq!(vec![79..93, 55..68], seeds);

        assert_eq!(
            Some(&Map {
                source_type: DataType::Seed,
                destination_type: DataType::Soil,
                ranges: vec![
                    DataRange {
                        destination_range_start: 50,
                        source_range: 98..100,
                    },
                    DataRange {
                        destination_range_start: 52,
                        source_range: 50..98,
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
                        source_range: 15..52,
                    },
                    DataRange {
                        destination_range_start: 37,
                        source_range: 52..54,
                    },
                    DataRange {
                        destination_range_start: 39,
                        source_range: 0..15,
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
                        source_range: 53..61,
                    },
                    DataRange {
                        destination_range_start: 0,
                        source_range: 11..53,
                    },
                    DataRange {
                        destination_range_start: 42,
                        source_range: 0..7,
                    },
                    DataRange {
                        destination_range_start: 57,
                        source_range: 7..11,
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
                        source_range: 18..25,
                    },
                    DataRange {
                        destination_range_start: 18,
                        source_range: 25..95,
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
                        source_range: 77..100,
                    },
                    DataRange {
                        destination_range_start: 81,
                        source_range: 45..64,
                    },
                    DataRange {
                        destination_range_start: 68,
                        source_range: 64..77,
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
                        source_range: 69..70,
                    },
                    DataRange {
                        destination_range_start: 1,
                        source_range: 0..69,
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
                        source_range: 56..93,
                    },
                    DataRange {
                        destination_range_start: 56,
                        source_range: 93..97,
                    },
                ],
            }),
            maps.get(&DataType::Humidity)
        );
    }

    #[test]
    fn test_remove_range() {
        assert_eq!((vec![3..7], None), remove_range(&(3..7), &(8..10)));
        assert_eq!(
            (vec![2..4, 6..7], Some(4..6)),
            remove_range(&(2..7), &(4..6))
        );
        assert_eq!((vec![], Some(4..6)), remove_range(&(4..6), &(2..7)));
        assert_eq!((vec![3..6], Some(6..8)), remove_range(&(3..8), &(6..10)));
        assert_eq!((vec![7..10], Some(5..7)), remove_range(&(5..10), &(3..7)));
    }

    #[test]
    fn test_map_convert() {
        let m = Map {
            source_type: DataType::Light,
            destination_type: DataType::Temperature,
            ranges: vec![DataRange {
                destination_range_start: 66,
                source_range: 20..30,
            }],
        };

        assert_eq!(vec![5..10], m.convert(vec![5..10]));
        assert_eq!(vec![35..50], m.convert(vec![35..50]));
        assert_eq!(vec![68..70], m.convert(vec![22..24]));

        let result0 = m.convert(vec![15..25]);
        assert!(result0.contains(&(15..20)));
        assert!(result0.contains(&(66..71)));

        let result1 = m.convert(vec![25..32]);
        assert!(result1.contains(&(71..76)));
        assert!(result1.contains(&(30..32)));

        let result2 = m.convert(vec![18..33]);
        assert!(result2.contains(&(66..76)));
        assert!(result2.contains(&(18..20)));
        assert!(result2.contains(&(30..33)));
    }

    #[test]
    fn test_do_full_mapping() {
        let (_, maps) = parse_input(TEST_INPUT);

        assert_eq!(vec![46..47], do_full_mapping(&maps, &vec![82..83]));
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(46, do_challenge(TEST_INPUT));
    }
}
