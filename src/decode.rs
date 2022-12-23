//! Decoding facilities

use core::fmt;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::{
    bits::{bitfield, merge_bitfields},
    common::{
        bimm::BImm, csr::Csr, fence_mask::FenceMask, fence_mode::FenceMode, funct12::Funct12,
        funct3::Funct3, funct7::Funct7, imm12::Imm12, jimm::JImm, opcode::Opcode, uimm5::Uimm5,
    },
    registers::Register,
};

/// Decodes a RISC-V instruction
#[allow(clippy::missing_errors_doc)]
pub const fn decode(instruction: u32) -> Result<Instruction, DecodeError> {
    match opcode(instruction) {
        Opcode::LUI => Ok(Instruction::Lui(U::decode(instruction))),
        Opcode::AUIPC => Ok(Instruction::Auipc(U::decode(instruction))),
        Opcode::JAL => Ok(Instruction::Jal(J::decode(instruction))),
        Opcode::JALR => Ok(Instruction::Jalr(I::decode(instruction))),
        Opcode::BRANCH => decode_branch(instruction),
        Opcode::LOAD => decode_load(instruction),
        Opcode::STORE => decode_store(instruction),
        Opcode::OP_IMM => decode_op_imm(instruction),
        Opcode::OP => decode_op(instruction),
        Opcode::MISC_MEM => decode_fence(instruction),
        Opcode::SYSTEM => decode_system(instruction),
        opcode => Err(DecodeError::InvalidOpcode(opcode)),
    }
}

const fn decode_branch(instruction: u32) -> Result<Instruction, DecodeError> {
    let b = B::decode(instruction);
    match funct3(instruction) {
        Funct3::BEQ => Ok(Instruction::Beq(b)),
        Funct3::BNE => Ok(Instruction::Bne(b)),
        Funct3::BLT => Ok(Instruction::Blt(b)),
        Funct3::BLTU => Ok(Instruction::Bltu(b)),
        Funct3::BGE => Ok(Instruction::Bge(b)),
        Funct3::BGEU => Ok(Instruction::Bgeu(b)),
        funct3 => Err(DecodeError::InvalidFunct3(funct3)),
    }
}

const fn decode_load(instruction: u32) -> Result<Instruction, DecodeError> {
    let i = I::decode(instruction);
    match funct3(instruction) {
        Funct3::LB => Ok(Instruction::Lb(i)),
        Funct3::LBU => Ok(Instruction::Lbu(i)),
        Funct3::LH => Ok(Instruction::Lh(i)),
        Funct3::LHU => Ok(Instruction::Lhu(i)),
        Funct3::LW => Ok(Instruction::Lw(i)),
        funct3 => Err(DecodeError::InvalidFunct3(funct3)),
    }
}

const fn decode_store(instruction: u32) -> Result<Instruction, DecodeError> {
    let s = S::decode(instruction);
    match funct3(instruction) {
        Funct3::SB => Ok(Instruction::Sb(s)),
        Funct3::SH => Ok(Instruction::Sh(s)),
        Funct3::SW => Ok(Instruction::Sw(s)),
        funct3 => Err(DecodeError::InvalidFunct3(funct3)),
    }
}

const fn decode_op_imm(instruction: u32) -> Result<Instruction, DecodeError> {
    match funct3(instruction) {
        Funct3::ADDI => Ok(Instruction::Addi(I::decode(instruction))),
        Funct3::SLTI => Ok(Instruction::Slti(I::decode(instruction))),
        Funct3::SLTIU => Ok(Instruction::Sltiu(I::decode(instruction))),
        Funct3::XORI => Ok(Instruction::Xori(I::decode(instruction))),
        Funct3::ORI => Ok(Instruction::Ori(I::decode(instruction))),
        Funct3::ANDI => Ok(Instruction::Andi(I::decode(instruction))),
        Funct3::SLLI => Ok(Instruction::Slli(IShift::decode(instruction))),

        Funct3::SRL => match shift_mode(instruction) {
            SHIFT_MODE_SRLI => Ok(Instruction::Srli(IShift::decode(instruction))),
            SHIFT_MODE_SRAI => Ok(Instruction::Srai(IShift::decode(instruction))),
            funct7 => Err(DecodeError::InvalidShiftMode(funct7)),
        },

        _ => unreachable!(), // all possible Funct3 values (0b000..=0b111) are handled
    }
}

