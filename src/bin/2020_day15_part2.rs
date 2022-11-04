//! Advent of Code 2020 Day 15
//! https://adventofcode.com/2020/day/15
//!
//! Challenge part 2
//!
//! Follows the game rules explained in the challenge until the given game turn is reached, at
//! which point the answer to the challenge is obtained. Part 2 of the challenge only increases
//! the number of game turns from 2,020 to 30,000,000.
//
// The increase in the number of turns causes the solution for part 1 to take an unacceptably long
// time to run, so the code is completely rewritten to provide a fast solution for even large
// numbers of turns.

use std::collections::HashMap;

const CHALLENGE_INPUT: &str = "7,14,0,17,11,1,2";
const STOP_AT_TURN: usize = 30_000_000;

/// The game state consisting of:
/// `state` - holding the last turn each game value was seen;
/// `next_num` - the number to added in the next game turn;
/// `turn` - the turn number (where the first turn is 1).
//
// The game rules rely on knowing the last turn each value was seen. Rather than recording the game
// result for every game turn, storing only the last turn each value was seen allows faster lookups
// and requires less memory. Before adding a new value, a lookup is performed to see if it has
// previously been added, and the result is stored in `next_num`. This is a little ugly, but is
// faster than the alternative of storing the last *two* occurrences of every value in the `Game`
// object.
#[derive(Clone, Debug)]
struct Game {
    state: HashMap<usize, usize>,
    next_num: usize,
    turn: usize,
}

impl Game {
    fn from_str(start_string: &str) -> Self {
        let mut state = HashMap::new();
        let mut next_num = 0;

        let nums: Vec<usize> = start_string
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        for (idx, num) in nums[..nums.len() - 1].iter().enumerate() {
            state.insert(*num, idx + 1);
        }

        if let Some(prior_turn) = state.get(nums.last().unwrap()) {
            next_num = nums.len() - prior_turn;
        }

        state.insert(*nums.last().unwrap(), nums.len());

        Self {
            state: state,
            next_num: next_num,
            turn: nums.len(),
        }
    }

    fn play_one_turn(&mut self) {
        let num_to_add = self.next_num;

        self.turn += 1;
        // print!("Turn {}: Adding {} ", &self.turn,& num_to_add);

        if let Some(prior_turn) = self.state.get(&num_to_add) {
            // println!("which was last seen on turn {}.", &prior_turn);
            self.next_num = self.turn - prior_turn;
        } else {
            // println!("which has not been seen before");
            self.next_num = 0;
        }

        self.state.insert(num_to_add, self.turn);
    }

    /// Play the game until the given turn is reached.
    //
    // This is implemented by iterating until one less than the desired turn, and looking in the
    // `next_num` field to see what the value stored in the next turn will be. This is required as
    // no record is kept of the last value added to the `state` HashMap, so if we iterated until
    // the given turn, we would not be able to determine the last value added, which is the
    // challenge answer.
    fn play_until_turn(&mut self, end_turn: usize) -> usize {
        while self.turn < end_turn - 1 {
            self.play_one_turn();
        }

        self.next_num
    }
}

fn main() {
    let mut game = Game::from_str(CHALLENGE_INPUT);
    let result = game.play_until_turn(STOP_AT_TURN);

    println!("The answer to the challenge is {:?}", result);
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
        let mut game = Game::from_str(&TEST_INPUT_0);
        let result = game.play_until_turn(STOP_AT_TURN);

        assert_eq!(result, 175594);
    }

    #[test]
    fn test_game_1() {
        let mut game = Game::from_str(&TEST_INPUT_1);
        let result = game.play_until_turn(STOP_AT_TURN);

        assert_eq!(result, 2578);
    }

    #[test]
    fn test_game_2() {
        let mut game = Game::from_str(&TEST_INPUT_2);
        let result = game.play_until_turn(STOP_AT_TURN);

        assert_eq!(result, 3544142);
    }

    #[test]
    fn test_game_3() {
        let mut game = Game::from_str(&TEST_INPUT_3);
        let result = game.play_until_turn(STOP_AT_TURN);

        assert_eq!(result, 261214);
    }

    #[test]
    fn test_game_4() {
        let mut game = Game::from_str(&TEST_INPUT_4);
        let result = game.play_until_turn(STOP_AT_TURN);

        assert_eq!(result, 6895259);
    }

    #[test]
    fn test_game_5() {
        let mut game = Game::from_str(&TEST_INPUT_5);
        let result = game.play_until_turn(STOP_AT_TURN);

        assert_eq!(result, 18);
    }

    #[test]
    fn test_game_6() {
        let mut game = Game::from_str(&TEST_INPUT_6);
        let result = game.play_until_turn(STOP_AT_TURN);

        assert_eq!(result, 362);
    }

    #[test]
    fn initialize_with_last_num_repeated() {
        let game = Game::from_str("1,7,8,9,1");

        assert_eq!(game.state.len(), 4);
        assert_eq!(game.state[&7], 2);
        assert_eq!(game.state[&8], 3);
        assert_eq!(game.state[&9], 4);
        assert_eq!(game.state[&1], 5);
        assert_eq!(game.next_num, 4);
        assert_eq!(game.turn, 5);
    }

    #[test]
    fn initialize_with_last_num_not_repeated() {
        let game = Game::from_str("1,7,8,9");

        assert_eq!(game.state.len(), 4);
        assert_eq!(game.state[&1], 1);
        assert_eq!(game.state[&7], 2);
        assert_eq!(game.state[&8], 3);
        assert_eq!(game.state[&9], 4);
        assert_eq!(game.next_num, 0);
        assert_eq!(game.turn, 4);
    }

    #[test]
    fn initialize_with_all_repeats() {
        let game = Game::from_str("7,7,7");

        assert_eq!(game.state.len(), 1);
        assert_eq!(game.state[&7], 3);
        assert_eq!(game.next_num, 1);
        assert_eq!(game.turn, 3);
    }

    #[test]
    fn one_turn_0() {
        let mut game = Game::from_str("33,33,29,78,1");
        game.play_one_turn();
        assert_eq!(game.state.len(), 5);
        assert_eq!(game.state[&33], 2);
        assert_eq!(game.state[&29], 3);
        assert_eq!(game.state[&78], 4);
        assert_eq!(game.state[&1], 5);
        assert_eq!(game.state[&0], 6);
        assert_eq!(game.next_num, 0);
        assert_eq!(game.turn, 6);
    }

    #[test]
    fn one_turn_1() {
        let mut game = Game::from_str("4,0,9,3");
        game.play_one_turn();
        assert_eq!(game.state.len(), 4);
        assert_eq!(game.state[&4], 1);
        assert_eq!(game.state[&9], 3);
        assert_eq!(game.state[&3], 4);
        assert_eq!(game.state[&0], 5);
        assert_eq!(game.next_num, 3);
        assert_eq!(game.turn, 5);
    }
}
