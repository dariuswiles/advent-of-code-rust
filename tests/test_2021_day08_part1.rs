mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day08_part1").trim(),
        "The digits 1, 4, 7 and 8 occur 530 times in the right hand side of the input"
    );
}
