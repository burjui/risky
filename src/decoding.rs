//! Decoding facilities

use core::fmt;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::{
    common::{
        bimm::BImm, csr::Csr, fence_mask::FenceMask, fence_mode::FenceMode, funct12::Funct12,
        funct3::Funct3, funct7::Funct7, imm12::Imm12, jimm::JImm, opcode::Opcode, uimm5::Uimm5,
    },
    instruction::{CsrImm, CsrReg, IShift, Instruction, B, I, J, R, S, U},
    registers::Register,
    util::bits::{bitfield, combine_bitfields},
};

/// Decodes a RISC-V instruction
#[allow(clippy::missing_errors_doc)]
pub const fn decode(instruction: u32) -> Result<Instruction, DecodeError> {
    match decode_opcode(instruction) {
        Opcode::LUI => Ok(Instruction::Lui(U::decode(instruction))),
        Opcode::AUIPC => Ok(Instruction::Auipc(U::decode(instruction))),
        Opcode::JAL => Ok(Instruction::Jal(J::decode(instruction))),
        Opcode::JALR => Ok(Instruction::Jalr(I::decode(instruction))),
        Opcode::BRANCH => decode_branch(instruction),
        Opcode::LOAD => decode_load(instruction),
        Opcode::STORE => decode_store(instruction),
        Opcode::OP_IMM => decode_op_imm(instruction),
        Opcode::OP => decode_op(instruction),
        Opcode::MISC_MEM => decode_fence(instruction),
        Opcode::SYSTEM => decode_system(instruction),
        opcode => Err(DecodeError::InvalidOpcode(opcode)),
    }
}

const fn decode_branch(instruction: u32) -> Result<Instruction, DecodeError> {
    let b = B::decode(instruction);
    match decode_funct3(instruction) {
        Funct3::BEQ => Ok(Instruction::Beq(b)),
        Funct3::BNE => Ok(Instruction::Bne(b)),
        Funct3::BLT => Ok(Instruction::Blt(b)),
        Funct3::BLTU => Ok(Instruction::Bltu(b)),
        Funct3::BGE => Ok(Instruction::Bge(b)),
        Funct3::BGEU => Ok(Instruction::Bgeu(b)),
        funct3 => Err(DecodeError::InvalidFunct3(funct3)),
    }
}

const fn decode_load(instruction: u32) -> Result<Instruction, DecodeError> {
    let i = I::decode(instruction);
    match decode_funct3(instruction) {
        Funct3::LB => Ok(Instruction::Lb(i)),
        Funct3::LBU => Ok(Instruction::Lbu(i)),
        Funct3::LH => Ok(Instruction::Lh(i)),
        Funct3::LHU => Ok(Instruction::Lhu(i)),
        Funct3::LW => Ok(Instruction::Lw(i)),
        funct3 => Err(DecodeError::InvalidFunct3(funct3)),
    }
}

const fn decode_store(instruction: u32) -> Result<Instruction, DecodeError> {
    let s = S::decode(instruction);
    match decode_funct3(instruction) {
        Funct3::SB => Ok(Instruction::Sb(s)),
        Funct3::SH => Ok(Instruction::Sh(s)),
        Funct3::SW => Ok(Instruction::Sw(s)),
        funct3 => Err(DecodeError::InvalidFunct3(funct3)),
    }
}

const fn decode_op_imm(instruction: u32) -> Result<Instruction, DecodeError> {
    match decode_funct3(instruction) {
        Funct3::ADDI => Ok(Instruction::Addi(I::decode(instruction))),
        Funct3::SLTI => Ok(Instruction::Slti(I::decode(instruction))),
        Funct3::SLTIU => Ok(Instruction::Sltiu(I::decode(instruction))),
        Funct3::XORI => Ok(Instruction::Xori(I::decode(instruction))),
        Funct3::ORI => Ok(Instruction::Ori(I::decode(instruction))),
        Funct3::ANDI => Ok(Instruction::Andi(I::decode(instruction))),
        Funct3::SLLI => Ok(Instruction::Slli(IShift::decode(instruction))),

        Funct3::SRL => match decode_shift_mode(instruction) {
            SHIFT_MODE_SRLI => Ok(Instruction::Srli(IShift::decode(instruction))),
            SHIFT_MODE_SRAI => Ok(Instruction::Srai(IShift::decode(instruction))),
            funct7 => Err(DecodeError::InvalidShiftMode(funct7)),
        },

        _ => unreachable!(), // all possible Funct3 values (0b000..=0b111) are handled
    }
}

