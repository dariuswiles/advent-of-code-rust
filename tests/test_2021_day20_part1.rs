mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day20_part1").trim(),
        "The enhanced image has 5663 light pixels"
    );
}
