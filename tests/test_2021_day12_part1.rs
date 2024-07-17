mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day12_part1").trim(),
        "There are 5178 paths through the cave system"
    );
}
