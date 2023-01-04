//! Advent of Code 2022 Day 10
//! https://adventofcode.com/2022/day/10
//!
//! Challenge part 2
//!
//! Executes the program code in the input file on an emulated processor. The processor has only
//! one register and two instruction types. The register's values are used to create a 2D screen
//! and the challenge answer is displayed as multiple capital letters on this screen.

use std::fmt;
use std::fs;

const INPUT_FILENAME: &str = "2022_day10_input.txt";
const SCREEN_HEIGHT: usize = 6;
const SCREEN_WIDTH: usize = 40;

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

/// The display screen, consisting of a 2D grid of pixels, where each pixel can be "lit",
/// represented with '#', or "dark", represented with '.'.
//
// All pixels are stored in a single array. The first pixel of the first row is at index 0, the
// first pixel of the second row is at index `SCREEN_WIDTH`, etc.
#[derive(Debug)]
struct Screen {
    pixels: [char; SCREEN_HEIGHT * SCREEN_WIDTH],
}

impl Screen {
    /// Returns a new `Screen` with all pixels initialized to their unset state, i.e., a period.
    fn new() -> Self {
        Screen {
            pixels: ['.'; SCREEN_HEIGHT * SCREEN_WIDTH],
        }
    }

    /// Determines the position of the pixel to write to the screen based on `cycle`, and if the
    /// 3-pixel wide sprite overlaps this position a lit pixel '#' is written. If not, the dark
    /// pixel '.' that was set when `Screen` was initialized is left unchanged.
    ///
    /// # Panics
    ///
    /// Panics if `cycle` is 0.
    //
    // Note: The code makes no effort to clip the 3-pixel sprite mask when it is at the very
    //       beginning or end of a row, allowing it to spill over. It is unclear from the challenge
    //       if this behavior should be prevented.
    fn write_to_pixel(&mut self, cycle: u32, register: i32) {
        assert!(
            cycle > 0,
            "Internal error: write_to_pixel must be called with a value of cycle > 0"
        );

        let pixel = cycle - 1;

        if (pixel as i32 % SCREEN_WIDTH as i32).abs_diff(register as i32) <= 1 {
            self.pixels[pixel as usize] = '#';
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.pixels.chunks(SCREEN_WIDTH).collect::<Vec<_>>() {
            let write_result = writeln!(f, "{}", row.iter().collect::<String>());
            if write_result.is_err() {
                return write_result;
            }
        }

        Ok(())
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

/// Calculates the challenge answer by running the program and recording the register value at each
/// cycle. These values are then used to write pixels to the `Screen`, which is returned.
fn do_challenge(program: &Vec<Instruction>) -> Screen {
    let mut screen = Screen::new();
    let history = run_program(&program);

    for i in 1..=(SCREEN_HEIGHT * SCREEN_WIDTH) as u32 {
        screen.write_to_pixel(i, history.get_emulator_state_at_cycle(i).register);
    }

    screen
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

    println!(
        "The challenge answer is\n{}",
        do_challenge(&program).to_string()
    );
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

    const EXPECTED_SCREEN_BLANK: &str = "\
........................................
........................................
........................................
........................................
........................................
........................................
";

    // Test register values given in challenge. There is no cycle #0, so the first value should
    // not be used.
    const REGISTER_ON_CYCLE: [i32; 21] = [
        0, 1, 1, 16, 16, 5, 5, 11, 11, 8, 8, 13, 13, 12, 12, 4, 4, 17, 17, 21, 21,
    ];

    // Values for the first row of the screen as it is being populated, as given in challenge.
    // There is no cycle #0, so the first string should not be used.
    const EXPECTED_ON_CYCLE: [&str; 21] = [
        "........................................", // Not used as no cycle #0
        "#.......................................",
        "##......................................",
        "##......................................",
        "##......................................",
        "##..#...................................",
        "##..##..................................",
        "##..##..................................",
        "##..##..................................",
        "##..##..#...............................",
        "##..##..##..............................", // Cycle 10
        "##..##..##..............................",
        "##..##..##..............................",
        "##..##..##..#...........................",
        "##..##..##..##..........................",
        "##..##..##..##..........................",
        "##..##..##..##..........................",
        "##..##..##..##..#.......................",
        "##..##..##..##..##......................",
        "##..##..##..##..##......................",
        "##..##..##..##..##......................", // Cycle 20
    ];

    const EXPECTED_SCREEN_IMAGE: &str = "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
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
    fn test_screen_blank() {
        let screen = Screen::new();

        assert_eq!(screen.to_string(), EXPECTED_SCREEN_BLANK);
    }

    #[test]
    fn test_screen() {
        let mut screen = Screen::new();

        for i in 1..=20 {
            println!("Cycle {}", i);
            screen.write_to_pixel(i, REGISTER_ON_CYCLE[i as usize]);
            assert_eq!(
                &screen.to_string()[0..SCREEN_WIDTH],
                EXPECTED_ON_CYCLE[i as usize]
            );
        }
    }

    #[test]
    fn test_screen_with_emulator() {
        let mut screen = Screen::new();
        let program = parse_input(&TEST_PROGRAM_1);
        let history = run_program(&program);

        for i in 1..=20 {
            println!("Cycle {}", i);
            screen.write_to_pixel(i, history.get_emulator_state_at_cycle(i).register);
            assert_eq!(
                &screen.to_string()[0..SCREEN_WIDTH],
                EXPECTED_ON_CYCLE[i as usize]
            );
        }

        for i in 21..=(SCREEN_HEIGHT * SCREEN_WIDTH) as u32 {
            println!("Cycle {}", i);
            screen.write_to_pixel(i, history.get_emulator_state_at_cycle(i).register);
        }

        assert_eq!(screen.to_string(), EXPECTED_SCREEN_IMAGE);
    }

    #[test]
    fn test_do_challenge() {
        let program = parse_input(&TEST_PROGRAM_1);

        assert_eq!(do_challenge(&program).to_string(), EXPECTED_SCREEN_IMAGE);
    }
}
