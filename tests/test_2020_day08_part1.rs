mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day08_part1").trim(),
        "Contents of accumulator `acc` at the point the program repeats is 1915"
    );
}
