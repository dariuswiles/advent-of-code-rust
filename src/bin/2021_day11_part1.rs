//! Advent of Code 2021 Day 11
//! https://adventofcode.com/2021/day/11
//!
//! Challenge part 1
//!
//! Simulate a group of octopuses as they gain energy and flash each cycle. Determine the total
//! flashes after a given number of iterations.

use std::fs;

const INPUT_FILENAME: &str = "2021_day11_input.txt";
const GRID_SIZE: usize = 10;
const FLASH_PROCESSED: EnergyLevel = 100;

type EnergyLevel = u8;

#[derive(Debug, PartialEq)]
struct Grid {
    octopus: Vec<Vec<EnergyLevel>>,
}

impl Grid {
    /// Creates a new `Grid` of octopuses from an input string.
    ///
    /// # Panics
    ///
    /// Panics if the input does not contain GRID_SIZE rows and columns.
    fn new(input: &str) -> Self {
        let mut octopus = Vec::new();

        for line in input.lines() {
            if line.is_empty() {
                continue;
            }

            if line.len() != GRID_SIZE {
                panic!("All input lines must contain {} octopuses", GRID_SIZE);
            }

            octopus.push(
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as EnergyLevel)
                    .collect(),
            );
        }

        if octopus.len() != GRID_SIZE {
            panic!("There must be exactly {} lines of octopuses", GRID_SIZE);
        }

        Self { octopus }
    }

    /// Increments the energy levels of all octopuses surrounding the one at the position defined
    /// by `row` and `col`.
    fn increment_adjacent_octopuses(&mut self, row: usize, col: usize) {
        let mut row_start = row;
        if row > 0 {
            row_start = row - 1;
        }

        let mut col_start = col;
        if col > 0 {
            col_start = col - 1;
        }

        let row_end = std::cmp::min(GRID_SIZE - 1, row + 1);
        let col_end = std::cmp::min(GRID_SIZE - 1, col + 1);

        for r in row_start..=row_end {
            for c in col_start..=col_end {
                self.octopus[r][c] += 1;
            }
        }

        // Undo the unnecessary increment of the octopus in the middle.
        self.octopus[row][col] -= 1;
    }

    /// Performs a single step of increasing the energy level of all octopuses and handling
    /// the flashing that results. Returns the number of octopuses that flashed.
    fn simulate_step(&mut self) -> u32 {
        // Increment energy levels.
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                self.octopus[row][col] += 1;
            }
        }

        let mut flashes_this_step = 0;
        let mut flashes_this_round; // A 'round' is once through the following loop.

        // Loop until all flashes have been processed.
        loop {
            flashes_this_round = 0;

            for row in 0..GRID_SIZE {
                for col in 0..GRID_SIZE {
                    let energy = &mut self.octopus[row][col];
                    if *energy > 9 && *energy < FLASH_PROCESSED {
                        *energy += FLASH_PROCESSED;
                        flashes_this_round += 1;
                        self.increment_adjacent_octopuses(row, col);
                    }
                }
            }
            flashes_this_step += flashes_this_round;

            if flashes_this_round == 0 {
                break;
            }
        }

        // Reset the energy level of octopuses that flashed during this step.
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if self.octopus[row][col] > 9 {
                    self.octopus[row][col] = 0;
                }
            }
        }

        flashes_this_step
    }

    /// Performs the given number of steps and returns the total number of octopus flashes.
    fn simulate_steps(&mut self, steps: usize) -> u32 {
        let mut total = 0;

        for _ in 0..steps {
            total += self.simulate_step();
        }
        total
    }
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let mut grid = Grid::new(&input_file);

    println!("The total number of flashes {}", grid.simulate_steps(100));
}

