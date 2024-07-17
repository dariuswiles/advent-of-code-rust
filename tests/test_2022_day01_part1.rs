mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2022_day01_part1").trim(),
        "The elf with the largest number of calories has: 72017 calories"
    );
}
