extern crate peg;
use crate::misc::error::{AoCError, AoCResult};
use crate::misc::read_vec_string;

const DEBUG: bool = false;
pub fn run() {
    let data = read_vec_string(String::from("Inputs/input18.txt")).unwrap();
    println!("    Part 1: {}", part_1(&data).unwrap());
    println!("    Part 2: {}", part_2(&data).unwrap());
}

const OPERATORS: [char; 2] = ['+', '*'];
struct Operation {
    a: usize,
    b: usize,
    operator: char,
}
impl Operation {
    fn new(a: usize, b: usize, operator: char) -> Operation {
        Operation { a, b, operator }
    }
    fn execute(&self) -> usize {
        if DEBUG {
            println!("    Executing: '{}' {} '{}'", self.a, self.operator, self.b);
        }
        match self.operator {
            '+' => self.a + self.b,
            '*' => self.a * self.b,
            _ => unimplemented!("Unknown command"),
        }
    }
}

fn find_brackets(line: &str) -> (usize, usize) {
    let mut bracket_index = 0;
    let mut opener_index = 0;
    let mut found = false;
    let mut closer_index = 0;
    for (index, char) in line.chars().enumerate() {
        if char.eq(&'(') {
            if !found {
                opener_index = index;
                found = true;
            }
            bracket_index += 1;
        }
        if char.eq(&')') {
            bracket_index -= 1;
            if bracket_index == 0 {
                closer_index = index;
                break;
            }
        }
    }
    (opener_index, closer_index)
}

fn parse_brackets_part_1(mut line: String, depth: u8) -> AoCResult<String> {
    if DEBUG {
        for _ in 0..depth {
            print!("    ");
        }
        println!("Splicing: '{}'", line);
    }
    let (opener_index, closer_index) = find_brackets(&line);
    line.replace_range(
        opener_index..(closer_index + 1),
        &parse_line_part_1(
            line[(opener_index + 1)..closer_index].to_string(),
            depth + 1,
        )
        .unwrap()
        .to_string(),
    );
    if DEBUG {
        for _ in 0..depth {
            print!("    ");
        }
        println!("Spliced : '{}'", line);
    }
    Ok(line)
}

fn parse_line_part_1(line: String, depth: u8) -> AoCResult<usize> {
    let mut line = line;
    line = line.replace(" ", "");
    let start_index = 0;
    loop {
        if DEBUG {
            for _ in 0..depth {
                print!("  ");
            }
            println!("Line: '{}'", line);
        }
        let mut a = String::new();
        let mut b = String::new();
        let mut operator = String::new();
        let mut operator_count = 0;
        let mut end_index = 0;
        while operator_count != 2 && end_index < line.len() {
            let chars: Vec<char> = line.chars().collect();
            let c = chars.get(end_index).unwrap();
            if DEBUG {
                for _ in 0..depth {
                    print!("  ");
                }
                println!("Current C: '{}'", c);
            }
            if c.eq(&'(') {
                if DEBUG {
                    for _ in 0..depth {
                        print!("  ");
                    }
                    println!("Replacing: '{}'", line);
                }
                line = parse_brackets_part_1(line.clone(), depth + 2).unwrap();
                end_index = 0;
                a = String::new();
                b = String::new();
                operator = String::new();
                operator_count = 0;
                if DEBUG {
                    for _ in 0..depth {
                        print!("  ");
                    }
                    println!("Updated: '{}'", line);
                }
            } else if c.is_numeric() {
                if operator_count == 0 {
                    a.push(*c);
                } else {
                    b.push(*c);
                }
                end_index += 1;
            } else {
                if operator_count == 0 {
                    operator.push(*c);
                }
                operator_count += 1;
                end_index += 1;
            }
        }
        if !a.is_empty() && !b.is_empty() && !operator.is_empty() {
            let op = Operation::new(
                a.parse().unwrap(),
                b.parse().unwrap(),
                AoCError::from_option(operator.chars().next()).unwrap(),
            );
            let result = op.execute().to_string();

            if end_index == line.len() {
                end_index += 1;
            }
            if DEBUG {
                for _ in 0..depth {
                    print!("  ");
                }
                println!(
                    "Start: {}, End: {} Result: {}",
                    start_index,
                    end_index - 1,
                    result
                );
            }
            line.replace_range(start_index..end_index - 1, &result);
            if DEBUG {
                for _ in 0..depth {
                    print!("  ");
                }
                println!("New: {}", line);
            }
        } else {
            break;
        }
    }
    if DEBUG {
        for _ in 0..depth {
            print!("    ");
        }
        println!("Result: {}", line);
    }
    Ok(line.trim().parse().unwrap())
}

