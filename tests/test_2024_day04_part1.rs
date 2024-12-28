mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2024_day04_part1").trim(),
        "The word 'XMAS' appears in the input wordsearch 2427 times"
    );
}
