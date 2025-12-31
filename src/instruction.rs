use crate::CpuError;

#[derive(Debug)]
pub enum RInstruction {
    Add,
    Sub,
    Xor,
    Or,
    And,
    Sll,
    Srl,
    Sra,
    Slt,
    Sltu,
}
#[derive(Debug)]
pub enum IInstruction {
    Addi,
    Xori,
    Ori,
    Andi,
    Slli,
    Srli,
    Srai,
    Slti,
    Sltiu,
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
    Jalr,
}
#[derive(Debug)]
pub enum SInstruction {
    Sb,
    Sh,
    Sw,
}
#[derive(Debug)]
pub enum BInstruction {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}
#[derive(Debug)]
pub enum JInstruction {
    Jal,
}
#[derive(Debug)]
pub enum UInstruction {
    Lui,
    Auipc,
}

#[derive(Debug)]
pub enum Instruction {
    Register {
        i: RInstruction,
        rd: u32,
        rs1: u32,
        rs2: u32,
    },
    Immediate {
        i: IInstruction,
        rd: u32,
        rs1: u32,
        imm: i32
    },
    Store {
        i: SInstruction,
        rs1: u32,
        rs2: u32,
        imm: i32,
    },
    Branch {
        i: BInstruction,
        rs1: u32,
        rs2: u32,
        imm: i32,
    },
    UpperImmediate {
        i: UInstruction,
        rd: u32,
        imm: i32
    },
    Jump {
        i: JInstruction,
        rd: u32,
        imm: i32
    }
}

