mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day03_part1").trim(),
        "268 trees hit"
    );
}
