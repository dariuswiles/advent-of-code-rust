//! Advent of Code 2024 Day 04
//! https://adventofcode.com/2024/day/4
//!
//! Challenge part 1
//!
//! Counts the number of occurrences of the string "XMAS" in the input grid. Occurrences are counted
//! from left to right, right to left, top to bottom, bottom to top, and diagonally from: top-left
//! to bottom-right (and the reverse), and from top-right to bottom-left (and the reverse).

use std::fs;

const INPUT_FILENAME: &str = "2024_day04_input.txt";
const SEARCH_TERM: &str = "XMAS";

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    println!(
        "The word '{SEARCH_TERM}' appears in the input wordsearch {} times",
        do_challenge(&input)
    );
}

fn do_challenge(input: &str) -> u32 {
    count_occurrences_all_directions(SEARCH_TERM, &parse_into_vec(input))
}

/// Splits the passed input into a `Vec` of separate lines and returns it. Empty lines are
/// discarded.
fn parse_into_vec(input: &str) -> Vec<&str> {
    input.lines().filter(|line| !line.is_empty()).collect()
}

/// Returns a `Vec` of `String`s that are the slice of str slices passed, except that each str slice
/// is reversed.
fn reverse(slices: &[&str]) -> Vec<String> {
    slices
        .iter()
        .map(|s: &&str| s.chars().rev().collect())
        .collect()
}

/// Returns a `Vec` of `String`s containing each column of data from the input, i.e., the first
/// `String` of the output contains the first column of input data.
fn top_to_bottom(slices: &[&str]) -> Vec<String> {
    let size = slices[0].len();
    assert_eq!(
        size,
        slices.len(),
        "The input must contain an equal number of rows and columns"
    );

    let mut output = Vec::new();

    for col in 0..size {
        let mut s = String::new();
        #[allow(clippy::needless_range_loop)]
        for row in 0..size {
            s.push_str(&slices[row][col..=col]);
        }
        output.push(s);
    }

    output
}

/// Returns an unordered `Vec` of `String`s containing diagonal columns of data from the input. For
/// example, the input:
///     ABC
///     DEF
///     GHI
///
/// returns a `Vec` containing "C", "BF", "AEI", "DH" and "G", but not necessarily in this order.
fn top_left_to_bottom_right(slices: &[&str]) -> Vec<String> {
    let size = slices[0].len();
    assert_eq!(
        size,
        slices.len(),
        "The input must contain an equal number of rows and columns"
    );

    let mut output = Vec::new();

    for offset in 0..size {
        let mut s_above = String::new();
        let mut s_below = String::new();
        for col in 0..size {
            let row = col + offset;
            if row >= size || col >= size {
                break;
            }
            s_above.push_str(&slices[row][col..=col]);
            s_below.push_str(&slices[col][row..=row]);
        }

        output.push(s_above);
        if offset > 0 {
            output.push(s_below);
        }
    }

    output
}

/// Returns an unordered `Vec` of `String`s containing diagonal columns of data from the input. For
/// example, the input:
///     ABC
///     DEF
///     GHI
///
/// returns a `Vec` containing "A", "BD", "CEG", "FH", "I", but not necessarily in this order.
fn top_right_to_bottom_left(slices: &[&str]) -> Vec<String> {
    let reversed: Vec<String> = reverse(slices);
    top_left_to_bottom_right(&reversed.iter().map(|s| s.as_str()).collect::<Vec<&str>>())
}

/// Returns the number of times the slice `needle` and its reverse appear in all the `slices`.
/// For example, the slice "XMAS" occurs 4 times in "SAMXMASAAASAMXMAS".
fn count_occurrences(needle: &str, slices: &[&str]) -> u32 {
    let mut total = u32::try_from(
        slices
            .iter()
            .map(|s| s.matches(needle).count())
            .sum::<usize>(),
    )
    .unwrap();

    let mut needle_chars: Vec<char> = needle.chars().collect();
    needle_chars.reverse();
    let needle_rev: String = needle_chars.iter().collect();

    total += u32::try_from(
        slices
            .iter()
            .map(|s| s.matches(&needle_rev).count())
            .sum::<usize>(),
    )
    .unwrap();

    total
}

