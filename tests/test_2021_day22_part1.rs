mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day22_part1").trim(),
        "582644 cells are in the 'on' state."
    );
}
