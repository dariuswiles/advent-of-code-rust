mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day20_part2").trim(),
        "The number of hash signs in the combined set of tiles that are *not* part of a sea monster is 2129"
    );
}
