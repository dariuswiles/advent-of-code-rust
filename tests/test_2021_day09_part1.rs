mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day09_part1").trim(),
        "The total risk is 562"
    );
}
