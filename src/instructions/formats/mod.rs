pub(crate) mod funct3;
pub(crate) mod funct7;
pub(crate) mod imm12;
pub(crate) mod j_imm;
pub(crate) mod opcode;
pub(crate) mod uimm5;

use super::Uimm5;
use crate::registers::Register;
use bitvec::order::Lsb0;
use bitvec::slice::BitSlice;
use bitvec::view::BitView;
use funct3::Funct3;
use funct7::Funct7;
use imm12::Imm12;
use j_imm::JImm;
use opcode::Opcode;

pub(crate) fn r_instruction(
    opcode: Opcode,
    rd: Register,
    funct3: Funct3,
    rs1: Register,
    rs2: RegOrUimm5,
    funct7: Funct7,
) -> u32 {
    let mut instruction = 0;
    let bits = instruction.view_bits_mut::<Lsb0>();
    bits[0..7].clone_from_bitslice(opcode.view_bits());
    bits[7..12].clone_from_bitslice(rd.view_bits());
    bits[12..15].clone_from_bitslice(funct3.view_bits());
    bits[15..20].clone_from_bitslice(rs1.view_bits());
    bits[20..25].clone_from_bitslice(rs2.view_bits());
    bits[25..32].clone_from_bitslice(funct7.view_bits());
    instruction
}

pub(crate) fn i_instruction(
    opcode: Opcode,
    rd: Register,
    funct3: Funct3,
    rs1: RegOrUimm5,
    imm: Imm12,
) -> u32 {
    let mut instruction = 0;
    let bits = instruction.view_bits_mut::<Lsb0>();
    bits[0..7].clone_from_bitslice(opcode.view_bits());
    bits[7..12].clone_from_bitslice(rd.view_bits());
    bits[12..15].clone_from_bitslice(funct3.view_bits());
    bits[15..20].clone_from_bitslice(rs1.view_bits());
    bits[20..32].clone_from_bitslice(imm.view_bits());
    instruction
}

pub(crate) fn s_instruction(
    opcode: Opcode,
    imm: Imm12,
    funct3: Funct3,
    rs1: Register,
    rs2: Register,
) -> u32 {
    let mut instruction = 0;
    let bits = instruction.view_bits_mut::<Lsb0>();
    let imm_bits = imm.view_bits();
    bits[0..7].clone_from_bitslice(opcode.view_bits());
    bits[7..12].clone_from_bitslice(&imm_bits[0..5]);
    bits[12..15].clone_from_bitslice(funct3.view_bits());
    bits[15..20].clone_from_bitslice(rs1.view_bits());
    bits[20..25].clone_from_bitslice(rs2.view_bits());
    bits[25..32].clone_from_bitslice(&imm_bits[5..12]);
    instruction
}

pub(crate) fn b_instruction(
    opcode: Opcode,
    imm: i16,
    funct3: Funct3,
    rs1: Register,
    rs2: Register,
) -> u32 {
    let mut instruction = 0;
    let bits = instruction.view_bits_mut::<Lsb0>();
    let imm = imm as u16;
    let imm_bits = imm.view_bits::<Lsb0>();
    bits[0..7].clone_from_bitslice(opcode.view_bits());
    bits.set(7, imm_bits[11]);
    bits[8..12].clone_from_bitslice(&imm_bits[1..5]);
    bits[12..15].clone_from_bitslice(funct3.view_bits());
    bits[15..20].clone_from_bitslice(rs1.view_bits());
    bits[20..25].clone_from_bitslice(rs2.view_bits());
    bits[25..31].clone_from_bitslice(&imm_bits[5..11]);
    bits.set(31, imm_bits[12]);
    instruction
}

pub(crate) fn u_instruction(opcode: Opcode, rd: Register, imm: i32) -> u32 {
    let mut instruction = 0;
    let bits = instruction.view_bits_mut::<Lsb0>();
    let imm = imm as u32;
    let imm_bits = imm.view_bits::<Lsb0>();
    bits[0..7].clone_from_bitslice(opcode.view_bits());
    bits[7..12].clone_from_bitslice(rd.view_bits());
    bits[12..32].clone_from_bitslice(&imm_bits[12..32]);
    instruction
}

pub(crate) fn j_instruction(opcode: Opcode, rd: Register, imm: JImm) -> u32 {
    let mut instruction = 0;
    let bits = instruction.view_bits_mut::<Lsb0>();
    let imm_bits = imm.view_bits();
    bits[0..7].clone_from_bitslice(opcode.view_bits());
    bits[7..12].clone_from_bitslice(rd.view_bits());
    bits[12..20].copy_from_bitslice(&imm_bits[12..20]);
    bits.set(20, imm_bits[11]);
    bits[21..31].copy_from_bitslice(&imm_bits[1..11]);
    bits.set(31, imm_bits[20]);
    instruction
}

pub(crate) enum RegOrUimm5 {
    Register(Register),
    Uimm5(Uimm5),
}

impl RegOrUimm5 {
    pub(crate) fn view_bits(&self) -> &BitSlice<u8, Lsb0> {
        match self {
            RegOrUimm5::Register(register) => register.view_bits(),
            RegOrUimm5::Uimm5(uimm) => uimm.view_bits(),
        }
    }
}
