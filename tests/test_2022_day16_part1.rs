mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2022_day16_part1").trim(),
        "The highest achievable flow is 1653"
    );
}
