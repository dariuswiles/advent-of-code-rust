mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2023_day02_part2").trim(),
        "The sum of the powers of the minimum cubes required for each game is 70387"
    );
}
