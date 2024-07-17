mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day11_part1").trim(),
        "The total number of flashes 1719"
    );
}