const fn decode_op(instruction: u32) -> Result<Instruction, DecodeError> {
    let r = R::decode(instruction);
    match (decode_funct3(instruction), decode_funct7(instruction)) {
        (Funct3::ADD, Funct7::ADD) => Ok(Instruction::Add(r)),
        (Funct3::SUB, Funct7::SUB) => Ok(Instruction::Sub(r)),
        (Funct3::SLL, Funct7::SLL) => Ok(Instruction::Sll(r)),
        (Funct3::SRL, Funct7::SRL) => Ok(Instruction::Srl(r)),
        (Funct3::SRA, Funct7::SRA) => Ok(Instruction::Sra(r)),
        (Funct3::SLT, Funct7::SLT) => Ok(Instruction::Slt(r)),
        (Funct3::SLTU, Funct7::SLTU) => Ok(Instruction::Sltu(r)),
        (Funct3::XOR, Funct7::XOR) => Ok(Instruction::Xor(r)),
        (Funct3::OR, Funct7::OR) => Ok(Instruction::Or(r)),
        (Funct3::AND, Funct7::AND) => Ok(Instruction::And(r)),
        (Funct3::MUL, Funct7::MULDIV) => Ok(Instruction::Mul(r)),
        (Funct3::MULH, Funct7::MULDIV) => Ok(Instruction::Mulh(r)),
        (Funct3::MULHSU, Funct7::MULDIV) => Ok(Instruction::Mulhsu(r)),
        (Funct3::MULHU, Funct7::MULDIV) => Ok(Instruction::Mulhu(r)),
        (Funct3::DIV, Funct7::MULDIV) => Ok(Instruction::Div(r)),
        (Funct3::DIVU, Funct7::MULDIV) => Ok(Instruction::Divu(r)),
        (Funct3::REM, Funct7::MULDIV) => Ok(Instruction::Rem(r)),
        (Funct3::REMU, Funct7::MULDIV) => Ok(Instruction::Remu(r)),
        (funct3, funct7) => Err(DecodeError::InvalidFunct3Funct7(funct3, funct7)),
    }
}

const fn decode_fence(instruction: u32) -> Result<Instruction, DecodeError> {
    match decode_fence_mode(instruction) {
        FenceMode::FENCE => Ok(Instruction::Fence {
            pred: decode_fence_pred(instruction),
            succ: decode_fence_succ(instruction),
        }),
        FenceMode::FENCE_TSO => Ok(Instruction::FenceTso),
        fence_mode => Err(DecodeError::InvalidFenceMode(fence_mode)),
    }
}

const fn decode_system(instruction: u32) -> Result<Instruction, DecodeError> {
    match decode_funct3(instruction) {
        Funct3::PRIV => match decode_funct12(instruction) {
            Funct12::ECALL => Ok(Instruction::Ecall),
            Funct12::EBREAK => Ok(Instruction::Ebreak),
            funct12 => Err(DecodeError::InvalidFunct12(funct12)),
        },

        Funct3::CSRRW => Ok(Instruction::Csrrw(CsrReg::decode(instruction))),
        Funct3::CSRRS => Ok(Instruction::Csrrs(CsrReg::decode(instruction))),
        Funct3::CSRRC => Ok(Instruction::Csrrc(CsrReg::decode(instruction))),
        Funct3::CSRRWI => Ok(Instruction::Csrrwi(CsrImm::decode(instruction))),
        Funct3::CSRRSI => Ok(Instruction::Csrrsi(CsrImm::decode(instruction))),
        Funct3::CSRRCI => Ok(Instruction::Csrrci(CsrImm::decode(instruction))),

        funct3 => Err(DecodeError::InvalidFunct3(funct3)),
    }
}

