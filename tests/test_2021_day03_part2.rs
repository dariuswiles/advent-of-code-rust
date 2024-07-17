mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day03_part2").trim(),
        "The submarine's life support rating is 4412188"
    );
}
