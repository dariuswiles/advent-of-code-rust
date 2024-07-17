mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2022_day03_part2").trim(),
        "The challenge answer is 2689"
    );
}
