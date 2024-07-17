mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2022_day15_part1").trim(),
        "A beacon cannot be present on 4879972 cells on row 2000000"
    );
}
