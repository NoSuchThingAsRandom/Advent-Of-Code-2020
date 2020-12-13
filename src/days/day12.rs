use crate::days::day12::Direction::{E, N, S, W};
use crate::misc::error::{AoCError, AoCResult};
use crate::misc::read_vec_string;
use std::str::FromStr;

const DEBUG: bool = false;
pub fn run() {
    //let data = read_vec_string(String::from("Inputs/test12.txt")).unwrap();
    let data = read_vec_string(String::from("Inputs/input12.txt")).unwrap();
    println!("Part 1: {}", part_1(&data).unwrap());
    println!("Part 2: {}", part_2(&data).unwrap());
}
#[derive(Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}
impl Direction {
    fn rotate(&mut self, degrees: usize) -> Direction {
        match self {
            Direction::N => match (degrees / 90) % 4 {
                0 => N,
                1 => E,
                2 => S,
                3 => W,
                _ => {
                    panic!("Reached limit");
                }
            },
            Direction::E => match (degrees / 90) % 4 {
                0 => E,
                1 => S,
                2 => W,
                3 => N,
                _ => {
                    panic!("Reached limit");
                }
            },
            Direction::S => match (degrees / 90) % 4 {
                0 => S,
                1 => W,
                2 => N,
                3 => E,
                _ => {
                    panic!("Reached limit");
                }
            },
            Direction::W => match (degrees / 90) % 4 {
                0 => W,
                1 => N,
                2 => E,
                3 => S,
                _ => {
                    panic!("Reached limit");
                }
            },
        }
    }

    fn apply(&self, amount: i32, current_position: &(i32, i32)) -> AoCResult<(i32, i32)> {
        match self {
            N => Ok((
                current_position.0,
                AoCError::from_option(current_position.1.checked_add(amount))?,
            )),
            E => Ok((
                AoCError::from_option(current_position.0.checked_add(amount))?,
                current_position.1,
            )),
            S => Ok((
                current_position.0,
                AoCError::from_option(current_position.1.checked_sub(amount))?,
            )),
            W => Ok((
                AoCError::from_option(current_position.0.checked_sub(amount))?,
                current_position.1,
            )),
        }
    }
    fn from_int(x: &str) -> AoCResult<Direction> {
        match x.to_string().parse::<usize>() {
            Ok(0) => Ok(N),
            Ok(90) => Ok(E),
            Ok(180) => Ok(S),
            Ok(270) => Ok(W),
            _ => Err(AoCError::new(String::from(
                "Not a valid multiple of degrees",
            ))),
        }
    }
}
impl FromStr for Direction {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().unwrap() {
            'N' => Ok(N),
            'E' => Ok(E),
            'W' => Ok(W),
            'S' => Ok(S),
            _ => Err(AoCError::new(String::from("Invalid Character"))),
        }
    }
}

fn part_1(data: &[String]) -> AoCResult<usize> {
    let mut position: (i32, i32) = (0, 0);
    let mut direction = Direction::E;
    for instruction in data {
        let mut parts = instruction.chars();
        let action = AoCError::from_option(parts.next())?.to_string();
        let amount: usize = parts.collect::<String>().parse().unwrap();
        if let Ok(new_direction) = Direction::from_str(&action) {
            position = new_direction.apply(amount as i32, &position)?;
        } else if action.eq("L") {
            direction = direction.rotate(360 - (amount % 360));
        } else if action.eq("R") {
            direction = direction.rotate(amount);
        } else if action.eq("F") {
            position = direction.apply(amount as i32, &position)?;
        } else if action.eq("B") {
            position = direction.apply(-(amount as i32), &position)?;
        } else {
            unimplemented!("Unknown action")
        }
        if DEBUG {
            println!("Position: {:?}", position);
            println!("Direction: {:?}\n", direction);
        }
    }
    Ok((position.0.abs() + position.1.abs()) as usize)
}
fn rotate_waypoint(waypoint: &(i32, i32), degrees: usize) -> (i32, i32) {
    match degrees % 360 {
        0 => (waypoint.0, waypoint.1),
        90 => (-waypoint.1, waypoint.0),
        180 => (-waypoint.0, -waypoint.1),
        270 => (waypoint.1, -waypoint.0),
        _ => unimplemented!("Cannot rotate in amounts less than 90 degrees"),
    }
}
fn part_2(data: &[String]) -> AoCResult<usize> {
    let mut ship: (i32, i32) = (0, 0);
    let mut waypoint: (i32, i32) = (10, 1);
    for instruction in data {
        let mut parts = instruction.chars();
        let action = AoCError::from_option(parts.next())?.to_string();
        let amount: usize = parts.collect::<String>().parse().unwrap();
        if let Ok(new_direction) = Direction::from_str(&action) {
            waypoint = new_direction.apply(amount as i32, &waypoint)?;
        } else if action.eq("L") {
            waypoint = rotate_waypoint(&waypoint, amount % 360);
        } else if action.eq("R") {
            waypoint = rotate_waypoint(&waypoint, 360 - (amount % 360));
        } else if action.eq("F") {
            ship.0 += waypoint.0 * amount as i32;
            ship.1 += waypoint.1 * amount as i32;
        } else if action.eq("B") {
            ship.0 -= waypoint.0 * amount as i32;
            ship.1 -= waypoint.1 * amount as i32;
        } else {
            unimplemented!("Unknown action")
        }
        if DEBUG {
            println!("Ship: {:?}", ship);
            println!("Waypoint: {:?}\n", waypoint);
        }
    }
    Ok((ship.0.abs() + ship.1.abs()) as usize)
}
#[cfg(test)]
mod tests {
    use crate::days::day12::{part_1, part_2};
    use crate::misc::read_vec_string;

    #[test]
    fn part_1_test() {
        let data = read_vec_string(String::from("Inputs/test12.txt")).unwrap();
        let result = part_1(&data).unwrap();
        assert_eq!(result, 25);
    }
    #[test]
    fn part_1_input() {
        let data = read_vec_string(String::from("Inputs/input12.txt")).unwrap();
        let result = part_1(&data).unwrap();
        assert_eq!(result, 1496);
    }
    #[test]
    fn part_2_test() {
        let data = read_vec_string(String::from("Inputs/test12.txt")).unwrap();
        let result = part_2(&data).unwrap();
        assert_eq!(result, 286);
    }
    #[test]
    fn part_2_input() {
        let data = read_vec_string(String::from("Inputs/input12.txt")).unwrap();
        let result = part_2(&data).unwrap();
        assert_eq!(result, 63843);
    }
}
