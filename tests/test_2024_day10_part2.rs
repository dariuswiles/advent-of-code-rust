mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2024_day10_part2").trim(),
        "The sum of the ratings of all trailheads is 1541"
    );
}
