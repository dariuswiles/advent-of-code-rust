//! Advent of Code 2020 Day 04
//! https://adventofcode.com/2020/day/4
//!
//! Challenge part 1
//!
//! Read passport information and display the number of valid passports.

use std::fs;

const INPUT_FILENAME: &str = "2020_day04_input.txt";

#[derive(Debug, Default)]
struct Passport<'a> {
    byr: Option<&'a str>, // Birth Year
    iyr: Option<&'a str>, // Issue Year
    eyr: Option<&'a str>, // Expiration Year
    hgt: Option<&'a str>, // Height
    hcl: Option<&'a str>, // Hair Color
    ecl: Option<&'a str>, // Eye Color
    pid: Option<&'a str>, // Passport ID
    cid: Option<&'a str>, // Country ID
}

impl Passport<'_> {
    /// Returns `true` if all mandatory passport fields have data, `false` otherwise. All fields
    /// are mandatory except `cid`.
    fn is_valid(&self) -> bool {
        // println!("{:?}", &self);

        self.byr.is_some()
            & self.iyr.is_some()
            & self.eyr.is_some()
            & self.hgt.is_some()
            & self.hcl.is_some()
            & self.ecl.is_some()
            & self.pid.is_some()
    }
}

/// Return the number of valid passports in `input` using the validity rules specified in the
/// challenge.
fn count_valid_passports(input: &str) -> u32 {
    let mut valid_passport_count = 0;

    let mut current_passport = Passport::default();
    for (line_num, line) in input.lines().enumerate() {
        // println!("{:?}", &line);

        if line.is_empty() {
            // A blank line indicates the end of all data for the current passport.
            if current_passport.is_valid() {
                valid_passport_count += 1;
                // println!("Passport is valid");
            }

            current_passport = Passport::default();
        } else {
            let line_fields = line.split(' ');

            for f in line_fields {
                let field_parts: Vec<&str> = f.split(':').collect();

                match field_parts[0] {
                    "byr" => {
                        current_passport.byr = Some(field_parts[1]);
                    }
                    "iyr" => {
                        current_passport.iyr = Some(field_parts[1]);
                    }
                    "eyr" => {
                        current_passport.eyr = Some(field_parts[1]);
                    }
                    "hgt" => {
                        current_passport.hgt = Some(field_parts[1]);
                    }
                    "hcl" => {
                        current_passport.hcl = Some(field_parts[1]);
                    }
                    "ecl" => {
                        current_passport.ecl = Some(field_parts[1]);
                    }
                    "pid" => {
                        current_passport.pid = Some(field_parts[1]);
                    }
                    "cid" => {
                        current_passport.cid = Some(field_parts[1]);
                    }
                    _ => {
                        panic!("Found unexpected passport field on input line {}", line_num);
                    }
                }
            }
        }
    }

    // In case input does not end with a blank line, check for a valid passport when we reach the
    // end of the input file.
    if current_passport.is_valid() {
        valid_passport_count += 1;
    }

    valid_passport_count
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    println!("{} passports are valid", count_valid_passports(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_0: &str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";

    const INPUT_1: &str = "\
iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929";

    const INPUT_2: &str = "\
hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm";

    const INPUT_3: &str = "\
hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn test_0_valid() {
        assert_eq!(count_valid_passports(INPUT_0), 1);
    }

    #[test]
    fn test_1_invalid() {
        assert_eq!(count_valid_passports(INPUT_1), 0);
    }

    #[test]
    fn test_2_valid() {
        assert_eq!(count_valid_passports(INPUT_2), 1);
    }

    #[test]
    fn test_3_invalid() {
        assert_eq!(count_valid_passports(INPUT_3), 0);
    }
}
