mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2022_day06_part1").trim(),
        "The challenge answer is 1640"
    );
}
