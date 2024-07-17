mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day21_part1").trim(),
        "The challenge answer is 1004670"
    );
}
