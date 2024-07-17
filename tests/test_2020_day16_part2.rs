mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day16_part2").trim(),
        "The answer to the challenge is 3429967441937"
    );
}
