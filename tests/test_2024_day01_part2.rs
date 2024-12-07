mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2024_day01_part2").trim(),
        "The total of all similarity scores is 21790168"
    );
}
