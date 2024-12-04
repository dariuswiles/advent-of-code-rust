mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2024_day01_part1").trim(),
        "The total distance between the two columns of numbers is 1151792"
    );
}
