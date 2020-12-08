mod computer_error;
use crate::computer::computer_error::{ComputerError, ComputerResult, ErrorKind};
use crate::computer::CommandType::{Acc, Jmp, Nop};

use std::collections::HashSet;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Clone)]
pub struct Computer {
    instruction_pointer: u32,
    accumulator: i32,
    pub(crate) memory: Vec<Instruction>,
    previous_state: Option<ComputerState>,
}
impl Computer {
    pub fn new_from_text(data: &[String]) -> Computer {
        let mut memory = Vec::new();
        for (id_count, line) in data.iter().enumerate() {
            //Instruction::new_from_line(line, id_count as u32).unwrap();
            match Instruction::new_from_line(line, id_count as u32) {
                Ok(instruction) => memory.push(instruction),
                Err(e) => println!("Failed instruction {}", e),
            }
        }
        Computer {
            instruction_pointer: 0,
            accumulator: 0,
            memory,
            previous_state: None,
        }
    }
    pub fn get_accumatalator_value(&self) -> i32 {
        self.accumulator
    }
    /// Attempts to revert the last instruction
    pub fn undo(&mut self) -> Option<ComputerState> {
        if let Some(state) = &self.previous_state {
            match state.executed_instruction.opcode {
                Nop => {
                    self.instruction_pointer = self.instruction_pointer.checked_sub(1)?;
                }
                Acc => {
                    self.accumulator = self
                        .accumulator
                        .checked_sub(state.executed_instruction.operand)?;
                    self.instruction_pointer = self.instruction_pointer.checked_sub(1)?;
                }
                Jmp => {
                    self.instruction_pointer = if state.executed_instruction.operand > 0 {
                        self.instruction_pointer
                            .checked_sub(state.executed_instruction.operand as u32)?
                    } else {
                        self.instruction_pointer
                            .checked_add(state.executed_instruction.operand.abs() as u32)?
                    };
                }
            };
        }
        Some(ComputerState {
            accumulator_value: self.accumulator,
            executed_instruction: self.memory.get(self.instruction_pointer as usize)?.clone(),
        })
    }

    /// Executes instructions, until it repeats a previously executed instruction
    pub fn loop_until_repeat(&mut self) {
        let mut visited = HashSet::new();
        for next in self {
            if !visited.insert(next.executed_instruction.id) {
                return;
            }
        }
    }
    /// Returns true if the program terminates
    /// False if it loops on an instruction
    pub fn does_terminate(&mut self) -> bool {
        let mut visited = HashSet::new();
        for next in self {
            if !visited.insert(next.executed_instruction.id) {
                return false;
            }
        }
        true
    }
}

impl Iterator for Computer {
    type Item = ComputerState;

    fn next(&mut self) -> Option<Self::Item> {
        let instruction = self.memory.get(self.instruction_pointer as usize)?;
        match instruction.opcode {
            Nop => {
                self.instruction_pointer = self.instruction_pointer.checked_add(1)?;
            }
            Acc => {
                self.accumulator = self.accumulator.checked_add(instruction.operand)?;
                self.instruction_pointer = self.instruction_pointer.checked_add(1)?;
            }
            Jmp => {
                self.instruction_pointer = if instruction.operand > 0 {
                    self.instruction_pointer
                        .checked_add(instruction.operand as u32)?
                } else {
                    self.instruction_pointer
                        .checked_sub(instruction.operand.abs() as u32)?
                };
            }
        }
        let state = ComputerState {
            accumulator_value: self.accumulator,
            executed_instruction: instruction.clone(),
        };
        self.previous_state = Some(state.clone());
        Some(state)
    }
}
#[derive(Clone)]
pub struct ComputerState {
    pub(crate) accumulator_value: i32,
    pub(crate) executed_instruction: Instruction,
}
#[derive(Clone)]
pub struct Instruction {
    pub opcode: CommandType,
    pub operand: i32,
    pub id: u32,
}
impl Instruction {
    fn new_from_line(line: &str, id: u32) -> ComputerResult<Instruction> {
        let mut parts = line.split(' ');
        let opcode: CommandType = parts.next().unwrap().parse()?;
        let operand: i32 = parts.next().unwrap().parse()?;
        Ok(Instruction {
            opcode,
            operand,
            id,
        })
    }
}
impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[ID: {}, Opcode: {}, Operand: {}]",
            self.id, self.opcode, self.operand
        )
    }
}
#[derive(Clone, PartialEq)]
pub enum CommandType {
    Nop,
    Acc,
    Jmp,
}
impl FromStr for CommandType {
    type Err = ComputerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nop" => Ok(Nop),
            "acc" => Ok(Acc),
            "jmp" => Ok(Jmp),
            _ => Err(ComputerError::from(ErrorKind::InputParse(format!(
                "Invalid instruction {}",
                s,
            )))),
        }
    }
}
impl Display for CommandType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Nop => {
                    "Nop"
                }
                Acc => {
                    "Acc"
                }
                Jmp => {
                    "Jmp"
                }
            }
        )
    }
}
