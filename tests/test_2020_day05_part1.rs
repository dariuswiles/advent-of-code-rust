mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day05_part1").trim(),
        "The highest seat ID present in the input data is 880"
    );
}
