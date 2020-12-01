extern crate misc;

use misc::error;
use std::cmp::Ordering;

fn main() {
    println!("Part 1: {}", part_1().unwrap());
    println!("Part 2: {}", part_2().unwrap());
}

// Result is 802011
fn part_1() -> error::AoCResult<usize> {
    let mut data = misc::read_vec(String::from("../Inputs/input1.txt")).unwrap();
    data.sort();
    let mut should_loop = true;
    let mut a_pointer = data.len() - 1;
    while should_loop {
        if let Some(a) = data.get(a_pointer) {
            for b in &data {
                let result = a + b;
                match result.partial_cmp(&2020).ok_or_else(|| {
                    misc::error::Error::new(String::from("Failed to find a solution"))
                })? {
                    Ordering::Less => {}
                    Ordering::Equal => {
                        return Ok(a * b);
                    }
                    Ordering::Greater => {
                        break;
                    }
                }
            }
        }
        a_pointer -= 1;
        if a_pointer == 0 {
            should_loop = false;
        }
    }
    Err(misc::error::Error::new(String::from(
        "Failed to find a solution",
    )))
}
// Result is 802011
fn part_2() -> error::AoCResult<usize> {
    let mut data = misc::read_vec(String::from("../Inputs/input1.txt")).unwrap();
    data.sort();
    let mut a_pointer = data.len() - 1;
    while a_pointer != 0 {
        if let Some(a) = data.get(a_pointer) {
            if a < &2020 {
                let mut b_pointer = data.len() - 1;
                while b_pointer != 0 {
                    if let Some(b) = data.get(b_pointer) {
                        if b < &2020 && a + b < 2020 {
                            for c in &data {
                                let result = a + b + c;
                                match result.partial_cmp(&2020).ok_or_else(|| {
                                    misc::error::Error::new(String::from("Failed comparison"))
                                })? {
                                    Ordering::Less => {}
                                    Ordering::Equal => {
                                        return Ok(a * b * c);
                                    }
                                    Ordering::Greater => {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    b_pointer -= 1;
                }
            }
        }
        a_pointer -= 1;
    }
    Err(misc::error::Error::new(String::from(
        "Failed to find a solution",
    )))
}
