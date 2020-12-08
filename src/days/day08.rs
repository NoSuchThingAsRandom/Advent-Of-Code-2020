use crate::computer::{CommandType, Computer};
use crate::misc::error::AoCResult;
use crate::misc::read_vec_string;

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
    for (index, instruction) in computer.memory.iter().enumerate() {
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
                    }
                });
            }
            _ => {}
        }
    }
    while pool.active_count() > 0 {
        sleep(Duration::from_secs(2));
    }
    Ok(0)
}
