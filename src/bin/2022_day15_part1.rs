//! Advent of Code 2022 Day 15
//! https://adventofcode.com/2022/day/15
//!
//! Challenge part 1
//!
//! Determine the number of locations on a given row that definitely cannot contain a beacon. This
//! is based on a set of sensors that each provide the locations of themselves and their nearest
//! beacon.

use std::collections::HashSet;
use std::fs;

const INPUT_FILENAME: &str = "2022_day15_input.txt";
const INPUT_TOKEN_SENSOR: &str = "Sensor at x=";
const INPUT_TOKEN_COORDINATE_SEPARATOR: &str = ", y=";
const INPUT_TOKEN_BEACON: &str = ": closest beacon is at x=";
const CHALLENGE_ROW: AxisType = 2000000;

type AxisType = i32;

/// Holds a coordinate in 2D space as 'x' and 'y' values (which can be negative).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coordinate {
    x: AxisType,
    y: AxisType,
}

/// Holds a sensor's location, the location of its closest beacon and the Manhattan distance to
/// that beacon.
#[derive(Clone, Debug, PartialEq)]
struct Sensor {
    location: Coordinate,
    closest_beacon: Coordinate,
}

impl Sensor {
    /// Returns a `HashSet` of `x` coordinates which cannot contain the emergency beacon because
    /// they are closer to this `Sensor` than its `closest_beacon`.
    fn impossible_columns_for_beacon(&self, row: AxisType) -> HashSet<AxisType> {
        let mut impossible_x = HashSet::new();

        let manhattan_distance = self.location.x.abs_diff(self.closest_beacon.x)
            + self.location.y.abs_diff(self.closest_beacon.y);

        let sensor_distance_to_row = self.location.y.abs_diff(row) as AxisType;

        let remaining_distance = manhattan_distance as AxisType - sensor_distance_to_row;

        for x in self.location.x - remaining_distance..=self.location.x + remaining_distance {
            if x != self.closest_beacon.x || row != self.closest_beacon.y {
                impossible_x.insert(x);
            }
        }

        impossible_x
    }
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
            x: AxisType::from_str_radix(sensor_x, 10).unwrap(),
            y: AxisType::from_str_radix(sensor_y, 10).unwrap(),
        },
        closest_beacon: Coordinate {
            x: AxisType::from_str_radix(beacon_x, 10).unwrap(),
            y: AxisType::from_str_radix(beacon_y, 10).unwrap(),
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
        if line == "" {
            continue;
        }

        sensors.push(parse_line(&line));
    }

    sensors
}

/// Returns a `HashSet` of `x` coordinates which cannot contain the emergency beacon because
/// they are closer to this `Sensor` than its `closest_beacon`.
fn impossible_columns_for_beacons(sensors: Vec<Sensor>, row: AxisType) -> HashSet<AxisType> {
    let mut impossibilities = HashSet::new();

    for sensor in sensors {
        impossibilities.extend(&sensor.impossible_columns_for_beacon(row));
    }

    impossibilities
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let sensors = parse_lines(&input_file);
    let impossibilities = impossible_columns_for_beacons(sensors, CHALLENGE_ROW);

    println!(
        "A beacon cannot be present on {} cells on row {}",
        impossibilities.len(),
        CHALLENGE_ROW,
    );
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_impossible_columns_for_beacon() {
        let sensors = parse_lines(TEST_INPUT);

        // Requested row is outside the area known by this sensor.
        let impossibilities0 = sensors[0].impossible_columns_for_beacon(10);
        assert_eq!(impossibilities0, HashSet::new());

        // Requested row is inside the area known by this sensor. This test is from the challenge.
        let impossibilities6 = sensors[6].impossible_columns_for_beacon(10);
        for expected in 3..=14 {
            assert!(impossibilities6.contains(&expected));
        }

        // Requested row contains the sensor and its beacon.
        // "Sensor at x=14, y=3: closest beacon is at x=15, y=3"
        let impossibilities12 = sensors[12].impossible_columns_for_beacon(3);
        assert_eq!(impossibilities12, HashSet::from([13, 14]));
    }

    #[test]
    fn test_impossible_columns_for_beacons() {
        let sensors = parse_lines(TEST_INPUT);
        let impossibilities = impossible_columns_for_beacons(sensors, 10);

        assert_eq!(impossibilities.len(), 26);
        assert_eq!(
            impossibilities,
            HashSet::from([
                -2, -1, 0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                22, 23, 24
            ])
        );
    }
}
