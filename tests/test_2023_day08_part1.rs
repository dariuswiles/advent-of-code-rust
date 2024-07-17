mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2023_day08_part1").trim(),
        "The number of steps to get from the start node to the end node is 21797"
    );
}
