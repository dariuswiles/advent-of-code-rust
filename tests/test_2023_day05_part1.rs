mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2023_day05_part1").trim(),
        "The points total of all scratch cards is 251346198"
    );
}
