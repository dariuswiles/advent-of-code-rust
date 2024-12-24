//! Advent of Code 2023 Day 04
//! https://adventofcode.com/2023/day/4
//!
//! Challenge part 2
//!
//! The input consists of scratch cards, one per line, containing a set of winning numbers, and a
//! set of "our numbers". The number of matching numbers on a card dictate the number of
//! subsequently numbered cards we receive in addition to the one copy of each card we start with.
//! For example, if card 5 has 3 winning numbers, we receive an additional card 6, card 7 and card
//! 8.
//!
//! The challenge answer is the total number of scratch cards we ultimately end up with.

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

    /// Returns the number of `winning_numbers` that match `our_numbers` for this `Card`.
    fn count_matches(&self) -> usize {
        self.winning_numbers.intersection(&self.our_numbers).count()
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    println!(
        "The total number of scratch cards is {}",
        do_challenge(&input)
    );
}

/// Looks at each card in turn, starting with card id 1, determining the number of matching numbers.
/// Matching numbers increase the number of cards with subsequent ids we have. For example, if we
/// have one card id 5, and it has two matching numbers, we increase the number of copies of card
/// id 6 and card id 7 by 1. We increase the number of copies of subsequent cards by the number of
/// copies of the winning card that we have.
///
/// After all cards are processed, the total number of scratch card copies is returned as the
/// challenge answer.
//
// Note that card ids start at 1, but Rust indices start at 0, so card id 1 is stored in index 0,
// card id 2 is in index 1, etc.
fn do_challenge(input: &str) -> u32 {
    let cards = parse_cards_from_input(input);
    let max_card_idx = cards.len();

    // card_copy_count holds the number of each card we have. We start with one copy of each card.
    let mut card_copy_count = Vec::new();
    card_copy_count.resize(max_card_idx, 1);

    for cards_idx in 0..max_card_idx {
        let winning_numbers = cards[cards_idx].count_matches();
        let last_card_to_inc = max_card_idx.min(cards_idx + winning_numbers);

        for cards_inc_idx in (cards_idx + 1)..=last_card_to_inc {
            card_copy_count[cards_inc_idx] += card_copy_count[cards_idx];
        }
    }

    card_copy_count.iter().sum()
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
    fn test_count_matches() {
        let cards = parse_cards_from_input(TEST_INPUT);

        assert_eq!(4, cards[0].count_matches());
        assert_eq!(2, cards[1].count_matches());
        assert_eq!(2, cards[2].count_matches());
        assert_eq!(1, cards[3].count_matches());
        assert_eq!(0, cards[4].count_matches());
        assert_eq!(0, cards[5].count_matches());
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(30, do_challenge(TEST_INPUT));
    }
}
