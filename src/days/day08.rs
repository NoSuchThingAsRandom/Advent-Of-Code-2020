use crate::computer::{CommandType, Computer};
use crate::misc::error::AoCResult;
use crate::misc::read_vec_string;

use std::sync::mpsc::channel;
use std::thread::sleep;
use std::time::Duration;
use threadpool::ThreadPool;

pub fn run() -> AoCResult<usize> {
    println!("Test");
    let data = read_vec_string(String::from("Inputs/input08.txt")).unwrap();
    println!("Accumulator value: {}", part_1(&data).unwrap());
    part_2(&data).unwrap();
    Ok(0)
}

fn part_1(data: &[String]) -> AoCResult<i32> {
    let mut computer = Computer::new_from_text(&data);
    computer.loop_until_repeat();
    computer.undo();
    Ok(computer.get_accumatalator_value())
}
fn part_2(data: &[String]) -> AoCResult<u16> {
    let pool = ThreadPool::new(8);
    let computer = Computer::new_from_text(data);
    let (tx, rx) = channel();
    for (index, instruction) in computer.memory.iter().enumerate() {
        let tx = tx.clone();
        match &instruction.opcode {
            CommandType::Nop => {
                let mut new = computer.clone();
                pool.execute(move || {
                    new.memory.get_mut(index).unwrap().opcode = CommandType::Jmp;
                    if new.does_terminate() {
                        println!(
                            "Valid solution: [index: {}, accum: {}]",
                            index,
                            new.get_accumatalator_value()
                        );
                    }
                });
            }
            CommandType::Jmp => {
                let mut new = computer.clone();
                pool.execute(move || {
                    new.memory.get_mut(index).unwrap().opcode = CommandType::Nop;
                    if new.does_terminate() {
                        println!(
                            "Valid solution: [index: {}, accum: {}]",
                            index,
                            new.get_accumatalator_value()
                        );
                        tx.send(new.get_accumatalator_value()).unwrap();
                    }
                });
            }
            _ => {}
        }
    }
    while pool.active_count() > 0 {}
    Ok(rx.recv().unwrap() as u16)
}
#[cfg(test)]
mod tests {
    use crate::days::day08::{part_1, part_2};
    use crate::misc::read_vec_string;

    #[test]
    fn part_1_test() {
        let data = read_vec_string(String::from("Inputs/test08a.txt")).unwrap();
        let res = part_1(&data);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 5);
    }
    #[test]
    fn part_2_test() {
        let data = read_vec_string(String::from("Inputs/test08a.txt")).unwrap();
        let res = part_2(&data);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 8);
    }
    #[test]
    fn part_1_input() {
        let data = read_vec_string(String::from("Inputs/input08.txt")).unwrap();
        let res = part_1(&data);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 1553);
    }
    #[test]
    fn part_2_input() {
        let data = read_vec_string(String::from("Inputs/input08.txt")).unwrap();
        let res = part_2(&data);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, 1877);
    }
}
