mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2024_day08_part2").trim(),
        "There are 1285 antinodes in the grid"
    );
}
