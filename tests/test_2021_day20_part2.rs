mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day20_part2").trim(),
        "The enhanced image has 19638 light pixels"
    );
}
