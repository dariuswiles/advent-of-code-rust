mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day11_part2").trim(),
        "The first simultaneous flash happens immediately after step 232"
    );
}
