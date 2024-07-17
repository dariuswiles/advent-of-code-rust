mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day01_part2").trim(),
        "Integers 352, 358 and 1310 sum to required total, and multiplying them gives 165080960"
    );
}
