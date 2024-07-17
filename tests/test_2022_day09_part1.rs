mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2022_day09_part1").trim(),
        "The rope tail passed through 5930 unique positions"
    );
}