impl TryFrom<String> for Instruction {
    type Error = CpuError;
    fn try_from(instruction: String) -> Result<Self, Self::Error> {
        if instruction.len() != 32 {
            return Err(CpuError::InvalidFormat);
        }
        let opcode = &instruction[25..=31];
        let f3 = &instruction[17..=19];
        let f7 = &instruction[0..=6];
        match opcode {
            "0110011" => {
                // R
                let rd = u32::from_str_radix(&instruction[20..=24], 2).map_err(|_| CpuError::InvalidRd)?;
                let rs1 = u32::from_str_radix(&instruction[12..=16], 2).map_err(|_| CpuError::InvalidRs1)?;
                let rs2 = u32::from_str_radix(&instruction[7..=11], 2).map_err(|_| CpuError::InvalidRs2)?;
                let i = match (f3, f7) {
                    ("000", "0000000") => Ok(RInstruction::Add),
                    ("000", "0100000") => Ok(RInstruction::Sub),
                    ("004", "0000000") => Ok(RInstruction::Xor),
                    ("006", "0000000") => Ok(RInstruction::Or),
                    ("007", "0000000") => Ok(RInstruction::And),
                    ("001", "0000000") => Ok(RInstruction::Sll),
                    ("005", "0000000") => Ok(RInstruction::Srl),
                    ("005", "0100000") => Ok(RInstruction::Sra),
                    ("002", "0000000") => Ok(RInstruction::Slt),
                    ("003", "0000000") => Ok(RInstruction::Sltu),
                    (&_, &_) => Err(CpuError::InvalidFunct)
                }?;
                Ok(Instruction::Register { i, rd, rs1, rs2 })
            },
            "0010011" | "0000011" | "1100111" => {
                // I
                let rd = u32::from_str_radix(&instruction[20..=24], 2).map_err(|_| CpuError::InvalidRd)?;
                let rs1 = u32::from_str_radix(&instruction[12..=16], 2).map_err(|_| CpuError::InvalidRs1)?;
                let imm = i32::from_str_radix(&instruction[0..=11], 2).map_err(|_| CpuError::InvalidImm)?;
                let i = match opcode {
                    "0010011" => {
                        match (f3, f7) {
                            ("000", &_) => Ok(IInstruction::Addi),
                            ("004", &_) => Ok(IInstruction::Xori),
                            ("006", &_) => Ok(IInstruction::Ori),
                            ("007", &_) => Ok(IInstruction::Andi),
                            ("001", "0000000") => Ok(IInstruction::Slli),
                            ("005", "0000000") => Ok(IInstruction::Srli),
                            ("005", "0100000") => Ok(IInstruction::Srai),
                            ("002", &_) => Ok(IInstruction::Slti),
                            ("003", &_) => Ok(IInstruction::Sltiu),
                            (&_, &_) => Err(CpuError::InvalidFunct)
                        }
                    },
                    "0000011" => {
                        match f3 {
                            "000" => Ok(IInstruction::Lb),
                            "001" => Ok(IInstruction::Lh),
                            "002" => Ok(IInstruction::Lw),
                            "004" => Ok(IInstruction::Lbu),
                            "005" => Ok(IInstruction::Lhu),
                            &_ => Err(CpuError::InvalidFunct)
                        }
                    },
                    "1100111" => Ok(IInstruction::Jalr),
                    &_ => unreachable!()
                }?;
                Ok(Instruction::Immediate { i, rd, rs1, imm })
            },
            "0100011" => {
                // S
                let rs1 = u32::from_str_radix(&instruction[12..=16], 2).map_err(|_| CpuError::InvalidRs1)?;
                let rs2 = u32::from_str_radix(&instruction[7..=11], 2).map_err(|_| CpuError::InvalidRs2)?;
                let imm = format!("{}{}", &instruction[0..=6], &instruction[20..=24]);
                let imm = i32::from_str_radix(&imm, 2).map_err(|_| CpuError::InvalidImm)?;
                let i = match f3 {
                    "000" => Ok(SInstruction::Sb),
                    "001" => Ok(SInstruction::Sh),
                    "002" => Ok(SInstruction::Sw),
                    &_ => Err(CpuError::InvalidFunct)
                }?;
                Ok(Instruction::Store { i, rs1, rs2, imm })
            },
            "1100011" => {
                // B
                let rs1 = u32::from_str_radix(&instruction[12..16], 2).map_err(|_| CpuError::InvalidRs1)?;
                let rs2 = u32::from_str_radix(&instruction[7..11], 2).map_err(|_| CpuError::InvalidRs2)?;
                let imm = format!("{}{}{}{}0", instruction.chars().nth(0).expect("len == 32"), &instruction.chars().nth(24).expect("len == 32"), &instruction[1..=6], &instruction[20..=23]);
                let imm = i32::from_str_radix(&imm, 2).map_err(|_| CpuError::InvalidImm)?;
                let i = match f3 {
                    "000" => Ok(BInstruction::Beq),
                    "001" => Ok(BInstruction::Bne),
                    "004" => Ok(BInstruction::Blt),
                    "005" => Ok(BInstruction::Bge),
                    "006" => Ok(BInstruction::Bltu),
                    "007" => Ok(BInstruction::Bgeu),
                    &_ => Err(CpuError::InvalidFunct)
                }?;
                Ok(Instruction::Branch { i, rs1, rs2, imm })
            },
            "1101111" => {
                // J
                let rd = u32::from_str_radix(&instruction[20..=24], 2).map_err(|_| CpuError::InvalidRd)?;
                let imm = format!("{}{}{}{}0", instruction.chars().nth(0).expect("len == 32"), &instruction[12..=19], instruction.chars().nth(11).expect("len == 32"), &instruction[1..=10]);
                let imm = i32::from_str_radix(&imm, 2).map_err(|_| CpuError::InvalidImm)?;
                Ok(Instruction::Jump { i: JInstruction::Jal, rd, imm })
            },
            "0110111" | "0010111" => {
                // U
                let rd = u32::from_str_radix(&instruction[20..=24], 2).map_err(|_| CpuError::InvalidRd)?;
                let imm = i32::from_str_radix(&instruction[..=19], 2).map_err(|_| CpuError::InvalidImm)?;
                let i = match opcode {
                    "0110111" => Ok(UInstruction::Lui),
                    "0010111" => Ok(UInstruction::Auipc),
                    &_ => unreachable!()
                }?;
                Ok(Instruction::UpperImmediate { i, rd, imm })
            }
            &_ => Err(CpuError::InvalidOpcode)
        }
    }
}
