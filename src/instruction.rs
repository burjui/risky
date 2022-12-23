//! RISC-V instruction definitions for decoding and encoding

use core::fmt;
use std::fmt::Display;

use crate::{
    common::{bimm::BImm, csr::Csr, fence_mask::FenceMask, imm12::Imm12, jimm::JImm, uimm5::Uimm5},
    decoding::{
        decode_bimm, decode_csr, decode_i_imm12, decode_jimm, decode_rd, decode_rs1_imm,
        decode_rs1_reg, decode_rs2_reg, decode_s_imm12, decode_shamt,
    },
    registers::Register,
};

/// RISC-V instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instruction {
    // --- RV32I ---
    /// [`lui`](crate::rv32i::lui)
    Lui(U),
    /// [`auipc`](crate::rv32i::auipc)
    Auipc(U),
    /// [`jal`](crate::rv32i::jal)
    Jal(J),
    /// [`jalr`](crate::rv32i::jalr)
    Jalr(I),
    /// [`beq`](crate::rv32i::beq)
    Beq(B),
    /// [`bne`](crate::rv32i::bne)
    Bne(B),
    /// [`blt`](crate::rv32i::blt)
    Blt(B),
    /// [`bltu`](crate::rv32i::bltu)
    Bltu(B),
    /// [`bge`](crate::rv32i::bge)
    Bge(B),
    /// [`bgeu`](crate::rv32i::bgeu)
    Bgeu(B),
    /// [`lb`](crate::rv32i::lb)
    Lb(I),
    /// [`lbu`](crate::rv32i::lbu)
    Lbu(I),
    /// [`lh`](crate::rv32i::lh)
    Lh(I),
    /// [`lhu`](crate::rv32i::lhu)
    Lhu(I),
    /// [`lw`](crate::rv32i::lw)
    Lw(I),
    /// [`sb`](crate::rv32i::sb)
    Sb(S),
    /// [`sh`](crate::rv32i::sh)
    Sh(S),
    /// [`sw`](crate::rv32i::sw)
    Sw(S),
    /// [`addi`](crate::rv32i::addi)
    Addi(I),
    /// [`slti`](crate::rv32i::slti)
    Slti(I),
    /// [`sltiu`](crate::rv32i::sltiu)
    Sltiu(I),
    /// [`xori`](crate::rv32i::xori)
    Xori(I),
    /// [`ori`](crate::rv32i::ori)
    Ori(I),
    /// [`andi`](crate::rv32i::andi)
    Andi(I),
    /// [`slli`](crate::rv32i::slli)
    Slli(IShift),
    /// [`srli`](crate::rv32i::srli)
    Srli(IShift),
    /// [`srai`](crate::rv32i::srai)
    Srai(IShift),
    /// [`add`](crate::rv32i::add)
    Add(R),
    /// [`sub`](crate::rv32i::sub)
    Sub(R),
    /// [`sll`](crate::rv32i::sll)
    Sll(R),
    /// [`srl`](crate::rv32i::srl)
    Srl(R),
    /// [`sra`](crate::rv32i::sra)
    Sra(R),
    /// [`slt`](crate::rv32i::slt)
    Slt(R),
    /// [`sltu`](crate::rv32i::sltu)
    Sltu(R),
    /// [`xor`](crate::rv32i::xor)
    Xor(R),
    /// [`or`](crate::rv32i::or)
    Or(R),
    /// [`and`](crate::rv32i::and)
    And(R),
    /// [`fence`](crate::rv32i::fence)
    Fence {
        /// Predecessor set
        ///
        /// Refer to [`fence`](crate::rv32i::fence) instruction documentation for details
        pred: FenceMask,
        /// Successor set
        ///
        /// Refer to [`fence`](crate::rv32i::fence) instruction documentation for details
        succ: FenceMask,
    },
    /// [`fence_tso`](crate::rv32i::fence_tso)
    FenceTso,
    /// [`ecall`](crate::rv32i::ecall)
    Ecall,
    /// [`ebreak`](crate::rv32i::ebreak)
    Ebreak,

    // --- M standard extension ---
    /// [`mul`](crate::m_ext::mul)
    Mul(R),
    /// [`mulh`](crate::m_ext::mulh)
    Mulh(R),
    /// [`mulhsu`](crate::m_ext::mulhsu)
    Mulhsu(R),
    /// [`mulhu`](crate::m_ext::mulhu)
    Mulhu(R),
    /// [`div`](crate::m_ext::div)
    Div(R),
    /// [`divu`](crate::m_ext::divu)
    Divu(R),
    /// [`rem`](crate::m_ext::rem)
    Rem(R),
    /// [`remu`](crate::m_ext::remu)
    Remu(R),

    // --- Zicsr standard extension ---
    /// [`csrrw`](crate::zicsr_ext::csrrw)
    Csrrw(CsrReg),
    /// [`csrrs`](crate::zicsr_ext::csrrs)
    Csrrs(CsrReg),
    /// [`csrrc`](crate::zicsr_ext::csrrc)
    Csrrc(CsrReg),
    /// [`csrrwi`](crate::zicsr_ext::csrrwi)
    Csrrwi(CsrImm),
    /// [`csrrsi`](crate::zicsr_ext::csrrsi)
    Csrrsi(CsrImm),
    /// [`csrrci`](crate::zicsr_ext::csrrci)
    Csrrci(CsrImm),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Lui(u) => write!(f, "lui {u}"),
            Instruction::Auipc(u) => write!(f, "auipc {u}"),
            Instruction::Jal(j) => write!(f, "jal {j}"),
            Instruction::Jalr(i) => write!(f, "jalr {i}"),
            Instruction::Beq(b) => write!(f, "beq {b}"),
            Instruction::Bne(b) => write!(f, "bne {b}"),
            Instruction::Blt(b) => write!(f, "blt {b}"),
            Instruction::Bltu(b) => write!(f, "bltu {b}"),
            Instruction::Bge(b) => write!(f, "bge {b}"),
            Instruction::Bgeu(b) => write!(f, "bgeu {b}"),
            Instruction::Lb(i) => write!(f, "lb {}", LoadDisplay(i)),
            Instruction::Lbu(i) => write!(f, "lbu {}", LoadDisplay(i)),
            Instruction::Lh(i) => write!(f, "lh {}", LoadDisplay(i)),
            Instruction::Lhu(i) => write!(f, "lhu {}", LoadDisplay(i)),
            Instruction::Lw(i) => write!(f, "lw {}", LoadDisplay(i)),
            Instruction::Sb(s) => write!(f, "sb {s}"),
            Instruction::Sh(s) => write!(f, "sh {s}"),
            Instruction::Sw(s) => write!(f, "sw {s}"),
            Instruction::Addi(i) => write!(f, "addi {i}"),
            Instruction::Slti(i) => write!(f, "slti {i}"),
            Instruction::Sltiu(i) => write!(f, "sltiu {i}"),
            Instruction::Xori(i) => write!(f, "xori {i}"),
            Instruction::Ori(i) => write!(f, "ori {i}"),
            Instruction::Andi(i) => write!(f, "andi {i}"),
            Instruction::Slli(i) => write!(f, "slli {i}"),
            Instruction::Srli(i) => write!(f, "srli {i}"),
            Instruction::Srai(i) => write!(f, "srai {i}"),
            Instruction::Add(r) => write!(f, "add {r}"),
            Instruction::Sub(r) => write!(f, "sub {r}"),
            Instruction::Sll(r) => write!(f, "sll {r}"),
            Instruction::Srl(r) => write!(f, "srl {r}"),
            Instruction::Sra(r) => write!(f, "sra {r}"),
            Instruction::Slt(r) => write!(f, "slt {r}"),
            Instruction::Sltu(r) => write!(f, "sltu {r}"),
            Instruction::Xor(r) => write!(f, "xor {r}"),
            Instruction::Or(r) => write!(f, "or {r}"),
            Instruction::And(r) => write!(f, "and {r}"),
            Instruction::Fence { pred, succ } => write!(f, "fence {pred}, {succ}"),
            Instruction::FenceTso => write!(f, "fence.tso"),
            Instruction::Ecall => write!(f, "ecall"),
            Instruction::Ebreak => write!(f, "ebreak"),
            Instruction::Mul(r) => write!(f, "mul {r}"),
            Instruction::Mulh(r) => write!(f, "mulh {r}"),
            Instruction::Mulhsu(r) => write!(f, "mulhsu {r}"),
            Instruction::Mulhu(r) => write!(f, "mulhu {r}"),
            Instruction::Div(r) => write!(f, "div {r}"),
            Instruction::Divu(r) => write!(f, "divu {r}"),
            Instruction::Rem(r) => write!(f, "rem {r}"),
            Instruction::Remu(r) => write!(f, "remu {r}"),
            Instruction::Csrrw(c) => write!(f, "csrrw {c}"),
            Instruction::Csrrs(c) => write!(f, "csrrs {c}"),
            Instruction::Csrrc(c) => write!(f, "csrrc {c}"),
            Instruction::Csrrwi(c) => write!(f, "csrrwi {c}"),
            Instruction::Csrrsi(c) => write!(f, "csrrsi {c}"),
            Instruction::Csrrci(c) => write!(f, "csrrci {c}"),
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
    pub(crate) const fn decode(instruction: u32) -> Self {
        Self {
            rd: decode_rd(instruction),
            imm: u_imm(instruction),
        }
    }
}

