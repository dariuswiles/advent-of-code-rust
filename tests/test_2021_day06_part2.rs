mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day06_part2").trim(),
        "The total number of fish after 256 days is 1710623015163"
    );
}
