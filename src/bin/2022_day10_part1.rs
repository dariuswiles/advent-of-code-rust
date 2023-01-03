//! Advent of Code 2022 Day 10
//! https://adventofcode.com/2022/day/10
//!
//! Challenge part 1
//!
//! Executes the program code in the input file on an emulated processor. The processor has only
//! one register and two instruction types. The challenge answer requires the value of the
//! register to be observed at given intervals.

use std::fs;

const INPUT_FILENAME: &str = "2022_day10_input.txt";

type AddxOperand = i32;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Instruction {
    Addx(AddxOperand),
    Noop,
}

/// Emulates the processing hardware described in the challenge. `cycle` is the elapsed time and
/// `register` contains the current register value.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Emulator {
    cycle: u32,
    register: i32,
}

impl Emulator {
    fn new() -> Self {
        Self {
            cycle: 1,
            register: 1,
        }
    }

    /// Executes the given instruction, updating the register and cycle count.
    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Addx(operand) => {
                self.register += operand;
                self.cycle += 2;
            }
            Instruction::Noop => {
                self.cycle += 1;
            }
        }
    }
}

/// Maintains a history of a program run.
#[derive(Debug)]
struct History {
    states: Vec<Emulator>,
}

impl History {
    /// Returns new History.
    fn new() -> Self {
        Self { states: Vec::new() }
    }

    /// Copies the passed `emulator` state to the end of internal state history.
    fn save(&mut self, emulator: &Emulator) {
        self.states.push(emulator.clone());
    }

    /// Returns the state of the emulator at `target_cycle`. If `target_cycle` falls within an
    /// instruction that takes two cycles, the emulator state at the time that instruction was
    /// started is returned.
    ///
    /// # Panics
    ///
    /// Panics if `target_cycle` is 0.
    fn get_emulator_state_at_cycle(&self, target_cycle: u32) -> &Emulator {
        let mut previous_state = None;

        for s in &self.states {
            if s.cycle >= target_cycle {
                if s.cycle == target_cycle {
                    return &s;
                } else if previous_state.is_some() {
                    return previous_state.unwrap();
                } else {
                    panic!(
                        "get_emulator_state_at_cycle was passed unexpected parameter {}",
                        target_cycle,
                    );
                }
            }
            previous_state = Some(s);
        }

        &self.states.last().unwrap()
    }
}

/// Executes all `Instruction`s in `program` and returns a vector of the state of the emulator at
/// the beginning of each instruction.
fn run_program(program: &Vec<Instruction>) -> History {
    let mut emulator = Emulator::new();
    let mut history = History::new();

    history.save(&emulator);

    for &instruction in program {
        emulator.execute_instruction(&instruction);
        history.save(&emulator);
    }

    history
}

/// Calculates the challenge answer by running the program, and multiplying the register contents
/// on the cycles given in the challenge. The answer is the sum of the multiplications.
fn do_challenge(program: &Vec<Instruction>) -> i32 {
    let history = run_program(&program);
    let mut cumulative_total = 0;

    for target_cycle in (20..=220).step_by(40) {
        let reg = history.get_emulator_state_at_cycle(target_cycle).register;
        cumulative_total += target_cycle as i32 * reg;
    }

    cumulative_total
}

