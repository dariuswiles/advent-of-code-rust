mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day20_part1").trim(),
        "The product of the ids of the corner tiles is 108603771107737"
    );
}
