//! RISC-V instruction definitions for decoding and encoding

use core::fmt;
use std::{
    fmt::Display,
    io::{self, Write},
};

use crate::{
    common::{bimm::BImm, csr::Csr, fence_mask::FenceMask, imm12::Imm12, jimm::JImm, uimm5::Uimm5},
    decoding::{
        decode_bimm, decode_csr, decode_i_imm12, decode_jimm, decode_rd, decode_rs1_imm,
        decode_rs1_reg, decode_rs2_reg, decode_s_imm12, decode_shamt, decode_u_imm,
    },
    m_ext::{div, divu, mul, mulh, mulhsu, mulhu, rem, remu},
    registers::Register,
    rv32i::{
        add, addi, and, andi, auipc, beq, bge, bgeu, blt, bltu, bne, ebreak, ecall, fence,
        fence_tso, jal, jalr, lb, lbu, lh, lhu, lui, lw, or, ori, sb, sh, sll, slli, slt, slti,
        sltiu, sltu, sra, srai, srl, srli, sub, sw, xor, xori,
    },
    zicsr_ext::{csrrc, csrrci, csrrs, csrrsi, csrrw, csrrwi},
};

/// RISC-V instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instruction {
    // --- RV32I ---
    /// [`lui`]
    Lui(U),
    /// [`auipc`]
    Auipc(U),
    /// [`jal`]
    Jal(J),
    /// [`jalr`]
    Jalr(I),
    /// [`beq`]
    Beq(B),
    /// [`bne`]
    Bne(B),
    /// [`blt`]
    Blt(B),
    /// [`bltu`]
    Bltu(B),
    /// [`bge`]
    Bge(B),
    /// [`bgeu`]
    Bgeu(B),
    /// [`lb`]
    Lb(I),
    /// [`lbu`]
    Lbu(I),
    /// [`lh`]
    Lh(I),
    /// [`lhu`]
    Lhu(I),
    /// [`lw`]
    Lw(I),
    /// [`sb`]
    Sb(S),
    /// [`sh`]
    Sh(S),
    /// [`sw`]
    Sw(S),
    /// [`addi`]
    Addi(I),
    /// [`slti`]
    Slti(I),
    /// [`sltiu`]
    Sltiu(I),
    /// [`xori`]
    Xori(I),
    /// [`ori`]
    Ori(I),
    /// [`andi`]
    Andi(I),
    /// [`slli`]
    Slli(IShift),
    /// [`srli`]
    Srli(IShift),
    /// [`srai`]
    Srai(IShift),
    /// [`add`]
    Add(R),
    /// [`sub`]
    Sub(R),
    /// [`sll`]
    Sll(R),
    /// [`srl`]
    Srl(R),
    /// [`sra`]
    Sra(R),
    /// [`slt`]
    Slt(R),
    /// [`sltu`]
    Sltu(R),
    /// [`xor`]
    Xor(R),
    /// [`or`]
    Or(R),
    /// [`and`]
    And(R),
    /// [`fence`]
    Fence {
        /// Predecessor set
        ///
        /// Refer to [`fence`] instruction documentation for details
        pred: FenceMask,
        /// Successor set
        ///
        /// Refer to [`fence`] instruction documentation for details
        succ: FenceMask,
    },
    /// [`fence_tso`]
    FenceTso,
    /// [`ecall`]
    Ecall,
    /// [`ebreak`]
    Ebreak,

    // --- M standard extension ---
    /// [`mul`]
    Mul(R),
    /// [`mulh`]
    Mulh(R),
    /// [`mulhsu`]
    Mulhsu(R),
    /// [`mulhu`]
    Mulhu(R),
    /// [`div`]
    Div(R),
    /// [`divu`]
    Divu(R),
    /// [`rem`]
    Rem(R),
    /// [`remu`]
    Remu(R),

    // --- Zicsr standard extension ---
    /// [`csrrw`]
    Csrrw(CsrReg),
    /// [`csrrs`]
    Csrrs(CsrReg),
    /// [`csrrc`]
    Csrrc(CsrReg),
    /// [`csrrwi`]
    Csrrwi(CsrImm),
    /// [`csrrsi`]
    Csrrsi(CsrImm),
    /// [`csrrci`]
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

