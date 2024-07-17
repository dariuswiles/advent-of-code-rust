mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day08_part2").trim(),
        "Contents of accumulator `acc` at time corrected program terminates is 944"
    );
}
