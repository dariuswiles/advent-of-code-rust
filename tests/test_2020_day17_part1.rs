mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day17_part1").trim(),
        "The answer to the challenge is 213"
    );
}