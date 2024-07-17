mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day12_part2").trim(),
        "There are 130094 paths through the cave system"
    );
}
