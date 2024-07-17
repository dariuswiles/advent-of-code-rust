mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day18_part1").trim(),
        "Iteratively adding all Snailfish numbers in the input gives a magnitude of 3305"
    );
}
