mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day10_part2").trim(),
        "The total score for all corrupted lines in the input files is 4329504793"
    );
}
