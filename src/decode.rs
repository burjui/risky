//! Decoding facilities

use core::fmt;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::{
    bits::{bitfield, merge_bitfields},
    common::{bimm::BImm, funct3::Funct3, imm12::Imm12, jimm::JImm, opcode::Opcode},
    registers::Register,
};

/// Decode a RISC-V instruction
/// # Errors
/// [`DecodeError`] if any step if decoding fails
pub const fn decode(instruction: u32) -> Result<Instruction, DecodeError> {
    #[allow(clippy::cast_possible_truncation)]
    let opcode = Opcode(bitfield::<0, 7>(instruction) as u8);
    match opcode {
        Opcode::LUI => Ok(Instruction::Lui(U::decode(instruction, opcode))),
        Opcode::AUIPC => Ok(Instruction::Auipc(U::decode(instruction, opcode))),
        Opcode::JAL => Ok(Instruction::Jal(J::decode(instruction, opcode))),
        Opcode::JALR => Ok(Instruction::Jalr(I::decode(instruction, opcode))),

        Opcode::BRANCH => {
            let b = B::decode(instruction, opcode);
            match b.funct3 {
                Funct3::BEQ => Ok(Instruction::Beq(b)),
                Funct3::BNE => Ok(Instruction::Bne(b)),
                Funct3::BLT => Ok(Instruction::Blt(b)),
                Funct3::BLTU => Ok(Instruction::Bltu(b)),
                Funct3::BGE => Ok(Instruction::Bge(b)),
                Funct3::BGEU => Ok(Instruction::Bgeu(b)),
                _ => Err(DecodeError::InvalidFunct3(b.funct3.0)),
            }
        }

        Opcode::LOAD => {
            let i = I::decode(instruction, opcode);
            match i.funct3 {
                Funct3::LB => Ok(Instruction::Lb(i)),
                Funct3::LBU => Ok(Instruction::Lbu(i)),
                Funct3::LH => Ok(Instruction::Lh(i)),
                Funct3::LHU => Ok(Instruction::Lhu(i)),
                Funct3::LW => Ok(Instruction::Lw(i)),
                _ => Err(DecodeError::InvalidFunct3(i.funct3.0)),
            }
        }

        Opcode::STORE => {
            let s = S::decode(instruction, opcode);
            match s.funct3 {
                Funct3::SB => Ok(Instruction::Sb(s)),
                Funct3::SH => Ok(Instruction::Sh(s)),
                Funct3::SW => Ok(Instruction::Sw(s)),
                _ => Err(DecodeError::InvalidFunct3(s.funct3.0)),
            }
        }

        _ => Err(DecodeError::InvalidOpcode(opcode.0)),
    }
}

#[test]
fn invalid_opcode() {
    assert_eq!(decode(0), Err(DecodeError::InvalidOpcode(0)));
}

#[test]
fn invalid_funct3() {
    assert_eq!(
        decode(Opcode::BRANCH.into_u32() | (0b010 << 12)),
        Err(DecodeError::InvalidFunct3(0b010))
    );
    assert_eq!(
        decode(Opcode::LOAD.into_u32() | (0b111 << 12)),
        Err(DecodeError::InvalidFunct3(0b111))
    );
    assert_eq!(
        decode(Opcode::STORE.into_u32() | (0b111 << 12)),
        Err(DecodeError::InvalidFunct3(0b111))
    );
}

/// RISC-V instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instruction {
    ///
    Lui(U),
    ///
    Auipc(U),
    ///
    Jal(J),
    ///
    Jalr(I),
    ///
    Beq(B),
    ///
    Bne(B),
    ///
    Blt(B),
    ///
    Bltu(B),
    ///
    Bge(B),
    ///
    Bgeu(B),
    ///
    Lb(I),
    ///
    Lbu(I),
    ///
    Lh(I),
    ///
    Lhu(I),
    ///
    Lw(I),
    ///
    Sb(S),
    ///
    Sh(S),
    ///
    Sw(S),
}

/// RISC-V U instruction format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct U {
    /// Opcode
    pub opcode: Opcode,
    /// Destination register
    pub rd: Register,
    /// 32-bit signed immediate
    pub imm: i32,
}

impl U {
    const fn decode(instruction: u32, opcode: Opcode) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        let rd = Register(bitfield::<7, 12>(instruction) as u8);
        #[allow(clippy::cast_possible_wrap)]
        let imm = (instruction & 0xFFFF_F000) as i32;
        Self { opcode, rd, imm }
    }
}

/// RISC-V J instruction format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct J {
    /// Opcode
    pub opcode: Opcode,
    /// Destination register
    pub rd: Register,
    /// 21-bit signed immediate
    pub imm: JImm,
}

