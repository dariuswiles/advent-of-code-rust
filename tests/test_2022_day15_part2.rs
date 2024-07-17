mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2022_day15_part2").trim(),
        "The tuning frequency of the emergency beacon is 12525726647448"
    );
}
