mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day06_part2").trim(),
        "Sum of question counts is 3512"
    );
}
