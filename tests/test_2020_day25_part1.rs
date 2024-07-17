mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day25_part1").trim(),
        "Shared encryption key is 17980581"
    );
}
