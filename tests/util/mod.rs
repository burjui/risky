use std::error::Error;

use risky::{
    common::{bimm::BImm, csr::Csr, imm12::Imm12, jimm::JImm, uimm5::Uimm5},
    decoding::{decode, DecodeError},
    instruction::{CsrImm, CsrReg, IShift, Instruction, B, I, J, R, S, U},
    registers::{Register, X1, X2, X29, X30, X31},
};

pub(crate) fn test_j(
    encode: impl Fn(Register, JImm) -> u32,
    variant: impl Fn(J) -> Instruction + Copy,
) -> Result<(), Box<dyn Error>> {
    let imm = (i32::MIN >> 11).try_into()?;
    test_j_case(encode(X31, imm), variant, X31, imm)?;

    let imm = (i32::MAX >> 11).try_into()?;
    test_j_case(encode(X31, imm), variant, X31, imm)?;

    Ok(())
}

fn test_j_case(
    instruction: u32,
    variant: impl Fn(J) -> Instruction + Copy,
    rd: Register,
    imm: JImm,
) -> Result<(), Box<dyn Error>> {
    assert_eq!(decode(instruction)?, variant(J { rd, imm }));
    Ok(())
}

pub(crate) fn test_b(
    encode: impl Fn(BImm, Register, Register) -> u32,
    variant: impl Fn(B) -> Instruction + Copy,
) -> Result<(), Box<dyn Error>> {
    let imm = (i16::MIN >> 3).try_into()?;
    test_b_case(encode(imm, X1, X2), variant, imm)?;

    let imm = (i16::MAX >> 3).try_into()?;
    test_b_case(encode(imm, X1, X2), variant, imm)?;

    Ok(())
}

fn test_b_case(
    instruction: u32,
    variant: impl Fn(B) -> Instruction,
    imm: BImm,
) -> Result<(), Box<dyn Error>> {
    assert_eq!(
        decode(instruction)?,
        variant(B {
            rs1: X1,
            rs2: X2,
            imm,
        })
    );
    Ok(())
}

pub(crate) fn test_u(
    encode: impl Fn(Register, i32) -> u32,
    variant: impl Fn(U) -> Instruction + Copy,
) -> Result<(), DecodeError> {
    test_u_case(encode(X31, i32::MIN), variant, X31, i32::MIN)?;
    test_u_case(encode(X31, i32::MAX), variant, X31, i32::MAX)?;
    Ok(())
}

fn test_u_case(
    instruction: u32,
    variant: impl Fn(U) -> Instruction,
    rd: Register,
    imm: i32,
) -> Result<(), DecodeError> {
    assert_eq!(
        decode(instruction)?,
        variant(U {
            rd,
            imm: imm & !0xFFF
        })
    );
    Ok(())
}

pub(crate) fn test_i(
    encode: impl Fn(Register, Register, Imm12) -> u32,
    variant: impl Fn(I) -> Instruction + Copy,
) -> Result<(), Box<dyn Error>> {
    let imm: Imm12 = (i16::MIN >> 4).try_into()?;
    test_i_case(encode(X30, X31, imm), variant, X30, X31, imm)?;

    let imm = (i16::MAX >> 4).try_into()?;
    test_i_case(encode(X30, X31, imm), variant, X30, X31, imm)?;
    Ok(())
}

pub(crate) fn test_i_case(
    instruction: u32,
    variant: impl Fn(I) -> Instruction,
    rd: Register,
    rs1: Register,
    imm: Imm12,
) -> Result<(), DecodeError> {
    assert_eq!(decode(instruction)?, variant(I { rd, rs1, imm }));
    Ok(())
}

pub(crate) fn test_s(
    encode: impl Fn(Register, Imm12, Register) -> u32,
    variant: impl Fn(S) -> Instruction + Copy,
) -> Result<(), Box<dyn Error>> {
    let imm: Imm12 = (i16::MIN >> 4).try_into()?;
    test_s_case(encode(X30, imm, X31), variant, X30, imm, X31)?;

    let imm = (i16::MAX >> 4).try_into()?;
    test_s_case(encode(X30, imm, X31), variant, X30, imm, X31)?;
    Ok(())
}

fn test_s_case(
    instruction: u32,
    variant: impl Fn(S) -> Instruction,

    rs1: Register,
    imm: Imm12,

    rs2: Register,
) -> Result<(), DecodeError> {
    assert_eq!(decode(instruction)?, variant(S { rs1, imm, rs2 }));
    Ok(())
}

pub(crate) fn test_r_reg(
    encode: impl Fn(Register, Register, Register) -> u32,
    variant: impl Fn(R) -> Instruction,
) -> Result<(), DecodeError> {
    test_r_reg_specific(encode, variant, X29, X30, X31)
}

pub(crate) fn test_r_reg_specific(
    encode: impl Fn(Register, Register, Register) -> u32,
    variant: impl Fn(R) -> Instruction,
    rd: Register,
    rs1: Register,
    rs2: Register,
) -> Result<(), DecodeError> {
    assert_eq!(decode(encode(rd, rs1, rs2))?, variant(R { rd, rs1, rs2 }));
    Ok(())
}

pub(crate) fn test_r_imm(
    encode: impl Fn(Register, Register, Uimm5) -> u32,
    variant: impl Fn(IShift) -> Instruction,
) -> Result<(), Box<dyn Error>> {
    let shamt = Uimm5::try_from(0b11111)?;
    let (rd, rs1) = (X30, X31);
    assert_eq!(
        decode(encode(rd, rs1, shamt))?,
        variant(IShift { rd, rs1, shamt })
    );
    Ok(())
}

pub(crate) fn test_csr_reg(
    encode: impl FnOnce(Csr) -> u32,
    variant: impl Fn(CsrReg) -> Instruction,
    rd: Register,
    rs1: Register,
) -> Result<(), Box<dyn Error>> {
    let csr = Csr::try_from(0xFFF)?;
    assert_eq!(decode(encode(csr))?, variant(CsrReg { rd, rs1, csr }));
    Ok(())
}

pub(crate) fn test_csr_imm(
    encode: impl FnOnce(Uimm5, Csr) -> u32,
    variant: impl Fn(CsrImm) -> Instruction,
    rd: Register,
) -> Result<(), Box<dyn Error>> {
    let imm = Uimm5::try_from(0b11111)?;
    let csr = Csr::try_from(0xFFF)?;
    assert_eq!(
        decode(encode(imm, csr))?,
        variant(CsrImm { rd, rs1: imm, csr })
    );
    Ok(())
}
