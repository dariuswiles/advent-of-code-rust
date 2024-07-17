mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day03_part1").trim(),
        "gamma = 010111100100, epsilon = 101000011011
The submarine's power consumption is 3901196"
    );
}
