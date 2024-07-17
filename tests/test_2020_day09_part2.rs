mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day09_part2").trim(),
        "The invalid number in the input is 1309761972
A sequence of numbers that sum to this value exists. Its smallest number is 55849911 and its largest 122139921.The sum of these two, and the answer to the challenge, is 177989832"
    );
}
