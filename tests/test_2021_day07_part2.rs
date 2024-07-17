mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day07_part2").trim(),
        "The total fuel cost is 94813675"
    );
}
