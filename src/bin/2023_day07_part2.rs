//! Advent of Code 2023 Day 07
//! https://adventofcode.com/2023/day/7
//!
//! Challenge part 2
//!
//! The challenge input is a list of card hands, one per line, with each hand having an associated
//! "bid" value. The challenge requires the hands to be sorted based on their strength relative to
//! other hands, using a scoring system similar to poker. The challenge answer is then based on
//! the relative rank of each card and its bid value.
//!
//! Part 2 of the challenge replaces Jacks with Jokers. Jokers take the value of whichever other
//! card results in a hand with the highest score.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

const INPUT_FILENAME: &str = "2023_day07_input.txt";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Card {
    Ace,
    King,
    Queen,
    Ten,
    Number(u8),
    Joker,
}

impl Card {
    /// Returns the numeric value for this card. Aces are high, so score 14. A King is worth 13,
    /// etc. Part 2 of the challenge replaces Jacks with Jokers, and states that the Joker should
    /// have less value than all other cards.
    fn value(&self) -> u8 {
        match self {
            Self::Ace => 14,
            Self::King => 13,
            Self::Queen => 12,
            Self::Ten => 10,
            Self::Number(n) if (2..10).contains(n) => *n,
            Self::Joker => 1,
            _ => {
                panic!("Non-picture cards can only have a value between 2 and 9 (inclusive)");
            }
        }
    }

