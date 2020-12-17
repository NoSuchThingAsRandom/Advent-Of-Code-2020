use crate::misc::error::AoCResult;
use std::collections::HashMap;

use std::time::Instant;

pub fn run() {
    let nums = vec![2, 20, 0, 4, 1, 17];
    //let nums = vec![0, 3, 6];
    println!("    Part 1: {}", solver_hashmap(&nums, 2020).unwrap());
    let start = Instant::now();
    /*println!("Part 2A: {},", solver_hashmap(&nums, 30000000).unwrap());
    println!("Time: {}ms", start.elapsed().as_millis());
    let start = Instant::now();*/
    println!("    Part 2: {},", solver_vec(&nums, 30000000).unwrap());
    //println!("Time: {}ms", start.elapsed().as_millis());
}
fn solver_hashmap(data: &[usize], target_number: usize) -> AoCResult<usize> {
    let mut spoken = HashMap::new();
    for (index, num) in data.iter().enumerate() {
        spoken.insert(*num, index + 1);
    }
    let mut current_num = *data.last().expect("Initial numbers list is empty!");
    spoken.remove(&current_num);
    //println!("Spoken {:?}", spoken);
    let mut next_num;
    for turn_index in data.len()..target_number {
        //println!("Turn {}, Says: {}", turn_index, current_num);
        if let Some(count) = spoken.get(&current_num) {
            next_num = turn_index - count;
        } else {
            next_num = 0;
        }
        spoken.insert(current_num, turn_index);
        current_num = next_num;
    }

    Ok(current_num)
}
fn solver_vec(data: &[usize], target_number: usize) -> AoCResult<usize> {
    let mut spoken = Vec::new();
    for (index, num) in data.iter().enumerate() {
        if spoken.len() < *num {
            spoken.resize(*num, 0);
        }
        spoken.push(index + 1);
    }
    let mut current_num = *data.last().expect("Initial numbers list is empty!");
    spoken.pop();
    //println!("Spoken {:?}", spoken);
    let mut next_num;
    for turn_index in data.len()..target_number {
        //println!("Turn {}, Says: {}", turn_index, current_num);
        if let Some(count) = spoken.get(current_num) {
            if *count == 0 {
                next_num = 0;
            } else {
                next_num = turn_index - count;
            }
        } else {
            next_num = 0;
        }
        if spoken.len() < current_num + 1 {
            spoken.resize(current_num, 0);
            spoken.push(turn_index);
        } else {
            spoken[current_num] = turn_index;
        }
        current_num = next_num;
    }
    //println!("Size: {}", spoken.len());

    Ok(current_num)
}
#[cfg(test)]
mod tests {
    use crate::days::day15::solver_vec;

    /*    #[test]
    fn part_1_hash_test() {
        let res = solver_hashmap(&[0, 3, 6], 2020);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 436);
    }
    #[test]
    fn part_2a_hash_test() {
        let res = solver_hashmap(&[0, 3, 6], 30000000);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 175594);
    }
    #[test]
    fn part_2b_hash_test() {
        let res = solver_hashmap(&[1, 3, 2], 30000000);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 2578);
    }
    #[test]
    fn part_2_hash_input() {
        let res = solver_hashmap(&[2, 20, 0, 4, 1, 17], 30000000);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 814);
    }*/

    #[test]
    fn part_2a_test() {
        let res = solver_vec(&[0, 3, 6], 30000000);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 175594);
    }
    #[test]
    fn part_2b_test() {
        let res = solver_vec(&[1, 3, 2], 30000000);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 2578);
    }
    #[test]
    fn part_2c_test() {
        let res = solver_vec(&[2, 1, 3], 30000000);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 3544142);
    }
    #[test]
    fn part_2d_test() {
        let res = solver_vec(&[1, 2, 3], 30000000);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 261214);
    }
    #[test]
    fn part_2e_test() {
        let res = solver_vec(&[2, 3, 1], 30000000);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 6895259);
    }
    #[test]
    fn part_2f_test() {
        let res = solver_vec(&[3, 2, 1], 30000000);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 18);
    }
    #[test]
    fn part_2g_test() {
        let res = solver_vec(&[3, 1, 2], 30000000);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 362);
    }

    #[test]
    fn part_1_input() {
        let res = solver_vec(&[2, 20, 0, 4, 1, 17], 2020);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 758);
    }
    #[test]
    fn part_2_input() {
        let res = solver_vec(&[2, 20, 0, 4, 1, 17], 30000000);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 814);
    }
}
