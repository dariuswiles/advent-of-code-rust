mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2024_day08_part1").trim(),
        "There are 367 antinodes in the grid"
    );
}