impl J {
    const fn decode(instruction: u32, opcode: Opcode) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        let rd = Register(bitfield::<7, 12>(instruction) as u8);
        let imm = merge_bitfields(&[
            (12..20, instruction, 12..20),
            (11..12, instruction, 20..21),
            (1..11, instruction, 21..31),
            (20..21, instruction, 31..32),
        ]);
        // Sign-extend
        #[allow(clippy::cast_possible_wrap)]
        let imm = JImm((imm << 11) as i32 >> 11);
        Self { opcode, rd, imm }
    }
}

/// RISC-V I instruction format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct I {
    /// Opcode
    pub opcode: Opcode,
    /// Destination register
    pub rd: Register,
    /// Source register
    pub rs1: Register,
    /// 12-bit signed immediate
    pub imm: Imm12,
    /// Function
    pub funct3: Funct3,
}

impl I {
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    const fn decode(instruction: u32, opcode: Opcode) -> Self {
        let rd = Register(bitfield::<7, 12>(instruction) as u8);
        let funct3 = Funct3(bitfield::<12, 15>(instruction) as u8);
        let rs1 = Register(bitfield::<15, 20>(instruction) as u8);
        let imm = Imm12(((bitfield::<20, 32>(instruction) << 20) as i32 >> 20) as i16);
        Self {
            opcode,
            rd,
            rs1,
            imm,
            funct3,
        }
    }
}

/// RISC-V S instruction format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct S {
    /// Opcode
    pub opcode: Opcode,
    /// Source register
    pub rs1: Register,
    /// 12-bit signed immediate
    pub imm: Imm12,
    /// Destination register
    pub rs2: Register,
    /// Function
    pub funct3: Funct3,
}

impl S {
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    const fn decode(instruction: u32, opcode: Opcode) -> Self {
        let funct3 = Funct3(bitfield::<12, 15>(instruction) as u8);
        let rs1 = Register(bitfield::<15, 20>(instruction) as u8);
        let rs2 = Register(bitfield::<20, 25>(instruction) as u8);
        let imm = merge_bitfields(&[(0..5, instruction, 7..12), (5..12, instruction, 25..32)]);
        let imm = Imm12(((imm << 20) as i32 >> 20) as i16);
        Self {
            opcode,
            rs1,
            imm,
            rs2,
            funct3,
        }
    }
}

/// RISC-V I instruction format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct B {
    /// Opcode
    pub opcode: Opcode,
    /// 13-bit signed immediate
    pub imm: BImm,
    /// Source register 1
    pub rs1: Register,
    /// Source register 2
    pub rs2: Register,
    /// Function
    pub funct3: Funct3,
}

impl B {
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    const fn decode(instruction: u32, opcode: Opcode) -> Self {
        let imm = merge_bitfields(&[
            (11..12, instruction, 7..8),
            (1..5, instruction, 8..12),
            (5..11, instruction, 25..31),
            (12..13, instruction, 31..32),
        ]);
        let funct3 = Funct3(bitfield::<12, 15>(instruction) as u8);
        let rs1 = Register(bitfield::<15, 20>(instruction) as u8);
        let rs2 = Register(bitfield::<20, 25>(instruction) as u8);

        let imm = BImm(((imm << 19) as i32 >> 19) as i16);
        Self {
            opcode,
            imm,
            rs1,
            rs2,
            funct3,
        }
    }
}

/// Instruction decode error
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecodeError {
    ///
    InvalidOpcode(u8),
    ///
    InvalidFunct3(u8),
}

impl Debug for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::InvalidOpcode(opcode) => write!(f, "InvalidOpcode(0b{opcode:08b})"),
            DecodeError::InvalidFunct3(funct3) => write!(f, "InvalidFunct3(0b{funct3:08b})"),
        }
    }
}

#[test]
fn decode_error_debug() {
    assert_eq!(
        format!("{:?}", DecodeError::InvalidOpcode(0b1111_1111)),
        "InvalidOpcode(0b11111111)"
    );
    assert_eq!(
        format!("{:?}", DecodeError::InvalidFunct3(0b1111_1111)),
        "InvalidFunct3(0b11111111)"
    );
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::InvalidOpcode(opcode) => write!(f, "invalid opcode: 0b{opcode:08b}"),
            DecodeError::InvalidFunct3(funct3) => write!(f, "invalid funct3: 0b{funct3:08b}"),
        }
    }
}

#[test]
fn decode_error_display() {
    assert_eq!(
        DecodeError::InvalidOpcode(0b1111_1111).to_string(),
        "invalid opcode: 0b11111111"
    );
    assert_eq!(
        DecodeError::InvalidFunct3(0b1111_1111).to_string(),
        "invalid funct3: 0b11111111"
    );
}

impl Error for DecodeError {}
