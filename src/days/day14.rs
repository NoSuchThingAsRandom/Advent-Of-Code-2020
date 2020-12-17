use crate::misc::error::AoCResult;
use crate::misc::read_vec_string;
use std::collections::HashMap;

const DEBUG: bool = false;
pub fn run() {
    let data = read_vec_string(String::from("Inputs/input14.txt")).unwrap();
    println!("    Part 1: {}", part_1(&data).unwrap());
    println!("    Part 2: {}", part_2(&data).unwrap());
}

fn bitstring_to_int(bits: String) -> usize {
    let mut total = 0;
    for (index, bit) in bits.chars().enumerate() {
        let two: usize = 2;
        if bit.eq(&'1') {
            total += two.pow(bits.len() as u32 - 1 - index as u32);
            if DEBUG {
                println!(
                    "      At index: {}, inserting value: {} with pow: {}",
                    index,
                    two.pow(bits.len() as u32 - 1 - index as u32),
                    bits.len() - index
                );
            }
        }
    }
    total
}
fn parse_line(line: &str) -> (String, String) {
    let mut location = String::new();
    let mut value = String::new();
    let mut is_location = false;
    let mut is_value = false;
    for c in line.chars() {
        match c {
            '[' => {
                is_location = true;
            }
            ']' => {
                is_location = false;
            }
            ' ' => {
                is_value = true;
            }
            x => {
                if x.is_numeric() {
                    if is_location {
                        location.push(x);
                    } else if is_value {
                        value.push(x);
                    }
                }
            }
        }
    }
    (location, value)
}
fn part_1(data: &[String]) -> AoCResult<usize> {
    let mut mask = String::new();
    let mut memory = HashMap::new();
    for line in data {
        if line.contains("mask") {
            mask = line.clone().replace("mask = ", "");
        } else {
            let (location, value) = parse_line(&line);
            let value: usize = value.parse().unwrap();
            let bit_string: String = format!("{:b}", value).chars().collect();
            let mut padded = String::new();
            for _ in 0..(mask.len() - bit_string.len()) {
                padded.push('0');
            }
            padded.push_str(&bit_string);
            for (index, bit) in mask.chars().enumerate() {
                if bit.eq(&'X') {
                    continue;
                }
                padded.replace_range(index..index + 1, &String::from(bit));
            }
            let num = bitstring_to_int(padded);
            memory.insert(location, num);
        }
    }
    let mut memory_total = 0;
    for (_, value) in memory.iter() {
        memory_total += value;
    }
    Ok(memory_total)
}

fn part_2(data: &[String]) -> AoCResult<usize> {
    let mut mask = String::new();
    let mut memory = HashMap::new();
    for line in data {
        if line.contains("mask") {
            mask = line.clone().replace("mask = ", "");
        } else {
            let (location, value) = parse_line(&line);
            let location: usize = location.parse().unwrap();
            let value: usize = value.parse().unwrap();
            let bit_string: String = format!("{:b}", location).chars().collect();
            let mut padded = String::new();
            for _ in 0..(mask.len() - bit_string.len()) {
                padded.push('0');
            }
            padded.push_str(&bit_string);
            //println!("Base string:    {}", padded);
            let mut floater_indexes = Vec::new();
            for (index, bit) in mask.chars().enumerate() {
                if bit.eq(&'X') {
                    floater_indexes.push(index);
                    padded.replace_range(index..index + 1, &String::from("0"));
                    continue;
                } else if bit.eq(&'1') {
                    padded.replace_range(index..index + 1, &String::from("1"));
                }
            }
            let mut addresses = vec![bitstring_to_int(padded)];
            for index in floater_indexes {
                let mut new = Vec::new();
                for address in &addresses {
                    let two: usize = 2;
                    new.push(address + two.pow(mask.len() as u32 - 1 - index as u32) as usize);
                }
                addresses.append(&mut new);
            }
            for address in addresses {
                memory.insert(address, value);
            }
        }
    }
    let mut memory_total = 0;
    for (location, value) in memory.iter() {
        if DEBUG {
            println!("{} @ {}", value, location);
        }
        memory_total += value;
    }
    Ok(memory_total)
}
#[cfg(test)]
mod tests {
    use crate::days::day14::{part_1, part_2};
    use crate::misc::read_vec_string;

    #[test]
    fn part_1_test() {
        let data = read_vec_string(String::from("Inputs/test14a.txt")).unwrap();
        let result = part_1(&data).unwrap();
        assert_eq!(result, 165);
    }
    #[test]
    fn part_1_input() {
        let data = read_vec_string(String::from("Inputs/input14.txt")).unwrap();
        let result = part_1(&data).unwrap();
        assert_eq!(result, 13476250121721);
    }
    #[test]
    fn part_2_test() {
        let data = read_vec_string(String::from("Inputs/test14b.txt")).unwrap();
        let result = part_2(&data).unwrap();
        assert_eq!(result, 208);
    }
    #[test]
    fn part_2_input() {
        let data = read_vec_string(String::from("Inputs/input14.txt")).unwrap();
        let result = part_2(&data).unwrap();
        assert_eq!(result, 4463708436768);
    }
}
