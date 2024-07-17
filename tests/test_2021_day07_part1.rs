mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day07_part1").trim(),
        "The total fuel cost is 336040"
    );
}
