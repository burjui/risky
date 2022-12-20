//! Zicsr standard extension

use super::encoding::i_instruction;
use crate::{
    common::{
        funct3::Funct3, imm12::Imm12, opcode::Opcode, reg_or_uimm5::RegOrUimm5, uimm5::Uimm5,
    },
    registers::{Register, X0},
};

/// "atomic CSR Read/Write" instruction atomically swaps values in the CSRs and general-purpose
/// registers. `csrrw` reads the old value of the CSR register `csr`, zero-extends the value to XLEN
/// bits, then writes it to the register `rd`. The initial value in `rs1` is written to the CSR. If
/// `rs1` = [X0], then the instruction will not write to the CSR at all, and so shall not cause any
/// of the side effects that might otherwise occur on a CSR write, nor raise illegal instruction
/// exceptions on accesses to read-only CSRs. `csrrw` always reads the addressed CSR and cause any
/// read side effects regardless of `rs1` and `rd` values. Note that if `rs1` specifies a register
/// holding a zero value other than [X0], the instruction will still attempt to write the unmodified
/// value back to the CSR and will cause any attendant side effects. A `csrrw` with `rs1` = [X0]
/// will attempt to write zero to the destination CSR.
///
/// Other instructions for accessing CSRs:
/// [csrrwi], [csrrs], [csrrsi], [csrs], [csrsi], [csrrc], [csrrci], [csrc], [csrci], [csrr]
#[must_use]
#[inline]
pub const fn csrrw(rd: Register, rs1: Register, csr: Imm12) -> u32 {
    csr_instruction(rd, RegOrUimm5::Register(rs1), csr, Funct3::CSRRW)
}

/// "atomic CSR Read and Set bits" instruction atomically reads the value of the CSR register `csr`,
/// zero-extends the value to XLEN bits, and writes it to the register `rd`. The initial value in
/// the register `rs1` is treated as a bit mask that specifies bit positions to be set in the CSR.
/// Any bit that is high in `rs1` will cause the corresponding bit to be set in the CSR, if that CSR
/// bit is writable. Other bits in the CSR are not explicitly written. If `rs1` = [X0], then the
/// instruction will not write to the CSR at all, and so shall not cause any of the side effects
/// that might otherwise occur on a CSR write, nor raise illegal instruction exceptions on accesses
/// to read-only CSRs. `csrrs` always reads the addressed CSR and cause any read side effects
/// regardless of `rs1` and `rd` values. Note that if `rs1` specifies a register holding a zero
/// value other than [X0], the instruction will still attempt to write the unmodified value back to
/// the CSR and will cause any attendant side effects.
///
/// Other instructions for accessing CSRs:
/// [csrrsi], [csrrw], [csrrwi], [csrs], [csrsi], [csrrc], [csrrci], [csrc], [csrci], [csrr]
#[must_use]
#[inline]
pub const fn csrrs(rd: Register, rs1: Register, csr: Imm12) -> u32 {
    csr_instruction(rd, RegOrUimm5::Register(rs1), csr, Funct3::CSRRS)
}

/// "atomic CSR Read" pseudoinstruction atomically reads the value of the CSR register `csr`,
/// zero-extends the value to XLEN bits, and writes it to the register `rd`. `csrr` always reads the
/// addressed CSR and cause any read side effects regardless of `rd` value.
///
/// `csrr rd, csr` is encoded as <code>[csrrs] rd, [X0], csr</code>.
///
/// Other instructions for accessing CSRs:
/// [csrrs], [csrrsi], [csrrw], [csrrwi], [csrs], [csrsi], [csrrc], [csrrci], [csrc], [csrci]
#[must_use]
#[inline]
pub const fn csrr(rd: Register, csr: Imm12) -> u32 {
    csrrs(rd, X0, csr)
}

/// "atomic CSR Set bits" pseudoinstruction atomically sets bits in the CSR register `csr` using the
/// register `rs1` as a bit mask that specifies bit positions to be set in the CSR. Any bit that is
/// high in `rs1` will cause the corresponding bit to be set in the CSR, if that CSR bit is
/// writable. Other bits in the CSR are not explicitly written. If `rs1` = [X0], then the
/// instruction will not write to the CSR at all, and so shall not cause any of the side effects
/// that might otherwise occur on a CSR write, nor raise illegal instruction exceptions on accesses
/// to read-only CSRs. `csrs` always reads the addressed CSR and cause any read side effects
/// regardless of `rs1` value. Note that if `rs1` specifies a register holding a zero value other
/// than [X0], the instruction will still attempt to write the unmodified value back to the CSR and
/// will cause any attendant side effects.
///
/// `csrs rs1, csr` is encoded as <code>[csrrs] [X0], rs1, csr</code>.
///
/// Other instructions for accessing CSRs:
/// [csrsi], [csrrs], [csrrsi], [csrrw], [csrrwi], [csrrc], [csrrci], [csrc], [csrci], [csrr]
#[must_use]
#[inline]
pub const fn csrs(rs1: Register, csr: Imm12) -> u32 {
    csrrs(X0, rs1, csr)
}

