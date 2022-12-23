#[cfg(test)]
mod util;

use std::error::Error;

use risky::{
    common::{fence_mask::FenceMask, imm12::Imm12},
    decode::{decode, DecodeError, Instruction},
    instructions::{
        m_ext::{div, divu, mul, mulh, mulhsu, mulhu, rem, remu},
        rv32i::{
            add, addi, and, andi, auipc, beq, bge, bgeu, blt, bltu, bne, ebreak, ecall, fence,
            fence_tso, jal, jalr, lb, lbu, lh, lhu, lui, lw, mv, nop, not, or, ori, sb, seqz, sh,
            sll, slli, slt, slti, sltiu, sltu, snez, sra, srai, srl, srli, sub, sw, xor, xori,
        },
        zicsr_ext::{csrc, csrci, csrr, csrrc, csrrci, csrrs, csrrsi, csrrw, csrrwi, csrs, csrsi},
    },
    registers::{X0, X30, X31},
};
use util::{
    test_b, test_csr_imm, test_csr_reg, test_i, test_i_case, test_j, test_r_imm, test_r_reg,
    test_r_reg_specific, test_s, test_u,
};

#[test]
fn _lui() -> Result<(), DecodeError> {
    test_u(lui, Instruction::Lui)
}

#[test]
fn _auipc() -> Result<(), DecodeError> {
    test_u(auipc, Instruction::Auipc)
}

#[test]
fn _jal() -> Result<(), Box<dyn Error>> {
    test_j(jal, Instruction::Jal)
}

#[test]
fn _jalr() -> Result<(), Box<dyn Error>> {
    test_i(jalr, Instruction::Jalr)
}

#[test]
fn _beq() -> Result<(), Box<dyn Error>> {
    test_b(beq, Instruction::Beq)
}

#[test]
fn _bne() -> Result<(), Box<dyn Error>> {
    test_b(bne, Instruction::Bne)
}

#[test]
fn _blt() -> Result<(), Box<dyn Error>> {
    test_b(blt, Instruction::Blt)
}

#[test]
fn _bltu() -> Result<(), Box<dyn Error>> {
    test_b(bltu, Instruction::Bltu)
}

#[test]
fn _bge() -> Result<(), Box<dyn Error>> {
    test_b(bge, Instruction::Bge)
}

#[test]
fn _bgeu() -> Result<(), Box<dyn Error>> {
    test_b(bgeu, Instruction::Bgeu)
}

#[test]
fn _lb() -> Result<(), Box<dyn Error>> {
    test_i(lb, Instruction::Lb)
}

#[test]
fn _lu() -> Result<(), Box<dyn Error>> {
    test_i(lbu, Instruction::Lbu)
}

#[test]
fn _lh() -> Result<(), Box<dyn Error>> {
    test_i(lh, Instruction::Lh)
}

#[test]
fn _lhu() -> Result<(), Box<dyn Error>> {
    test_i(lhu, Instruction::Lhu)
}

#[test]
fn _lw() -> Result<(), Box<dyn Error>> {
    test_i(lw, Instruction::Lw)
}

#[test]
fn _sb() -> Result<(), Box<dyn Error>> {
    test_s(sb, Instruction::Sb)
}

#[test]
fn _sh() -> Result<(), Box<dyn Error>> {
    test_s(sh, Instruction::Sh)
}

#[test]
fn _sw() -> Result<(), Box<dyn Error>> {
    test_s(sw, Instruction::Sw)
}

#[test]
fn _addi() -> Result<(), Box<dyn Error>> {
    test_i(addi, Instruction::Addi)
}

#[test]
fn _mv() -> Result<(), DecodeError> {
    test_i_case(mv(X30, X31), Instruction::Addi, X30, X31, Imm12::ZERO)
}

#[test]
fn _nop() -> Result<(), DecodeError> {
    test_i_case(nop(), Instruction::Addi, X0, X0, Imm12::ZERO)
}

#[test]
fn _slti() -> Result<(), Box<dyn Error>> {
    test_i(slti, Instruction::Slti)
}

#[test]
fn _sltiu() -> Result<(), Box<dyn Error>> {
    test_i(sltiu, Instruction::Sltiu)
}

#[test]
fn _seqz() -> Result<(), Box<dyn Error>> {
    test_i_case(
        seqz(X30, X31),
        Instruction::Sltiu,
        X30,
        X31,
        Imm12::try_from(1)?,
    )?;
    Ok(())
}

#[test]
fn _xori() -> Result<(), Box<dyn Error>> {
    test_i(xori, Instruction::Xori)
}

#[test]
fn _not() -> Result<(), Box<dyn Error>> {
    test_i_case(
        not(X30, X31),
        Instruction::Xori,
        X30,
        X31,
        Imm12::try_from(-1)?,
    )?;
    Ok(())
}

#[test]
fn _ori() -> Result<(), Box<dyn Error>> {
    test_i(ori, Instruction::Ori)
}

#[test]
fn _andi() -> Result<(), Box<dyn Error>> {
    test_i(andi, Instruction::Andi)
}

#[test]
fn _slli() -> Result<(), Box<dyn Error>> {
    test_r_imm(slli, Instruction::Slli)
}

#[test]
fn _srli() -> Result<(), Box<dyn Error>> {
    test_r_imm(srli, Instruction::Srli)
}

#[test]
fn _srai() -> Result<(), Box<dyn Error>> {
    test_r_imm(srai, Instruction::Srai)
}

