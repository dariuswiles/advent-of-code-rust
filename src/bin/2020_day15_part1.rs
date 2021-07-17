//! Advent of Code 2020 Day 15
//! https://adventofcode.com/2020/day/15
//!
//! Challenge part 1
//!
//! Follows the game rules explained in the challenge until the given game round is reached, at
//! which point the answer to the challenge is obtained.

const CHALLENGE_INPUT: &str = "7,14,0,17,11,1,2";
const STOP_AT_ROUND: usize = 2020;

type GameState = Vec<u32>;


/// Searches backwards through vector `gs` for up to two occurrences of `num`. If not found,
/// returns (None, None). If two occurrences are found, both are returned, with the one with the
/// lowest vector index returned first. If only one occurrence is found, returns 'None' and the
/// vector index in that order.
fn rsearch_for_number(gs: &GameState, num: u32) -> (Option<usize>, Option<usize>) {
    let mut earlier = None;

    let later = gs.iter().rposition(|n| *n == num);

    if let Some(pos) = later {
        earlier = gs[..pos].iter().rposition(|n| *n == num);
    }

//     println!("\tSearching {} in vec {:?} returns ({:?}, {:?})", num, gs, earlier, later);
    (earlier, later)
}


fn initiliaze_game(start_string: &str) -> GameState{
    start_string.split(',').map(|n| n.parse().unwrap()).collect()
}


fn play_one_round(game: &mut GameState) {
    let last_number = game.last().unwrap();

    let (earlier, later) = rsearch_for_number(game, *last_number);

    if (earlier == Option::None) && (later != Option::None) {
        game.push(0);
    } else if (earlier != Option::None) && (later != Option::None) {
        game.push((later.unwrap() - earlier.unwrap()) as u32);
    } else {
        panic!("Unexpected error - search could not find any occurrences of last number");
    }
}


fn play_game(game: &mut GameState, play_until_round: usize) {
    while game.len() < play_until_round {
        play_one_round(game);
    }
}


fn main() {
    let mut game = initiliaze_game(CHALLENGE_INPUT);
    play_game(&mut game, STOP_AT_ROUND);
    println!("The answer to the challenge is {:?}", game.last().unwrap());
}


// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_0: &str = "0,3,6";
    const TEST_INPUT_1: &str = "1,3,2";
    const TEST_INPUT_2: &str = "2,1,3";
    const TEST_INPUT_3: &str = "1,2,3";
    const TEST_INPUT_4: &str = "2,3,1";
    const TEST_INPUT_5: &str = "3,2,1";
    const TEST_INPUT_6: &str = "3,1,2";

    #[test]
    fn test_game_0() {
        let mut game = initiliaze_game(TEST_INPUT_0);
        play_game(&mut game, STOP_AT_ROUND);

        assert_eq!(*game.last().unwrap(), 436);
    }

    #[test]
    fn test_game_1() {
        let mut game = initiliaze_game(TEST_INPUT_1);
        play_game(&mut game, STOP_AT_ROUND);

        assert_eq!(*game.last().unwrap(), 1);
    }

    #[test]
    fn test_game_2() {
        let mut game = initiliaze_game(TEST_INPUT_2);
        play_game(&mut game, STOP_AT_ROUND);

        assert_eq!(*game.last().unwrap(), 10);
    }

    #[test]
    fn test_game_3() {
        let mut game = initiliaze_game(TEST_INPUT_3);
        play_game(&mut game, STOP_AT_ROUND);

        assert_eq!(*game.last().unwrap(), 27);
    }

    #[test]
    fn test_game_4() {
        let mut game = initiliaze_game(TEST_INPUT_4);
        play_game(&mut game, STOP_AT_ROUND);

        assert_eq!(*game.last().unwrap(), 78);
    }

    #[test]
    fn test_game_5() {
        let mut game = initiliaze_game(TEST_INPUT_5);
        play_game(&mut game, STOP_AT_ROUND);

        assert_eq!(*game.last().unwrap(), 438);
    }

    #[test]
    fn test_game_6() {
        let mut game = initiliaze_game(TEST_INPUT_6);
        play_game(&mut game, STOP_AT_ROUND);

        assert_eq!(*game.last().unwrap(), 1836);
    }

    #[test]
    fn test_rsearch_for_number_0() {
        assert_eq!(rsearch_for_number(&vec![16, 26, 16, 7, 9, 0, 16, 44], 99), (None, None));
    }

    #[test]
    fn test_rsearch_for_number_1() {
        assert_eq!(rsearch_for_number(&vec![16, 26, 16, 7, 9, 0, 16, 44], 26), (None, Some(1)));
    }

    #[test]
    fn test_rsearch_for_number_2() {
        assert_eq!(rsearch_for_number(&vec![16, 26, 16, 7, 9, 0, 16, 44], 16), (Some(2), Some(6)));
    }
}
