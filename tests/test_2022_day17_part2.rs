mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2022_day17_part2").trim(),
        "The number of rows in the cavern containing rocks is 1585673352422"
    );
}