/// "atomic CSR Read and Clear bits" instruction atomically reads the value of the CSR register
/// `csr`, zero-extends the value to XLEN bits, and writes it to the register `rd`. The value in the
/// register `rs1` is treated as a bit mask that specifies bit positions to be cleared in the CSR.
/// Any bit that is high in `rs1` will cause the corresponding bit to be cleared in the CSR, if that
/// CSR bit is writable. Other bits in the CSR are not explicitly written. If `rs1` = [X0], then the
/// instruction will not write to the CSR at all, and so shall not cause any of the side effects
/// that might otherwise occur on a CSR write, nor raise illegal instruction exceptions on accesses
/// to read-only CSRs. `csrrc` always reads the addressed CSR and cause any read side effects
/// regardless of `rs1` and `rd` values. Note that if `rs1` specifies a register holding a zero
/// value other than [X0], the instruction will still attempt to write the unmodified value back to
/// the CSR and will cause any attendant side effects.
///
/// Other instructions for accessing CSRs:
/// [csrrci], [csrc], [csrci], [csrrw], [csrrwi], [csrrs], [csrrsi], [csrs], [csrsi], [csrr]
#[must_use]
#[inline]
pub const fn csrrc(rd: Register, rs1: Register, csr: Imm12) -> u32 {
    csr_instruction(rd, RegOrUimm5::Register(rs1), csr, Funct3::CSRRC)
}

/// "atomic CSR Clear bits" pseudoinstruction atomically clears bits in the CSR register `csr` using
/// the value in the register `rs1` as a bit mask that specifies bit positions to be cleared in the
/// CSR. Any bit that is high in `rs1` will cause the corresponding bit to be cleared in the CSR, if
/// that CSR bit is writable. Other bits in the CSR are not explicitly written. If `rs1` = [X0],
/// then the instruction will not write to the CSR at all, and so shall not cause any of the side
/// effects that might otherwise occur on a CSR write, nor raise illegal instruction exceptions on
/// accesses to read-only CSRs. `csrc` always reads the addressed CSR and cause any read side
/// effects regardless of `rs1` value. Note that if `rs1` specifies a register holding a zero value
/// other than [X0], the instruction will still attempt to write the unmodified value back to the
/// CSR and will cause any attendant side effects.
///
/// `csrc rs1, csr` is encoded as <code>[csrrc] [X0], rs1, csr</code>.
///
/// Other instructions for accessing CSRs:
/// [csrci], [csrrc], [csrrci], [csrrw], [csrrwi], [csrrs], [csrrsi], [csrs], [csrsi], [csrr]
#[must_use]
#[inline]
pub const fn csrc(rs1: Register, csr: Imm12) -> u32 {
    csrrc(X0, rs1, csr)
}

/// "atomic CSR Read/Write with Immediate" instruction atomically swaps values in CSRs and
/// general-purpose registers. `csrrwi` reads the old value of the CSR register `csr`, zero-extends
/// the value to XLEN bits, then writes it to the register `rd`. An XLEN-bit value obtained by
/// zero-extending a 5-bit unsigned immediate `uimm` is written to the CSR. `csrrwi` always reads
/// the addressed CSR and cause any read side effects regardless of `uimm` and `rd` values.
///
/// Other instructions for accessing CSRs:
/// [csrrw], [csrrs], [csrrsi], [csrs], [csrsi], [csrrc], [csrrci], [csrc], [csrci], [csrr]
#[must_use]
#[inline]
pub const fn csrrwi(rd: Register, uimm: Uimm5, csr: Imm12) -> u32 {
    csr_instruction(rd, RegOrUimm5::Uimm5(uimm), csr, Funct3::CSRRWI)
}

/// "atomic CSR Read and Set bits with Immediate" instruction atomically reads the value of the CSR
/// register `csr`, zero-extends the value to XLEN bits, and writes it to the register `rd`. An
/// XLEN-bit value obtained by zero-extending a 5-bit unsigned immediate `uimm` is treated as a bit
/// mask that specifies bit positions to be set in the CSR. Any bit that is high in `uimm` will
/// cause the corresponding bit to be set in the CSR, if that CSR bit is writable. Other bits in the
/// CSR are not explicitly written. `csrrsi` always reads the addressed CSR and cause any read side
/// effects regardless of `uimm` and `rd` values.
///
/// Other instructions for accessing CSRs:
/// [csrrs], [csrs], [csrsi], [csrrw], [csrrwi], [csrrc], [csrrci], [csrc], [csrci], [csrr]
#[must_use]
#[inline]
pub const fn csrrsi(rd: Register, uimm: Uimm5, csr: Imm12) -> u32 {
    csr_instruction(rd, RegOrUimm5::Uimm5(uimm), csr, Funct3::CSRRSI)
}

