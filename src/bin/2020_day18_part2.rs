//! Advent of Code 2020 Day 18
//! https://adventofcode.com/2020/day/18
//!
//! Challenge part 2
//!
//! Evaluate math expressions where add has a higher order of precedence than multiply.

use std::fs;

const INPUT_FILENAME: &str = "2020_day18_input.txt";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Token {
    Add,
    Multiply,
    Number(u64),
    SubExpression(Vec<Token>),
}

fn tokenize(chars: &mut Vec<char>) -> Vec<Token> {
    let mut output = Vec::new();

    loop {
        if chars.is_empty() {
            return output;
        }

        let c = chars.remove(0);
        match c {
            ' ' => {}
            '(' => {
                output.push(Token::SubExpression(tokenize(chars)));
            }
            ')' => {
                return output;
            }
            '+' => {
                output.push(Token::Add);
            }
            '*' => {
                output.push(Token::Multiply);
            }
            _ => {
                if c.is_ascii_digit() {
                    output.push(Token::Number(c.to_digit(10).unwrap() as u64));
                } else {
                    panic!("Input contains unexpected character '{}'", c);
                }
            }
        }
    }
}

// To simulate the operator precedence specified in the challenge, evaluate the vector of tokens in
// multiple passes.
fn evaluate_tokens(tokens: &mut Vec<Token>) -> u64 {
    // Pass 1 - replace sub-expressions with the result of evaluating them.
    for t in tokens.iter_mut() {
        if let Token::SubExpression(sub) = t {
            *t = Token::Number(evaluate_tokens(&mut (*sub).to_vec()));
        }
    }

    // Pass 2 - evaluate '+' and replace each instance with the sum of the two numbers in the
    // adjacent vectors.
    let mut i = 0;
    while i < tokens.len() {
        if let Token::Add = &tokens[i] {
            if let Token::Number(left) = tokens[i - 1] {
                if let Token::Number(right) = tokens[i + 1] {
                    tokens.splice(i - 1..=i + 1, vec![Token::Number(left + right)]);
                    i -= 1;
                }
            }
        }
        i += 1;
    }

    // Pass 3 - evaluate '*'.
    tokens.iter().fold(1, |acc, t| match t {
        Token::Number(num) => acc * num,
        Token::Multiply => acc,
        _ => {
            panic!("Unexpected token found during last evaluation pass");
        }
    })
}

fn evaluate(expression: &str) -> u64 {
    let mut c = expression.chars().collect();
    let tokens = tokenize(&mut c);

    // println!("{:#?}", &tokens);

    evaluate_tokens(&mut tokens.clone())
}

fn do_challenge(input: &str) -> u64 {
    input.lines().fold(0, |acc, line| acc + evaluate(line))
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let answer = do_challenge(&input_file);
    println!("The answer to the challenge is {:?}", answer);
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_0: &str = "1 + 2 * 3 + 4 * 5 + 6";
    const TEST_INPUT_1: &str = "1 + (2 * 3) + (4 * (5 + 6))";
    const TEST_INPUT_2: &str = "2 * 3 + (4 * 5)";
    const TEST_INPUT_3: &str = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    const TEST_INPUT_4: &str = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    const TEST_INPUT_5: &str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    #[test]
    fn test_evaluate_0() {
        assert_eq!(evaluate(TEST_INPUT_0), 231);
    }

    #[test]
    fn test_evaluate_1() {
        assert_eq!(evaluate(TEST_INPUT_1), 51);
    }

    #[test]
    fn test_evaluate_2() {
        assert_eq!(evaluate(TEST_INPUT_2), 46);
    }

    #[test]
    fn test_evaluate_3() {
        assert_eq!(evaluate(TEST_INPUT_3), 1445);
    }

    #[test]
    fn test_evaluate_4() {
        assert_eq!(evaluate(TEST_INPUT_4), 669060);
    }

    #[test]
    fn test_evaluate_5() {
        assert_eq!(evaluate(TEST_INPUT_5), 23340);
    }
}
