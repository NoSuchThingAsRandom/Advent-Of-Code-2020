use crate::misc::error::{AoCError, AoCResult};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() -> AoCResult<usize> {
    println!("{}", part_1()?);
    Ok(0)
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
fn part_1() -> AoCResult<usize> {
    let file = File::open("Inputs/input04.txt")?;
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
