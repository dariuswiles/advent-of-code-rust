//! Advent of Code 2021 Day 10
//! https://adventofcode.com/2021/day/10
//!
//! Challenge part 2
//!
//! Reads a file of opening and closing symbols and determines if each line is corrupt or
//! incomplete. Incomplete lines are scored based on the symbols that must be added to complete
//! them and score is calculated based on these symbols. The median score is the challenge answer.

use std::fs;

const INPUT_FILENAME: &str = "2021_day10_input.txt";
const OPENERS: &str = "([{<";
const CLOSERS: &str = ")]}>";

const SCORE_PARENTHESIS: u64 = 1;
const SCORE_BRACKET: u64 = 2;
const SCORE_BRACE: u64 = 3;
const SCORE_ANGLE_BRACKET: u64 = 4;

#[derive(Debug, PartialEq)]
enum Validity {
    Corrupted(char),
    Incomplete(Vec<char>),
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
                    if ((opening == '(') & (c != ')')) |
                        ((opening == '[') & (c != ']')) |
                        ((opening == '{') & (c != '}')) |
                        ((opening == '<') & (c != '>')) {
                        return Validity::Corrupted(c);
                    }
                } else {    // Stack is empty, so there is no matching opening symbol.
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
        Validity::Incomplete(stack)
    }
}


/// Calculates a score based on the symbols required to complete the line.
//
// The stack passed contains unmatched symbols in the order they were opened, so this needs to be
// reversed so the order is correct for closing. For simplicity, the opening symbols are used
// to avoid the need to translate them to closing symbols.
fn score_incomplete(stack: &Vec <char>) -> u64 {
    let mut score = 0;

    let mut reversed_stack = stack.clone();
    reversed_stack.reverse();
    for c in reversed_stack {
        score = score * 5 + match c {
            '(' => { SCORE_PARENTHESIS }
            '[' => { SCORE_BRACKET }
            '{' => { SCORE_BRACE }
            '<' => { SCORE_ANGLE_BRACKET }
            _ => { panic!("Unrecognized symbol '{}' found on stack", c); }
        };
    }

    score
}


/// Validates each line of the input file, scoring only incomplete lines based on the symbols
/// required to complete the line. The scores for all incomplete lines are sorted and the
/// median score returned.
fn score_bad_lines(input: &str) -> u64 {
    let mut scores = Vec::new();

    for line in input.lines() {
        if line == "" {
            continue;
        }

        let result = validate_line(&line);

        if let Validity::Incomplete(stack) = result {
//             println!("Line '{}' is incomplete due to missing symbols '{:?}'", &line, &stack);
            scores.push(score_incomplete(&stack));
        }
    }

    scores.sort_unstable();
    scores[(scores.len() - 1) / 2]
}


fn main() {
    let input_file =
        fs::read_to_string(INPUT_FILENAME)
            .expect("Error reading input file");

    println!("The total score for all corrupted lines in the input files is {}",
        score_bad_lines(&input_file)
    );
}


// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

    const TEST_LINE_0: &str = r#"[({(<(())[]>[[{[]{<()<>>"#;
    const TEST_LINE_1: &str = r#"[(()[<>])]({[<{<<[]>>("#;
    const TEST_LINE_2: &str = r#"(((({<>}<{<{<>}{[]{[]{}"#;
    const TEST_LINE_3: &str = r#"{<[[]]>}<{[{[{[]{()[[[]"#;
    const TEST_LINE_4: &str = r#"<{([{{}}[<[[[<>{}]]]>[]]"#;

    // Note that the expected output differs from the challenge test output because the stack
    // contains opening symbols only, so closing symbols need swapping for opening symbols. Also
    // the stack is a FIFO, so needs to be reversed to see the correct order of symbols required
    // to complete a line.
    #[test]
    fn test_incomplete_lines() {
        assert_eq!(validate_line(&TEST_LINE_0),
            Validity::Incomplete("{{[[({([".chars().rev().collect())
        );

        assert_eq!(validate_line(&TEST_LINE_1),
            Validity::Incomplete("({<[{(".chars().rev().collect())
        );

        assert_eq!(validate_line(&TEST_LINE_2),
            Validity::Incomplete("{{<{<((((".chars().rev().collect())
        );

        assert_eq!(validate_line(&TEST_LINE_3),
            Validity::Incomplete("[[{{[{[{<".chars().rev().collect())
        );

        assert_eq!(validate_line(&TEST_LINE_4),
            Validity::Incomplete("[({<".chars().rev().collect())
        );
    }

    #[test]
    fn test_score_incomplete() {
        assert_eq!(score_incomplete(&"{{[[({([".chars().rev().collect()), 288957);
        assert_eq!(score_incomplete(&"({<[{(".chars().rev().collect()), 5566);
        assert_eq!(score_incomplete(&"{{<{<((((".chars().rev().collect()), 1480781);
        assert_eq!(score_incomplete(&"[[{{[{[{<".chars().rev().collect()), 995444);
        assert_eq!(score_incomplete(&"[({<".chars().rev().collect()), 294);
    }

    #[test]
    fn test_score_bad_lines() {
        assert_eq!(score_bad_lines(&TEST_INPUT), 288957);
    }

    #[test]
    #[should_panic]
    fn test_invalid_input() {
        validate_line("a");
    }
}
