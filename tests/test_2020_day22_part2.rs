mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day22_part2").trim(),
        "Player 1 won and their score is 31854."
    );
}
