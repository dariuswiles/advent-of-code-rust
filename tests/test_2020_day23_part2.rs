mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2020_day23_part2").trim(),
        "Challenge answer is 235551949822"
    );
}
