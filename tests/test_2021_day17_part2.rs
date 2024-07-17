mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day17_part2").trim(),
        "The number of initial (x, y) velocities that land the within the target is 2709"
    );
}