/// Counts the occurrences of the string "XMAS" in the `slices` passed. Occurrences are counted
/// from left to right, right to left, top to bottom, bottom to top, and diagonally from: top-left
/// to bottom-right (and the reverse), and from top-right to bottom-left (and the reverse).
fn count_occurrences_all_directions(needle: &str, slices: &[&str]) -> u32 {
    let t2b = top_to_bottom(slices);
    let tl2br = top_left_to_bottom_right(slices);
    let tr2bl = top_right_to_bottom_left(slices);

    count_occurrences(needle, slices)
        + count_occurrences(
            needle,
            t2b.iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>()
                .as_slice(),
        )
        + count_occurrences(
            needle,
            tl2br
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>()
                .as_slice(),
        )
        + count_occurrences(
            needle,
            tr2bl
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>()
                .as_slice(),
        )
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_reverse() {
        let rtl = reverse(&parse_into_vec(TEST_INPUT));

        assert_eq!(Some(&"MSAMXXSMMM".to_string()), rtl.get(0));
        assert_eq!(Some(&"ASMSMXMASM".to_string()), rtl.get(1));
        assert_eq!(Some(&"MMAAMXSXMA".to_string()), rtl.get(2));
        assert_eq!(Some(&"XMSMSAMASM".to_string()), rtl.get(3));
        assert_eq!(Some(&"MMAXMASAMX".to_string()), rtl.get(4));
        assert_eq!(Some(&"AMAXXMMAXX".to_string()), rtl.get(5));
        assert_eq!(Some(&"SSXSASMSMS".to_string()), rtl.get(6));
        assert_eq!(Some(&"AAASAMAXAS".to_string()), rtl.get(7));
        assert_eq!(Some(&"MMMMXMMMAM".to_string()), rtl.get(8));
        assert_eq!(Some(&"XSAMXAXMXM".to_string()), rtl.get(9));
    }

    #[test]
    fn test_top_to_bottom() {
        let ttb = top_to_bottom(&parse_into_vec(TEST_INPUT));

        assert_eq!(Some(&"MMAMXXSSMM".to_string()), ttb.get(0));
        assert_eq!(Some(&"MSMSMXMAAX".to_string()), ttb.get(1));
        assert_eq!(Some(&"MAXAAASXMM".to_string()), ttb.get(2));
        assert_eq!(Some(&"SMSMSMMAMX".to_string()), ttb.get(3));
        assert_eq!(Some(&"XXXAAMSMMA".to_string()), ttb.get(4));
        assert_eq!(Some(&"XMMSMXAAXX".to_string()), ttb.get(5));
        assert_eq!(Some(&"MSAMXXSSMM".to_string()), ttb.get(6));
        assert_eq!(Some(&"AMASAAXAMA".to_string()), ttb.get(7));
        assert_eq!(Some(&"SSMMMMSAMS".to_string()), ttb.get(8));
        assert_eq!(Some(&"MAMXMASAMX".to_string()), ttb.get(9));
    }

    #[test]
    fn test_top_left_to_bottom_right() {
        let tltbr = top_left_to_bottom_right(&parse_into_vec(TEST_INPUT));

        assert!(tltbr.contains(&"M".to_string()));
        assert!(tltbr.contains(&"MX".to_string()));
        assert!(tltbr.contains(&"SAM".to_string()));
        assert!(tltbr.contains(&"SAMX".to_string()));
        assert!(tltbr.contains(&"XMXMA".to_string()));
        assert!(tltbr.contains(&"XXSAMX".to_string()));
        assert!(tltbr.contains(&"MMAMMXM".to_string()));
        assert!(tltbr.contains(&"ASAMSAMA".to_string()));
        assert!(tltbr.contains(&"MMASMASMS".to_string()));
        assert!(tltbr.contains(&"MSXMAXSAMX".to_string()));
        assert!(tltbr.contains(&"MASAMXXAM".to_string()));
        assert!(tltbr.contains(&"MMXSXASA".to_string()));
        assert!(tltbr.contains(&"SXMMAMS".to_string()));
        assert!(tltbr.contains(&"XMASMA".to_string()));
        assert!(tltbr.contains(&"XSAMM".to_string()));
        assert!(tltbr.contains(&"MMMX".to_string()));
        assert!(tltbr.contains(&"ASM".to_string()));
        assert!(tltbr.contains(&"SA".to_string()));
        assert!(tltbr.contains(&"M".to_string()));
    }

    #[test]
    fn test_top_right_to_bottom_left() {
        let trtbl = top_right_to_bottom_left(&parse_into_vec(TEST_INPUT));

        assert!(trtbl.contains(&"M".to_string()));
        assert!(trtbl.contains(&"MM".to_string()));
        assert!(trtbl.contains(&"MSA".to_string()));
        assert!(trtbl.contains(&"SAMM".to_string()));
        assert!(trtbl.contains(&"XMXSX".to_string()));
        assert!(trtbl.contains(&"XXSAMX".to_string()));
        assert!(trtbl.contains(&"MMXMAXS".to_string()));
        assert!(trtbl.contains(&"ASMASAMS".to_string()));
        assert!(trtbl.contains(&"SMASAMSAM".to_string()));
        assert!(trtbl.contains(&"MSAMMMMXAM".to_string()));
        assert!(trtbl.contains(&"AMSXXSAMX".to_string()));
        assert!(trtbl.contains(&"MMAXAMMM".to_string()));
        assert!(trtbl.contains(&"XMASAMX".to_string()));
        assert!(trtbl.contains(&"MMXSXA".to_string()));
        assert!(trtbl.contains(&"ASAMX".to_string()));
        assert!(trtbl.contains(&"SAMM".to_string()));
        assert!(trtbl.contains(&"AMA".to_string()));
        assert!(trtbl.contains(&"MS".to_string()));
        assert!(trtbl.contains(&"X".to_string()));
    }

    #[test]
    fn test_count_occurrences() {
        assert_eq!(
            3,
            count_occurrences("XMAS", &parse_into_vec("XMASXMASXMAS"))
        );
        assert_eq!(
            2,
            count_occurrences("XMAS", &parse_into_vec("XMASXXAAXXSAMX"))
        );
        assert_eq!(
            4,
            count_occurrences("XMAS", &parse_into_vec("SAMXMASAAASAMXMAS"))
        );
        assert_eq!(5, count_occurrences("XMAS", &parse_into_vec(TEST_INPUT)));
    }

    #[test]
    fn test_count_occurrences_all_directions() {
        assert_eq!(
            18,
            count_occurrences_all_directions("XMAS", &parse_into_vec(TEST_INPUT))
        );
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(do_challenge(TEST_INPUT), 18);
    }
}
