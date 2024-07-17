mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2022_day14_part1").trim(),
        "The number of cells of sand that come to rest is 825"
    );
}