#[test]
fn decode_edge_cases() -> Result<(), Box<dyn Error>> {
    assert_eq!(decode(0), Err(DecodeError::InvalidOpcode(Opcode(0))));
    assert_eq!(
        decode(Opcode::BRANCH.into_u32() | (0b010 << 12)),
        Err(DecodeError::InvalidFunct3(Funct3(0b010)))
    );
    assert_eq!(
        decode(Opcode::LOAD.into_u32() | (0b111 << 12)),
        Err(DecodeError::InvalidFunct3(Funct3(0b111)))
    );
    assert_eq!(
        decode(Opcode::STORE.into_u32() | (0b111 << 12)),
        Err(DecodeError::InvalidFunct3(Funct3(0b111)))
    );
    assert_eq!(
        decode(Opcode::OP_IMM.into_u32() | (Funct3::SRL.into_u32() << 12) | (0b11_1111 << 26)),
        Err(DecodeError::InvalidShiftMode(0b11_1111))
    );
    assert_eq!(
        decode(Opcode::OP.into_u32() | (0b111 << 12) | (0b111_1111 << 25)),
        Err(DecodeError::InvalidFunct3Funct7(
            Funct3(0b111),
            Funct7(0b111_1111)
        ))
    );
    assert_eq!(
        decode(Opcode::MISC_MEM.into_u32() | (0b1111 << 28)),
        Err(DecodeError::InvalidFenceMode(FenceMode(0b1111)))
    );
    assert_eq!(
        decode(
            Opcode::SYSTEM.into_u32()
                | (Funct3::PRIV.into_u32() << 12)
                | Imm12::try_from(0b111_1111_1111)?.into_u32() << 20
        ),
        Err(DecodeError::InvalidFunct12(Funct12(0b111_1111_1111)))
    );
    assert_eq!(
        decode(Opcode::SYSTEM.into_u32() | (0b100 << 12)),
        Err(DecodeError::InvalidFunct3(Funct3(0b100)))
    );
    Ok(())
}

/// Instruction decode error
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecodeError {
    ///
    InvalidOpcode(Opcode),
    ///
    InvalidFunct3(Funct3),
    ///
    InvalidFunct7(Funct7),
    ///
    InvalidFenceMode(FenceMode),
    ///
    InvalidFunct12(Funct12),
    ///
    InvalidFunct3Funct7(Funct3, Funct7),
    ///
    InvalidShiftMode(u8),
}

impl Debug for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::InvalidOpcode(opcode) => write!(f, "InvalidOpcode({opcode:?})"),
            DecodeError::InvalidFunct3(funct3) => write!(f, "InvalidFunct3({funct3:?})"),
            DecodeError::InvalidFunct7(funct7) => write!(f, "InvalidFunct7({funct7:?})"),
            DecodeError::InvalidFenceMode(fence_mode) => {
                write!(f, "InvalidFenceMode({fence_mode:?})")
            }
            DecodeError::InvalidFunct12(funct12) => write!(f, "InvalidSystemCall({funct12:?})"),
            DecodeError::InvalidFunct3Funct7(funct3, funct7) => {
                write!(f, "InvalidFunct3Funct7({funct3:?}, {funct7:?})")
            }
            DecodeError::InvalidShiftMode(mode) => write!(f, "InvalidShiftMode(0b{mode:06b})"),
        }
    }
}