const fn decode_op(instruction: u32) -> Result<Instruction, DecodeError> {
    let r = R::decode(instruction);
    match (funct3(instruction), funct7(instruction)) {
        (Funct3::ADD, Funct7::ADD) => Ok(Instruction::Add(r)),
        (Funct3::SUB, Funct7::SUB) => Ok(Instruction::Sub(r)),
        (Funct3::SLL, Funct7::SLL) => Ok(Instruction::Sll(r)),
        (Funct3::SRL, Funct7::SRL) => Ok(Instruction::Srl(r)),
        (Funct3::SRA, Funct7::SRA) => Ok(Instruction::Sra(r)),
        (Funct3::SLT, Funct7::SLT) => Ok(Instruction::Slt(r)),
        (Funct3::SLTU, Funct7::SLTU) => Ok(Instruction::Sltu(r)),
        (Funct3::XOR, Funct7::XOR) => Ok(Instruction::Xor(r)),
        (Funct3::OR, Funct7::OR) => Ok(Instruction::Or(r)),
        (Funct3::AND, Funct7::AND) => Ok(Instruction::And(r)),
        (Funct3::MUL, Funct7::MULDIV) => Ok(Instruction::Mul(r)),
        (Funct3::MULH, Funct7::MULDIV) => Ok(Instruction::Mulh(r)),
        (Funct3::MULHSU, Funct7::MULDIV) => Ok(Instruction::Mulhsu(r)),
        (Funct3::MULHU, Funct7::MULDIV) => Ok(Instruction::Mulhu(r)),
        (Funct3::DIV, Funct7::MULDIV) => Ok(Instruction::Div(r)),
        (Funct3::DIVU, Funct7::MULDIV) => Ok(Instruction::Divu(r)),
        (Funct3::REM, Funct7::MULDIV) => Ok(Instruction::Rem(r)),
        (Funct3::REMU, Funct7::MULDIV) => Ok(Instruction::Remu(r)),
        (funct3, funct7) => Err(DecodeError::InvalidFunct3Funct7(funct3, funct7)),
    }
}

const fn decode_fence(instruction: u32) -> Result<Instruction, DecodeError> {
    match fence_mode(instruction) {
        FenceMode::FENCE => Ok(Instruction::Fence {
            pred: fence_pred(instruction),
            succ: fence_succ(instruction),
        }),
        FenceMode::FENCE_TSO => Ok(Instruction::FenceTso),
        fence_mode => Err(DecodeError::InvalidFenceMode(fence_mode)),
    }
}

const fn decode_system(instruction: u32) -> Result<Instruction, DecodeError> {
    match funct3(instruction) {
        Funct3::PRIV => match funct12(instruction) {
            Funct12::ECALL => Ok(Instruction::Ecall),
            Funct12::EBREAK => Ok(Instruction::Ebreak),
            funct12 => Err(DecodeError::InvalidFunct12(funct12)),
        },

        Funct3::CSRRW => Ok(Instruction::Csrrw(CsrReg::decode(instruction))),
        Funct3::CSRRS => Ok(Instruction::Csrrs(CsrReg::decode(instruction))),
        Funct3::CSRRC => Ok(Instruction::Csrrc(CsrReg::decode(instruction))),
        Funct3::CSRRWI => Ok(Instruction::Csrrwi(CsrImm::decode(instruction))),
        Funct3::CSRRSI => Ok(Instruction::Csrrsi(CsrImm::decode(instruction))),
        Funct3::CSRRCI => Ok(Instruction::Csrrci(CsrImm::decode(instruction))),

        funct3 => Err(DecodeError::InvalidFunct3(funct3)),
    }
}

