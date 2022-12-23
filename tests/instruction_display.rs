use std::error::Error;

use risky::{
    common::{bimm::BImm, csr::Csr, fence_mask::FenceMask, imm12::Imm12, jimm::JImm, uimm5::Uimm5},
    decoding::{decode, DecodeError},
    m_ext::{div, divu, mul, mulh, mulhsu, mulhu, rem, remu},
    registers::{Register, X29, X30, X31},
    rv32i::{
        add, addi, and, andi, auipc, beq, bge, bgeu, blt, bltu, bne, ebreak, ecall, fence,
        fence_tso, jal, jalr, lb, lbu, lh, lhu, lui, lw, or, ori, sb, sh, sll, slli, slt, slti,
        sltiu, sltu, sra, srai, srl, srli, sub, sw, xor, xori,
    },
    zicsr_ext::{csrrc, csrrci, csrrs, csrrsi, csrrw, csrrwi},
};

#[test]
fn _lui() -> Result<(), DecodeError> {
    test_u(lui, "lui")
}

#[test]
fn _auipc() -> Result<(), DecodeError> {
    test_u(auipc, "auipc")
}

#[test]
fn _jal() -> Result<(), DecodeError> {
    test_j(jal, "jal")
}

#[test]
fn _jalr() -> Result<(), DecodeError> {
    test_i(jalr, "jalr")
}
#[test]
fn _beq() -> Result<(), DecodeError> {
    test_b(beq, "beq")
}

#[test]
fn _bne() -> Result<(), DecodeError> {
    test_b(bne, "bne")
}

#[test]
fn _blt() -> Result<(), DecodeError> {
    test_b(blt, "blt")
}

#[test]
fn _bltu() -> Result<(), DecodeError> {
    test_b(bltu, "bltu")
}

#[test]
fn _bge() -> Result<(), DecodeError> {
    test_b(bge, "bge")
}

#[test]
fn _bgeu() -> Result<(), DecodeError> {
    test_b(bgeu, "bgeu")
}

#[test]
fn _lb() -> Result<(), DecodeError> {
    test_load(lb, "lb")
}

#[test]
fn _lbu() -> Result<(), DecodeError> {
    test_load(lbu, "lbu")
}

#[test]
fn _lh() -> Result<(), DecodeError> {
    test_load(lh, "lh")
}

#[test]
fn _lhu() -> Result<(), DecodeError> {
    test_load(lhu, "lhu")
}

#[test]
fn _lw() -> Result<(), DecodeError> {
    test_load(lw, "lw")
}

#[test]
fn _sb() -> Result<(), DecodeError> {
    test_store(sb, "sb")
}

#[test]
fn _sh() -> Result<(), DecodeError> {
    test_store(sh, "sh")
}

#[test]
fn _sw() -> Result<(), DecodeError> {
    test_store(sw, "sw")
}

#[test]
fn _addi() -> Result<(), DecodeError> {
    test_i(addi, "addi")
}

#[test]
fn _slti() -> Result<(), DecodeError> {
    test_i(slti, "slti")
}

#[test]
fn _sltiu() -> Result<(), DecodeError> {
    test_i(sltiu, "sltiu")
}

#[test]
fn _xori() -> Result<(), DecodeError> {
    test_i(xori, "xori")
}

#[test]
fn _ori() -> Result<(), DecodeError> {
    test_i(ori, "ori")
}

#[test]
fn _andi() -> Result<(), DecodeError> {
    test_i(andi, "andi")
}

#[test]
fn _slli() -> Result<(), DecodeError> {
    test_ishift(slli, "slli")
}

#[test]
fn _srli() -> Result<(), DecodeError> {
    test_ishift(srli, "srli")
}

#[test]
fn _srai() -> Result<(), DecodeError> {
    test_ishift(srai, "srai")
}

#[test]
fn _add() -> Result<(), DecodeError> {
    test_r(add, "add")
}

#[test]
fn _sub() -> Result<(), DecodeError> {
    test_r(sub, "sub")
}

#[test]
fn _sll() -> Result<(), DecodeError> {
    test_r(sll, "sll")
}

#[test]
fn _srl() -> Result<(), DecodeError> {
    test_r(srl, "srl")
}

#[test]
fn _sra() -> Result<(), DecodeError> {
    test_r(sra, "sra")
}

#[test]
fn _slt() -> Result<(), DecodeError> {
    test_r(slt, "slt")
}

#[test]
fn _sltu() -> Result<(), DecodeError> {
    test_r(sltu, "sltu")
}

#[test]
fn _xor() -> Result<(), DecodeError> {
    test_r(xor, "xor")
}

#[test]
fn _or() -> Result<(), DecodeError> {
    test_r(or, "or")
}

#[test]
fn _and() -> Result<(), DecodeError> {
    test_r(and, "and")
}

#[test]
fn _fence() -> Result<(), Box<dyn Error>> {
    test_fence()
}

#[test]
fn _fence_tso() -> Result<(), DecodeError> {
    test_fence_tso()
}

#[test]
fn _ecall() -> Result<(), DecodeError> {
    test_ecall()
}

#[test]
fn _ebreak() -> Result<(), DecodeError> {
    test_ebreak()
}

#[test]
fn _mul() -> Result<(), DecodeError> {
    test_r(mul, "mul")
}

#[test]
fn _mulh() -> Result<(), DecodeError> {
    test_r(mulh, "mulh")
}

#[test]
fn _mulhsu() -> Result<(), DecodeError> {
    test_r(mulhsu, "mulhsu")
}

#[test]
fn _mulhu() -> Result<(), DecodeError> {
    test_r(mulhu, "mulhu")
}

#[test]
fn _div() -> Result<(), DecodeError> {
    test_r(div, "div")
}