impl Instruction {
    /// Encode the instruction
    #[must_use]
    pub const fn encode(self) -> EncodedInstruction {
        match self {
            Instruction::Lui(u) => EncodedInstruction::Standard(lui(u.rd, u.imm)),
            Instruction::Auipc(u) => EncodedInstruction::Standard(auipc(u.rd, u.imm)),
            Instruction::Jal(j) => EncodedInstruction::Standard(jal(j.rd, j.imm)),
            Instruction::Jalr(i) => EncodedInstruction::Standard(jalr(i.rd, i.rs1, i.imm)),
            Instruction::Beq(b) => EncodedInstruction::Standard(beq(b.imm, b.rs1, b.rs2)),
            Instruction::Bne(b) => EncodedInstruction::Standard(bne(b.imm, b.rs1, b.rs2)),
            Instruction::Blt(b) => EncodedInstruction::Standard(blt(b.imm, b.rs1, b.rs2)),
            Instruction::Bltu(b) => EncodedInstruction::Standard(bltu(b.imm, b.rs1, b.rs2)),
            Instruction::Bge(b) => EncodedInstruction::Standard(bge(b.imm, b.rs1, b.rs2)),
            Instruction::Bgeu(b) => EncodedInstruction::Standard(bgeu(b.imm, b.rs1, b.rs2)),
            Instruction::Lb(i) => EncodedInstruction::Standard(lb(i.rd, i.rs1, i.imm)),
            Instruction::Lbu(i) => EncodedInstruction::Standard(lbu(i.rd, i.rs1, i.imm)),
            Instruction::Lh(i) => EncodedInstruction::Standard(lh(i.rd, i.rs1, i.imm)),
            Instruction::Lhu(i) => EncodedInstruction::Standard(lhu(i.rd, i.rs1, i.imm)),
            Instruction::Lw(i) => EncodedInstruction::Standard(lw(i.rd, i.rs1, i.imm)),
            Instruction::Sb(s) => EncodedInstruction::Standard(sb(s.rs1, s.imm, s.rs2)),
            Instruction::Sh(s) => EncodedInstruction::Standard(sh(s.rs1, s.imm, s.rs2)),
            Instruction::Sw(s) => EncodedInstruction::Standard(sw(s.rs1, s.imm, s.rs2)),
            Instruction::Addi(i) => EncodedInstruction::Standard(addi(i.rd, i.rs1, i.imm)),
            Instruction::Slti(i) => EncodedInstruction::Standard(slti(i.rd, i.rs1, i.imm)),
            Instruction::Sltiu(i) => EncodedInstruction::Standard(sltiu(i.rd, i.rs1, i.imm)),
            Instruction::Xori(i) => EncodedInstruction::Standard(xori(i.rd, i.rs1, i.imm)),
            Instruction::Ori(i) => EncodedInstruction::Standard(ori(i.rd, i.rs1, i.imm)),
            Instruction::Andi(i) => EncodedInstruction::Standard(andi(i.rd, i.rs1, i.imm)),
            Instruction::Slli(i) => EncodedInstruction::Standard(slli(i.rd, i.rs1, i.shamt)),
            Instruction::Srli(i) => EncodedInstruction::Standard(srli(i.rd, i.rs1, i.shamt)),
            Instruction::Srai(i) => EncodedInstruction::Standard(srai(i.rd, i.rs1, i.shamt)),
            Instruction::Add(r) => EncodedInstruction::Standard(add(r.rd, r.rs1, r.rs2)),
            Instruction::Sub(r) => EncodedInstruction::Standard(sub(r.rd, r.rs1, r.rs2)),
            Instruction::Sll(r) => EncodedInstruction::Standard(sll(r.rd, r.rs1, r.rs2)),
            Instruction::Srl(r) => EncodedInstruction::Standard(srl(r.rd, r.rs1, r.rs2)),
            Instruction::Sra(r) => EncodedInstruction::Standard(sra(r.rd, r.rs1, r.rs2)),
            Instruction::Slt(r) => EncodedInstruction::Standard(slt(r.rd, r.rs1, r.rs2)),
            Instruction::Sltu(r) => EncodedInstruction::Standard(sltu(r.rd, r.rs1, r.rs2)),
            Instruction::Xor(r) => EncodedInstruction::Standard(xor(r.rd, r.rs1, r.rs2)),
            Instruction::Or(r) => EncodedInstruction::Standard(or(r.rd, r.rs1, r.rs2)),
            Instruction::And(r) => EncodedInstruction::Standard(and(r.rd, r.rs1, r.rs2)),
            Instruction::Fence { pred, succ } => EncodedInstruction::Standard(fence(pred, succ)),
            Instruction::FenceTso => EncodedInstruction::Standard(fence_tso()),
            Instruction::Ecall => EncodedInstruction::Standard(ecall()),
            Instruction::Ebreak => EncodedInstruction::Standard(ebreak()),
            Instruction::Mul(r) => EncodedInstruction::Standard(mul(r.rd, r.rs1, r.rs2)),
            Instruction::Mulh(r) => EncodedInstruction::Standard(mulh(r.rd, r.rs1, r.rs2)),
            Instruction::Mulhsu(r) => EncodedInstruction::Standard(mulhsu(r.rd, r.rs1, r.rs2)),
            Instruction::Mulhu(r) => EncodedInstruction::Standard(mulhu(r.rd, r.rs1, r.rs2)),
            Instruction::Div(r) => EncodedInstruction::Standard(div(r.rd, r.rs1, r.rs2)),
            Instruction::Divu(r) => EncodedInstruction::Standard(divu(r.rd, r.rs1, r.rs2)),
            Instruction::Rem(r) => EncodedInstruction::Standard(rem(r.rd, r.rs1, r.rs2)),
            Instruction::Remu(r) => EncodedInstruction::Standard(remu(r.rd, r.rs1, r.rs2)),
            Instruction::Csrrw(c) => EncodedInstruction::Standard(csrrw(c.rd, c.rs1, c.csr)),
            Instruction::Csrrs(c) => EncodedInstruction::Standard(csrrs(c.rd, c.rs1, c.csr)),
            Instruction::Csrrc(c) => EncodedInstruction::Standard(csrrc(c.rd, c.rs1, c.csr)),
            Instruction::Csrrwi(c) => EncodedInstruction::Standard(csrrwi(c.rd, c.rs1, c.csr)),
            Instruction::Csrrsi(c) => EncodedInstruction::Standard(csrrsi(c.rd, c.rs1, c.csr)),
            Instruction::Csrrci(c) => EncodedInstruction::Standard(csrrci(c.rd, c.rs1, c.csr)),
        }
    }

