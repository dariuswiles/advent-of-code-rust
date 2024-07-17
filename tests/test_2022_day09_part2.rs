mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2022_day09_part2").trim(),
        "The rope tail passed through 2443 unique positions"
    );
}
