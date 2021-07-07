//! Advent of Code 2020 Day 04
//! https://adventofcode.com/2020/day/4
//!
//! Challenge part 2
//!
//! Read passport information and display the number of valid passports. Part 2 of the challenge
//! requires validation of the content of the passport fields, rather than just checking whether
//! each field has been provided.

use std::fs;

const INPUT_FILENAME: &str = "2020_day04_input.txt";
const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

#[derive(Debug, Default)]
struct Passport<'a> {
    byr: Option<&'a str>,  // Birth Year
    iyr: Option<&'a str>,  // Issue Year
    eyr: Option<&'a str>,  // Expiration Year
    hgt: Option<&'a str>,  // Height
    hcl: Option<&'a str>,  // Hair Color
    ecl: Option<&'a str>,  // Eye Color
    pid: Option<&'a str>,  // Passport ID
    cid: Option<&'a str>,  // Country ID
}

impl Passport<'_> {
    /// Returns `true` if all mandatory passport fields have data, `false` otherwise. All fields
    /// are mandatory except `cid`.
    fn is_valid(&self) -> bool {
//         println!("{:?}", &self);

        if (self.byr == None) | (self.iyr == None) | (self.eyr == None) | (self.hgt == None)
            | (self.hcl == None) | (self.ecl == None) | (self.pid == None) {
            return false;
        }

        let byr = self.byr.unwrap().parse::<u16>();
        let iyr = self.iyr.unwrap().parse::<u16>();
        let eyr = self.eyr.unwrap().parse::<u16>();

        if byr.is_err() & iyr.is_err() & eyr.is_err() {
//             println!("A date passport field failed validation because it is not a number");
            return false;
        }

        let byr = byr.unwrap();
        let iyr = iyr.unwrap();
        let eyr = eyr.unwrap();

        if (byr < 1920) | (byr > 2002)
            | (iyr < 2010) | (iyr > 2020)
            | (eyr < 2020) | (eyr > 2030)
        {
//             println!("A date passport field failed validation");
            return false;
        }

        let hgt = self.hgt.unwrap();
        if hgt.ends_with("cm") {
            if let Ok(h) = hgt[..hgt.len()-2].parse::<u8>() {
                if (h < 150) | (h > 193) {
//                     println!("Height, given in cm, is outside valid range");
                    return false;
                }
            } else {
//                 println!("Height was given in cm, but a valid number was not found.");
                return false;
            }
        } else if hgt.ends_with("in") {
            if let Ok(h) = hgt[..hgt.len()-2].parse::<u8>() {
                if (h < 59) | (h > 76) {
//                     println!("Height, given in inches, is outside valid range");
                    return false;
                }
            } else {
//                 println!("Height was given in inches, but a valid number was not found.");
                return false;
            }
        } else {
//             println!("Height is invalid as it does not end in 'cm' or 'in'.");
            return false;
        }

        if self.hcl.unwrap().len() == 7 {
            let hcl_chars: Vec<char> = self.hcl.unwrap().chars().collect();

            if hcl_chars[0] != '#' {
//                 println!("'hcl' is invalid as it does not start with a '#' character");
                return false;
            }


            if !hcl_chars[1..].iter().fold(true, |acc, c| acc & c.is_ascii_hexdigit()) {
//                 println!("'hcl' is invalid as it contains a non-hex character");
                return false;
            }
        } else {
//             println!("'hcl' is the incorrect length");
            return false;
        }


        if EYE_COLORS.iter().position(|ec| ec == &self.ecl.unwrap()) == None {
//             println!("Eye color is invalid");
            return false;
        }


        let pid = self.pid.unwrap();
        if pid.len() == 9 {
            if !pid.chars().fold(true, |acc, d| acc & d.is_numeric()) {
//                 println!("'pid' is invalid as it contains a character that is not a digit");
                return false;
            }
        } else {
//             println!("'pid' is the incorrect length");
            return false;
        }

        true
    }


}


/// Return the number of valid passports in `input` using the validity rules specified in the
/// challenge.
fn count_valid_passports(input: &str) -> u32 {
    let mut valid_passport_count = 0;

    let mut current_passport = Passport::default();
    for (line_num, line) in input.lines().enumerate() {
//         println!("{:?}", &line);

        if line == "" {  // A blank line indicates the end of all data for the current passport.
            if current_passport.is_valid() {
                valid_passport_count += 1;
//                 println!("Passport is valid");
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
                        panic!(format!("Found unexpected passport field on input line {}",
                                line_num));
                    }
                }
            }
        }
    }

    // In case input does not end with a blank line, check for a valid passport when we reach the
    // end of the input file.
    if current_passport.is_valid() {
//         println!("Passport is valid");
        valid_passport_count += 1;
    }

    valid_passport_count
}


fn main() {
    let input =
        fs::read_to_string(INPUT_FILENAME)
            .expect("Error reading input file");

    println!("{} passports are valid", count_valid_passports(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    const INVALID_0: &str = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926";

    const INVALID_1: &str = "\
iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946";

    const INVALID_2: &str = "\
hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277";

    const INVALID_3: &str = "\
hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    const VALID_0: &str = "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f";

    const VALID_1: &str = "\
eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm";

    const VALID_2: &str = "\
hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022";

    const VALID_3: &str = "\
iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn invalid_0() {
        assert_eq!(count_valid_passports(&INVALID_0), 0);
    }

    #[test]
    fn invalid_1() {
        assert_eq!(count_valid_passports(&INVALID_1), 0);
    }

    #[test]
    fn invalid_2() {
        assert_eq!(count_valid_passports(&INVALID_2), 0);
    }

    #[test]
    fn invalid_3() {
        assert_eq!(count_valid_passports(&INVALID_3), 0);
    }

    #[test]
    fn valid_0() {
        assert_eq!(count_valid_passports(&VALID_0), 1);
    }

    #[test]
    fn valid_1() {
        assert_eq!(count_valid_passports(&VALID_1), 1);
    }

    #[test]
    fn valid_2() {
        assert_eq!(count_valid_passports(&VALID_2), 1);
    }

    #[test]
    fn valid_3() {
        assert_eq!(count_valid_passports(&VALID_3), 1);
    }


}
