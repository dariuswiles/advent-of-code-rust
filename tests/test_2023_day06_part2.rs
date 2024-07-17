mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2023_day06_part2").trim(),
        "The race can be won in 43663323 different ways"
    );
}
