use std::error::Error;

use risky::{
    common::{bimm::BImm, funct3::Funct3, imm12::Imm12, jimm::JImm, opcode::Opcode},
    decode::{decode, DecodeError, Instruction, B, I, J, S, U},
    registers::{Register, X1, X2, X30, X31},
};

pub fn test_j(
    encode: impl Fn(Register, JImm) -> u32,
    variant: impl Fn(J) -> Instruction + Copy,
) -> Result<(), Box<dyn Error>> {
    let imm = (i32::MIN >> 11).try_into()?;
    test_j_case(encode(X31, imm), variant, Opcode::JAL, X31, imm)?;

    let imm = (i32::MAX >> 11).try_into()?;
    test_j_case(encode(X31, imm), variant, Opcode::JAL, X31, imm)?;

    Ok(())
}

fn test_j_case(
    instruction: u32,
    variant: impl Fn(J) -> Instruction + Copy,
    opcode: Opcode,
    rd: Register,
    imm: JImm,
) -> Result<(), Box<dyn Error>> {
    assert_eq!(decode(instruction)?, variant(J { opcode, rd, imm }));
    Ok(())
}

pub fn test_b(
    encode: impl Fn(BImm, Register, Register) -> u32,
    variant: impl Fn(B) -> Instruction + Copy,
    funct3: Funct3,
) -> Result<(), Box<dyn Error>> {
    let imm = (i16::MIN >> 3).try_into()?;
    test_b_case(encode(imm, X1, X2), variant, funct3, imm)?;

    let imm = (i16::MAX >> 3).try_into()?;
    test_b_case(encode(imm, X1, X2), variant, funct3, imm)?;

    Ok(())
}

fn test_b_case(
    instruction: u32,
    variant: impl Fn(B) -> Instruction,
    funct3: Funct3,
    imm: BImm,
) -> Result<(), Box<dyn Error>> {
    assert_eq!(
        decode(instruction)?,
        variant(B {
            opcode: Opcode::BRANCH,
            rs1: X1,
            rs2: X2,
            imm,
            funct3
        })
    );
    Ok(())
}

pub fn test_u(
    encode: impl Fn(Register, i32) -> u32,
    variant: impl Fn(U) -> Instruction + Copy,
    opcode: Opcode,
) -> Result<(), DecodeError> {
    test_u_case(encode(X31, i32::MIN), variant, opcode, X31, i32::MIN)?;
    test_u_case(encode(X31, i32::MAX), variant, opcode, X31, i32::MAX)?;
    Ok(())
}

fn test_u_case(
    instruction: u32,
    variant: impl Fn(U) -> Instruction,
    opcode: Opcode,
    rd: Register,
    imm: i32,
) -> Result<(), DecodeError> {
    assert_eq!(
        decode(instruction)?,
        variant(U {
            opcode,
            rd,
            imm: imm & !0xFFF
        })
    );
    Ok(())
}

pub fn test_i(
    encode: impl Fn(Register, Register, Imm12) -> u32,
    variant: impl Fn(I) -> Instruction + Copy,
    opcode: Opcode,
    funct3: Funct3,
) -> Result<(), Box<dyn Error>> {
    let imm: Imm12 = (i16::MIN >> 4).try_into()?;
    test_i_case(
        encode(X30, X31, imm),
        variant,
        opcode,
        funct3,
        X30,
        X31,
        imm,
    )?;

    let imm = (i16::MAX >> 4).try_into()?;
    test_i_case(
        encode(X30, X31, imm),
        variant,
        opcode,
        funct3,
        X30,
        X31,
        imm,
    )?;
    Ok(())
}

fn test_i_case(
    instruction: u32,
    variant: impl Fn(I) -> Instruction,
    opcode: Opcode,
    funct3: Funct3,
    rd: Register,
    rs1: Register,
    imm: Imm12,
) -> Result<(), DecodeError> {
    assert_eq!(
        decode(instruction)?,
        variant(I {
            opcode,
            rd,
            rs1,
            imm,
            funct3
        })
    );
    Ok(())
}

pub fn test_s(
    encode: impl Fn(Register, Imm12, Register) -> u32,
    variant: impl Fn(S) -> Instruction + Copy,
    opcode: Opcode,
    funct3: Funct3,
) -> Result<(), Box<dyn Error>> {
    let imm: Imm12 = (i16::MIN >> 4).try_into()?;
    test_s_case(
        encode(X30, imm, X31),
        variant,
        opcode,
        funct3,
        X30,
        imm,
        X31,
    )?;

    let imm = (i16::MAX >> 4).try_into()?;
    test_s_case(
        encode(X30, imm, X31),
        variant,
        opcode,
        funct3,
        X30,
        imm,
        X31,
    )?;
    Ok(())
}

fn test_s_case(
    instruction: u32,
    variant: impl Fn(S) -> Instruction,
    opcode: Opcode,
    funct3: Funct3,

    rs1: Register,
    imm: Imm12,

    rs2: Register,
) -> Result<(), DecodeError> {
    assert_eq!(
        decode(instruction)?,
        variant(S {
            opcode,
            rs1,
            imm,
            rs2,
            funct3
        })
    );
    Ok(())
}
