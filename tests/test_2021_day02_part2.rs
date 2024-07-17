mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day02_part2").trim(),
        "The product of the submarine's final position is 2120734350"
    );
}
