mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2023_day04_part2").trim(),
        "The total number of scratch cards is 9881048"
    );
}
