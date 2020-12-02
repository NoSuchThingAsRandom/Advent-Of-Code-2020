use crate::misc::error::{AoCError, AoCResult};
use crate::misc::read_vec_ints;
use rand::seq::SliceRandom;
use std::cmp::Ordering;
use std::time::Instant;

pub fn run() {
    let mut rng = rand::thread_rng();
    let original = read_vec_ints(String::from("../Inputs/input1.txt")).unwrap();

    const COUNT: u128 = 10000;
    let mut sum = 0;
    for _ in 0..COUNT {
        let mut data = original.clone();
        data.shuffle(&mut rng);
        let start = Instant::now();
        data.sort();
        let _a = part_1(&data);
        let _b = part_2_a(&data);
        let end = Instant::now();
        sum += end.duration_since(start).as_micros()
    }
    println!("Time A {}", (sum / COUNT));
    //println!("Part 1: {}", part_1(&data).unwrap());
    //println!("Part 2: {}", part_2(&data).unwrap());
}

// Result is 802011
fn part_1(data: &[usize]) -> AoCResult<usize> {
    let mut should_loop = true;
    let mut a_pointer = data.len() - 1;
    while should_loop {
        if let Some(a) = data.get(a_pointer) {
            for b in data {
                let result = a + b;
                match result
                    .partial_cmp(&2020)
                    .ok_or_else(|| AoCError::new(String::from("Failed to find a solution")))?
                {
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
    Err(AoCError::new(String::from("Failed to find a solution")))
}
pub fn bench(data: &mut Vec<usize>) {
    data.sort();
    part_2_a(data).unwrap();
    part_2_b(data).unwrap();
}
// Result is 802011
fn part_2_a(data: &[usize]) -> AoCResult<usize> {
    let mut a_pointer = data.len() - 1;
    while a_pointer != 0 {
        if let Some(a) = data.get(a_pointer) {
            if a < &2020 {
                let mut b_pointer = data.len() - 1;
                while b_pointer != 0 {
                    if let Some(b) = data.get(b_pointer) {
                        if b < &2020 && a + b < 2020 {
                            for c in data {
                                let result = a + b + c;
                                match result.partial_cmp(&2020).ok_or_else(|| {
                                    AoCError::new(String::from("Failed comparison"))
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
    Err(AoCError::new(String::from("Failed to find a solution")))
}
// Result is 802011
fn part_2_b(data: &[usize]) -> AoCResult<usize> {
    let mut a_pointer = data.len() - 1;
    while a_pointer != 0 {
        let a = data[a_pointer];
        if a < 2020 {
            let mut b_pointer = data.len() - 1;
            while b_pointer != 0 {
                let b = data[b_pointer];
                if b < 2020 && a + b < 2020 {
                    for c in data {
                        let result = a + b + c;
                        match result
                            .partial_cmp(&2020)
                            .ok_or_else(|| AoCError::new(String::from("Failed comparison")))?
                        {
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
                b_pointer -= 1;
            }
        }

        a_pointer -= 1;
    }
    Err(AoCError::new(String::from("Failed to find a solution")))
}
