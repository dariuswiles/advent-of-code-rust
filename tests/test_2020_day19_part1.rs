mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day19_part1").trim(),
        "165 messages are valid"
    );
}
