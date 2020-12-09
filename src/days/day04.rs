use crate::misc::error::{AoCError, AoCResult};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() -> AoCResult<usize> {
    println!("{}", part_1(String::from("Inputs/input04.txt"))?);
    println!("{}", part_2(String::from("Inputs/input04.txt"))?);
    Ok(0)
}

fn part_1(filename: String) -> AoCResult<usize> {
    const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();
    let mut current_passport: HashMap<String, String> = HashMap::new();
    let mut valid_passport_count = 0;
    for line in reader.lines() {
        let line = line?;
        if line.eq("") {
            let mut valid = true;
            for field in REQUIRED_FIELDS.iter() {
                if current_passport.get(*field).is_none() {
                    valid = false;
                    break;
                }
            }
            if valid {
                valid_passport_count += 1;
            }
            data.push(current_passport);
            current_passport = HashMap::new();
        } else {
            let values = line.split(' ');
            for entry in values {
                let mut split_entry = entry.split(':');
                current_passport.insert(
                    split_entry
                        .next()
                        .ok_or_else(|| AoCError::new(String::from("Couldn't get key value")))?
                        .to_string(),
                    split_entry
                        .next()
                        .ok_or_else(|| AoCError::new("Couldn\'t get values: ".to_string()))?
                        .to_string(),
                );
            }
        }
    }
    Ok(valid_passport_count)
}

fn check_passport(passport: &HashMap<String, String>) -> AoCResult<bool> {
    const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for field in REQUIRED_FIELDS.iter() {
        if let Some(value) = passport.get(*field) {
            match *field {
                "byr" => {
                    let year: u16 = value.parse()?;
                    if value.len() != 4 || year < 1920 || 2002 < year {
                        return Ok(false);
                    }
                }
                "iyr" => {
                    let year: u16 = value.parse()?;
                    if value.len() != 4 || year < 2010 || 2020 < year {
                        return Ok(false);
                    }
                }
                "eyr" => {
                    let year: u16 = value.parse()?;
                    if value.len() != 4 || year < 2020 || 2030 < year {
                        return Ok(false);
                    }
                }
                "hgt" => {
                    let mut unit = String::new();
                    let mut num = String::new();

                    for c in value.chars() {
                        if c.is_numeric() {
                            num.push(c)
                        } else {
                            unit.push(c);
                        }
                    }
                    let num: u8 = num.parse()?;
                    if !((unit.eq("cm") && 150 <= num && num <= 193)
                        || (unit.eq("in") && 59 <= num && num <= 76))
                    {
                        return Ok(false);
                    }
                }
                "hcl" => {
                    let re = Regex::new(r"^(#[0-9a-f]{6})$").unwrap();
                    if !re.is_match(value) {
                        return Ok(false);
                    }
                }
                "ecl" => {
                    let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
                    if !re.is_match(value) {
                        return Ok(false);
                    }
                }
                "pid" => {
                    let re = Regex::new(r"^([0-9]{9})$").unwrap();
                    if !re.is_match(value) {
                        return Ok(false);
                    }
                }
                &_ => {}
            }
        } else {
            return Ok(false);
        }
    }
    Ok(true)
}
fn part_2(filename: String) -> AoCResult<usize> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();
    let mut current_passport: HashMap<String, String> = HashMap::new();
    let mut valid_passport_count = 0;
    for line in reader.lines() {
        let line = line?;
        if line.eq("") {
            if check_passport(&current_passport)? {
                valid_passport_count += 1;
            }
            data.push(current_passport);
            current_passport = HashMap::new();
        } else {
            let values = line.split(' ');
            for entry in values {
                let mut split_entry = entry.split(':');
                current_passport.insert(
                    split_entry
                        .next()
                        .ok_or_else(|| AoCError::new(String::from("Couldn't get key value")))?
                        .to_string(),
                    split_entry
                        .next()
                        .ok_or_else(|| AoCError::new("Couldn\'t get valueas: ".to_string()))?
                        .to_string(),
                );
            }
        }
    }
    if check_passport(&current_passport)? {
        valid_passport_count += 1;
    }
    Ok(valid_passport_count)
}
#[cfg(test)]
mod tests {
    use crate::days::day04::{part_1, part_2};
    use crate::misc::read_vec_string;

    #[test]
    fn part_1a_test() {
        let res = part_1(String::from("Inputs/test04a.txt"));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 2);
    }
    #[test]
    fn part_1b_test() {
        let res = part_1(String::from("Inputs/test04b.txt"));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 3);
    }
    #[test]
    fn part_1c_test() {
        let res = part_1(String::from("Inputs/test04c.txt"));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 3);
    }
    #[test]
    fn part_2a_test() {
        let res = part_2(String::from("Inputs/test04a.txt"));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 2);
    }
    #[test]
    fn part_2b_test() {
        let res = part_2(String::from("Inputs/test04b.txt"));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 0);
    }
    #[test]
    fn part_2c_test() {
        let res = part_2(String::from("Inputs/test04c.txt"));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 4);
    }

    #[test]
    fn part_1_input() {
        let res = part_1(String::from("Inputs/input04.txt"));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 202);
    }
    #[test]
    fn part_2_input() {
        let res = part_2(String::from("Inputs/input04.txt"));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 137);
    }
}
