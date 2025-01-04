mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2024_day05_part2").trim(),
        "The sum of all corrected invalid page update sequences is 5799"
    );
}
