use std::{error::Error, fmt::Display};

mod instruction;
use instruction::Instruction;

use crate::instruction::{RInstruction, IInstruction, SInstruction, BInstruction, UInstruction, JInstruction};

const REGISTER_COUNT: usize = 33;
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

#[derive(Debug, PartialEq)]
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
        if self.pc / 4 > self.instructions.len() - 1 {
            return Err(CpuError::PCOutOfBounds);
        }
        let instruction = self.instructions[self.pc / 4].clone();
        let instruction: Instruction = instruction.try_into()?;
        match instruction {
            Instruction::Register { i, rd, rs1, rs2 } => {
                let closure = match i {
                    RInstruction::Add => |a,b| a + b,
                    RInstruction::Sub => |a,b| a - b,
                    RInstruction::Xor => |a,b| a ^ b,
                    RInstruction::Or => |a,b| a | b,
                    RInstruction::And => |a,b| a & b,
                    RInstruction::Sll => |a,b| a << b,
                    RInstruction::Srl => |a,b| (a as u32 >> b as u32) as i32,
                    RInstruction::Sra => |a,b| a >> b,
                    RInstruction::Slt => |a,b| if a < b { 1 } else { 0 },
                    RInstruction::Sltu => |a,b| if (a as u32) < (b as u32) { 1 } else { 0 },
                };
                self.registers[rd as usize] = closure(self.registers[rs1 as usize], self.registers[rs2 as usize]);
            },
            Instruction::Immediate { i, rd, rs1, imm } => {
                // this is disgusting, but I prefer it over bypassing static analysis using nested
                // match statements
                // we heap allocate, because trait objects are not sized, and being inside an
                // Option seems to require that
                let closure: Option<Box<dyn Fn(i32, i32) -> i32>> = match i {
                    IInstruction::Addi => Some(Box::new(|a,b| a + b)),
                    IInstruction::Xori => Some(Box::new(|a,b| a ^ b)),
                    IInstruction::Ori =>  Some(Box::new(|a,b| a | b)),
                    IInstruction::Andi => Some(Box::new(|a,b| a & b)),
                    IInstruction::Slli => Some(Box::new(|a,b| a << b)),
                    IInstruction::Srli => Some(Box::new(|a,b| (a as u32 >> b as u32) as i32)),
                    IInstruction::Srai => Some(Box::new(|a,b| a >> b)),
                    IInstruction::Slti => Some(Box::new(|a,b| if a < b { 1 } else { 0 })),
                    IInstruction::Sltiu => Some(Box::new(|a,b| if (a as u32) < (b as u32) { 1 } else { 0 })),
                    IInstruction::Lb => {
                        self.registers[rd as usize] = self.memory[self.registers[rs1 as usize] as usize + imm as usize] as i8 as i32;
                        None
                    }
                    IInstruction::Lh => {
                        self.registers[rd as usize] = self.memory[self.registers[rs1 as usize] as usize + imm as usize] as i16 as i32;
                        None
                    }
                    IInstruction::Lw => {
                        self.registers[rd as usize] = self.memory[self.registers[rs1 as usize] as usize + imm as usize];
                        None
                    }
                    IInstruction::Lbu => {
                        self.registers[rd as usize] = self.memory[self.registers[rs1 as usize] as usize + imm as usize] as u8 as i32;
                        None
                    }
                    IInstruction::Lhu => {
                        self.registers[rd as usize] = self.memory[self.registers[rs1 as usize] as usize + imm as usize] as u16 as i32;
                        None
                    }
                    IInstruction::Jalr => {
                        self.registers[rd as usize] = self.pc as i32 + 4;
                        self.pc = self.registers[rs1 as usize + imm as usize] as usize;
                        return Ok(());
                    }
                };
                if let Some(callable_closure) = closure {
                    self.registers[rd as usize] = callable_closure(self.registers[rs1 as usize], imm);
                }
            },
            Instruction::Store { i, rs1, rs2, imm } => {
                match i {
                    SInstruction::Sb => {
                        let old = self.memory[rs1 as usize + imm as usize];
                        self.memory[rs1 as usize + imm as usize] = (old ^ (old as u8) as i32) + self.registers[rs2 as usize] as i8 as i32;
                    }
                    SInstruction::Sh => {
                        let old = self.memory[rs1 as usize + imm as usize];
                        self.memory[rs1 as usize + imm as usize] = (old ^ (old as u16) as i32) + self.registers[rs2 as usize] as i16 as i32;
                    }
                    SInstruction::Sw => {
                        self.memory[rs1 as usize + imm as usize] = self.registers[rs2 as usize];
                    }
                }
            },
            Instruction::Branch { i, rs1, rs2, imm } => {
                match i {
                    BInstruction::Beq => {
                        if self.registers[rs1 as usize] == self.registers[rs2 as usize] {
                            self.pc += imm as usize;
                            return Ok(()); // avoid further program counter incrementation
                        }
                    }
                    BInstruction::Bne => {
                        if self.registers[rs1 as usize] != self.registers[rs2 as usize] {
                            self.pc += imm as usize;
                            return Ok(());
                        }
                    }
                    BInstruction::Blt => {
                        if self.registers[rs1 as usize] < self.registers[rs2 as usize] {
                            self.pc += imm as usize;
                            return Ok(());
                        }
                    }
                    BInstruction::Bge => {
                        if self.registers[rs1 as usize] >= self.registers[rs2 as usize] {
                            self.pc += imm as usize;
                            return Ok(());
                        }
                    }
                    BInstruction::Bltu => {
                        if (self.registers[rs1 as usize] as u32) < self.registers[rs2 as usize] as u32 {
                            self.pc += imm as usize;
                            return Ok(());
                        }
                    }
                    BInstruction::Bgeu => {
                        if self.registers[rs1 as usize] as u32 >= self.registers[rs2 as usize] as u32 {
                            self.pc += imm as usize;
                            return Ok(());
                        }
                    }
                }
            },
            Instruction::UpperImmediate { i, rd, imm }=> {
                match i {
                    UInstruction::Lui => {
                        self.registers[rd as usize] = imm << 12;
                    }
                    UInstruction::Auipc => {
                        self.registers[rd as usize] = self.pc as i32 + (imm << 12);
                    }
                }
            },
            Instruction::Jump { i, rd, imm } => {
                match i {
                    JInstruction::Jal => {
                        self.registers[rd as usize] = self.pc as i32 + 4;
                        self.pc += imm as usize;
                        return Ok(());
                    }
                }
            },
        }
        self.pc += 4;
        Ok(())
    }
    pub fn memory(&self) -> Box<[i32; MEMORY_SIZE]> {
        self.memory.clone()
    }
    pub fn registers(&self) -> Box<[i32; REGISTER_COUNT]> {
        self.registers.clone()
    }
}