#[test]
fn decode_edge_cases() -> Result<(), Box<dyn Error>> {
    assert_eq!(decode(0), Err(DecodeError::InvalidOpcode(Opcode(0))));
    assert_eq!(
        decode(Opcode::BRANCH.into_u32() | (0b010 << 12)),
        Err(DecodeError::InvalidFunct3(Funct3(0b010)))
    );
    assert_eq!(
        decode(Opcode::LOAD.into_u32() | (0b111 << 12)),
        Err(DecodeError::InvalidFunct3(Funct3(0b111)))
    );
    assert_eq!(
        decode(Opcode::STORE.into_u32() | (0b111 << 12)),
        Err(DecodeError::InvalidFunct3(Funct3(0b111)))
    );
    assert_eq!(
        decode(Opcode::OP_IMM.into_u32() | (Funct3::SRL.into_u32() << 12) | (0b11_1111 << 26)),
        Err(DecodeError::InvalidShiftMode(0b11_1111))
    );
    assert_eq!(
        decode(Opcode::OP.into_u32() | (0b111 << 12) | (0b111_1111 << 25)),
        Err(DecodeError::InvalidFunct3Funct7(
            Funct3(0b111),
            Funct7(0b111_1111)
        ))
    );
    assert_eq!(
        decode(Opcode::MISC_MEM.into_u32() | (0b1111 << 28)),
        Err(DecodeError::InvalidFenceMode(FenceMode(0b1111)))
    );
    assert_eq!(
        decode(
            Opcode::SYSTEM.into_u32()
                | (Funct3::PRIV.into_u32() << 12)
                | Imm12::try_from(0b111_1111_1111)?.into_u32() << 20
        ),
        Err(DecodeError::InvalidFunct12(Funct12(0b111_1111_1111)))
    );
    assert_eq!(
        decode(Opcode::SYSTEM.into_u32() | (0b100 << 12)),
        Err(DecodeError::InvalidFunct3(Funct3(0b100)))
    );
    Ok(())
}

