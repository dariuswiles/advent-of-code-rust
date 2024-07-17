mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2023_day02_part1").trim(),
        "The sum of all possible games is 1734"
    );
}
