use crate::misc::error::{AoCError, AoCResult};
use crate::misc::read_vec_string;

pub fn run() -> AoCResult<usize> {
    let original = read_vec_string(String::from("Inputs/input05.txt")).unwrap();
    println!("    Highest Seat ID: {}", part_1(&original).unwrap());
    println!("    Seats: {}", part_2(&original).unwrap());
    Ok(0)
}

fn get_seat_id(ticket: &str) -> usize {
    let mut lowest_row = 0;
    let mut highest_row = 127;
    let mut lowest_col = 0;
    let mut highest_col = 7;
    for part in ticket.chars() {
        if part.eq(&'F') {
            highest_row = ((highest_row - lowest_row) / 2) as usize + lowest_row;
        } else if part.eq(&'B') {
            lowest_row = ((highest_row - lowest_row) / 2) as usize + lowest_row + 1;
        } else if part.eq(&'L') {
            highest_col = ((highest_col - lowest_col) / 2) as usize + lowest_col;
        } else if part.eq(&'R') {
            lowest_col = ((highest_col - lowest_col) / 2) as usize + lowest_col + 1;
        }
    }
    (lowest_row * 8) + lowest_col
}

pub fn part_1(data: &[String]) -> AoCResult<usize> {
    let mut highest_seat = 0;
    for ticket in data {
        //println!("Seat  pos: {}, {}, {}", lowest_row, lowest_col, seat_id);
        let seat_id = get_seat_id(ticket);
        if seat_id > highest_seat {
            highest_seat = seat_id;
        }
    }
    Ok(highest_seat)
}
pub fn part_2(data: &[String]) -> AoCResult<usize> {
    let mut seats: Vec<usize> = data.iter().map(|ticket| get_seat_id(ticket)).collect();
    seats.sort();
    let mut index = *seats.get(0).unwrap();
    for seat in seats {
        if seat != index {
            return Ok(index);
        }
        index += 1;
    }
    Err(AoCError::new(String::from("Couldn't get seat!")))
}
#[cfg(test)]
mod tests {
    use crate::days::day05::{get_seat_id, part_1, part_2};
    use crate::misc::read_vec_string;

    #[test]
    fn check_seat_id() {
        let data = read_vec_string(String::from("Inputs/test05.txt")).unwrap();
        let values: [usize; 4] = [357, 567, 119, 820];
        for (index, seat) in data.iter().enumerate() {
            assert_eq!(get_seat_id(seat), values[index]);
        }
    }

    #[test]
    fn part_1_test() {
        let data = read_vec_string(String::from("Inputs/test05.txt")).unwrap();
        let res = part_1(&data);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 820);
    }
    #[test]
    fn part_1_input() {
        let data = read_vec_string(String::from("Inputs/input05.txt")).unwrap();
        let res = part_1(&data);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 848);
    }
    #[test]
    fn part_2_input() {
        let data = read_vec_string(String::from("Inputs/input05.txt")).unwrap();
        let res = part_2(&data);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 682);
    }
}
