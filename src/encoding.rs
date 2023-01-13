use crate::{
    common::{
        bimm::BImm, csr::Csr, funct3::Funct3, funct7::Funct7, imm12::Imm12, jimm::JImm,
        opcode::Opcode, reg_or_uimm5::RegOrUimm5,
    },
    registers::Register,
    util::bits::combine_bitfields,
};

pub(crate) const fn r_instruction(
    opcode: Opcode,
    rd: Register,
    funct3: Funct3,
    rs1: Register,
    rs2: RegOrUimm5,
    funct7: Funct7,
) -> u32 {
    combine_bitfields(&[
        (0..=6, opcode.into_u32(), 0..=6),
        (7..=11, rd.into_u32(), 0..=4),
        (12..=14, funct3.into_u32(), 0..=2),
        (15..=19, rs1.into_u32(), 0..=4),
        (20..=24, rs2.into_u32(), 0..=4),
        (25..=31, funct7.into_u32(), 0..=6),
    ])
}

pub(crate) enum Imm12OrCsr {
    Imm12(Imm12),
    Csr(Csr),
}

impl Imm12OrCsr {
    const fn into_u32(self) -> u32 {
        match self {
            Imm12OrCsr::Imm12(imm) => imm.into_u32(),
            Imm12OrCsr::Csr(csr) => csr.into_u32(),
        }
    }
}

pub(crate) const fn i_instruction(
    opcode: Opcode,
    rd: Register,
    funct3: Funct3,
    rs1: RegOrUimm5,
    imm: Imm12OrCsr,
) -> u32 {
    combine_bitfields(&[
        (0..=6, opcode.into_u32(), 0..=6),
        (7..=11, rd.into_u32(), 0..=4),
        (12..=14, funct3.into_u32(), 0..=2),
        (15..=19, rs1.into_u32(), 0..=4),
        (20..=31, imm.into_u32(), 0..=11),
    ])
}

pub(crate) const fn s_instruction(
    opcode: Opcode,
    imm: Imm12,
    funct3: Funct3,
    rs1: Register,
    rs2: Register,
) -> u32 {
    combine_bitfields(&[
        (0..=6, opcode.into_u32(), 0..=6),
        (7..=11, imm.into_u32(), 0..=4),
        (12..=14, funct3.into_u32(), 0..=2),
        (15..=19, rs1.into_u32(), 0..=4),
        (20..=24, rs2.into_u32(), 0..=4),
        (25..=31, imm.into_u32(), 5..=11),
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
    combine_bitfields(&[
        (0..=6, opcode.into_u32(), 0..=6),
        (7..=7, imm, 11..=11),
        (8..=11, imm, 1..=4),
        (12..=14, funct3.into_u32(), 0..=2),
        (15..=19, rs1.into_u32(), 0..=4),
        (20..=24, rs2.into_u32(), 0..=4),
        (25..=30, imm, 5..=10),
        (31..=31, imm, 12..=12),
    ])
}

#[allow(clippy::cast_sign_loss)]
pub(crate) const fn u_instruction(opcode: Opcode, rd: Register, imm: i32) -> u32 {
    combine_bitfields(&[
        (0..=6, opcode.into_u32(), 0..=6),
        (7..=11, rd.into_u32(), 0..=4),
        (12..=31, imm as u32, 12..=31),
    ])
}

pub(crate) const fn j_instruction(opcode: Opcode, rd: Register, imm: JImm) -> u32 {
    let imm = imm.into_u32();
    combine_bitfields(&[
        (0..=6, opcode.into_u32(), 0..=6),
        (7..=11, rd.into_u32(), 0..=4),
        (12..=19, imm, 12..=19),
        (20..=20, imm, 11..=11),
        (21..=30, imm, 1..=10),
        (31..=31, imm, 20..=20),
    ])
}
