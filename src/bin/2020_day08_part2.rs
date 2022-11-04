//! Advent of Code 2020 Day 08
//! https://adventofcode.com/2020/day/8
//!
//! Challenge part 2
//!
//! Parse a program in a simple language, and execute it to determine the point at which it runs an
//! instruction twice, indicating the beginning of an infinite loop. When this happens, stop and
//! examine all the instructions executed to see which instruction can be changed to allow the
//! program to terminate without entering an infinite loop. Then execute the modified program to
//! determine its output.

use std::fs;

const INPUT_FILENAME: &str = "2020_day08_input.txt";

#[derive(Clone, Copy, Debug, PartialEq)]
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

    /// Run a potentially modified version of the program. If `modify_line` is not `None` it
    /// indicates which line in the program (with the first line being 0), should be switched. As
    /// per the challenge instructions, this involves changing a `jmp` statement to a `nop` and
    /// vice versa, but `acc` instructions remain unchanged.
    /// If the program goes into an infinite loop, return `None`. If it terminates successfully,
    /// return the content of the accumulator `acc` at that point.
    fn correct_and_run_program(&self) -> i32 {
        let mut ip = 0;
        let mut acc = 0;
        let program_length = self.instructions.len();
        let mut initial_run = Vec::with_capacity(program_length);
        initial_run.resize(program_length, false);

        while !initial_run[ip] {
            initial_run[ip] = true;
            // println!("Before executing instruction, `ip`={} and `acc`={}", ip, acc);
            let offset = Program::execute_instruction(self.instructions[ip], &mut acc);
            ip = (ip as i32 + offset) as usize;
            // println!("After executing instruction, `ip`={} and `acc`={}\n", ip, acc);
        }

        for line in 0..program_length {
            if !initial_run[line] {
                // println!("\nNot modifying line {} because it is never run", line);
                continue;
            }

            if let Instruction::Acc(_) = self.instructions[line] {
                // println!("\nNot modifying line {} because it is an `acc` instruction", line);
                continue;
            }

            // println!("\n** Running program with modified instruction on line {} **", line);

            ip = 0;
            acc = 0;
            let mut run = Vec::with_capacity(program_length);
            run.resize(program_length, false);
            while !run[ip] {
                run[ip] = true;
                // println!("Before executing instruction, `ip`={} and `acc`={}", ip, acc);

                let mut instruction = self.instructions[ip];
                if ip == line {
                    instruction = match instruction {
                        Instruction::Acc(_) => {
                            panic!("Internal error: should never modify an `acc` instruction");
                        }
                        Instruction::Jmp(o) => Instruction::Nop(o),
                        Instruction::Nop(o) => Instruction::Jmp(o),
                    };
                }

                let offset = Program::execute_instruction(instruction, &mut acc);
                ip = (ip as i32 + offset) as usize;
                // println!("After executing instruction {:?}, `ip`={} and `acc`={}",
                //     instruction, ip, acc
                // );

                // Check for successful program termination
                if ip >= program_length {
                    // println!("Program terminated successfully with `ip`={} and `acc`={}", ip, acc);
                    return acc;
                }
            }
        }
        panic!("No modifications to program instructions result in successful program run");
    }
}

fn main() {
    let program_code = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let program = Program::parse_program(&program_code);
    let result = program.correct_and_run_program();

    println!(
        "Contents of accumulator `acc` at time corrected program terminates is {}",
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
    fn test_program() {
        let program = Program::parse_program(&TEST_PROGRAM);
        println!("{:#?}", program);

        let result = program.correct_and_run_program();

        assert_eq!(result, 8);
    }
}
