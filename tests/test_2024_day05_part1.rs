mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2024_day05_part1").trim(),
        "The sum of all valid page update sequences is 5268"
    );
}