#[test]
fn _divu() -> Result<(), DecodeError> {
    test_r(divu, "divu")
}

#[test]
fn _rem() -> Result<(), DecodeError> {
    test_r(rem, "rem")
}

#[test]
fn _remu() -> Result<(), DecodeError> {
    test_r(remu, "remu")
}

#[test]
fn _csrrw() -> Result<(), DecodeError> {
    test_csr_reg(csrrw, "csrrw")
}

#[test]
fn _csrrs() -> Result<(), DecodeError> {
    test_csr_reg(csrrs, "csrrs")
}

#[test]
fn _csrrc() -> Result<(), DecodeError> {
    test_csr_reg(csrrc, "csrrc")
}

#[test]
fn _csrrwi() -> Result<(), DecodeError> {
    test_csr_imm(csrrwi, "csrrwi")
}

#[test]
fn _csrrsi() -> Result<(), DecodeError> {
    test_csr_imm(csrrsi, "csrrsi")
}

#[test]
fn _csrrci() -> Result<(), DecodeError> {
    test_csr_imm(csrrci, "csrrci")
}

fn test_u(
    encode: impl Fn(Register, i32) -> u32,
    instruction_name: &str,
) -> Result<(), DecodeError> {
    test(
        encode(X31, i32::MIN),
        &format!("{instruction_name} x31, {}", i32::MIN & !0xFFF),
    )?;
    test(
        encode(X31, i32::MAX),
        &format!("{instruction_name} x31, {}", i32::MAX & !0xFFF),
    )?;
    Ok(())
}

fn test_j(
    encode: impl Fn(Register, JImm) -> u32,
    instruction_name: &str,
) -> Result<(), DecodeError> {
    test(
        encode(X31, JImm::MIN),
        &format!("{instruction_name} x31, {}", JImm::MIN),
    )?;
    test(
        encode(X31, JImm::MAX),
        &format!("{instruction_name} x31, {}", JImm::MAX),
    )?;
    Ok(())
}

fn test_i(
    encode: impl Fn(Register, Register, Imm12) -> u32,
    instruction_name: &str,
) -> Result<(), DecodeError> {
    test(
        encode(X30, X31, Imm12::MIN),
        &format!("{instruction_name} x30, x31, {}", Imm12::MIN),
    )?;
    test(
        encode(X30, X31, Imm12::MAX),
        &format!("{instruction_name} x30, x31, {}", Imm12::MAX),
    )?;
    Ok(())
}

fn test_b(
    encode: impl Fn(BImm, Register, Register) -> u32,
    instruction_name: &str,
) -> Result<(), DecodeError> {
    test(
        encode(BImm::MIN, X30, X31),
        &format!("{instruction_name} {}, x30, x31", BImm::MIN),
    )?;
    test(
        encode(BImm::MAX, X30, X31),
        &format!("{instruction_name} {}, x30, x31", BImm::MAX),
    )?;
    Ok(())
}

fn test_load(
    encode: impl Fn(Register, Register, Imm12) -> u32,
    instruction_name: &str,
) -> Result<(), DecodeError> {
    test(
        encode(X30, X31, Imm12::MIN),
        &format!("{instruction_name} x30, x31[{}]", Imm12::MIN),
    )?;
    test(
        encode(X30, X31, Imm12::MAX),
        &format!("{instruction_name} x30, x31[{}]", Imm12::MAX),
    )?;
    Ok(())
}

fn test_store(
    encode: impl Fn(Register, Imm12, Register) -> u32,
    instruction_name: &str,
) -> Result<(), DecodeError> {
    test(
        encode(X30, Imm12::MIN, X31),
        &format!("{instruction_name} x30[{}], x31", Imm12::MIN),
    )?;
    test(
        encode(X30, Imm12::MAX, X31),
        &format!("{instruction_name} x30[{}], x31", Imm12::MAX),
    )?;
    Ok(())
}

fn test_ishift(
    encode: impl Fn(Register, Register, Uimm5) -> u32,
    instruction_name: &str,
) -> Result<(), DecodeError> {
    test(
        encode(X30, X31, Uimm5::MAX),
        &format!("{instruction_name} x30, x31, {}", Uimm5::MAX),
    )
}

fn test_r(
    encode: impl Fn(Register, Register, Register) -> u32,
    instruction_name: &str,
) -> Result<(), DecodeError> {
    test(
        encode(X29, X30, X31),
        &format!("{instruction_name} x29, x30, x31"),
    )
}

fn test_fence() -> Result<(), Box<dyn Error>> {
    let pred = FenceMask::try_from("io")?;
    let succ = FenceMask::try_from("rw")?;
    test(fence(pred, succ), "fence oi, wr")?;
    Ok(())
}

fn test_fence_tso() -> Result<(), DecodeError> {
    test(fence_tso(), "fence.tso")
}

fn test_ecall() -> Result<(), DecodeError> {
    test(ecall(), "ecall")
}

fn test_ebreak() -> Result<(), DecodeError> {
    test(ebreak(), "ebreak")
}

fn test_csr_reg(
    encode: impl Fn(Register, Register, Csr) -> u32,
    instruction_name: &str,
) -> Result<(), DecodeError> {
    test(
        encode(X30, X31, Csr::FFLAGS),
        &format!("{instruction_name} x30, x31, 0x001"),
    )
}

fn test_csr_imm(
    encode: impl Fn(Register, Uimm5, Csr) -> u32,
    instruction_name: &str,
) -> Result<(), DecodeError> {
    test(
        encode(X31, Uimm5::MAX, Csr::FFLAGS),
        &format!("{instruction_name} x31, 31, 0x001"),
    )
}

fn test(instruction: u32, expected: &str) -> Result<(), DecodeError> {
    assert_eq!(decode(instruction)?.to_string(), expected);
    Ok(())
}