// Test using data from the examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    const TEST_INPUT_BAD_LINE_LENGTH: &str = "\
5483143223
27458
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    const AFTER_STEP_1: &str = "\
6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637";

    const AFTER_STEP_2: &str = "\
8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848";

    const AFTER_STEP_3: &str = "\
0050900866
8500800575
9900000039
9700000041
9935080063
7712300000
7911250009
2211130000
0421125000
0021119000";

    const AFTER_STEP_4: &str = "\
2263031977
0923031697
0032221150
0041111163
0076191174
0053411122
0042361120
5532241122
1532247211
1132230211";

    const AFTER_STEP_5: &str = "\
4484144000
2044144000
2253333493
1152333274
1187303285
1164633233
1153472231
6643352233
2643358322
2243341322";

    const AFTER_STEP_6: &str = "\
5595255111
3155255222
3364444605
2263444496
2298414396
2275744344
2264583342
7754463344
3754469433
3354452433";

    const AFTER_STEP_7: &str = "\
6707366222
4377366333
4475555827
3496655709
3500625609
3509955566
3486694453
8865585555
4865580644
4465574644";

    const AFTER_STEP_8: &str = "\
7818477333
5488477444
5697666949
4608766830
4734946730
4740097688
6900007564
0000009666
8000004755
6800007755";

    const AFTER_STEP_9: &str = "\
9060000644
7800000976
6900000080
5840000082
5858000093
6962400000
8021250009
2221130009
9111128097
7911119976";

    const AFTER_STEP_10: &str = "\
0481112976
0031112009
0041112504
0081111406
0099111306
0093511233
0442361130
5532252350
0532250600
0032240000";

    const AFTER_STEP_20: &str = "\
3936556452
5686556806
4496555690
4448655580
4456865570
5680086577
7000009896
0000000344
6000000364
4600009543";

    const AFTER_STEP_30: &str = "\
0643334118
4253334611
3374333458
2225333337
2229333338
2276733333
2754574565
5544458511
9444447111
7944446119";

    const AFTER_STEP_40: &str = "\
6211111981
0421111119
0042111115
0003111115
0003111116
0065611111
0532351111
3322234597
2222222976
2222222762";

    const AFTER_STEP_50: &str = "\
9655556447
4865556805
4486555690
4458655580
4574865570
5700086566
6000009887
8000000533
6800000633
5680000538";

    const AFTER_STEP_60: &str = "\
2533334200
2743334640
2264333458
2225333337
2225333338
2287833333
3854573455
1854458611
1175447111
1115446111";

    const AFTER_STEP_70: &str = "\
8211111164
0421111166
0042111114
0004211115
0000211116
0065611111
0532351111
7322235117
5722223475
4572222754";

    const AFTER_STEP_80: &str = "\
1755555697
5965555609
4486555680
4458655580
4570865570
5700086566
7000008666
0000000990
0000000800
0000000000";

    const AFTER_STEP_90: &str = "\
7433333522
2643333522
2264333458
2226433337
2222433338
2287833333
2854573333
4854458333
3387779333
3333333333";

    const AFTER_STEP_100: &str = "\
0397666866
0749766918
0053976933
0004297822
0004229892
0053222877
0532222966
9322228966
7922286866
6789998766";

    #[test]
    fn parse_test_input() {
        let grid = Grid::new(TEST_INPUT);
        assert_eq!(grid.octopus[1][3], 5);
        assert_eq!(grid.octopus[9][8], 2);
    }

    #[test]
    fn after_step_1() {
        let mut grid = Grid::new(TEST_INPUT);
        let flashes = grid.simulate_step();
        assert_eq!(grid, Grid::new(AFTER_STEP_1));
        assert_eq!(flashes, 0);
    }

    #[test]
    fn after_more_steps() {
        let mut grid = Grid::new(TEST_INPUT);

        let mut flashes = grid.simulate_steps(2);
        assert_eq!(grid, Grid::new(AFTER_STEP_2));
        flashes += grid.simulate_step();
        assert_eq!(grid, Grid::new(AFTER_STEP_3));
        flashes += grid.simulate_step();
        assert_eq!(grid, Grid::new(AFTER_STEP_4));
        flashes += grid.simulate_step();
        assert_eq!(grid, Grid::new(AFTER_STEP_5));
        flashes += grid.simulate_step();
        assert_eq!(grid, Grid::new(AFTER_STEP_6));
        flashes += grid.simulate_step();
        assert_eq!(grid, Grid::new(AFTER_STEP_7));
        flashes += grid.simulate_step();
        assert_eq!(grid, Grid::new(AFTER_STEP_8));
        flashes += grid.simulate_step();
        assert_eq!(grid, Grid::new(AFTER_STEP_9));
        flashes += grid.simulate_step();
        assert_eq!(grid, Grid::new(AFTER_STEP_10));
        assert_eq!(flashes, 204);
    }

    #[test]
    fn after_even_more_steps() {
        let mut grid = Grid::new(TEST_INPUT);

        let mut flashes = grid.simulate_steps(20);
        assert_eq!(grid, Grid::new(AFTER_STEP_20));
        flashes += grid.simulate_steps(10);
        assert_eq!(grid, Grid::new(AFTER_STEP_30));
        flashes += grid.simulate_steps(10);
        assert_eq!(grid, Grid::new(AFTER_STEP_40));
        flashes += grid.simulate_steps(10);
        assert_eq!(grid, Grid::new(AFTER_STEP_50));
        flashes += grid.simulate_steps(10);
        assert_eq!(grid, Grid::new(AFTER_STEP_60));
        flashes += grid.simulate_steps(10);
        assert_eq!(grid, Grid::new(AFTER_STEP_70));
        flashes += grid.simulate_steps(10);
        assert_eq!(grid, Grid::new(AFTER_STEP_80));
        flashes += grid.simulate_steps(10);
        assert_eq!(grid, Grid::new(AFTER_STEP_90));
        flashes += grid.simulate_steps(10);
        assert_eq!(grid, Grid::new(AFTER_STEP_100));
        assert_eq!(flashes, 1656);
    }

    #[test]
    #[should_panic]
    fn incorrect_line_lengths() {
        let _ = Grid::new(TEST_INPUT_BAD_LINE_LENGTH);
    }

    #[test]
    #[should_panic]
    fn incorrect_number_of_lines() {
        let _ = Grid::new(&TEST_INPUT_BAD_LINE_LENGTH[..3]);
    }
}
