mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day23_part1").trim(),
        "Challenge answer is 45798623"
    );
}