    /// Returns a `Card` created from the `char` provided as input.
    ///
    /// # Panics
    ///
    /// Panics if the `char` does not correspond to a valid `Card`.
    fn from_char(c: char) -> Self {
        match c {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'T' => Self::Ten,
            '2'..='9' => Self::Number(
                c.to_digit(10)
                    .expect("Non-picture cards can only have a value between 2 and 9 (inclusive)")
                    as u8,
            ),
            'J' => Self::Joker,
            _ => {
                panic!("Unrecognized card value {c}");
            }
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A hand of five `Card`s, its associated "bid" value, and its `HandType`. The latter is determined
/// from the `Card`s and is not provided in the input.
#[derive(Clone, Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    hand_type: HandType,
}

impl Hand {
    /// Creates and returns a `Hand` from the given `String`, which consists of a group of letters
    /// and numbers representing individual cards, a single space, and an integer representing the
    /// bid value of the hand.
    fn from_str(s: &str) -> Self {
        let tokens: Vec<_> = s.split(' ').collect();
        assert_eq!(tokens.len(), 2, "Expected exactly one space in input {s}");

        let cards = parse_card_group(tokens[0]);
        assert_eq!(
            5,
            cards.len(),
            "A card hand must consist of exactly 5 cards"
        );

        let hand_type = Self::score_cards(&cards);

        Self {
            cards,
            bid: tokens[1]
                .parse()
                .expect("Could not parse bid value in input {}"),
            hand_type,
        }
    }

    /// Returns the hand type for the given `cards` that scores most highly. The Jokers introduced
    /// by part 2 of the challenge are treated as wildcards that take the face value of whichever
    /// other card results in the strongest hand.
    ///
    /// # Panics
    ///
    /// Panics if `cards` does not consist of five valid cards.
    fn score_cards(cards: &Vec<Card>) -> HandType {
        let mut score_set = HashMap::new();

        for c in cards {
            match score_set.get_mut(&c) {
                Some(n) => {
                    *n += 1;
                }
                None => {
                    score_set.insert(c, 1);
                }
            }
        }

        let max_same_card = score_set.values().max().unwrap();

        match max_same_card {
            5 => HandType::FiveOfAKind,
            4 => {
                if score_set.contains_key(&Card::Joker) {
                    HandType::FiveOfAKind
                } else {
                    HandType::FourOfAKind
                }
            }
            3 => match score_set.get(&Card::Joker) {
                Some(&3) => {
                    if score_set.values().any(|&count| count == 2) {
                        HandType::FiveOfAKind
                    } else {
                        HandType::FourOfAKind
                    }
                }
                Some(&2) => HandType::FiveOfAKind,
                Some(&1) => HandType::FourOfAKind,
                None => {
                    if score_set.values().any(|&count| count == 2) {
                        HandType::FullHouse
                    } else {
                        HandType::ThreeOfAKind
                    }
                }
                _ => {
                    panic!("Internal error in code to identify hand with 3 matching cards");
                }
            },
            2 => {
                let pairs: Vec<_> = score_set
                    .iter()
                    .filter_map(|(&&c, &count)| if count == 2 { Some(c) } else { None })
                    .collect();

                if pairs.len() == 2 {
                    if pairs.contains(&Card::Joker) {
                        HandType::FourOfAKind
                    } else if score_set.contains_key(&Card::Joker) {
                        HandType::FullHouse
                    } else {
                        HandType::TwoPair
                    }
                } else if score_set.contains_key(&Card::Joker) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::OnePair
                }
            }
            1 => {
                if score_set.contains_key(&Card::Joker) {
                    HandType::OnePair
                } else {
                    HandType::HighCard
                }
            }
            _ => {
                panic!("Failed to determine the type of hand");
            }
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let comparison = self.hand_type.cmp(&other.hand_type);

        if comparison != Ordering::Equal {
            return comparison;
        }

        self.cards.cmp(&other.cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Type of hand, ordered from weakest to strongest.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    println!(
        "The sum of each card's bid multiplied by its rank is {}",
        do_challenge(&input)
    );
}

/// Calculates and returns the challenge answer. This is the sum of the `bid` value of each hand
/// multiplied by its rank. A `Hand`'s rank is based on its relative strength, where 1 indicates
/// the weakest `Hand`.
fn do_challenge(input: &str) -> u64 {
    let mut hands = parse_hands(input);
    sort_hands(&mut hands);

    hands.into_iter().enumerate().fold(0, |acc, (index, hand)| {
        acc + (index + 1) as u64 * hand.bid as u64
    })
}

/// Parses the passed string as a group of `Card`s and returns them in a `Vec`.
fn parse_card_group(s: &str) -> Vec<Card> {
    s.chars().map(Card::from_char).collect()
}

/// Parses non-empty lines passed in `s` into a `Vec` of `Hands`. Each line contains a five
/// character string, one character for each card, one space, and an integer providing the
/// associated bid amount.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_hands(s: &str) -> Vec<Hand> {
    let mut hands = Vec::new();

    for line in s.lines() {
        if line.is_empty() {
            continue;
        }

        hands.push(Hand::from_str(line));
    }

    hands
}

/// Sorts the `Vec` of `Hand` passed such that the weakest hand is the first element in the `Vec`
/// and the strongest is the last.
fn sort_hands(hands: &mut [Hand]) {
    hands.sort_unstable();
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_card_value() {
        assert_eq!(14, Card::Ace.value());
        assert_eq!(13, Card::King.value());
        assert_eq!(12, Card::Queen.value());
        assert_eq!(10, Card::Ten.value());
        assert_eq!(6, Card::Number(6).value());
        assert_eq!(1, Card::Joker.value());
    }

    #[test]
    #[should_panic]
    fn test_card_value_panic() {
        Card::Number(99).value();
    }

    #[test]
    fn test_card_ordering() {
        assert!(Card::Ace > Card::King);
        assert!(Card::King > Card::Queen);
        assert!(Card::Queen > Card::Joker);
        assert!(Card::Ten > Card::Number(9));
        assert!(Card::Joker < Card::Number(2));
        assert!(Card::Number(9) > Card::Number(2));
        assert!(Card::Number(5) < Card::Ten);
        assert!(Card::Number(3) <= Card::Number(3));
        assert!(Card::Queen == Card::Queen);
        assert!(Card::Number(7) == Card::Number(7));
        assert!(Card::Ten != Card::Joker);
        assert_eq!(Card::Number(4), Card::Number(4));
    }

    #[test]
    fn test_card_from_char() {
        assert_eq!(Card::Ace, Card::from_char('A'));
        assert_eq!(Card::Ten, Card::from_char('T'));
        assert_eq!(Card::Number(3), Card::from_char('3'));
    }

    #[test]
    #[should_panic]
    fn test_card_from_char_panic() {
        Card::from_char('1');
    }

    #[test]
    fn test_hand_from_str() {
        assert_eq!(
            Hand {
                cards: vec![
                    Card::Number(7),
                    Card::Ace,
                    Card::Number(2),
                    Card::Joker,
                    Card::Ten,
                ],
                bid: 123,
                hand_type: HandType::OnePair,
            },
            Hand::from_str("7A2JT 123")
        );
    }

    #[test]
    fn test_parse_hands() {
        assert_eq!(
            vec![
                Hand {
                    cards: vec![
                        Card::Number(3),
                        Card::Number(2),
                        Card::Ten,
                        Card::Number(3),
                        Card::King,
                    ],
                    bid: 765,
                    hand_type: HandType::OnePair,
                },
                Hand {
                    cards: vec![
                        Card::Ten,
                        Card::Number(5),
                        Card::Number(5),
                        Card::Joker,
                        Card::Number(5),
                    ],
                    bid: 684,
                    hand_type: HandType::FourOfAKind,
                },
                Hand {
                    cards: vec![
                        Card::King,
                        Card::King,
                        Card::Number(6),
                        Card::Number(7),
                        Card::Number(7),
                    ],
                    bid: 28,
                    hand_type: HandType::TwoPair,
                },
                Hand {
                    cards: vec![Card::King, Card::Ten, Card::Joker, Card::Joker, Card::Ten,],
                    bid: 220,
                    hand_type: HandType::FourOfAKind,
                },
                Hand {
                    cards: vec![
                        Card::Queen,
                        Card::Queen,
                        Card::Queen,
                        Card::Joker,
                        Card::Ace,
                    ],
                    bid: 483,
                    hand_type: HandType::FourOfAKind,
                },
            ],
            parse_hands(TEST_INPUT)
        );
    }

    #[test]
    fn test_hand_type_ordering() {
        assert!(HandType::HighCard < HandType::OnePair);
        assert!(HandType::FullHouse < HandType::FourOfAKind);
        assert!(HandType::FiveOfAKind == HandType::FiveOfAKind);
        assert!(HandType::ThreeOfAKind >= HandType::TwoPair);
        assert!(HandType::OnePair <= HandType::FourOfAKind);
        assert!(HandType::HighCard <= HandType::HighCard);
    }

    #[test]
    fn test_cmp_hands() {
        let hands = parse_hands(TEST_INPUT);

        assert!(hands[0] < hands[1]);
        assert!(hands[0] < hands[2]);
        assert!(hands[0] < hands[3]);
        assert!(hands[0] < hands[4]);

        assert!(hands[1] > hands[0]);
        assert!(hands[1] > hands[2]);
        assert!(hands[1] < hands[3]);
        assert!(hands[1] < hands[4]);

        assert!(hands[2] > hands[0]);
        assert!(hands[2] < hands[1]);
        assert!(hands[2] < hands[3]);
        assert!(hands[2] < hands[4]);

        assert!(hands[3] > hands[0]);
        assert!(hands[3] > hands[1]);
        assert!(hands[3] > hands[2]);
        assert!(hands[3] > hands[4]);

        assert!(hands[4] > hands[0]);
        assert!(hands[4] > hands[1]);
        assert!(hands[4] > hands[2]);
        assert!(hands[4] < hands[3]);
    }

    #[test]
    fn test_sorting_hands() {
        let mut hands = parse_hands(TEST_INPUT);
        sort_hands(&mut hands);

        assert_eq!(765, hands[0].bid);
        assert_eq!(28, hands[1].bid);
        assert_eq!(684, hands[2].bid);
        assert_eq!(483, hands[3].bid);
        assert_eq!(220, hands[4].bid);
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(5905, do_challenge(TEST_INPUT));
    }
}
