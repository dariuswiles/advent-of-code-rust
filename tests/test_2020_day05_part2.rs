mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day05_part2").trim(),
        "Seat ID 731 is vacant"
    );
}
