mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day18_part2").trim(),
        "The maximum magnitude obtainable from adding a pair of Snailfish numbers is 4563"
    );
}
