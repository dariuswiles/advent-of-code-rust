mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day02_part1").trim(),
        "607 strings are valid"
    );
}
