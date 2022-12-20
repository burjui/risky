use crate::{
    bits::merge_bitfields,
    common::{
        bimm::BImm, funct3::Funct3, funct7::Funct7, imm12::Imm12, jimm::JImm, opcode::Opcode,
        reg_or_uimm5::RegOrUimm5,
    },
    registers::Register,
};

pub(crate) const fn r_instruction(
    opcode: Opcode,
    rd: Register,
    funct3: Funct3,
    rs1: Register,
    rs2: RegOrUimm5,
    funct7: Funct7,
) -> u32 {
    merge_bitfields(&[
        (0..7, opcode.into_u32(), 0..7),
        (7..12, rd.into_u32(), 0..5),
        (12..15, funct3.into_u32(), 0..3),
        (15..20, rs1.into_u32(), 0..5),
        (20..25, rs2.into_u32(), 0..5),
        (25..32, funct7.into_u32(), 0..7),
    ])
}

pub(crate) const fn i_instruction(
    opcode: Opcode,
    rd: Register,
    funct3: Funct3,
    rs1: RegOrUimm5,
    imm: Imm12,
) -> u32 {
    merge_bitfields(&[
        (0..7, opcode.into_u32(), 0..7),
        (7..12, rd.into_u32(), 0..5),
        (12..15, funct3.into_u32(), 0..3),
        (15..20, rs1.into_u32(), 0..5),
        (20..32, imm.into_u32(), 0..12),
    ])
}

pub(crate) const fn s_instruction(
    opcode: Opcode,
    imm: Imm12,
    funct3: Funct3,
    rs1: Register,
    rs2: Register,
) -> u32 {
    merge_bitfields(&[
        (0..7, opcode.into_u32(), 0..7),
        (7..12, imm.into_u32(), 0..5),
        (12..15, funct3.into_u32(), 0..3),
        (15..20, rs1.into_u32(), 0..5),
        (20..25, rs2.into_u32(), 0..5),
        (25..32, imm.into_u32(), 5..12),
    ])
}

pub(crate) const fn b_instruction(
    opcode: Opcode,
    imm: BImm,
    funct3: Funct3,
    rs1: Register,
    rs2: Register,
) -> u32 {
    let imm = imm.into_u32();
    merge_bitfields(&[
        (0..7, opcode.into_u32(), 0..7),
        (7..8, imm, 11..12),
        (8..12, imm, 1..5),
        (12..15, funct3.into_u32(), 0..3),
        (15..20, rs1.into_u32(), 0..5),
        (20..25, rs2.into_u32(), 0..5),
        (25..31, imm, 5..11),
        (31..32, imm, 12..13),
    ])
}

#[allow(clippy::cast_sign_loss)]
pub(crate) const fn u_instruction(opcode: Opcode, rd: Register, imm: i32) -> u32 {
    merge_bitfields(&[
        (0..7, opcode.into_u32(), 0..7),
        (7..12, rd.into_u32(), 0..5),
        (12..32, imm as u32, 12..32),
    ])
}

pub(crate) const fn j_instruction(opcode: Opcode, rd: Register, imm: JImm) -> u32 {
    let imm = imm.into_u32();
    merge_bitfields(&[
        (0..7, opcode.into_u32(), 0..7),
        (7..12, rd.into_u32(), 0..5),
        (12..20, imm, 12..20),
        (20..21, imm, 11..12),
        (21..31, imm, 1..11),
        (31..32, imm, 20..21),
    ])
}
