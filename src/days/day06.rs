use crate::misc::error::AoCResult;
use crate::misc::read_vec_string;
use std::collections::HashSet;

pub fn run() -> AoCResult<usize> {
    println!("Test");
    let original = read_vec_string(String::from("Inputs/input06.txt")).unwrap();
    println!("Total Questions: {}", part_1(&original).unwrap());
    println!("Seats: {}", part_2(&original).unwrap());
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
