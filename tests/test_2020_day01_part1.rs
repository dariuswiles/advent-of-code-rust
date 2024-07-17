mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day01_part1").trim(),
        "Integers 81 and 1939 sum to required total, and multiplying them gives 157059"
    );
}
