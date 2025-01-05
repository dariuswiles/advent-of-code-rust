mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2024_day06_part1").trim(),
        "The guard visits 5551 distinct locations in the grid"
    );
}
