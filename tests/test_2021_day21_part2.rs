mod common;

#[test]
fn run_test() {
    assert_eq!(
        common::run_challenge("2021_day21_part2").trim(),
        "Player 1 wins 492043106122795 times and Player 2 wins 267086464416104 times
The challenge answer is the larger of these numbers, which is: 492043106122795"
    );
}
