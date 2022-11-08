//! Advent of Code 2021 Day 10
//! https://adventofcode.com/2021/day/10
//!
//! Challenge part 1
//!
//! Read a file of opening and closing symbols and determine which lines are corrupt or
//! incomplete. Corrupt lines are scored and a total score is returned as the challenge answer.

use std::fs;

const INPUT_FILENAME: &str = "2021_day10_input.txt";
const OPENERS: &str = "([{<";
const CLOSERS: &str = ")]}>";

const SCORE_PARENTHESIS: u32 = 3;
const SCORE_BRACKET: u32 = 57;
const SCORE_BRACE: u32 = 1197;
const SCORE_ANGLE_BRACKET: u32 = 25137;

#[derive(Debug, PartialEq)]
enum Validity {
    Corrupted(char),
    Incomplete,
    Valid,
}

/// Validates a single line to determine if every closing symbol has a corresponding opening
/// symbol. If a closing symbol that has no matching opening symbol is found, the line is
/// considered corrupt. If no such discrepancies are found, but the end of line is reached before
/// all opening symbols have corresponding closing symbols, the line is considered incomplete.
///
/// # Panics
///
/// Panics if an unexpected symbol is found in the input.
fn validate_line(line: &str) -> Validity {
    let mut stack = Vec::new();

    for c in line.chars() {
        if OPENERS.contains(c) {
            stack.push(c);
        } else {
            if CLOSERS.contains(c) {
                if let Some(opening) = stack.pop() {
                    if ((opening == '(') && (c != ')'))
                        || ((opening == '[') && (c != ']'))
                        || ((opening == '{') && (c != '}'))
                        || ((opening == '<') && (c != '>'))
                    {
                        return Validity::Corrupted(c);
                    }
                } else {
                    // Stack is empty, so there is no matching opening symbol.
                    return Validity::Corrupted(c);
                }
            } else {
                panic!("Unexpected symbol '{}' found in input", c);
            }
        }
    }

    if stack.is_empty() {
        Validity::Valid
    } else {
        Validity::Incomplete
    }
}

/// Return the scoring value of the bad closing character passed.
fn score_bad_closer(c: char) -> u32 {
    match c {
        ')' => {
            return SCORE_PARENTHESIS;
        }
        ']' => {
            return SCORE_BRACKET;
        }
        '}' => {
            return SCORE_BRACE;
        }
        '>' => {
            return SCORE_ANGLE_BRACKET;
        }
        _ => {
            panic!("Unrecognized closing symbol '{}'", c);
        }
    }
}

/// Validate each line of the input file, scoring only corrupted lines based on the first corrupt
/// character.
fn score_corrupted_lines(input: &str) -> u32 {
    let mut total = 0;

    for line in input.lines() {
        if line == "" {
            continue;
        }

        let result = validate_line(&line);
        if let Validity::Corrupted(bad_closer) = result {
            // println!("Line '{}' is corrupted due to closing symbol '{}'", &line, bad_closer);
            total += score_bad_closer(bad_closer);
        }
    }
    total
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    println!(
        "The total score for all corrupted lines in the input files is {}",
        score_corrupted_lines(&input_file)
    );
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    const TEST_LINE_0: &str = "{([(<{}[<>[]}>{[]{[(<()>";
    const TEST_LINE_1: &str = "[[<[([]))<([[{}[[()]]]";
    const TEST_LINE_2: &str = "[{[{({}]{}}([{[{{{}}([]";
    const TEST_LINE_3: &str = "[<(<(<(<{}))><([]([]()";
    const TEST_LINE_4: &str = "<{([([[(<>()){}]>(<<{{";

    #[test]
    fn test_corrupted_lines() {
        assert_eq!(validate_line(&TEST_LINE_0), Validity::Corrupted('}'));
        assert_eq!(validate_line(&TEST_LINE_1), Validity::Corrupted(')'));
        assert_eq!(validate_line(&TEST_LINE_2), Validity::Corrupted(']'));
        assert_eq!(validate_line(&TEST_LINE_3), Validity::Corrupted(')'));
        assert_eq!(validate_line(&TEST_LINE_4), Validity::Corrupted('>'));
    }

    #[test]
    fn test_score_corrupted_lines() {
        assert_eq!(score_corrupted_lines(&TEST_INPUT), 26397);
    }

    #[test]
    #[should_panic]
    fn test_invalid_input() {
        validate_line("a");
    }
}
