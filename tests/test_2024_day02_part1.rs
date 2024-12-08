mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2024_day02_part1").trim(),
        "The number of reports whose levels are safe is 686"
    );
}
