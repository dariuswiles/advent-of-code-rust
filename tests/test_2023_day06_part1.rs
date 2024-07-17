mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2023_day06_part1").trim(),
        "The product of the number of ways each race can be run is 625968"
    );
}
