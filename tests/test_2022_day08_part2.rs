mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2022_day08_part2").trim(),
        "The highest scenic score is 291840"
    );
}
