mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2024_day06_part2").trim(),
        "Their are 1939 locations where an obstacle can be added to cause an endless patrol loop"
    );
}
