mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day01_part1").trim(),
        "1301 integers are greater than their preceding integer"
    );
}
