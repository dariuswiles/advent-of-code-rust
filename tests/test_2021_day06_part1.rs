mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day06_part1").trim(),
        "The total number of fish after 80 days is 380758"
    );
}
