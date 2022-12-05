use crate::bits::merge_bitfields;

pub mod fence;
pub mod funct3;
pub mod funct7;
pub mod opcode;

pub(crate) const fn r_instruction(
    opcode: u8,
    rd: u8,
    funct3: u8,
    rs1: u8,
    rs2: u8,
    funct7: u8,
) -> u32 {
    merge_bitfields(&[
        (0..7, opcode as u32, 0..7),
        (7..12, rd as u32, 0..5),
        (12..15, funct3 as u32, 0..3),
        (15..20, rs1 as u32, 0..5),
        (20..25, rs2 as u32, 0..5),
        (25..32, funct7 as u32, 0..7),
    ])
}

pub(crate) const fn i_instruction(opcode: u8, rd: u8, funct3: u8, rs1: u8, imm: i16) -> u32 {
    #[allow(clippy::cast_sign_loss)]
    let imm = imm as u32;
    merge_bitfields(&[
        (0..7, opcode as u32, 0..7),
        (7..12, rd as u32, 0..5),
        (12..15, funct3 as u32, 0..3),
        (15..20, rs1 as u32, 0..5),
        (20..32, imm, 0..12),
    ])
}

pub(crate) const fn s_instruction(opcode: u8, imm: i16, funct3: u8, rs1: u8, rs2: u8) -> u32 {
    #[allow(clippy::cast_sign_loss)]
    let imm = imm as u32;
    merge_bitfields(&[
        (0..7, opcode as u32, 0..7),
        (7..12, imm, 0..5),
        (12..15, funct3 as u32, 0..3),
        (15..20, rs1 as u32, 0..5),
        (20..25, rs2 as u32, 0..5),
        (25..32, imm, 5..12),
    ])
}

pub(crate) const fn b_instruction(opcode: u8, imm: i16, funct3: u8, rs1: u8, rs2: u8) -> u32 {
    #[allow(clippy::cast_sign_loss)]
    let imm = imm as u32;
    merge_bitfields(&[
        (0..7, opcode as u32, 0..7),
        (7..8, imm, 11..12),
        (8..12, imm, 1..5),
        (12..15, funct3 as u32, 0..3),
        (15..20, rs1 as u32, 0..5),
        (20..25, rs2 as u32, 0..5),
        (25..31, imm, 5..11),
        (31..32, imm, 12..13),
    ])
}

#[allow(clippy::cast_sign_loss)]
pub(crate) const fn u_instruction(opcode: u8, rd: u8, imm: i32) -> u32 {
    merge_bitfields(&[
        (0..7, opcode as u32, 0..7),
        (7..12, rd as u32, 0..5),
        (12..32, imm as u32, 12..32),
    ])
}

pub(crate) const fn j_instruction(opcode: u8, rd: u8, imm: i32) -> u32 {
    #[allow(clippy::cast_sign_loss)]
    let imm = imm as u32;
    merge_bitfields(&[
        (0..7, opcode as u32, 0..7),
        (7..12, rd as u32, 0..5),
        (12..20, imm, 12..20),
        (20..21, imm, 11..12),
        (21..31, imm, 1..11),
        (31..32, imm, 20..21),
    ])
}