#[test]
fn _add() -> Result<(), DecodeError> {
    test_r_reg(add, Instruction::Add)
}

#[test]
fn _sub() -> Result<(), DecodeError> {
    test_r_reg(sub, Instruction::Sub)
}

#[test]
fn _sll() -> Result<(), DecodeError> {
    test_r_reg(sll, Instruction::Sll)
}

#[test]
fn _srl() -> Result<(), DecodeError> {
    test_r_reg(srl, Instruction::Srl)
}

#[test]
fn _sra() -> Result<(), DecodeError> {
    test_r_reg(sra, Instruction::Sra)
}

#[test]
fn _slt() -> Result<(), DecodeError> {
    test_r_reg(slt, Instruction::Slt)
}

#[test]
fn _sltu() -> Result<(), DecodeError> {
    test_r_reg(sltu, Instruction::Sltu)
}

#[test]
fn _snez() -> Result<(), DecodeError> {
    test_r_reg_specific(|rd, _, rs2| snez(rd, rs2), Instruction::Sltu, X30, X0, X31)
}

#[test]
fn _xor() -> Result<(), DecodeError> {
    test_r_reg(xor, Instruction::Xor)
}

#[test]
fn _or() -> Result<(), DecodeError> {
    test_r_reg(or, Instruction::Or)
}

#[test]
fn _and() -> Result<(), DecodeError> {
    test_r_reg(and, Instruction::And)
}

#[test]
fn _fence() -> Result<(), Box<dyn Error>> {
    let pred = FenceMask::try_from("io")?;
    let succ = FenceMask::try_from("rw")?;
    assert_eq!(
        decode(fence(pred, succ))?,
        Instruction::Fence {
            pred: FenceMask::try_from(0b1100_u8)?,
            succ: FenceMask::try_from(0b0011_u8)?,
        }
    );
    Ok(())
}

#[test]
fn _fence_tso() -> Result<(), DecodeError> {
    assert_eq!(decode(fence_tso())?, Instruction::FenceTso);
    Ok(())
}

#[test]
fn _ecall() -> Result<(), DecodeError> {
    assert_eq!(decode(ecall())?, Instruction::Ecall);
    Ok(())
}

#[test]
fn _ebreak() -> Result<(), DecodeError> {
    assert_eq!(decode(ebreak())?, Instruction::Ebreak);
    Ok(())
}

#[test]
fn _mul() -> Result<(), DecodeError> {
    test_r_reg(mul, Instruction::Mul)
}

#[test]
fn _mulh() -> Result<(), DecodeError> {
    test_r_reg(mulh, Instruction::Mulh)
}

#[test]
fn _mulhsu() -> Result<(), DecodeError> {
    test_r_reg(mulhsu, Instruction::Mulhsu)
}

#[test]
fn _mulhu() -> Result<(), DecodeError> {
    test_r_reg(mulhu, Instruction::Mulhu)
}

#[test]
fn _div() -> Result<(), DecodeError> {
    test_r_reg(div, Instruction::Div)
}

#[test]
fn _divu() -> Result<(), DecodeError> {
    test_r_reg(divu, Instruction::Divu)
}

#[test]
fn _rem() -> Result<(), DecodeError> {
    test_r_reg(rem, Instruction::Rem)
}

#[test]
fn _remu() -> Result<(), DecodeError> {
    test_r_reg(remu, Instruction::Remu)
}

#[test]
fn _csrrw() -> Result<(), Box<dyn Error>> {
    test_csr_reg(|csr| csrrw(X30, X31, csr), Instruction::Csrrw, X30, X31)
}

#[test]
fn _csrrs() -> Result<(), Box<dyn Error>> {
    test_csr_reg(|csr| csrrs(X30, X31, csr), Instruction::Csrrs, X30, X31)
}

#[test]
fn _csrr() -> Result<(), Box<dyn Error>> {
    test_csr_reg(|csr| csrr(X31, csr), Instruction::Csrrs, X31, X0)
}

#[test]
fn _csrs() -> Result<(), Box<dyn Error>> {
    test_csr_reg(|csr| csrs(X31, csr), Instruction::Csrrs, X0, X31)
}

#[test]
fn _csrrc() -> Result<(), Box<dyn Error>> {
    test_csr_reg(|csr| csrrc(X30, X31, csr), Instruction::Csrrc, X30, X31)
}

#[test]
fn _csrc() -> Result<(), Box<dyn Error>> {
    test_csr_reg(|csr| csrc(X31, csr), Instruction::Csrrc, X0, X31)
}

#[test]
fn _csrrwi() -> Result<(), Box<dyn Error>> {
    test_csr_imm(|imm, csr| csrrwi(X31, imm, csr), Instruction::Csrrwi, X31)
}

#[test]
fn _csrrsi() -> Result<(), Box<dyn Error>> {
    test_csr_imm(|imm, csr| csrrsi(X31, imm, csr), Instruction::Csrrsi, X31)
}

#[test]
fn _csrsi() -> Result<(), Box<dyn Error>> {
    test_csr_imm(csrsi, Instruction::Csrrsi, X0)
}

#[test]
fn _csrrci() -> Result<(), Box<dyn Error>> {
    test_csr_imm(|imm, csr| csrrci(X31, imm, csr), Instruction::Csrrci, X31)
}

#[test]
fn _csrci() -> Result<(), Box<dyn Error>> {
    test_csr_imm(csrci, Instruction::Csrrci, X0)
}
