//! Advent of Code 2020 Day 25
//! https://adventofcode.com/2020/day/25
//!
//! Challenge part 1
//!
//! Brute force the shared encryption key for a simple encryption protocol that shares some
//! characteristics with Diffie-Hellman key exchange.

use std::fs;

const INPUT_FILENAME: &str = "2020_day25_input.txt";
const SUBJECT_NUMBER: u64 = 7;
const MODULUS: u64 = 20201227;
const MAX_ITERATIONS: u64 = 10_000_000;

type PublicKey = u64;
type CardPK = PublicKey;
type DoorPK = PublicKey;
type LoopSize = u64;

/// Extracts the public keys for the card and door from `input`, and returns them in a tuple. These
/// must be provided one per line, card first, in `input`.
///
/// # Panics
///
/// Panics if the card and door public key integers are not on the first two lines of `input`.
fn read_keys(input: &str) -> (CardPK, DoorPK) {
    let mut lines = input.lines();
    let card_pk = lines.next().unwrap().parse().unwrap();
    let door_pk = lines.next().unwrap().parse().unwrap();

    (card_pk, door_pk)
}

/// Given a public key, `modulus` and `subject_number`, find the number of loops of the algorithm
/// given in the challenge that generate the public key.
fn find_loop_size(pk: PublicKey, modulus: u64, subject_number: u64) -> LoopSize {
    let mut value = 1;

    for iterations in 0..MAX_ITERATIONS {
        value = (value * subject_number) % modulus;

        if value == pk {
            return iterations + 1;
        }
    }

    panic!("Maximum number of iterations reached while searching for public key loop size");
}

/// Given the `pk` of one device (either the card or the door), and the `loop_size` of the *other*
/// device, returns the encryption key both devices are using. As the encryption key is shared,
/// the result will be the same regardless of which way round the data is provided.
fn generate_encryption_key(subject_number: u64, loop_size: LoopSize, modulus: u64) -> u64 {
    let mut ek = 1;

    for _ in 0..loop_size {
        ek = (ek * subject_number) % modulus;
    }

    ek
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let (card_pk, door_pk) = read_keys(&input_file);
    let card_loop_size = find_loop_size(card_pk, MODULUS, SUBJECT_NUMBER);
    let card_ek = generate_encryption_key(door_pk, card_loop_size, MODULUS);

    println!("Shared encryption key is {}", card_ek);
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
5764801
17807724";

    #[test]
    fn test_read_keys() {
        let keys = read_keys(TEST_INPUT);

        assert_eq!((5764801, 17807724), keys);
    }

    #[test]
    fn test_find_loop_size() {
        assert_eq!(8, find_loop_size(5764801, MODULUS, SUBJECT_NUMBER));
        assert_eq!(11, find_loop_size(17807724, MODULUS, SUBJECT_NUMBER));
    }

    #[test]
    fn generate_encryption_keys() {
        let (card_pk, door_pk) = read_keys(TEST_INPUT);
        assert_eq!(5764801, card_pk);
        assert_eq!(17807724, door_pk);

        let card_loop_size = find_loop_size(card_pk, MODULUS, SUBJECT_NUMBER);
        let door_loop_size = find_loop_size(door_pk, MODULUS, SUBJECT_NUMBER);

        let card_ek = generate_encryption_key(door_pk, card_loop_size, MODULUS);
        let door_ek = generate_encryption_key(card_pk, door_loop_size, MODULUS);

        assert_eq!(14897079, card_ek);
        assert_eq!(14897079, door_ek);
    }
}
