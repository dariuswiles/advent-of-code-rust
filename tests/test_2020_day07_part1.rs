mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day07_part1").trim(),
        "Number of outer bag options is 265"
    );
}
