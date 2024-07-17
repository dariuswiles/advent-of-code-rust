mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day16_part2").trim(),
        "The sum of all versions is 2223947372407"
    );
}