fn part_1(data: &[String]) -> AoCResult<usize> {
    let mut total = 0;
    for line in data {
        total += parse_line_part_1(line.to_string(), 0)?;
    }
    Ok(total)
}
fn parse_brackets_part_2(mut line: String, depth: u8) -> AoCResult<String> {
    if DEBUG {
        for _ in 0..depth {
            print!("    ");
        }
        println!("Splicing: '{}'", line);
    }
    let (opener_index, closer_index) = find_brackets(&line);
    line.replace_range(
        opener_index..(closer_index + 1),
        &parse_line_part_2(
            line[(opener_index + 1)..closer_index].to_string(),
            depth + 1,
        )
        .unwrap()
        .to_string(),
    );
    if DEBUG {
        for _ in 0..depth {
            print!("    ");
        }
        println!("Spliced : '{}'", line);
    }
    Ok(line)
}

fn parse_line_part_2(line: String, depth: u8) -> AoCResult<usize> {
    let mut line = line;
    line = line.replace(" ", "");
    let mut start_index = 0;
    let mut current_operator = 0;
    loop {
        if !line.contains(OPERATORS[current_operator]) {
            current_operator += 1;
            start_index = 0;
            if current_operator == OPERATORS.len() {
                break;
            }
        }
        if DEBUG {
            for _ in 0..depth {
                print!("  ");
            }
            println!("Line: '{}'", line);
        }
        let mut a = String::new();
        let mut b = String::new();
        let mut operator = String::new();
        let mut operator_count = 0;
        let mut end_index = start_index;
        while operator_count != 2 && end_index < line.len() {
            let chars: Vec<char> = line.chars().collect();
            let c = chars.get(end_index).unwrap();
            if DEBUG {
                for _ in 0..depth {
                    print!("  ");
                }
                println!("Current C: '{}'", c);
            }
            if c.eq(&'(') {
                if DEBUG {
                    for _ in 0..depth {
                        print!("  ");
                    }
                    println!("Replacing: '{}'", line);
                }
                line = parse_brackets_part_2(line.clone(), depth + 2).unwrap();
                start_index = 0;
                end_index = start_index;
                a = String::new();
                b = String::new();
                operator = String::new();
                operator_count = 0;
                if DEBUG {
                    for _ in 0..depth {
                        print!("  ");
                    }
                    println!("Updated: '{}'", line);
                }
            } else if c.is_numeric() {
                if operator_count == 0 {
                    a.push(*c);
                } else {
                    b.push(*c);
                }
                end_index += 1;
            } else {
                if operator_count == 0 && c.eq(&OPERATORS[current_operator]) {
                    operator.push(*c);
                }
                operator_count += 1;
                end_index += 1;
            }
        }
        if a.is_empty() || b.is_empty() {
            break;
        }

        if operator.is_empty() {
            start_index = end_index - b.len() - 1;
        } else {
            let op = Operation::new(
                a.parse().unwrap(),
                b.parse().unwrap(),
                AoCError::from_option(operator.chars().next()).unwrap(),
            );
            let result = op.execute().to_string();

            if end_index == line.len() {
                end_index += 1;
            }
            if DEBUG {
                for _ in 0..depth {
                    print!("  ");
                }
                println!(
                    "Start: {}, End: {} Result: {}",
                    start_index,
                    end_index - 1,
                    result
                );
            }
            line.replace_range(start_index..end_index - 1, &result);
            if DEBUG {
                for _ in 0..depth {
                    print!("  ");
                }
                println!("New: {}", line);
            }
            start_index = 0;
        }
    }
    if DEBUG {
        for _ in 0..depth {
            print!("    ");
        }
        println!("Result: {}", line);
    }
    Ok(line.trim().parse().unwrap())
}

