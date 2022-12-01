pub(crate) mod funct3;
pub(crate) mod funct7;
pub(crate) mod opcode;

use funct3::Funct3;
use funct7::Funct7;
use opcode::Opcode;

use super::{
    BImm,
    Imm12,
    JImm,
    Uimm5,
};
use crate::{
    bits::merge_bitfields,
    registers::Register,
};

pub(crate) enum RegOrUimm5 {
    Register(Register),
    Uimm5(Uimm5),
}

impl RegOrUimm5 {
    fn u32(&self) -> u32 {
        match self {
            RegOrUimm5::Register(reg) => reg.0,
            RegOrUimm5::Uimm5(imm) => imm.0,
        }
    }
}

pub(crate) fn r_instruction(
    opcode: Opcode,
    rd: Register,
    funct3: Funct3,
    rs1: Register,
    rs2: RegOrUimm5,
    funct7: Funct7,
) -> u32 {
    merge_bitfields([
        (0..7, opcode.0, 0..7),
        (7..12, rd.0, 0..5),
        (12..15, funct3.0, 0..3),
        (15..20, rs1.0, 0..5),
        (20..25, rs2.u32(), 0..5),
        (25..32, funct7.0, 0..7),
    ])
}

pub(crate) fn i_instruction(
    opcode: Opcode,
    rd: Register,
    funct3: Funct3,
    rs1: RegOrUimm5,
    imm: Imm12,
) -> u32 {
    merge_bitfields([
        (0..7, opcode.0, 0..7),
        (7..12, rd.0, 0..5),
        (12..15, funct3.0, 0..3),
        (15..20, rs1.u32(), 0..5),
        (20..32, imm.0, 0..12),
    ])
}

pub(crate) fn s_instruction(
    opcode: Opcode,
    imm: Imm12,
    funct3: Funct3,
    rs1: Register,
    rs2: Register,
) -> u32 {
    merge_bitfields([
        (0..7, opcode.0, 0..7),
        (7..12, imm.0, 0..5),
        (12..15, funct3.0, 0..3),
        (15..20, rs1.0, 0..5),
        (20..25, rs2.0, 0..5),
        (25..32, imm.0, 5..12),
    ])
}

pub(crate) fn b_instruction(
    opcode: Opcode,
    imm: BImm,
    funct3: Funct3,
    rs1: Register,
    rs2: Register,
) -> u32 {
    merge_bitfields([
        (0..7, opcode.0, 0..7),
        (7..8, imm.0, 11..12),
        (8..12, imm.0, 1..5),
        (12..15, funct3.0, 0..3),
        (15..20, rs1.0, 0..5),
        (20..25, rs2.0, 0..5),
        (25..31, imm.0, 5..11),
        (31..32, imm.0, 12..13),
    ])
}

pub(crate) fn u_instruction(opcode: Opcode, rd: Register, imm: i32) -> u32 {
    merge_bitfields([
        (0..7, opcode.0, 0..7),
        (7..12, rd.0, 0..5),
        (12..32, imm as u32, 12..32),
    ])
}

pub(crate) fn j_instruction(opcode: Opcode, rd: Register, imm: JImm) -> u32 {
    merge_bitfields([
        (0..7, opcode.0, 0..7),
        (7..12, rd.0, 0..5),
        (12..20, imm.0, 12..20),
        (20..21, imm.0, 11..12),
        (21..31, imm.0, 1..11),
        (31..32, imm.0, 20..21),
    ])
}
