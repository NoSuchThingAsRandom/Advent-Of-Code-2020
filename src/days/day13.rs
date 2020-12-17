use crate::misc::error::{AoCError, AoCResult};
use crate::misc::read_vec_string;

const DEBUG: bool = false;
pub fn run() {
    /*    println!(
        "C: {}\n\n",
        chinese_remainder(&[1, 4, 6], &[3, 5, 7]).unwrap()
    );
    println!(
        "\nA: {}",
        chinese_remainder(&[0, 12, 55, 25, 12], &[7, 13, 59, 31, 19]).unwrap()
    );*/
    /*    let x = 1068781;
    println!("{} {}", 7, x % 7);
    println!("{} {}", 13, 13 - (x % 13));
    println!("{} {}", 59, 59 - (x % 59));
    println!("{} {}", 31, 31 - (x % 31));
    println!("{} {}", 19, 19 - (x % 19));

    println!("\n\n{} {}", 7, x % 7);
    println!("{} {}", 13, (x % 13));
    println!("{} {}", 59, (x % 59));
    println!("{} {}", 31, (x % 31));
    println!("{} {}", 19, (x % 19));

    println!(
        "B: {}",
        chinese_reminder(vec![7, 13, 59, 31, 19], vec![0, 1, 4, 6, 7])
    );
    println!("C: {}\n\n", chinese_reminder(vec![3, 5, 7], vec![1, 4, 6]));

    println!("D: {}", chinese_reminder(vec![3, 4, 5], vec![2, 3, 1]));
    //println!("A: {}", euclid_algorithm_iterative(5086356080, 4812496243));*/
    //println!("B: {}", euclid_algorithm_recursive(5086356080, 4812496243));
    //let data = read_vec_string(String::from("Inputs/test13.txt")).unwrap();
    let data = read_vec_string(String::from("Inputs/input13.txt")).unwrap();
    println!("    Part 1: {}", part_1(&data).unwrap());
    println!("    Part 2: {}", part_2(&data, 100000000000000).unwrap());
}

fn part_1(data: &[String]) -> AoCResult<usize> {
    let start_time: usize = AoCError::from_option(data.get(0))?.parse()?;
    let busses = AoCError::from_option(data.get(1))?.split(',');
    let mut closest_id = 0;
    let mut min_wait = usize::MAX;
    for bus in busses {
        if bus == "x" {
            continue;
        }
        let id: usize = bus.parse()?;
        let mult = (start_time / id) + 1;
        let wait = (mult * id) - start_time;
        if wait < min_wait {
            min_wait = wait;
            closest_id = id;
        }
    }
    //println!("Closest id {}", closest_id);
    //println!("Wait {}", min_wait);
    Ok(min_wait * closest_id)
}
fn check_timestamp(time: usize, busses: &[(usize, usize)]) -> bool {
    for bus in busses {
        if (time + bus.1) % bus.0 != 0 {
            return false;
        }
    }
    true
}

fn part_2_brute(data: &[String], start: usize) -> AoCResult<usize> {
    let busses_str = AoCError::from_option(data.get(1))?.split(',');
    // (Bus ID, Offset)
    let mut busses = Vec::new();
    let mut ids = Vec::new();
    for (index, bus) in busses_str.enumerate() {
        if bus == "x" {
            continue;
        }
        let id: usize = bus.parse()?;
        ids.push(id);
        busses.push((id, index));
    }
    ids.sort();
    ids.reverse();
    let mut current_timestamp = start / ids.first().unwrap();
    while !check_timestamp(current_timestamp, &busses) {
        current_timestamp += ids.first().unwrap();
    }
    //println!("Busses: {:?}", busses);
    //println!("Time: {}", current_timestamp);
    Ok(current_timestamp)
}
fn part_2(data: &[String], _start: usize) -> AoCResult<usize> {
    let busses_str = AoCError::from_option(data.get(1))?.split(',');
    // (Bus ID, Offset)
    let mut busses = Vec::new();
    let mut offsets = Vec::new();
    let mut ids = Vec::new();
    for (index, bus) in busses_str.enumerate() {
        if bus == "x" {
            continue;
        }
        let id: usize = bus.parse()?;
        //println!("ID: {}, Index: {}", id, index);
        ids.push(id);
        busses.push(id as i64);
        let mut rem: i32 = (id as i32) - (index as i32);
        while rem < 0 {
            rem += id as i32;
        }
        offsets.push(rem as i64);
    }
    //println!("Busses: {:?}", busses);
    //println!("Offsets: {:?}", offsets);
    let current_timestamp = chinese_remainder(&offsets, &busses).unwrap();
    //println!("Time: {}", current_timestamp);
    Ok(current_timestamp as usize)
}
// From rosettacode src: https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(num_a: i64, num_b: i64) -> (i64, i64, i64) {
    if num_a == 0 {
        (num_b, 0, 1)
    } else {
        let (gcd, x, y) = egcd(num_b % num_a, num_a);
        (gcd, y - (num_b / num_a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (gcd, x, _) = egcd(x, n);
    if gcd == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

#[cfg(test)]
mod tests {
    use crate::days::day13::{part_1, part_2};
    use crate::misc::read_vec_string;

    #[test]
    fn part_1_test() {
        let data = read_vec_string(String::from("Inputs/test13.txt")).unwrap();
        let result = part_1(&data).unwrap();
        assert_eq!(result, 295);
    }
    #[test]
    fn part_1_input() {
        let data = read_vec_string(String::from("Inputs/input13.txt")).unwrap();
        let result = part_1(&data).unwrap();
        assert_eq!(result, 4207);
    }
    #[test]
    fn part_2a_test() {
        let data = read_vec_string(String::from("Inputs/test13.txt")).unwrap();
        let result = part_2(&data, 1000).unwrap();
        assert_eq!(result, 1068781);
    }
    #[test]
    fn part_2b_test() {
        let data = vec![String::new(), String::from("17,x,13,19")];
        let result = part_2(&data, 1000).unwrap();
        assert_eq!(result, 3417);
    }
    #[test]
    fn part_2c_test() {
        let data = vec![String::new(), String::from("67,7,59,61")];
        let result = part_2(&data, 100000).unwrap();
        assert_eq!(result, 754018);
    }
    #[test]
    fn part_2d_test() {
        let data = vec![String::new(), String::from("67,x,7,59,61")];
        let result = part_2(&data, 100000).unwrap();
        assert_eq!(result, 779210);
    }
    #[test]
    fn part_2e_test() {
        let data = vec![String::new(), String::from("67,7,x,59,61")];
        let result = part_2(&data, 100000).unwrap();
        assert_eq!(result, 1261476);
    }
    #[test]
    fn part_2f_test() {
        let data = vec![String::new(), String::from("1789,37,47,1889")];
        let result = part_2(&data, 1000000000).unwrap();
        assert_eq!(result, 1202161486);
    }

    #[test]
    fn part_2_input() {
        let data = read_vec_string(String::from("Inputs/input12.txt")).unwrap();
        let result = part_2(&data, 1000000000).unwrap();
        assert_eq!(result, 725850285300475);
    }
}
