mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2024_day03_part2").trim(),
        "The sum of the result of each multiplication instruction is 95846796"
    );
}