#[test]
fn decode_error_debug() {
    assert_eq!(
        format!("{:?}", DecodeError::InvalidOpcode(Opcode(0b111_1111))),
        "InvalidOpcode(Opcode(0b1111111))"
    );
    assert_eq!(
        format!("{:?}", DecodeError::InvalidFunct3(Funct3(0b111))),
        "InvalidFunct3(Funct3(0b111))"
    );
    assert_eq!(
        format!("{:?}", DecodeError::InvalidFunct7(Funct7(0b111_1111))),
        "InvalidFunct7(Funct7(0b1111111))"
    );
    assert_eq!(
        format!("{:?}", DecodeError::InvalidFenceMode(FenceMode(0b1111))),
        "InvalidFenceMode(FenceMode(0b1111))"
    );
    assert_eq!(
        format!(
            "{:?}",
            DecodeError::InvalidFunct12(Funct12(0b1111_1111_1111))
        ),
        "InvalidSystemCall(Funct12(0b111111111111))"
    );
    assert_eq!(
        format!(
            "{:?}",
            DecodeError::InvalidFunct3Funct7(Funct3(0b111), Funct7(0b111_1111))
        ),
        "InvalidFunct3Funct7(Funct3(0b111), Funct7(0b1111111))"
    );
    assert_eq!(
        format!("{:?}", DecodeError::InvalidShiftMode(0b11_1111)),
        "InvalidShiftMode(0b111111)"
    );
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::InvalidOpcode(opcode) => write!(f, "invalid opcode: {opcode}"),
            DecodeError::InvalidFunct3(funct3) => write!(f, "invalid funct3: {funct3}"),
            DecodeError::InvalidFunct7(funct7) => write!(f, "invalid funct7: {funct7}"),
            DecodeError::InvalidFenceMode(fence_mode) => {
                write!(f, "invalid fence mode: {fence_mode}")
            }
            DecodeError::InvalidFunct12(funct12) => {
                write!(f, "invalid system call: {funct12}")
            }
            DecodeError::InvalidFunct3Funct7(funct3, funct7) => write!(
                f,
                "invalid funct3 and funct7 combination: {funct3}, {funct7}",
            ),
            DecodeError::InvalidShiftMode(mode) => write!(f, "invalid shift mode: 0b{mode:06b}"),
        }
    }
}

#[test]
fn decode_error_display() {
    assert_eq!(
        DecodeError::InvalidOpcode(Opcode(0b111_1111)).to_string(),
        "invalid opcode: 0b1111111"
    );
    assert_eq!(
        DecodeError::InvalidFenceMode(FenceMode(0b1111)).to_string(),
        "invalid fence mode: 0b1111"
    );
    assert_eq!(
        DecodeError::InvalidFunct3(Funct3(0b111)).to_string(),
        "invalid funct3: 0b111"
    );
    assert_eq!(
        DecodeError::InvalidFunct7(Funct7(0b111_1111)).to_string(),
        "invalid funct7: 0b1111111"
    );
    assert_eq!(
        DecodeError::InvalidFunct12(Funct12(0b1111_1111_1111)).to_string(),
        "invalid system call: 0b111111111111"
    );
    assert_eq!(
        DecodeError::InvalidFunct3Funct7(Funct3(0b111), Funct7(0b111_1111)).to_string(),
        "invalid funct3 and funct7 combination: 0b111, 0b1111111"
    );
    assert_eq!(
        DecodeError::InvalidShiftMode(0b11_1111).to_string(),
        "invalid shift mode: 0b111111"
    );
}

impl Error for DecodeError {}

#[allow(clippy::cast_possible_truncation)]
const fn decode_opcode(instruction: u32) -> Opcode {
    Opcode(bitfield::<0, 6>(instruction) as u8)
}

#[allow(clippy::cast_possible_truncation)]
pub(crate) const fn decode_rd(instruction: u32) -> Register {
    Register(bitfield::<7, 11>(instruction) as u8)
}

#[allow(clippy::cast_possible_wrap)]
pub(crate) const fn decode_u_imm(instruction: u32) -> i32 {
    (instruction & !0xFFF) as i32
}