/// RISC-V instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instruction {
    /// [`lui`](crate::instructions::rv32i::lui)
    Lui(U),
    /// [`auipc`](crate::instructions::rv32i::auipc)
    Auipc(U),
    /// [`jal`](crate::instructions::rv32i::jal)
    Jal(J),
    /// [`jalr`](crate::instructions::rv32i::jalr)
    Jalr(I),
    /// [`beq`](crate::instructions::rv32i::beq)
    Beq(B),
    /// [`bne`](crate::instructions::rv32i::bne)
    Bne(B),
    /// [`blt`](crate::instructions::rv32i::blt)
    Blt(B),
    /// [`bltu`](crate::instructions::rv32i::bltu)
    Bltu(B),
    /// [`bge`](crate::instructions::rv32i::bge)
    Bge(B),
    /// [`bgeu`](crate::instructions::rv32i::bgeu)
    Bgeu(B),
    /// [`lb`](crate::instructions::rv32i::lb)
    Lb(I),
    /// [`lbu`](crate::instructions::rv32i::lbu)
    Lbu(I),
    /// [`lh`](crate::instructions::rv32i::lh)
    Lh(I),
    /// [`lhu`](crate::instructions::rv32i::lhu)
    Lhu(I),
    /// [`lw`](crate::instructions::rv32i::lw)
    Lw(I),
    /// [`sb`](crate::instructions::rv32i::sb)
    Sb(S),
    /// [`sh`](crate::instructions::rv32i::sh)
    Sh(S),
    /// [`sw`](crate::instructions::rv32i::sw)
    Sw(S),
    /// [`addi`](crate::instructions::rv32i::addi)
    Addi(I),
    /// [`slti`](crate::instructions::rv32i::slti)
    Slti(I),
    /// [`sltiu`](crate::instructions::rv32i::sltiu)
    Sltiu(I),
    /// [`xori`](crate::instructions::rv32i::xori)
    Xori(I),
    /// [`ori`](crate::instructions::rv32i::ori)
    Ori(I),
    /// [`andi`](crate::instructions::rv32i::andi)
    Andi(I),
    /// [`slli`](crate::instructions::rv32i::slli)
    Slli(IShift),
    /// [`srli`](crate::instructions::rv32i::srli)
    Srli(IShift),
    /// [`srai`](crate::instructions::rv32i::srai)
    Srai(IShift),
    /// [`add`](crate::instructions::rv32i::add)
    Add(R),
    /// [`sub`](crate::instructions::rv32i::sub)
    Sub(R),
    /// [`sll`](crate::instructions::rv32i::sll)
    Sll(R),
    /// [`srl`](crate::instructions::rv32i::srl)
    Srl(R),
    /// [`sra`](crate::instructions::rv32i::sra)
    Sra(R),
    /// [`slt`](crate::instructions::rv32i::slt)
    Slt(R),
    /// [`sltu`](crate::instructions::rv32i::sltu)
    Sltu(R),
    /// [`xor`](crate::instructions::rv32i::xor)
    Xor(R),
    /// [`or`](crate::instructions::rv32i::or)
    Or(R),
    /// [`and`](crate::instructions::rv32i::and)
    And(R),
    /// [`fence`](crate::instructions::rv32i::fence)
    Fence {
        /// Predecessor set
        ///
        /// Refer to [`fence`](crate::instructions::rv32i::fence) instruction documentation for details
        pred: FenceMask,
        /// Successor set
        ///
        /// Refer to [`fence`](crate::instructions::rv32i::fence) instruction documentation for details
        succ: FenceMask,
    },
    /// [`fence_tso`](crate::instructions::rv32i::fence_tso)
    FenceTso,
    /// [`ecall`](crate::instructions::rv32i::ecall)
    Ecall,
    /// [`ebreak`](crate::instructions::rv32i::ebreak)
    Ebreak,
    /// [`mul`](crate::instructions::m_ext::mul)
    Mul(R),
    /// [`mulh`](crate::instructions::m_ext::mulh)
    Mulh(R),
    /// [`mulhsu`](crate::instructions::m_ext::mulhsu)
    Mulhsu(R),
    /// [`mulhu`](crate::instructions::m_ext::mulhu)
    Mulhu(R),
    /// [`div`](crate::instructions::m_ext::div)
    Div(R),
    /// [`divu`](crate::instructions::m_ext::divu)
    Divu(R),
    /// [`rem`](crate::instructions::m_ext::rem)
    Rem(R),
    /// [`remu`](crate::instructions::m_ext::remu)
    Remu(R),
    /// [`csrrw`](crate::instructions::zicsr_ext::csrrw)
    Csrrw(CsrReg),
    /// [`csrrs`](crate::instructions::zicsr_ext::csrrs)
    Csrrs(CsrReg),
    /// [`csrrc`](crate::instructions::zicsr_ext::csrrc)
    Csrrc(CsrReg),
    /// [`csrrwi`](crate::instructions::zicsr_ext::csrrwi)
    Csrrwi(CsrImm),
    /// [`csrrsi`](crate::instructions::zicsr_ext::csrrsi)
    Csrrsi(CsrImm),
    /// [`csrrci`](crate::instructions::zicsr_ext::csrrci)
    Csrrci(CsrImm),
}

/// CSR instruction where the value argument is a register
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CsrReg {
    /// Destination register
    pub rd: Register,
    /// Source register containing the CSR address
    pub rs1: Register,
    /// CSR address
    pub csr: Csr,
}

impl CsrReg {
    const fn decode(instruction: u32) -> Self {
        Self {
            rd: rd(instruction),
            rs1: rs1_reg(instruction),
            csr: csr(instruction),
        }
    }
}

/// CSR instruction where the value argument is a 5-bit unsigned immediate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CsrImm {
    /// Destination register
    pub rd: Register,
    /// 5-bit unsigned immediate representing the CSR address
    pub rs1: Uimm5,
    /// CSR address
    pub csr: Csr,
}

impl CsrImm {
    const fn decode(instruction: u32) -> Self {
        Self {
            rd: rd(instruction),
            rs1: rs1_imm(instruction),
            csr: csr(instruction),
        }
    }
}

