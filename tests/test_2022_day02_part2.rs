mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2022_day02_part2").trim(),
        "My total score for the game is 13509"
    );
}
