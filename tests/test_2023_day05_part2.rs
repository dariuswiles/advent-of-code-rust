mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2023_day05_part2").trim(),
        "The lowest location value is 72263011"
    );
}
