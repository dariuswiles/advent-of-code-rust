mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2022_day12_part2").trim(),
        "The shortest path from start to finish is 525"
    );
}
