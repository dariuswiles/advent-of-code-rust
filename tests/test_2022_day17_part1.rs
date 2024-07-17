mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2022_day17_part1").trim(),
        "The number of rows in the cavern containing rocks is 3217"
    );
}
