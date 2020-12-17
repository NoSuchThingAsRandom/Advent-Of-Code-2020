use crate::misc::error::AoCResult;
use crate::misc::read_vec_string;
use std::collections::HashSet;

pub fn run() -> AoCResult<usize> {
    let original = read_vec_string(String::from("Inputs/input06.txt")).unwrap();
    println!("    Total Questions: {}", part_1(&original).unwrap());
    println!("    Seats: {}", part_2(&original).unwrap());
    Ok(0)
}

pub fn part_1(data: &[String]) -> AoCResult<usize> {
    let mut total_count = 0;
    let mut answered_questions = HashSet::new();
    for line in data {
        if line.eq("") {
            total_count += answered_questions.len();
            answered_questions = HashSet::new();
        } else {
            for question in line.chars() {
                answered_questions.insert(question);
            }
        }
    }
    total_count += answered_questions.len();
    Ok(total_count)
}
pub fn part_2(data: &[String]) -> AoCResult<usize> {
    let mut total_count = 0;
    let mut answered_questions = HashSet::new();
    let mut start = true;
    for line in data {
        if line.eq("") {
            total_count += answered_questions.len();
            answered_questions = HashSet::new();
            start = true;
        } else if start {
            for question in line.chars() {
                answered_questions.insert(question);
            }
            start = false;
        } else {
            answered_questions.retain(|x| line.contains(*x));
        }
    }
    total_count += answered_questions.len();
    Ok(total_count)
}
#[cfg(test)]
mod tests {
    use crate::days::day06::{part_1, part_2};
    use crate::misc::read_vec_string;

    #[test]
    fn part_1_test() {
        let data = read_vec_string(String::from("Inputs/test06.txt")).unwrap();
        let res = part_1(&data);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 11);
    }
    #[test]
    fn part_1_input() {
        let data = read_vec_string(String::from("Inputs/input06.txt")).unwrap();
        let res = part_1(&data);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 6565);
    }
    #[test]
    fn part_2_test() {
        let data = read_vec_string(String::from("Inputs/test06.txt")).unwrap();
        let res = part_2(&data);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 6);
    }
    fn part_2_input() {
        let data = read_vec_string(String::from("Inputs/input06.txt")).unwrap();
        let res = part_1(&data);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 3137);
    }
}
