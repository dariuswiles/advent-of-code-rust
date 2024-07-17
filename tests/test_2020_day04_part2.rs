mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day04_part2").trim(),
        "179 passports are valid"
    );
}
