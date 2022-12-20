mod util;

use std::error::Error;

use risky::{
    common::{fence_mask::FenceMask, funct3::Funct3, funct7::Funct7, imm12::Imm12, opcode::Opcode},
    decode::{decode, DecodeError, Instruction},
    instructions::rv32i::{
        add, addi, and, andi, auipc, beq, bge, bgeu, blt, bltu, bne, ebreak, ecall, fence,
        fence_tso, jal, jalr, lb, lbu, lh, lhu, lui, lw, mv, nop, not, or, ori, sb, seqz, sh, sll,
        slli, slt, slti, sltiu, sltu, snez, sra, srai, srl, srli, sub, sw, xor, xori,
    },
    registers::{X0, X30, X31},
};
use util::{
    test_b, test_i, test_i_case, test_j, test_r_imm, test_r_reg, test_r_reg_spec, test_s, test_u,
};

#[test]
fn _lui() -> Result<(), DecodeError> {
    test_u(lui, Instruction::Lui, Opcode::LUI)
}

#[test]
fn _auipc() -> Result<(), DecodeError> {
    test_u(auipc, Instruction::Auipc, Opcode::AUIPC)
}

#[test]
fn _jal() -> Result<(), Box<dyn Error>> {
    test_j(jal, Instruction::Jal)
}

