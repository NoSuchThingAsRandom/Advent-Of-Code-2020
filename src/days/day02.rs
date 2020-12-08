use crate::misc::error::{AoCError, AoCResult};
use crate::misc::{get_values, read_vec_string};

pub fn run() -> AoCResult<usize> {
    let original = read_vec_string(String::from("Inputs/input02.txt")).unwrap();
    println!("Valid passwords: {}", part_1(&original).unwrap());
    println!("Valid passwords: {}", part_2(&original).unwrap());
    println!("Test!");
    Ok(0)
}

fn build_args(line: &str) -> AoCResult<Vec<String>> {
    const ARG_SEPARATORS: [&char; 3] = [&'-', &' ', &':'];
    let mut args = Vec::new();
    let mut arg_index = 0;

    let mut arg_builder = String::new();
    for char in line.chars() {
        if arg_index != ARG_SEPARATORS.len()
            && char.eq(*get_values(&ARG_SEPARATORS.to_vec(), arg_index)?)
        {
            arg_index += 1;
            args.push(arg_builder);
            arg_builder = String::new();
        } else {
            arg_builder.push(char)
        }
    }
    args.push(arg_builder);
    Ok(args)
}

pub fn part_1(data: &[String]) -> AoCResult<usize> {
    let mut valid_count = 0;
    for line in data {
        let args = build_args(line)?;
        assert_eq!(args.len(), 4);
        let mut letter_count = 0;
        let target_letter = get_values(&args, 2)?
            .chars()
            .next()
            .ok_or_else(|| AoCError::new(String::from("Couldn't get target letter")))?;
        for char in get_values(&args, 3)?.chars() {
            if char.eq(&target_letter) {
                letter_count += 1;
            }
        }

        if get_values(&args, 0)?.parse::<usize>()? <= letter_count
            && letter_count <= get_values(&args, 1)?.parse::<usize>()?
        {
            valid_count += 1;
        }
    }
    Ok(valid_count)
}
pub fn part_2(data: &[String]) -> AoCResult<usize> {
    let mut valid_count = 0;
    for line in data {
        let args = build_args(line)?;
        assert_eq!(args.len(), 4);
        let target_letter = get_values(&args, 2)?
            .chars()
            .next()
            .ok_or_else(|| AoCError::new(String::from("Couldn't get target letter")))?;
        let password = get_values(&args, 3)?.chars().collect::<Vec<char>>();
        let first_letter =
            get_values(&password, get_values(&args, 0)?.parse()?)?.eq(&target_letter);
        let second_letter =
            get_values(&password, get_values(&args, 1)?.parse()?)?.eq(&target_letter);
        if first_letter && !second_letter || !first_letter && second_letter {
            valid_count += 1;
        }
    }
    Ok(valid_count)
}
#[cfg(test)]
mod tests {
    use crate::days::day02::{part_1, part_2};
    use crate::misc::read_vec_string;

    #[test]
    fn part_1_input() {
        let data = read_vec_string(String::from("Inputs/input02.txt")).unwrap();
        let res = part_1(&data);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 638);
    }
    #[test]
    fn part_2_input() {
        let data = read_vec_string(String::from("Inputs/input02.txt")).unwrap();
        let res = part_2(&data);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 699);
    }
}