/// Takes a string containing the entire input file and converts it into a vector of instructions.
/// Each line of input must either:
///     noop
///     addx <signed integer to add>
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> Vec<Instruction> {
    let mut program = Vec::new();

    for line in input.lines() {
        if line != "" {
            if line.starts_with("noop") {
                program.push(Instruction::Noop);
            } else if line.starts_with("addx ") {
                let operand =
                    AddxOperand::from_str_radix(line.strip_prefix("addx ").unwrap().trim(), 10)
                        .unwrap();
                program.push(Instruction::Addx(operand));
            } else {
                panic!("Unrecognized instruction in input");
            }
        }
    }
    program
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    let program = parse_input(&input);

    println!("The challenge answer is {}", do_challenge(&program));
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PROGRAM_0: &str = "\
noop
addx 3
addx -5
";

    const TEST_PROGRAM_1: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn test_parse_input_0() {
        let program = parse_input(&TEST_PROGRAM_0);

        assert_eq!(
            program,
            vec![
                Instruction::Noop,
                Instruction::Addx(3),
                Instruction::Addx(-5),
            ]
        );
    }

    #[test]
    fn test_parse_input_1() {
        let program = parse_input(&TEST_PROGRAM_1);

        assert_eq!(program[0], Instruction::Addx(15));
        assert_eq!(program[28], Instruction::Addx(21));
        assert_eq!(program[78], Instruction::Noop);
        assert_eq!(program[121], Instruction::Addx(-37));
        assert_eq!(program[124], Instruction::Noop);
        assert_eq!(program[142], Instruction::Addx(-11));
    }

    #[test]
    fn test_execute_instruction() {
        let mut emulator = Emulator::new();
        assert_eq!(emulator.cycle, 1);
        assert_eq!(emulator.register, 1);

        emulator.execute_instruction(&Instruction::Noop);
        assert_eq!(emulator.cycle, 2);
        assert_eq!(emulator.register, 1);

        emulator.execute_instruction(&Instruction::Addx(3));
        assert_eq!(emulator.cycle, 4);
        assert_eq!(emulator.register, 4);

        emulator.execute_instruction(&Instruction::Addx(-5));
        assert_eq!(emulator.cycle, 6);
        assert_eq!(emulator.register, -1);
    }

    #[test]
    fn test_run_program() {
        let program = parse_input(&TEST_PROGRAM_0);
        let history = run_program(&program);

        assert_eq!(
            history.states[0],
            Emulator {
                cycle: 1,
                register: 1
            }
        );
        assert_eq!(
            history.states[1],
            Emulator {
                cycle: 2,
                register: 1
            }
        );
        assert_eq!(
            history.states[2],
            Emulator {
                cycle: 4,
                register: 4
            }
        );
        assert_eq!(
            history.states[3],
            Emulator {
                cycle: 6,
                register: -1
            }
        );
    }

    #[test]
    fn test_get_emulator_state_at_cycle_0() {
        let program = parse_input(&TEST_PROGRAM_0);
        let history = run_program(&program);

        assert_eq!(
            history.get_emulator_state_at_cycle(1),
            &Emulator {
                cycle: 1,
                register: 1
            }
        );
        assert_eq!(
            history.get_emulator_state_at_cycle(2),
            &Emulator {
                cycle: 2,
                register: 1
            }
        );
        assert_eq!(
            history.get_emulator_state_at_cycle(3),
            &Emulator {
                cycle: 2,
                register: 1
            }
        );
        assert_eq!(
            history.get_emulator_state_at_cycle(4),
            &Emulator {
                cycle: 4,
                register: 4
            }
        );
        assert_eq!(
            history.get_emulator_state_at_cycle(5),
            &Emulator {
                cycle: 4,
                register: 4
            }
        );
        assert_eq!(
            history.get_emulator_state_at_cycle(6),
            &Emulator {
                cycle: 6,
                register: -1
            }
        );
    }

    #[test]
    #[should_panic]
    fn test_get_emulator_state_at_cycle_panic() {
        let program = parse_input(&TEST_PROGRAM_0);
        let history = run_program(&program);

        history.get_emulator_state_at_cycle(0);
    }

    #[test]
    fn test_get_emulator_state_at_cycle_1() {
        let program = parse_input(&TEST_PROGRAM_1);
        let history = run_program(&program);

        assert_eq!(history.get_emulator_state_at_cycle(20).register, 21);
        assert_eq!(history.get_emulator_state_at_cycle(60).register, 19);
        assert_eq!(history.get_emulator_state_at_cycle(100).register, 18);
        assert_eq!(history.get_emulator_state_at_cycle(140).register, 21);
        assert_eq!(history.get_emulator_state_at_cycle(180).register, 16);
        assert_eq!(history.get_emulator_state_at_cycle(220).register, 18);
    }

    #[test]
    fn test_do_challenge() {
        let program = parse_input(&TEST_PROGRAM_1);

        assert_eq!(do_challenge(&program), 13140);
    }
}
