mod util;

use std::error::Error;

use risky::{
    common::{funct3::Funct3, opcode::Opcode},
    decode::{DecodeError, Instruction},
    instructions::rv32i::{
        auipc, beq, bge, bgeu, blt, bltu, bne, jal, jalr, lb, lbu, lh, lhu, lui, lw, sb, sh, sw,
    },
};
use util::{test_b, test_i, test_j, test_s, test_u};

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
