mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day14_part1").trim(),
        "The frequency of the most common letter in the output minus the least common is 3143"
    );
}
