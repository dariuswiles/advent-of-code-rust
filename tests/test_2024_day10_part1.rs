mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2024_day10_part1").trim(),
        "The sum of the scores of all trailheads is 746"
    );
}
