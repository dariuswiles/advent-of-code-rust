mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day07_part2").trim(),
        "Number of bags the given bag needs to contain is 14177"
    );
}
