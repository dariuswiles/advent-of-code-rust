mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2024_day07_part2").trim(),
        "The sum of the test values of all equations that can possible be true is 500335179214836"
    );
}
