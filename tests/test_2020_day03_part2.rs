mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day03_part2").trim(),
        "Challenge answer is 3093068400"
    );
}
