//! Advent of Code 2020 Day 06
//! https://adventofcode.com/2020/day/6
//!
//! Challenge part 1
//!
//! For each block of data in the input file, determine the unique set of questions that were
//! answered (labelled `a` to `z`). Sum the totals to obtain the answer to the challenge.

use std::collections::HashSet;
use std::fs;

const INPUT_FILENAME: &str = "2020_day06_input.txt";


/// A set of questions, with each character being represented by a `char`.
#[derive(Debug)]
struct QuestionSet {
    questions: HashSet<char>,
}

impl QuestionSet {
    fn new() -> Self {
        Self { questions: HashSet::new(), }
    }

    /// Treats each `char` in the given string as a separate question, and stores each in the
    /// question set. A set is used to eliminate duplicate questions.
    fn add_string_of_questions(&mut self, new_questions: &str) {
        for q in new_questions.chars() {
            self.questions.insert(q);
        }
    }

    /// Return the number of questions currently in the question set.
    fn count_unique_questions(&self) -> usize {
        self.questions.len()
    }

    /// Return an ordered string of the questions in `self`.
    #[allow(dead_code)]
    fn as_string(&self) -> String {
        let mut q_vec: Vec<&char> = self.questions.iter().collect();
        q_vec.sort_unstable();
        q_vec.into_iter().collect()
    }
}


fn parse_question_sets(input: &str) -> Vec<QuestionSet> {
    let mut question_vec = Vec::new();
    let mut question_set = QuestionSet::new();

    for line in input.lines() {
        if line == "" {
            question_vec.push(question_set);
            question_set = QuestionSet::new();
        } else {
            question_set.add_string_of_questions(&line);
        }
    }
    question_vec.push(question_set);
    question_vec
}


fn main() {
    let input =
        fs::read_to_string(INPUT_FILENAME)
            .expect("Error reading input file");

    let mut total = 0;
    for qs in parse_question_sets(&input) {
//         println!("{:?} = {}", qs.as_string(), qs.count_unique_questions());
        total += qs.count_unique_questions();
    }

    println!("Sum of question counts is {}", total);
}


// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const QUESTION_SET_0: &str = "abc";

    const QUESTION_SET_1: &str = "\
a
b
c";

    const QUESTION_SET_2: &str = "\
ab
ac";

    const QUESTION_SET_3: &str = "\
a
a
a
a";

    const QUESTION_SET_4: &str = "b";


    #[test]
    fn set_0() {
        assert_eq!(parse_question_sets(&QUESTION_SET_0)[0].as_string(), "abc");
    }

    #[test]
    fn set_1() {
        assert_eq!(parse_question_sets(&QUESTION_SET_1)[0].as_string(), "abc");
    }

    #[test]
    fn set_2() {
        assert_eq!(parse_question_sets(&QUESTION_SET_2)[0].as_string(), "abc");
    }

    #[test]
    fn set_3() {
        assert_eq!(parse_question_sets(&QUESTION_SET_3)[0].as_string(), "a");
    }

    #[test]
    fn set_4() {
        assert_eq!(parse_question_sets(&QUESTION_SET_4)[0].as_string(), "b");
    }
}
