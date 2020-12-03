use crate::misc::error::{AoCError, AoCResult};
use crate::misc::{get_values, read_vec_string};

pub fn run() -> AoCResult<usize> {
    let data = read_vec_string(String::from("Inputs/input03.txt"))?;
    println!("Trees: {}", part_1(&data)?);
    println!("Trees: {}", part_2(&data)?);
    Ok(0)
}

fn part_1(data: &[String]) -> AoCResult<usize> {
    let mut index = 0;
    let mut trees = 0;
    for line in data {
        if index >= line.len() {
            index -= line.len();
        }
        if get_values(&line.chars().collect::<Vec<char>>(), index)?.eq(&'#') {
            trees += 1;
        }
        index += 3;
    }
    Ok(trees)
}
fn part_2(data: &[String]) -> AoCResult<usize> {
    let mut tree_count = 0;
    for slope in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        if tree_count == 0 {
            tree_count = slope_checker(data, slope.0, slope.1)?;
        } else {
            tree_count *= slope_checker(data, slope.0, slope.1)?;
        }
    }
    Ok(tree_count)
}
fn slope_checker(data: &[String], x_increment: usize, y_increment: usize) -> AoCResult<usize> {
    let mut x_index = 0;
    let mut trees = 0;
    for y_index in (0..data.len()).step_by(y_increment) {
        if x_index >= data[y_index].len() {
            x_index -= data[y_index].len();
        }
        if let Some(val) = data[y_index].chars().nth(x_index) {
            if val.eq(&'#') {
                trees += 1;
            }
        }
        x_index += x_increment;
    }
    Ok(trees)
}
