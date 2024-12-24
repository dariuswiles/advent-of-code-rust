mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2024_day03_part1").trim(),
        "The sum of the result of each multiplication instruction is 167650499"
    );
}
