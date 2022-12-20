use std::error::Error;

use risky::{
    common::{bimm::BImm, funct3::Funct3, imm12::Imm12, jimm::JImm, opcode::Opcode},
    decode::{decode, DecodeError, Instruction, B, I, J, U},
    instructions::rv32i::{auipc, beq, jal, jalr, lui},
    registers::{X1, X2, X22, X31},
};

#[test]
fn _lui() -> Result<(), DecodeError> {
    with_imm(i32::MIN)?;
    with_imm(i32::MAX)?;

    fn with_imm(imm: i32) -> Result<(), DecodeError> {
        assert_eq!(
            decode(lui(X31, imm))?,
            Instruction::Lui(U {
                opcode: Opcode::LUI,
                rd: X31,
                imm: imm & !0xFFF,
            })
        );
        Ok(())
    }

    Ok(())
}

#[test]
fn _auipc() -> Result<(), DecodeError> {
    with_imm(i32::MIN)?;
    with_imm(i32::MAX)?;

    fn with_imm(imm: i32) -> Result<(), DecodeError> {
        assert_eq!(
            decode(auipc(X22, imm))?,
            Instruction::Auipc(U {
                opcode: Opcode::AUIPC,
                rd: X22,
                imm: imm & !0xFFF
            })
        );
        Ok(())
    }

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
    with_imm(i16::MIN >> 3)?;
    with_imm(i16::MAX >> 3)?;

    fn with_imm(imm: i16) -> Result<(), Box<dyn Error>> {
        let imm = BImm::try_from(imm)?;
        assert_eq!(
            decode(beq(imm, X1, X2))?,
            Instruction::Beq(B {
                opcode: Opcode::BRANCH,
                rs1: X1,
                rs2: X2,
                imm,
                funct3: Funct3::BEQ
            })
        );
        Ok(())
    }

    Ok(())
}
