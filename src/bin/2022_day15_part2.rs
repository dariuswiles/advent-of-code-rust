//! Advent of Code 2022 Day 15
//! https://adventofcode.com/2022/day/15
//!
//! Challenge part 2
//!
//! Determine the coordinates of an emergency beacon within a large 2D grid. The location is found
//! by determining all the coordinates that cannot contain the emergency beacon, which should
//! leave a single possibility.
//
// Although part 2 of the challenge is similar to part 1, the code is substantially different
// due to the significant increase in the size of the search space. Using the part 1 code to search
// the grid required for part 2 would take the runtime from 1 second to about 2 weeks.

use std::fs;

const INPUT_FILENAME: &str = "2022_day15_input.txt";
const INPUT_TOKEN_SENSOR: &str = "Sensor at x=";
const INPUT_TOKEN_COORDINATE_SEPARATOR: &str = ", y=";
const INPUT_TOKEN_BEACON: &str = ": closest beacon is at x=";
const SEARCH_GRID_END: AxisType = 4000000;

type AxisType = i32;

/// Holds a coordinate in 2D space as 'x' and 'y' values (which can be negative).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coordinate {
    x: AxisType,
    y: AxisType,
}

/// Holds a sensor's location and the location of its closest beacon.
#[derive(Clone, Debug, PartialEq)]
struct Sensor {
    location: Coordinate,
    closest_beacon: Coordinate,
}

/// Parses a line in the format specified in the challenge (see example below), and returns the
/// data it contains as a new `Sensor`. The input should be of the form:
///     Sensor at x=2, y=18: closest beacon is at x=-2, y=15
///
/// # Panics
///
/// Panics if the input is not in the expected form (or is an empty string).
fn parse_line(input: &str) -> Sensor {
    let sensor_x_onwards = input.strip_prefix(INPUT_TOKEN_SENSOR).unwrap();

    let (sensor_x, sensor_y_onwards) = sensor_x_onwards
        .split_once(INPUT_TOKEN_COORDINATE_SEPARATOR)
        .unwrap();

    let (sensor_y, beacon_x_onwards) = sensor_y_onwards.split_once(INPUT_TOKEN_BEACON).unwrap();

    let (beacon_x, beacon_y) = beacon_x_onwards
        .split_once(INPUT_TOKEN_COORDINATE_SEPARATOR)
        .unwrap();

    Sensor {
        location: Coordinate {
            x: sensor_x.parse().unwrap(),
            y: sensor_y.parse().unwrap(),
        },
        closest_beacon: Coordinate {
            x: beacon_x.parse().unwrap(),
            y: beacon_y.parse().unwrap(),
        },
    }
}

/// Parses all lines in `input` into a vector of `Sensor`s which is then returned. Empty lines are
/// skipped.
///
/// # Panics
///
/// Panics if the input is not in the expected form.
fn parse_lines(input: &str) -> Vec<Sensor> {
    let mut sensors = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        sensors.push(parse_line(line));
    }

    sensors
}

/// Returns the `Coordinate` of the emergency beacon given a vector of `Sensor`s (which each have
/// an associated beacon), and the size of the area to search (expressed as the maximum `x` and
/// `y` values to search, inclusive).
///
/// # Panics
///
/// Panics if exactly one emergency beacon is not found.
//
// The algorithm is based on looking at each row in turn and determining which coordinates cannot
// contain the emergency beacon because they are closer to a `Sensor` than that `Sensor`'s
// nearest beacon. As the rows are large the coordinates that cannot contain the emergency
// beacon are stored as RangeInclusive objects. The ranges for each row are sorted by their
// starting coordinate (i.e., their starting column), and all ranges iterated through to see if
// there are any coordinates not covered by at least one range.
fn find_emergency_beacon(sensors: &Vec<Sensor>, search_grid_end: AxisType) -> Coordinate {
    let mut possible_location = Vec::new();

    // Compute the Manhatten distance between each sensor and its closest beacon. This is used
    // within the subsequent loop but is computed outside it as an optimization.
    let mut sensor_to_beacon = Vec::new();
    for sensor in sensors {
        sensor_to_beacon.push(
            (sensor.location.x.abs_diff(sensor.closest_beacon.x)
                + sensor.location.y.abs_diff(sensor.closest_beacon.y)) as AxisType,
        );
    }

    for row in 0..=search_grid_end {
        let mut impossible_ranges = Vec::new();

        for (index, sensor) in sensors.iter().enumerate() {
            let distance_to_row = sensor.location.y.abs_diff(row);

            let extent = sensor_to_beacon[index] - distance_to_row as AxisType;
            if extent < 0 {
                continue;
            }

            impossible_ranges.push(sensor.location.x - extent..=sensor.location.x + extent);
        }

        impossible_ranges.sort_unstable_by(|a, b| a.start().partial_cmp(b.start()).unwrap());

        let mut hwm = 0; // hwm = high water mark
        for ir in impossible_ranges {
            if *ir.start() > hwm {
                for x in hwm + 1..*ir.start() {
                    possible_location.push(Coordinate { x, y: row });
                }
            }
            hwm = AxisType::max(*ir.end(), hwm);
        }
    }

    if possible_location.len() != 1 {
        panic!(
            "{} possible locations for the emergency beacon were found when 1 is expected.",
            possible_location.len()
        );
    }

    possible_location[0]
}

/// Returns the tuning frequency of the `Coordinate` passed, as per the formula in the challenge.
fn tuning_frequency(c: &Coordinate) -> i64 {
    (c.x as i64 * 4_000_000) + c.y as i64
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let sensors = parse_lines(&input_file);
    let emergency_beacon = find_emergency_beacon(&sensors, SEARCH_GRID_END);

    println!(
        "The tuning frequency of the emergency beacon is {}",
        tuning_frequency(&emergency_beacon)
    );
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const SEARCH_GRID_END_TESTING: AxisType = 20;

    const TEST_INPUT: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("Sensor at x=2, y=18: closest beacon is at x=-2, y=15"),
            Sensor {
                location: Coordinate { x: 2, y: 18 },
                closest_beacon: Coordinate { x: -2, y: 15 },
            }
        );
    }

    #[test]
    fn test_parse_lines() {
        let sensors = parse_lines(TEST_INPUT);

        assert_eq!(sensors.len(), 14);

        assert_eq!(
            sensors[0],
            Sensor {
                location: Coordinate { x: 2, y: 18 },
                closest_beacon: Coordinate { x: -2, y: 15 },
            }
        );
    }

    #[test]
    fn test_find_emergency_beacon() {
        let sensors = parse_lines(TEST_INPUT);

        assert_eq!(
            find_emergency_beacon(&sensors, SEARCH_GRID_END_TESTING),
            Coordinate { x: 14, y: 11 }
        );
    }

    #[test]
    fn test_tuning_frequency() {
        assert_eq!(tuning_frequency(&Coordinate { x: 14, y: 11 }), 56000011);
    }
}
