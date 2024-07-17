mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2023_day07_part2").trim(),
        "The sum of each card's bid multiplied by its rank is 247885995"
    );
}