impl Display for U {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.rd, self.imm)
    }
}

#[allow(clippy::cast_possible_wrap)]
const fn u_imm(instruction: u32) -> i32 {
    (instruction & !0xFFF) as i32
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
    pub(crate) const fn decode(instruction: u32) -> Self {
        Self {
            rd: decode_rd(instruction),
            imm: decode_jimm(instruction),
        }
    }
}

impl Display for J {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.rd, self.imm)
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
    pub(crate) const fn decode(instruction: u32) -> Self {
        Self {
            rd: decode_rd(instruction),
            rs1: decode_rs1_reg(instruction),
            imm: decode_i_imm12(instruction),
        }
    }
}

impl Display for I {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.rd, self.rs1, self.imm)
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
    pub(crate) const fn decode(instruction: u32) -> Self {
        Self {
            imm: decode_bimm(instruction),
            rs1: decode_rs1_reg(instruction),
            rs2: decode_rs2_reg(instruction),
        }
    }
}

impl Display for B {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.imm, self.rs1, self.rs2)
    }
}

struct LoadDisplay<'a>(&'a I);

impl Display for LoadDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}[{}]", self.0.rd, self.0.rs1, self.0.imm)
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
    pub(crate) const fn decode(instruction: u32) -> Self {
        Self {
            rs1: decode_rs1_reg(instruction),
            imm: decode_s_imm12(instruction),
            rs2: decode_rs2_reg(instruction),
        }
    }
}

