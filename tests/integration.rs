use std::error::Error;

use risky::{
    common::{bimm::BImm, funct3::Funct3, imm12::Imm12, jimm::JImm, opcode::Opcode},
    decode::{decode, DecodeError, Instruction, B, I, J, U},
    instructions::rv32i::{auipc, beq, bge, bgeu, blt, bltu, bne, jal, jalr, lui},
    registers::{Register, X1, X2, X31},
};

#[test]
fn _lui() -> Result<(), DecodeError> {
    test_u(lui, Instruction::Lui, Opcode::LUI)
}

#[test]
fn _auipc() -> Result<(), DecodeError> {
    test_u(auipc, Instruction::Auipc, Opcode::AUIPC)
}

fn test_u(
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

#[test]
fn _jal() -> Result<(), Box<dyn Error>> {
    with_imm(i32::MIN >> 11)?;
    with_imm(i32::MAX >> 11)?;

    fn with_imm(imm: i32) -> Result<(), Box<dyn Error>> {
        let imm = JImm::try_from(imm)?;
        assert_eq!(
            decode(jal(X1, imm))?,
            Instruction::Jal(J {
                opcode: Opcode::JAL,
                rd: X1,
                imm
            })
        );
        Ok(())
    }

    Ok(())
}

#[test]
fn _jalr() -> Result<(), Box<dyn Error>> {
    with_imm(i16::MIN >> 4)?;
    with_imm(i16::MAX >> 4)?;

    fn with_imm(imm: i16) -> Result<(), Box<dyn Error>> {
        let imm = Imm12::try_from(imm)?;
        assert_eq!(
            decode(jalr(X1, X2, imm))?,
            Instruction::Jalr(I {
                opcode: Opcode::JALR,
                rd: X1,
                rs1: X2,
                imm,
                funct3: Funct3::JALR
            })
        );
        Ok(())
    }

    Ok(())
}

#[test]
fn _beq() -> Result<(), Box<dyn Error>> {
    test_b(beq, Instruction::Beq, Funct3::BEQ)
}

#[test]
fn _bne() -> Result<(), Box<dyn Error>> {
    test_b(bne, Instruction::Bne, Funct3::BNE)
}

#[test]
fn _blt() -> Result<(), Box<dyn Error>> {
    test_b(blt, Instruction::Blt, Funct3::BLT)
}

#[test]
fn _bltu() -> Result<(), Box<dyn Error>> {
    test_b(bltu, Instruction::Bltu, Funct3::BLTU)
}

#[test]
fn _bge() -> Result<(), Box<dyn Error>> {
    test_b(bge, Instruction::Bge, Funct3::BGE)
}

#[test]
fn _bgeu() -> Result<(), Box<dyn Error>> {
    test_b(bgeu, Instruction::Bgeu, Funct3::BGEU)
}

fn test_b(
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
