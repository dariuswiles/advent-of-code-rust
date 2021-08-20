//! Advent of Code 2020 Day 18
//! https://adventofcode.com/2020/day/18
//!
//! Challenge part 1
//!
//! Evaluate math expressions that have an equal order of precedence for all operators, allowing
//! them to be evaluated left-to-right.

use std::fs;

const INPUT_FILENAME: &str = "2020_day18_input.txt";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Token {
    Add,
    Multiply,
    Number(u64),
    SubExpression(Box<Vec<Token>>),
}


fn tokenize(chars: &mut Vec<char>) -> Vec<Token> {
    let mut output = Vec::new();

    loop {
        if chars.len() == 0 {
            return output;
        }

        let c = chars.remove(0);
        match c {
            ' ' => {}
            '(' => {
                output.push(Token::SubExpression(Box::new(tokenize(chars))));
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
                if c.is_digit(10) {
                    output.push(Token::Number(c.to_digit(10).unwrap() as u64));
                } else {
                    panic!("Input contains unexpected character '{}'", c);
                }
            }
        }
    }
}


fn evaluate_tokens(tokens: &mut Vec<Token>) -> u64 {
    let mut total = 0;
    let mut operator = None;

    loop {
        if tokens.len() == 0 {
            return total;
        }

        let t = tokens.remove(0);
        match t {
            Token::SubExpression(mut sub) => {
                tokens.insert(0, Token::Number(evaluate_tokens(&mut sub)));
            }
            Token::Add => {
                if operator == None {
                    operator = Some(Token::Add);
                } else {
                    panic!("Input contains adjacent operators");
                }
            }
            Token::Multiply => {
                if operator == None {
                    operator = Some(Token::Multiply);
                } else {
                    panic!("Input contains adjacent operators");
                }
            }
            Token::Number(n) => {
                match operator {
                    None => {
                        total = n;
                    }
                    Some(op) => {
                        match op {
                            Token::Add => {
                                total += n;
                                operator = None;
                            }
                            Token::Multiply => {
                                total *= n;
                                operator = None;
                            }
                            _ => {
                                panic!("Internal error due to unexpected token operator");
                            }
                        }
                    }
                }
            }
        }
    }
}


fn evaluate(expression: &str) -> u64 {
    let mut c = expression.chars().collect();
    let tokens = tokenize(&mut c);

//     println!("{:#?}", &tokens);

    evaluate_tokens(&mut tokens.clone())
}


fn do_challenge(input: &str) -> u64 {
    input.lines().fold(0, |acc, line| acc + evaluate(line))
}

fn main() {
    let input_file =
        fs::read_to_string(INPUT_FILENAME)
            .expect("Error reading input file");

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
        assert_eq!(evaluate(&TEST_INPUT_0), 71);
    }

    #[test]
    fn test_evaluate_1() {
        assert_eq!(evaluate(&TEST_INPUT_1), 51);
    }

    #[test]
    fn test_evaluate_2() {
        assert_eq!(evaluate(&TEST_INPUT_2), 26);
    }

    #[test]
    fn test_evaluate_3() {
        assert_eq!(evaluate(&TEST_INPUT_3), 437);
    }

    #[test]
    fn test_evaluate_4() {
        assert_eq!(evaluate(&TEST_INPUT_4), 12240);
    }

    #[test]
    fn test_evaluate_5() {
        assert_eq!(evaluate(&TEST_INPUT_5), 13632);
    }
}
