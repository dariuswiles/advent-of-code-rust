mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day01_part2").trim(),
        "1346 integers are greater than their preceding integer"
    );
}
