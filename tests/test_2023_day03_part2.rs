mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2023_day03_part2").trim(),
        "The sum of the power of all the gears is 75220503"
    );
}