#[test]
fn _jalr() -> Result<(), Box<dyn Error>> {
    test_i(jalr, Instruction::Jalr, Opcode::JALR, Funct3::JALR)
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

#[test]
fn _lb() -> Result<(), Box<dyn Error>> {
    test_i(lb, Instruction::Lb, Opcode::LOAD, Funct3::LB)
}

#[test]
fn _lu() -> Result<(), Box<dyn Error>> {
    test_i(lbu, Instruction::Lbu, Opcode::LOAD, Funct3::LBU)
}

#[test]
fn _lh() -> Result<(), Box<dyn Error>> {
    test_i(lh, Instruction::Lh, Opcode::LOAD, Funct3::LH)
}

#[test]
fn _lhu() -> Result<(), Box<dyn Error>> {
    test_i(lhu, Instruction::Lhu, Opcode::LOAD, Funct3::LHU)
}

#[test]
fn _lw() -> Result<(), Box<dyn Error>> {
    test_i(lw, Instruction::Lw, Opcode::LOAD, Funct3::LW)
}

#[test]
fn _sb() -> Result<(), Box<dyn Error>> {
    test_s(sb, Instruction::Sb, Opcode::STORE, Funct3::SB)
}

#[test]
fn _sh() -> Result<(), Box<dyn Error>> {
    test_s(sh, Instruction::Sh, Opcode::STORE, Funct3::SH)
}

#[test]
fn _sw() -> Result<(), Box<dyn Error>> {
    test_s(sw, Instruction::Sw, Opcode::STORE, Funct3::SW)
}

#[test]
fn _addi() -> Result<(), Box<dyn Error>> {
    test_i(addi, Instruction::Addi, Opcode::OP_IMM, Funct3::ADDI)
}

#[test]
fn _mv() -> Result<(), DecodeError> {
    test_i_case(
        mv(X30, X31),
        Instruction::Addi,
        Opcode::OP_IMM,
        Funct3::ADDI,
        X30,
        X31,
        Imm12::ZERO,
    )
}

#[test]
fn _nop() -> Result<(), DecodeError> {
    test_i_case(
        nop(),
        Instruction::Addi,
        Opcode::OP_IMM,
        Funct3::ADDI,
        X0,
        X0,
        Imm12::ZERO,
    )
}

#[test]
fn _slti() -> Result<(), Box<dyn Error>> {
    test_i(slti, Instruction::Slti, Opcode::OP_IMM, Funct3::SLTI)
}

#[test]
fn _sltiu() -> Result<(), Box<dyn Error>> {
    test_i(sltiu, Instruction::Sltiu, Opcode::OP_IMM, Funct3::SLTIU)
}

#[test]
fn _seqz() -> Result<(), Box<dyn Error>> {
    test_i_case(
        seqz(X30, X31),
        Instruction::Sltiu,
        Opcode::OP_IMM,
        Funct3::SLTIU,
        X30,
        X31,
        Imm12::try_from(1)?,
    )?;
    Ok(())
}

#[test]
fn _xori() -> Result<(), Box<dyn Error>> {
    test_i(xori, Instruction::Xori, Opcode::OP_IMM, Funct3::XORI)
}

#[test]
fn _not() -> Result<(), Box<dyn Error>> {
    test_i_case(
        not(X30, X31),
        Instruction::Xori,
        Opcode::OP_IMM,
        Funct3::XORI,
        X30,
        X31,
        Imm12::try_from(-1)?,
    )?;
    Ok(())
}

#[test]
fn _ori() -> Result<(), Box<dyn Error>> {
    test_i(ori, Instruction::Ori, Opcode::OP_IMM, Funct3::ORI)
}

#[test]
fn _andi() -> Result<(), Box<dyn Error>> {
    test_i(andi, Instruction::Andi, Opcode::OP_IMM, Funct3::ANDI)
}

#[test]
fn _slli() -> Result<(), Box<dyn Error>> {
    test_r_imm(
        slli,
        Instruction::Slli,
        Opcode::OP_IMM,
        Funct3::SLLI,
        Funct7::SLL_SRL,
    )
}

#[test]
fn _srli() -> Result<(), Box<dyn Error>> {
    test_r_imm(
        srli,
        Instruction::Srli,
        Opcode::OP_IMM,
        Funct3::SRL_SRA,
        Funct7::SLL_SRL,
    )
}

#[test]
fn _srai() -> Result<(), Box<dyn Error>> {
    test_r_imm(
        srai,
        Instruction::Srai,
        Opcode::OP_IMM,
        Funct3::SRL_SRA,
        Funct7::SRA,
    )
}

#[test]
fn _add() -> Result<(), Box<dyn Error>> {
    test_r_reg(
        add,
        Instruction::Add,
        Opcode::OP,
        Funct3::ADD_SUB,
        Funct7::ADD,
    )
}

#[test]
fn _sub() -> Result<(), Box<dyn Error>> {
    test_r_reg(
        sub,
        Instruction::Sub,
        Opcode::OP,
        Funct3::ADD_SUB,
        Funct7::SUB,
    )
}

#[test]
fn _sll() -> Result<(), Box<dyn Error>> {
    test_r_reg(
        sll,
        Instruction::Sll,
        Opcode::OP,
        Funct3::SLL,
        Funct7::SLL_SRL,
    )
}

#[test]
fn _srl() -> Result<(), Box<dyn Error>> {
    test_r_reg(
        srl,
        Instruction::Srl,
        Opcode::OP,
        Funct3::SRL_SRA,
        Funct7::SLL_SRL,
    )
}

#[test]
fn _sra() -> Result<(), Box<dyn Error>> {
    test_r_reg(
        sra,
        Instruction::Sra,
        Opcode::OP,
        Funct3::SRL_SRA,
        Funct7::SRA,
    )
}

#[test]
fn _slt() -> Result<(), Box<dyn Error>> {
    test_r_reg(slt, Instruction::Slt, Opcode::OP, Funct3::SLT, Funct7::SLT)
}

#[test]
fn _sltu() -> Result<(), Box<dyn Error>> {
    test_r_reg(
        sltu,
        Instruction::Sltu,
        Opcode::OP,
        Funct3::SLTU,
        Funct7::SLTU,
    )
}

#[test]
fn _snez() -> Result<(), Box<dyn Error>> {
    test_r_reg_spec(
        |rd, _, rs2| snez(rd, rs2),
        Instruction::Sltu,
        Opcode::OP,
        Funct3::SLTU,
        Funct7::SLTU,
        X30,
        X0,
        X31,
    )
}

#[test]
fn _xor() -> Result<(), Box<dyn Error>> {
    test_r_reg(xor, Instruction::Xor, Opcode::OP, Funct3::XOR, Funct7::XOR)
}

#[test]
fn _or() -> Result<(), Box<dyn Error>> {
    test_r_reg(or, Instruction::Or, Opcode::OP, Funct3::OR, Funct7::OR)
}

#[test]
fn _and() -> Result<(), Box<dyn Error>> {
    test_r_reg(and, Instruction::And, Opcode::OP, Funct3::AND, Funct7::AND)
}

#[test]
fn _fence() -> Result<(), Box<dyn Error>> {
    let fence_mask = FenceMask::try_from(0b1111_u8)?;
    assert_eq!(
        decode(fence(fence_mask, fence_mask))?,
        Instruction::Fence {
            pred: fence_mask,
            succ: fence_mask
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
