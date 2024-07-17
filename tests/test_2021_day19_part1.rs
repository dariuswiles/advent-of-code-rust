mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day19_part1").trim(),
        "There are 335 unique beacons"
    );
}