    /// Write the instruction to the destination
    #[allow(clippy::missing_errors_doc)]
    pub fn write(&self, w: impl Write) -> io::Result<()> {
        self.encode().write(w)
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
            imm: decode_u_imm(instruction),
        }
    }
}

impl Display for U {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.rd, self.imm)
    }
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
/// [`slli`],
/// [`srli`],
/// [`srai`]
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

/// RISC-V instruction encoded in a little-endian value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncodedInstruction {
    /// Standard 32-bit instruction
    Standard(u32),
    /// Compressed 16-bit instruction (C standard extension)
    Compressed(u16),
}

impl EncodedInstruction {
    /// Length of the encoded instruction in bytes
    #[allow(clippy::len_without_is_empty)]
    #[must_use]
    pub const fn len(&self) -> usize {
        match self {
            EncodedInstruction::Standard(_) => 4,
            EncodedInstruction::Compressed(_) => 2,
        }
    }

    /// Write the instruction to the destination
    #[allow(clippy::missing_errors_doc)]
    pub fn write(&self, mut w: impl Write) -> io::Result<()> {
        match self {
            EncodedInstruction::Standard(i) => w.write_all(&i.to_le_bytes()),
            EncodedInstruction::Compressed(i) => w.write_all(&i.to_le_bytes()),
        }
    }
}