/// RISC-V U instruction format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct U {
    /// Destination register
    pub rd: Register,
    /// 32-bit signed immediate
    pub imm: i32,
}

impl U {
    const fn decode(instruction: u32) -> Self {
        Self {
            rd: rd(instruction),
            imm: u_imm(instruction),
        }
    }
}

#[allow(clippy::cast_possible_wrap)]
const fn u_imm(instruction: u32) -> i32 {
    (instruction & 0xFFFF_F000) as i32
}

/// RISC-V J instruction format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct J {
    /// Destination register
    pub rd: Register,
    /// 21-bit signed immediate
    pub imm: JImm,
}

impl J {
    const fn decode(instruction: u32) -> Self {
        Self {
            rd: rd(instruction),
            imm: jimm(instruction),
        }
    }
}

/// RISC-V I instruction format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct I {
    /// Destination register
    pub rd: Register,
    /// Source register
    pub rs1: Register,
    /// 12-bit signed immediate
    pub imm: Imm12,
}

impl I {
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    const fn decode(instruction: u32) -> Self {
        Self {
            rd: rd(instruction),
            rs1: rs1_reg(instruction),
            imm: i_imm12(instruction),
        }
    }
}

/// RISC-V special I instruction format (
/// [`slli`](crate::instructions::rv32i::slli),
/// [`srli`](crate::instructions::rv32i::srli),
/// [`srai`](crate::instructions::rv32i::srai)
/// )
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IShift {
    /// Destination register
    pub rd: Register,
    /// Source register 1
    pub rs1: Register,
    /// Source register 2 or 5-bit unsigned immediate
    pub shamt: Uimm5,
}

impl IShift {
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    const fn decode(instruction: u32) -> Self {
        Self {
            rd: rd(instruction),
            rs1: rs1_reg(instruction),
            shamt: shamt(instruction),
        }
    }
}

/// RISC-V S instruction format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct S {
    /// Source register
    pub rs1: Register,
    /// 12-bit signed immediate
    pub imm: Imm12,
    /// Destination register
    pub rs2: Register,
}

impl S {
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    const fn decode(instruction: u32) -> Self {
        Self {
            rs1: rs1_reg(instruction),
            imm: s_imm12(instruction),
            rs2: rs2_reg(instruction),
        }
    }
}

/// RISC-V R instruction format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct R {
    /// Destination register
    pub rd: Register,
    /// Source register 1
    pub rs1: Register,
    /// Source register 2 or 5-bit unsigned immediate
    pub rs2: Register,
}

impl R {
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    const fn decode(instruction: u32) -> Self {
        Self {
            rd: rd(instruction),
            rs1: rs1_reg(instruction),
            rs2: rs2_reg(instruction),
        }
    }
}

/// RISC-V B instruction format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct B {
    /// 13-bit signed immediate
    pub imm: BImm,
    /// Source register 1
    pub rs1: Register,
    /// Source register 2
    pub rs2: Register,
}

impl B {
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    const fn decode(instruction: u32) -> Self {
        Self {
            imm: bimm(instruction),
            rs1: rs1_reg(instruction),
            rs2: rs2_reg(instruction),
        }
    }
}

/// Instruction decode error
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecodeError {
    ///
    InvalidOpcode(Opcode),
    ///
    InvalidFunct3(Funct3),
    ///
    InvalidFunct7(Funct7),
    ///
    InvalidFenceMode(FenceMode),
    ///
    InvalidFunct12(Funct12),
    ///
    InvalidFunct3Funct7(Funct3, Funct7),
    ///
    InvalidShiftMode(u8),
}

