mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day09_part2").trim(),
        "The answer to the challenge is 1076922"
    );
}
