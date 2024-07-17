mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day21_part1").trim(),
        "Allergen-free ingredients appear in the list of foods 2659 times"
    );
}
