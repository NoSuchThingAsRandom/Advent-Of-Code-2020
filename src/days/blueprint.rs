use crate::misc::error::AoCResult;
use crate::misc::read_vec_string;

pub fn run() {
    //let data = read_vec_string(String::from("Inputs/test.txt")).unwrap();
    let data = read_vec_string(String::from("Inputs/input.txt")).unwrap();
    println!("    Part 1: {}", part_1(&data).unwrap());
    println!("    Part 2: {}", part_2(&data).unwrap());
}
fn part_1(data: &[String]) -> AoCResult<usize> {
    Ok(0)
}
fn part_2(data: &[String]) -> AoCResult<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::days::day::{part_1, part_2};
    use crate::misc::read_vec_string;

    #[test]
    fn part_1_test() {
        let data = read_vec_string(String::from("Inputs/test17.txt")).unwrap();
        let result = part_1(&data).unwrap();
        assert_eq!(result, 112);
    }

    #[test]
    fn part_1_input() {
        let data = read_vec_string(String::from("Inputs/input17.txt")).unwrap();
        let result = part_1(&data).unwrap();
        assert_eq!(result, 273);
    }

    #[test]
    fn part_2_test() {
        let data = read_vec_string(String::from("Inputs/test17.txt")).unwrap();
        let result = part_2(&data).unwrap();
        assert_eq!(result, 848);
    }

    #[test]
    fn part_2_input() {
        let data = read_vec_string(String::from("Inputs/test17.txt")).unwrap();
        let result = part_2(&data).unwrap();
        assert_eq!(result, 848);
    }
}
