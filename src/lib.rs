use std::{error::Error, fmt::Display};

mod instruction;
use instruction::Instruction;

use crate::instruction::{RInstruction, IInstruction, SInstruction, BInstruction, UInstruction, JInstruction};

const REGISTER_COUNT: usize = 32;
const MEMORY_SIZE: usize = 2048;

#[derive(Debug)]
pub struct Cpu {
    registers: Box<[i32; REGISTER_COUNT]>,
    memory: Box<[i32; MEMORY_SIZE]>,
    pub pc: usize,
    pub instructions: Vec<String>
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            registers: Box::new([0; REGISTER_COUNT]),
            memory: Box::new([0; MEMORY_SIZE]),
            pc: 0,
            instructions: Vec::new()
        }
    }
}

#[derive(Debug)]
pub enum CpuError {
    InvalidOpcode,
    InvalidFunct,
    InvalidRd,
    InvalidRs1,
    InvalidRs2,
    InvalidImm,
    InvalidFormat,
    PCOutOfBounds,
}

impl Display for CpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CpuError::InvalidOpcode => f.write_str("invalid opcode"),
            CpuError::InvalidFunct => f.write_str("invalid function"),
            CpuError::InvalidRd => f.write_str("invalid destination register"),
            CpuError::InvalidRs1 => f.write_str("invalid supply register 1"),
            CpuError::InvalidRs2 => f.write_str("invalid supply register 2"),
            CpuError::InvalidImm => f.write_str("invalid immediate"),
            CpuError::InvalidFormat => f.write_str("invalid format"),
            CpuError::PCOutOfBounds => f.write_str("program counter out of bounds"),
        }
    }
}

impl Error for CpuError {}

impl Cpu {
    pub fn new(instructions: Vec<String>) -> Self {
        Self {
            registers: Box::new([0; REGISTER_COUNT]),
            memory: Box::new([0; MEMORY_SIZE]),
            pc: 0,
            instructions
        }
    }
    pub fn execute(&mut self) -> Result<(), CpuError> {
        if self.pc >= self.instructions.len() {
            return Err(CpuError::PCOutOfBounds);
        }
        let instruction = self.instructions[self.pc].clone();
        let instruction: Instruction = instruction.try_into()?;
        dbg!(&instruction);
        match instruction {
            Instruction::Register { i, rd, rs1, rs2 } => {
                match i {
                    RInstruction::Add => {
                        self.registers[rd as usize] = self.registers[rs1 as usize] + self.registers[rs2 as usize];
                    }
                    _ => println!("{:?} not supported yet", i)
                }
            },
            Instruction::Immediate { i, rd, rs1, imm } => {
                match i {
                    IInstruction::Addi => {
                        self.registers[rd as usize] = self.registers[rs1 as usize] + imm;
                    }
                    _ => println!("{:?} not supported yet", i)
                }
            },
            Instruction::Store { i, rs1, rs2, imm } => {
                match i {
                    SInstruction::Sb => {
                        
                    }
                    _ => println!("{:?} not supported yet", i)
                }
            },
            Instruction::Branch { i, rs1, rs2, imm } => {
                match i {
                    BInstruction::Beq => {
                        if self.registers[rs1 as usize] == self.registers[rs2 as usize] {
                            self.pc += imm as usize;
                        }
                    }
                    _ => println!("{:?} not supported yet", i)
                }
            },
            Instruction::UpperImmediate { i, rd, imm }=> {
                match i {
                    UInstruction::Lui => {

                    }
                    _ => println!("{:?} not supported yet", i)
                }
            },
            Instruction::Jump { i, rd, imm } => {
                match i {
                    JInstruction::Jal => {

                    }
                    _ => println!("{:?} not supported yet", i)
                }
            },
            _ => println!("{:?} not supported yet", instruction)
        }
        self.pc += 1;
        Ok(())
    }
    pub fn memory(&self) -> Box<[i32; MEMORY_SIZE]> {
        self.memory.clone()
    }
    pub fn registers(&self) -> Box<[i32; REGISTER_COUNT]> {
        self.registers.clone()
    }
}