#[allow(clippy::cast_possible_truncation)]
const fn decode_funct3(instruction: u32) -> Funct3 {
    Funct3(bitfield::<12, 14>(instruction) as u8)
}

#[allow(clippy::cast_possible_truncation)]
pub(crate) const fn decode_rs1_reg(instruction: u32) -> Register {
    Register(bitfield::<15, 19>(instruction) as u8)
}

#[allow(clippy::cast_possible_truncation)]
pub(crate) const fn decode_rs1_imm(instruction: u32) -> Uimm5 {
    Uimm5(bitfield::<15, 19>(instruction) as u8)
}

#[allow(clippy::cast_possible_truncation)]
pub(crate) const fn decode_rs2_reg(instruction: u32) -> Register {
    Register(bitfield::<20, 24>(instruction) as u8)
}

#[allow(clippy::cast_possible_truncation)]
pub(crate) const fn decode_shamt(instruction: u32) -> Uimm5 {
    Uimm5(bitfield::<20, 24>(instruction) as u8)
}

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub(crate) const fn decode_i_imm12(instruction: u32) -> Imm12 {
    Imm12((((bitfield::<20, 31>(instruction) as u16) << 4) as i16) >> 4) // sign-extend
}

#[allow(clippy::cast_possible_truncation)]
pub(crate) const fn decode_csr(instruction: u32) -> Csr {
    Csr(bitfield::<20, 31>(instruction) as u16)
}

#[allow(clippy::cast_possible_truncation)]
const fn decode_funct7(instruction: u32) -> Funct7 {
    Funct7(bitfield::<25, 31>(instruction) as u8)
}

const SHIFT_MODE_SRLI: u8 = 0b00_0000;
const SHIFT_MODE_SRAI: u8 = 0b01_0000;

#[allow(clippy::cast_possible_truncation)]
const fn decode_shift_mode(instruction: u32) -> u8 {
    bitfield::<26, 31>(instruction) as u8
}

#[allow(clippy::cast_possible_truncation)]
const fn decode_funct12(instruction: u32) -> Funct12 {
    Funct12(bitfield::<20, 31>(instruction) as u16)
}

#[allow(clippy::cast_possible_truncation)]
const fn decode_fence_pred(instruction: u32) -> FenceMask {
    FenceMask(bitfield::<20, 23>(instruction) as u8)
}

#[allow(clippy::cast_possible_truncation)]
const fn decode_fence_succ(instruction: u32) -> FenceMask {
    FenceMask(bitfield::<24, 27>(instruction) as u8)
}

#[allow(clippy::cast_possible_truncation)]
const fn decode_fence_mode(instruction: u32) -> FenceMode {
    FenceMode(bitfield::<28, 31>(instruction) as u8)
}

#[allow(clippy::cast_possible_wrap)]
pub(crate) const fn decode_bimm(instruction: u32) -> BImm {
    let imm = combine_bitfields(&[
        (11..=11, instruction, 7..=7),
        (1..=4, instruction, 8..=11),
        (5..=10, instruction, 25..=30),
        (12..=12, instruction, 31..=31),
    ]);
    BImm(((imm << 19) as i32 >> 19) as i16)
}

#[allow(clippy::cast_possible_wrap)]
pub(crate) const fn decode_jimm(instruction: u32) -> JImm {
    #[allow(clippy::cast_possible_truncation)]
    let imm = combine_bitfields(&[
        (12..=19, instruction, 12..=19),
        (11..=11, instruction, 20..=20),
        (1..=10, instruction, 21..=30),
        (20..=20, instruction, 31..=31),
    ]);
    JImm((imm << 11) as i32 >> 11) // sign-extend
}

#[allow(clippy::cast_possible_wrap)]
pub(crate) const fn decode_s_imm12(instruction: u32) -> Imm12 {
    let imm = combine_bitfields(&[(0..=4, instruction, 7..=11), (5..=11, instruction, 25..=31)]);
    Imm12(((imm << 20) as i32 >> 20) as i16)
}
