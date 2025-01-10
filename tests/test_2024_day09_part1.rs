mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2024_day09_part1").trim(),
        "The checksum over all compacted files is 6386640365805"
    );
}
