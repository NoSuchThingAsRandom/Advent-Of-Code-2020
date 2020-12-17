use crate::misc::error::{AoCError, AoCResult};
use crate::misc::read_vec_string;
use std::collections::HashMap;
const DEBUG: bool = false;
pub fn run() {
    let data = read_vec_string(String::from("Inputs/input11.txt")).unwrap();
    //let data = read_vec_string(String::from("Inputs/test11.txt")).unwrap();
    println!("    Part 1: {}", part_1(&data).unwrap());
    println!("    Part 2: {}", part_2(&data).unwrap());
}

fn build_seats_from_input(data: &[String]) -> AoCResult<HashMap<(i32, i32), bool>> {
    // (X coord, Y coord), is_occupied)
    let mut seats = HashMap::new();
    for x_index in 0..data[0].len() {
        for y_index in 0..data.len() {
            if AoCError::from_option(
                AoCError::from_option(data.get(y_index))?
                    .chars()
                    .nth(x_index),
            )?
            .eq(&'L')
            {
                seats.insert((x_index as i32, y_index as i32), false);
            }
        }
    }
    Ok(seats)
}
fn part_1(data: &[String]) -> AoCResult<usize> {
    let mut seats = build_seats_from_input(&data)?;
    let mut changed = 1;
    let mut pass = 1;
    let mut total_occupied = 0;
    while changed != 0 {
        let previous_seats = seats.clone();
        changed = 0;
        // Update seats
        for (position, occupied) in seats.iter_mut() {
            let mut occupied_count = 0;
            // Top, Top Right, Right, Bottom Right, Bottom, Bottom Left, Left, Top Left
            let options: [(i32, i32); 8] = [
                (0, 1),
                (1, 1),
                (1, 0),
                (1, -1),
                (0, -1),
                (-1, -1),
                (-1, 0),
                (-1, 1),
            ];
            for adjust in &options {
                if let Some(seat) =
                    previous_seats.get(&(position.0 + adjust.0, position.1 + adjust.1))
                {
                    if *seat {
                        occupied_count += 1;
                    }
                }
            }
            if DEBUG {
                println!(
                    "      {},{} has adjacent {} with status {}",
                    position.0, position.1, occupied_count, occupied
                );
            }
            if *occupied {
                if occupied_count >= 4 {
                    *occupied = false;
                    total_occupied -= 1;
                    changed += 1;
                }
            } else if occupied_count == 0 {
                *occupied = true;
                total_occupied += 1;
                changed += 1;
            }
        }
        if DEBUG {
            println!("Pass: {}, Changed {} ", pass, changed);
            pass += 1;
            if pass == 5 {
                break;
            }
        }
    }
    Ok(total_occupied)
}
fn part_2(data: &[String]) -> AoCResult<usize> {
    let mut seats = build_seats_from_input(&data)?;
    let mut changed = 1;
    let mut pass = 1;
    let mut total_occupied = 0;
    while changed != 0 {
        let previous_seats = seats.clone();
        changed = 0;
        // Update seats
        for (position, occupied) in seats.iter_mut() {
            let mut occupied_count = 0;
            // Top, Top Right, Right, Bottom Right, Bottom, Bottom Left, Left, Top Left
            let options: [(i32, i32); 8] = [
                (0, 1),
                (1, 1),
                (1, 0),
                (1, -1),
                (0, -1),
                (-1, -1),
                (-1, 0),
                (-1, 1),
            ];
            for adjust in &options {
                for multiplier in 1..data.len() {
                    if let Some(seat) = previous_seats.get(&(
                        (position.0 + (adjust.0 * multiplier as i32)),
                        (position.1 + (adjust.1 * (multiplier as i32))),
                    )) {
                        if *seat {
                            occupied_count += 1;
                        }
                        break;
                    }
                }
            }
            if DEBUG {
                println!(
                    "      {},{} has adjacent {} with status {}",
                    position.0, position.1, occupied_count, occupied
                );
            }
            if *occupied {
                if occupied_count >= 5 {
                    *occupied = false;
                    total_occupied -= 1;
                    changed += 1;
                }
            } else if occupied_count == 0 {
                *occupied = true;
                total_occupied += 1;
                changed += 1;
            }
        }
        if DEBUG {
            println!("Pass: {}, Changed {} ", pass, changed);
            pass += 1;
            if pass == 10 {
                break;
            }
        }
    }
    Ok(total_occupied)
}
#[cfg(test)]
mod tests {
    use crate::days::day11::{part_1, part_2};
    use crate::misc::read_vec_string;

    #[test]
    fn part_1_test() {
        let data = read_vec_string(String::from("Inputs/test11.txt")).unwrap();
        let result = part_1(&data).unwrap();
        assert_eq!(result, 37);
    }
    #[test]
    fn part_2_test() {
        let data = read_vec_string(String::from("Inputs/test11.txt")).unwrap();
        let result = part_2(&data).unwrap();
        assert_eq!(result, 26);
    }
    #[test]
    fn part_1_input() {
        let data = read_vec_string(String::from("Inputs/input11.txt")).unwrap();
        let result = part_1(&data).unwrap();
        assert_eq!(result, 2412);
    }
    #[test]
    fn part_2_input() {
        let data = read_vec_string(String::from("Inputs/input11.txt")).unwrap();
        let result = part_2(&data).unwrap();
        assert_eq!(result, 2176);
    }
}
