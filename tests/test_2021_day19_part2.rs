mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day19_part2").trim(),
        "The maximum Manhattan distance between any two scanners is 10864"
    );
}
