mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2022_day01_part2").trim(),
        "The sum of the largest sum of elf calories is: 212520 calories"
    );
}