impl Debug for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::InvalidOpcode(opcode) => write!(f, "InvalidOpcode({opcode:?})"),
            DecodeError::InvalidFunct3(funct3) => write!(f, "InvalidFunct3({funct3:?})"),
            DecodeError::InvalidFunct7(funct7) => write!(f, "InvalidFunct7({funct7:?})"),
            DecodeError::InvalidFenceMode(fence_mode) => {
                write!(f, "InvalidFenceMode({fence_mode:?})")
            }
            DecodeError::InvalidFunct12(funct12) => write!(f, "InvalidSystemCall({funct12:?})"),
            DecodeError::InvalidFunct3Funct7(funct3, funct7) => {
                write!(f, "InvalidFunct3Funct7({funct3:?}, {funct7:?})")
            }
            DecodeError::InvalidShiftMode(mode) => write!(f, "InvalidShiftMode(0b{mode:06b})"),
        }
    }
}

#[test]
fn decode_error_debug() {
    assert_eq!(
        format!("{:?}", DecodeError::InvalidOpcode(Opcode(0b111_1111))),
        "InvalidOpcode(Opcode(0b1111111))"
    );
    assert_eq!(
        format!("{:?}", DecodeError::InvalidFunct3(Funct3(0b111))),
        "InvalidFunct3(Funct3(0b111))"
    );
    assert_eq!(
        format!("{:?}", DecodeError::InvalidFunct7(Funct7(0b111_1111))),
        "InvalidFunct7(Funct7(0b1111111))"
    );
    assert_eq!(
        format!("{:?}", DecodeError::InvalidFenceMode(FenceMode(0b1111))),
        "InvalidFenceMode(FenceMode(0b1111))"
    );
    assert_eq!(
        format!(
            "{:?}",
            DecodeError::InvalidFunct12(Funct12(0b1111_1111_1111))
        ),
        "InvalidSystemCall(Funct12(0b111111111111))"
    );
    assert_eq!(
        format!(
            "{:?}",
            DecodeError::InvalidFunct3Funct7(Funct3(0b111), Funct7(0b111_1111))
        ),
        "InvalidFunct3Funct7(Funct3(0b111), Funct7(0b1111111))"
    );
    assert_eq!(
        format!("{:?}", DecodeError::InvalidShiftMode(0b11_1111)),
        "InvalidShiftMode(0b111111)"
    );
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::InvalidOpcode(opcode) => write!(f, "invalid opcode: {opcode}"),
            DecodeError::InvalidFunct3(funct3) => write!(f, "invalid funct3: {funct3}"),
            DecodeError::InvalidFunct7(funct7) => write!(f, "invalid funct7: {funct7}"),
            DecodeError::InvalidFenceMode(fence_mode) => {
                write!(f, "invalid fence mode: {fence_mode}")
            }
            DecodeError::InvalidFunct12(funct12) => {
                write!(f, "invalid system call: {funct12}")
            }
            DecodeError::InvalidFunct3Funct7(funct3, funct7) => write!(
                f,
                "invalid funct3 and funct7 combination: {funct3}, {funct7}",
            ),
            DecodeError::InvalidShiftMode(mode) => write!(f, "invalid shift mode: 0b{mode:06b}"),
        }
    }
}

#[test]
fn decode_error_display() {
    assert_eq!(
        DecodeError::InvalidOpcode(Opcode(0b111_1111)).to_string(),
        "invalid opcode: 0b1111111"
    );
    assert_eq!(
        DecodeError::InvalidFenceMode(FenceMode(0b1111)).to_string(),
        "invalid fence mode: 0b1111"
    );
    assert_eq!(
        DecodeError::InvalidFunct3(Funct3(0b111)).to_string(),
        "invalid funct3: 0b111"
    );
    assert_eq!(
        DecodeError::InvalidFunct7(Funct7(0b111_1111)).to_string(),
        "invalid funct7: 0b1111111"
    );
    assert_eq!(
        DecodeError::InvalidFunct12(Funct12(0b1111_1111_1111)).to_string(),
        "invalid system call: 0b111111111111"
    );
    assert_eq!(
        DecodeError::InvalidFunct3Funct7(Funct3(0b111), Funct7(0b111_1111)).to_string(),
        "invalid funct3 and funct7 combination: 0b111, 0b1111111"
    );
    assert_eq!(
        DecodeError::InvalidShiftMode(0b11_1111).to_string(),
        "invalid shift mode: 0b111111"
    );
}

impl Error for DecodeError {}

