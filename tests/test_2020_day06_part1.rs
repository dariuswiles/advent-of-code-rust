mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day06_part1").trim(),
        "Sum of question counts is 6763"
    );
}
