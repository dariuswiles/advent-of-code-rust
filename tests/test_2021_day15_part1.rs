mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day15_part1").trim(),
        "The total risk of the most efficient path is 720"
    );
}
