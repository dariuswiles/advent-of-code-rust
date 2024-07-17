mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day04_part2").trim(),
        "The challenge answer is 24742"
    );
}
