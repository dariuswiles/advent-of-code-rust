mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day09_part1").trim(),
        "The invalid number in the input is 1309761972"
    );
}
