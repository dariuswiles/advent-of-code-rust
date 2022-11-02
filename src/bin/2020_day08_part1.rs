//! Advent of Code 2020 Day 08
//! https://adventofcode.com/2020/day/8
//!
//! Challenge part 1
//!
//! Parse a program in a simple language, and execute it to determine the point at which it runs an
//! instruction twice, indicating the beginning of an infinite loop. When this happens, return the
//! contents of the accumulator register.

use std::fs;

const INPUT_FILENAME: &str = "2020_day08_input.txt";

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn parse_program(code: &str) -> Self {
        let mut instructions = Vec::new();

        for line in code.lines() {
            // println!("Parsing line: {}", &line);

            if line == "" {
                println!("\tSkipping blank line");
                continue;
            }

            let tokens: Vec<&str> = line.split(" ").collect();

            if tokens.len() != 2 {
                let error_message = format!("Malformed program code: {}", &line);
                panic!("{}", error_message);
            }

            match tokens[0] {
                "acc" => {
                    // println!("Found: acc with operand {}", tokens[1]);
                    instructions.push(Instruction::Acc(tokens[1].parse().unwrap()));
                }
                "jmp" => {
                    // println!("Found: jmp with operand {}", tokens[1]);
                    instructions.push(Instruction::Jmp(tokens[1].parse().unwrap()));
                }
                "nop" => {
                    // println!("Found: nop with operand {}", tokens[1]);
                    instructions.push(Instruction::Nop(tokens[1].parse().unwrap()));
                }
                _ => {
                    let error_message = format!("Unrecognized instruction in code: {}", &line);
                    panic!("{}", error_message);
                }
            }
        }

        Self {
            instructions: instructions,
        }
    }

    /// Executes given instruction and updates the accumulator `acc`, if necessary. Returns the
    /// offset to the next instruction, with 1 meaning the instruction following this one in the
    /// program code should be executed next.
    fn execute_instruction(i: Instruction, acc: &mut i32) -> i32 {
        let mut offset = 1;
        match i {
            Instruction::Acc(delta) => {
                *acc += delta;
                // println!("Executing: acc with operand {}. Now, `acc`={}", delta, *acc);
            }
            Instruction::Jmp(o) => {
                // println!("Executing: jmp with operand {}", o);
                offset = o;
            }
            Instruction::Nop(_) => {
                // println!("Executing: nop");
            }
        }
        offset
    }

    fn run_until_infinite_loop(&mut self) -> i32 {
        let mut ip = 0;
        let mut acc = 0;
        let program_length = self.instructions.len();
        let mut run = Vec::with_capacity(program_length);
        run.resize(program_length, false);

        while !run[ip] {
            run[ip] = true;
            // println!("Before executing instruction, `ip`={} and `acc`={}", ip, acc);
            let offset = Program::execute_instruction(self.instructions[ip], &mut acc);
            ip = (ip as i32 + offset) as usize;
            // println!("After executing instruction, `ip`={} and `acc`={}\n", ip, acc);
        }

        return acc;
    }
}

fn main() {
    let program_code = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let mut program = Program::parse_program(&program_code);
    let result = program.run_until_infinite_loop();

    println!(
        "Contents of accumulator `acc` at the point the program repeats is {}",
        result
    );
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PROGRAM: &str = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";

    #[test]
    fn test_program_0() {
        let mut program = Program::parse_program(&TEST_PROGRAM);
        println!("{:#?}", program);

        let result = program.run_until_infinite_loop();

        assert_eq!(result, 5);
    }
}
