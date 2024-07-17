mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day19_part2").trim(),
        "274 messages are valid"
    );
}
