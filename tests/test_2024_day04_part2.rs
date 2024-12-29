mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2024_day04_part2").trim(),
        "The X-MAS pattern appears in the input wordsearch 1900 times"
    );
}
