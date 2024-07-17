mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day05_part1").trim(),
        "The number of positions with intersecting geothermal vents is 7436"
    );
}