#[allow(clippy::cast_possible_truncation)]
const fn opcode(instruction: u32) -> Opcode {
    Opcode(bitfield::<0, 7>(instruction) as u8)
}

#[allow(clippy::cast_possible_truncation)]
const fn rd(instruction: u32) -> Register {
    Register(bitfield::<7, 12>(instruction) as u8)
}

#[allow(clippy::cast_possible_truncation)]
const fn funct3(instruction: u32) -> Funct3 {
    Funct3(bitfield::<12, 15>(instruction) as u8)
}

#[allow(clippy::cast_possible_truncation)]
const fn rs1_reg(instruction: u32) -> Register {
    Register(bitfield::<15, 20>(instruction) as u8)
}

#[allow(clippy::cast_possible_truncation)]
const fn rs1_imm(instruction: u32) -> Uimm5 {
    Uimm5(bitfield::<15, 20>(instruction) as u8)
}

#[allow(clippy::cast_possible_truncation)]
const fn rs2_reg(instruction: u32) -> Register {
    Register(bitfield::<20, 25>(instruction) as u8)
}

#[allow(clippy::cast_possible_truncation)]
const fn shamt(instruction: u32) -> Uimm5 {
    Uimm5(bitfield::<20, 25>(instruction) as u8)
}

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
const fn i_imm12(instruction: u32) -> Imm12 {
    Imm12((((bitfield::<20, 32>(instruction) as u16) << 4) as i16) >> 4) // sign-extend
}

#[allow(clippy::cast_possible_truncation)]
const fn csr(instruction: u32) -> Csr {
    Csr(bitfield::<20, 32>(instruction) as u16)
}

#[allow(clippy::cast_possible_truncation)]
const fn funct7(instruction: u32) -> Funct7 {
    Funct7(bitfield::<25, 32>(instruction) as u8)
}

const SHIFT_MODE_SRLI: u8 = 0b00_0000;
const SHIFT_MODE_SRAI: u8 = 0b01_0000;

#[allow(clippy::cast_possible_truncation)]
const fn shift_mode(instruction: u32) -> u8 {
    bitfield::<26, 32>(instruction) as u8
}

#[allow(clippy::cast_possible_truncation)]
const fn funct12(instruction: u32) -> Funct12 {
    Funct12(bitfield::<20, 32>(instruction) as u16)
}

#[allow(clippy::cast_possible_truncation)]
const fn fence_pred(instruction: u32) -> FenceMask {
    FenceMask(bitfield::<20, 24>(instruction) as u8)
}

#[allow(clippy::cast_possible_truncation)]
const fn fence_succ(instruction: u32) -> FenceMask {
    FenceMask(bitfield::<24, 28>(instruction) as u8)
}

#[allow(clippy::cast_possible_truncation)]
const fn fence_mode(instruction: u32) -> FenceMode {
    FenceMode(bitfield::<28, 32>(instruction) as u8)
}

#[allow(clippy::cast_possible_wrap)]
const fn bimm(instruction: u32) -> BImm {
    let imm = merge_bitfields(&[
        (11..12, instruction, 7..8),
        (1..5, instruction, 8..12),
        (5..11, instruction, 25..31),
        (12..13, instruction, 31..32),
    ]);
    BImm(((imm << 19) as i32 >> 19) as i16)
}

#[allow(clippy::cast_possible_wrap)]
const fn jimm(instruction: u32) -> JImm {
    #[allow(clippy::cast_possible_truncation)]
    let imm = merge_bitfields(&[
        (12..20, instruction, 12..20),
        (11..12, instruction, 20..21),
        (1..11, instruction, 21..31),
        (20..21, instruction, 31..32),
    ]);
    JImm((imm << 11) as i32 >> 11) // sign-extend
}

#[allow(clippy::cast_possible_wrap)]
const fn s_imm12(instruction: u32) -> Imm12 {
    let imm = merge_bitfields(&[(0..5, instruction, 7..12), (5..12, instruction, 25..32)]);
    Imm12(((imm << 20) as i32 >> 20) as i16)
}
