mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2023_day03_part1").trim(),
        "The sum of all part numbers adjacent to a symbol is 509115"
    );
}