impl Display for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}[{}], {}", self.rs1, self.imm, self.rs2)
    }
}

/// RISC-V special I instruction format (
/// [`slli`](crate::rv32i::slli),
/// [`srli`](crate::rv32i::srli),
/// [`srai`](crate::rv32i::srai)
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
    pub(crate) const fn decode(instruction: u32) -> Self {
        Self {
            rd: decode_rd(instruction),
            rs1: decode_rs1_reg(instruction),
            shamt: decode_shamt(instruction),
        }
    }
}

impl Display for IShift {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.rd, self.rs1, self.shamt)
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
    pub(crate) const fn decode(instruction: u32) -> Self {
        Self {
            rd: decode_rd(instruction),
            rs1: decode_rs1_reg(instruction),
            rs2: decode_rs2_reg(instruction),
        }
    }
}

impl Display for R {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.rd, self.rs1, self.rs2)
    }
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
    pub(crate) const fn decode(instruction: u32) -> Self {
        Self {
            rd: decode_rd(instruction),
            rs1: decode_rs1_reg(instruction),
            csr: decode_csr(instruction),
        }
    }
}

impl Display for CsrReg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.rd, self.rs1, self.csr)
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
    pub(crate) const fn decode(instruction: u32) -> Self {
        Self {
            rd: decode_rd(instruction),
            rs1: decode_rs1_imm(instruction),
            csr: decode_csr(instruction),
        }
    }
}

impl Display for CsrImm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.rd, self.rs1, self.csr)
    }
}
