mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2023_day01_part2").trim(),
        "The sum of all 2-digit numbers is 53894"
    );
}