/// "atomic CSR Set bits with Immediate" pseudoinstruction atomically sets bits in the CSR register
/// `csr` using an XLEN-bit value obtained by zero-extending a 5-bit unsigned immediate `uimm` as a
/// bit mask that specifies bit positions to be set in the CSR. Any bit that is high in `uimm` will
/// cause the corresponding bit to be set in the CSR, if that CSR bit is writable. Other bits in the
/// CSR are not explicitly written. `csrsi` always reads the addressed CSR and cause any read side
/// effects regardless of `uimm` value.
///
/// `csrsi uimm, csr` is encoded as <code>[csrrsi] [X0], uimm, csr</code>.
///
/// Other instructions for accessing CSRs:
/// [csrrs], [csrrsi], [csrs], [csrrw], [csrrwi], [csrrc], [csrrci], [csrc], [csrci], [csrr]
#[must_use]
#[inline]
pub const fn csrsi(uimm: Uimm5, csr: Imm12) -> u32 {
    csrrsi(X0, uimm, csr)
}

/// "atomic CSR Read and Clear bits with Immediate" instruction atomically reads the value of the
/// CSR register `csr`, zero-extends the value to XLEN bits, and writes it to the register `rd`. An
/// XLEN-bit value obtained by zero-extending a 5-bit unsigned immediate `uimm` is treated as a bit
/// mask that specifies bit positions to be cleared in the CSR. Any bit that is high in `rs1` will
/// cause the corresponding bit to be cleared in the CSR, if that CSR bit is writable. Other bits in
/// the CSR are not explicitly written. `csrrci` always reads the addressed CSR and cause any read
/// side effects regardless of `uimm` and `rd` values.
///
/// Other instructions for accessing CSRs:
/// [csrrc], [csrc], [csrci], [csrrw], [csrrwi], [csrrs], [csrrsi], [csrs], [csrsi], [csrr]
#[must_use]
#[inline]
pub const fn csrrci(rd: Register, uimm: Uimm5, csr: Imm12) -> u32 {
    csr_instruction(rd, RegOrUimm5::Uimm5(uimm), csr, Funct3::CSRRCI)
}

/// "atomic CSR Clear bits with Immediate" pseudoinstruction atomically clears bits in the CSR
/// register `csr`, using an XLEN-bit value obtained by zero-extending the 5-bit unsigned immediate
/// `uimm` as a bit mask that specifies bit positions to be cleared in the CSR. Any bit that is high
/// in `uimm` will cause the corresponding bit to be cleared in the CSR, if that CSR bit is
/// writable. Other bits in the CSR are not explicitly written. `csrci` always reads the addressed
/// CSR and cause any read side effects regardless of `uimm`.
///
/// `csrci uimm, csr` is encoded as <code>[csrrci] [X0], uimm, csr</code>.
///
/// Other instructions for accessing CSRs:
/// [csrrc], [csrrci], [csrc], [csrrw], [csrrwi], [csrrs], [csrrsi], [csrs], [csrsi], [csrr]
#[must_use]
#[inline]
pub const fn csrci(uimm: Uimm5, csr: Imm12) -> u32 {
    csrrci(X0, uimm, csr)
}

/// CSRR-type instruction encoding.<br/>
/// ```text
/// Bit range     |    31:20    | 19:15  | 14:12  | 11:7 |  6:0   |
/// Bit count     |     12      |   5    |   3    |  5   |   7    |
/// Field name    |     csr     |  rs1   | funct3 |  rd  | opcode |
/// Description   | source/dest | source | CSRRW  | dest | SYSTEM |
///               | source/dest | source | CSRRS  | dest | SYSTEM |
///               | source/dest | source | CSRRC  | dest | SYSTEM |
///               | source/dest |  uimm  | CSRRWI | dest | SYSTEM |
///               | source/dest |  uimm  | CSRRSI | dest | SYSTEM |
///               | source/dest |  uimm  | CSRRCI | dest | SYSTEM |
/// ```
const fn csr_instruction(rd: Register, mask: RegOrUimm5, csr: Imm12, funct3: Funct3) -> u32 {
    i_instruction(Opcode::SYSTEM, rd, funct3, mask, csr)
}
