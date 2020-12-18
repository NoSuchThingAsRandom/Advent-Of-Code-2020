use crate::misc::error::{AoCError, AoCResult};
use crate::misc::read_vec_ints;
use std::collections::{HashMap, VecDeque};
use std::time::Instant;

pub fn run() {
    let nums = read_vec_ints(String::from("Inputs/input09.txt")).unwrap();
    let _start = Instant::now();
    println!("    Part 1: {}", part_1(nums.clone(), 25).unwrap());
    println!("    Part 2: {},", part_2(nums, 167829540).unwrap());
    //print!("Time: {},s", start.elapsed().as_millis());
}

fn part_1(nums: Vec<usize>, preamble_size: usize) -> AoCResult<usize> {
    let mut possible_values: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    let mut preamble = VecDeque::new();
    for y in 0..preamble_size {
        preamble.push_back(AoCError::from_option(nums.get(y))?);
        for x in 0..preamble_size {
            if x != y {
                let num_x = AoCError::from_option(nums.get(x))?;
                let num_y = AoCError::from_option(nums.get(y))?;
                if let Some(entry) = possible_values.get_mut(&(num_x + num_y)) {
                    if !(entry.contains(&(*num_x, *num_y)) || entry.contains(&(*num_y, *num_x))) {
                        entry.push((*num_x, *num_y));
                    }
                } else {
                    possible_values.insert(num_x + num_y, vec![(*num_x, *num_y)]);
                }
            }
        }
    }
    let mut index = preamble_size;
    while let Some(new_val) = nums.get(index) {
        if possible_values.contains_key(new_val) {
            let to_remove = AoCError::from_option(preamble.pop_front())?;
            possible_values.retain(|_, value| {
                value.retain(|(sub_x, sub_y)| !(sub_x == to_remove || sub_y == to_remove));
                !value.is_empty()
            });
            preamble.push_back(new_val);
            for new_val_addition in &preamble {
                let new_val_addition = **new_val_addition;
                if let Some(entry) = possible_values.get_mut(&(*new_val + new_val_addition)) {
                    if !(entry.contains(&(*new_val, new_val_addition))
                        || entry.contains(&(*new_val, new_val_addition)))
                    {
                        entry.push((*new_val, new_val_addition));
                    }
                } else {
                    possible_values.insert(
                        new_val + new_val_addition,
                        vec![(*new_val, new_val_addition)],
                    );
                }
            }
        } else {
            break;
        }
        index += 1;
    }
    Ok(nums[index])
}

fn sum_vec(data: &VecDeque<usize>) -> usize {
    let mut total = 0;
    for i in data {
        total += i;
    }
    total
}
fn part_2(nums: Vec<usize>, target: usize) -> AoCResult<usize> {
    let mut current_addition = VecDeque::new();
    let mut index = 0;
    let mut current_val = 0;
    while current_val != target {
        if current_val > target {
            current_addition.pop_front();
        } else {
            current_addition.push_back(*AoCError::from_option(nums.get(index))?);
            index += 1;
        }
        current_val = sum_vec(&current_addition);
    }
    let mut current_addition = Vec::from(current_addition);
    current_addition.sort();
    Ok(AoCError::from_option(current_addition.get(0))?
        + AoCError::from_option(current_addition.last())?)
}
#[cfg(test)]
mod tests {
    use crate::days::day09::{part_1, part_2};
    use crate::misc::read_vec_ints;

    #[test]
    fn part_1_test() {
        let data = read_vec_ints(String::from("Inputs/test09.txt")).unwrap();
        let res = part_1(data, 5);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 127);
    }
    #[test]
    fn part_1_input() {
        let data = read_vec_ints(String::from("Inputs/input09.txt")).unwrap();
        let res = part_1(data, 25);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 167829540);
    }
    #[test]
    fn part_2_test() {
        let data = read_vec_ints(String::from("Inputs/test09.txt")).unwrap();
        let res = part_2(data, 127);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 62);
    }
    #[test]
    fn part_2_input() {
        let data = read_vec_ints(String::from("Inputs/input09.txt")).unwrap();
        let res = part_2(data, 167829540);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 28045630);
    }
}