fn part_2(data: &[String]) -> AoCResult<usize> {
    let mut total = 0;
    for line in data {
        total += parse_line_part_2(line.to_string(), 0)?;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use crate::days::day18::{part_1, part_2};
    use crate::misc::read_vec_string;

    #[test]
    fn part_1_test_a() {
        let data = vec![String::from("1 + 2 * 3 + 4 * 5 + 6")];
        let result = part_1(&data).unwrap();
        assert_eq!(result, 71);
    }

    #[test]
    fn part_1_test_b() {
        let data = vec![String::from("1 + (2 * 3) + (4 * (5 + 6))")];
        let result = part_1(&data).unwrap();
        assert_eq!(result, 51);
    }
    #[test]
    fn part_1_test_c() {
        let data = vec![String::from("2 * 3 + (4 * 5)")];
        let result = part_1(&data).unwrap();
        assert_eq!(result, 26);
    }
    #[test]
    fn part_1_test_d() {
        let data = vec![String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)")];
        let result = part_1(&data).unwrap();
        assert_eq!(result, 437);
    }
    #[test]
    fn part_1_test_e() {
        let data = vec![String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")];
        let result = part_1(&data).unwrap();
        assert_eq!(result, 12240);
    }

    #[test]
    fn part_1_test_f() {
        let data = vec![String::from(
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
        )];
        let result = part_1(&data).unwrap();
        assert_eq!(result, 13632);
    }

    #[test]
    fn part_1_input() {
        let data = read_vec_string(String::from("Inputs/input18.txt")).unwrap();
        let result = part_1(&data).unwrap();
        assert_eq!(result, 3647606140187);
    }

    #[test]
    fn part_1_test() {
        let data = read_vec_string(String::from("Inputs/test18.txt")).unwrap();
        let result = part_1(&data).unwrap();
        assert_eq!(result, 71 + 51 + 26 + 437 + 12240 + 13632);
    }

    #[test]
    fn part_2_test_a() {
        let data = vec![String::from("1 + 2 * 3 + 4 * 5 + 6")];
        let result = part_2(&data).unwrap();
        assert_eq!(result, 231);
    }

    #[test]
    fn part_2_test_b() {
        let data = vec![String::from("1 + (2 * 3) + (4 * (5 + 6))")];
        let result = part_2(&data).unwrap();
        assert_eq!(result, 51);
    }
    #[test]
    fn part_2_test_c() {
        let data = vec![String::from("2 * 3 + (4 * 5)")];
        let result = part_2(&data).unwrap();
        assert_eq!(result, 46);
    }
    #[test]
    fn part_2_test_d() {
        let data = vec![String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)")];
        let result = part_2(&data).unwrap();
        assert_eq!(result, 1445);
    }
    #[test]
    fn part_2_test_e() {
        let data = vec![String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")];
        let result = part_2(&data).unwrap();
        assert_eq!(result, 669060);
    }

    #[test]
    fn part_2_test_f() {
        let data = vec![String::from(
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
        )];
        let result = part_2(&data).unwrap();
        assert_eq!(result, 23340);
    }

    #[test]
    fn part_2_test() {
        let data = read_vec_string(String::from("Inputs/test18.txt")).unwrap();
        let result = part_2(&data).unwrap();
        assert_eq!(result, 231 + 51 + 46 + 1445 + 669060 + 23340);
    }

    #[test]
    fn part_2_input() {
        let data = read_vec_string(String::from("Inputs/input18.txt")).unwrap();
        let result = part_2(&data).unwrap();
        assert_eq!(result, 323802071857594);
    }
}
