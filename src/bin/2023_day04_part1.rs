//! Advent of Code 2023 Day 04
//! https://adventofcode.com/2023/day/4
//!
//! Challenge part 1
//!
//! The input consists of scratch cards, one per line, containing a set of winning numbers, and
//! "our numbers". The points value of each card is based on the number of matching numbers. The
//! challenge answer is the sum of the points value of all cards.

use std::collections::HashSet;
use std::fs;

const INPUT_FILENAME: &str = "2023_day04_input.txt";

#[derive(Debug, PartialEq)]
struct Card {
    id: u8,
    winning_numbers: HashSet<u8>,
    our_numbers: HashSet<u8>,
}

impl Card {
    /// Creates a `Card` from the string passed. The string must contain the card id, a colon
    /// delimiter, a space-delimited set of winning numbers, a pipe symbol delimiter, and a space-
    /// delimited set of "our" numbers. For example:
    /// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    ///
    /// # Panics
    ///
    /// Panics if the string is malformed.
    fn from_str(s: &str) -> Self {
        let mut winning_numbers = HashSet::new();
        let mut our_numbers = HashSet::new();

        let card_and_numbers: Vec<&str> = s.split(": ").collect();
        assert_eq!(2, card_and_numbers.len(), "Malformed input in: {s}");

        let card_id_text = card_and_numbers[0].strip_prefix("Card").unwrap().trim();
        let card_id = card_id_text
            .parse()
            .expect("Problem parsing card id '{card_id_text}'");

        let winning_and_our_numbers: Vec<&str> = card_and_numbers[1].split(" | ").collect();
        assert_eq!(
            2,
            winning_and_our_numbers.len(),
            "Input for each card must contain exactly one pipe symbol"
        );

        for w in winning_and_our_numbers[0].split(' ') {
            if w.is_empty() {
                continue;
            }

            winning_numbers.insert(w.parse().expect("Error parsing winning number '{w}'"));
        }

        for w in winning_and_our_numbers[1].split(' ') {
            if w.is_empty() {
                continue;
            }

            our_numbers.insert(w.parse().expect("Error parsing our number '{w}'"));
        }

        Card {
            id: card_id,
            winning_numbers,
            our_numbers,
        }
    }

    /// Returns the points this `Card` is worth.
    //
    // This is the number of `winning_numbers` that match `our_numbers`.
    fn calculate_points(&self) -> u32 {
        let number_matches = self.winning_numbers.intersection(&self.our_numbers).count();

        if number_matches == 0 {
            0
        } else {
            u32::pow(2, number_matches as u32 - 1)
        }
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    println!(
        "The points total of all scratch cards is {}",
        do_challenge(&input)
    );
}

/// Calculates the sum of the points total of each card passed in the input.
fn do_challenge(input: &str) -> u32 {
    let cards = parse_cards_from_input(input);
    cards.iter().map(|c| c.calculate_points()).sum()
}

/// Converts every non-empty line of `input` to a `Card` object, and returns them as a `Vec`.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_cards_from_input(input: &str) -> Vec<Card> {
    let mut cards = Vec::new();

    for card in input.lines() {
        if card.is_empty() {
            continue;
        }

        cards.push(Card::from_str(card));
    }

    cards
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn card_from_str() {
        assert_eq!(
            Card {
                id: 1,
                winning_numbers: HashSet::from_iter(vec![11, 2, 33]),
                our_numbers: HashSet::from_iter(vec![14, 5, 16]),
            },
            Card::from_str("Card   1: 11  2 33 | 14  5 16")
        );
    }

    #[test]
    fn test_parse_cards_from_input() {
        let cards = parse_cards_from_input(TEST_INPUT);

        assert_eq!(
            vec![
                Card {
                    id: 1,
                    winning_numbers: HashSet::from_iter(vec![41, 48, 83, 86, 17]),
                    our_numbers: HashSet::from_iter(vec![83, 86, 6, 31, 17, 9, 48, 53]),
                },
                Card {
                    id: 2,
                    winning_numbers: HashSet::from_iter(vec![13, 32, 20, 16, 61]),
                    our_numbers: HashSet::from_iter(vec![61, 30, 68, 82, 17, 32, 24, 19]),
                },
                Card {
                    id: 3,
                    winning_numbers: HashSet::from_iter(vec![1, 21, 53, 59, 44]),
                    our_numbers: HashSet::from_iter(vec![69, 82, 63, 72, 16, 21, 14, 1]),
                },
                Card {
                    id: 4,
                    winning_numbers: HashSet::from_iter(vec![41, 92, 73, 84, 69]),
                    our_numbers: HashSet::from_iter(vec![59, 84, 76, 51, 58, 5, 54, 83]),
                },
                Card {
                    id: 5,
                    winning_numbers: HashSet::from_iter(vec![87, 83, 26, 28, 32]),
                    our_numbers: HashSet::from_iter(vec![88, 30, 70, 12, 93, 22, 82, 36]),
                },
                Card {
                    id: 6,
                    winning_numbers: HashSet::from_iter(vec![31, 18, 13, 56, 72]),
                    our_numbers: HashSet::from_iter(vec![74, 77, 10, 23, 35, 67, 36, 11]),
                },
            ],
            cards
        );
    }

    #[test]
    fn test_calculate_points() {
        let card = Card {
            id: 1,
            winning_numbers: HashSet::from_iter(vec![41, 48, 83, 86, 17]),
            our_numbers: HashSet::from_iter(vec![83, 86, 6, 31, 17, 9, 48, 53]),
        };

        assert_eq!(8, card.calculate_points());
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(13, do_challenge(TEST_INPUT));
    }
}
