mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day08_part2").trim(),
        "The sum of all output digits is 1051087"
    );
}
