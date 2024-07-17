mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day17_part1").trim(),
        "The highest y position that the probe can reach and pass through the target is 4186"
    );
}
