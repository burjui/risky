//! Zicsr standard extension

use super::formats::{funct3::Funct3, i_instruction, opcode::Opcode, RegOrUimm5};
pub use super::{imm12::*, uimm5::*};
pub use crate::registers::*;

/// `CSRRW` (atomic read/write CSR) instruction atomically swaps values in the CSRs and general-purpose registers.
/// `CSRRW` reads the old value of the CSR register `csr`, zero-extends the value to XLEN bits, then writes it to the
/// register `rd`. The initial value in `rs1` is written to the CSR. If `rs1` = [X0], then the instruction will not
/// write to the CSR at all, and so shall not cause any of the side effects that might otherwise occur on a CSR write,
/// nor raise illegal instruction exceptions on accesses to read-only CSRs. `CSRRW` always reads the addressed CSR and
/// cause any read side effects regardless of `rs1` and `rd` values. Note that if `rs1` specifies a register holding a
/// zero value other than [X0], the instruction will still attempt to write the unmodified value back to the CSR and
/// will cause any attendant side effects.
/// A `CSRRW` with `rs1` = [X0] will attempt to write zero to the destination CSR.<br/><br/>
/// Similar instructions and pseudoinstructions for accessing CSRs:
/// [CSRRS](csrrs), [CSRR](csrr), [CSRS](csrs), [CSRRC](csrrc), [CSRC](csrc),
/// [CSRRWI](csrrwi), [CSRRSI](csrrsi), [CSRSI](csrsi), [CSRRCI](csrrci), [CSRCI](csrci)
pub fn csrrw(rd: Register, rs1: Register, csr: Imm12) -> u32 {
    csr_instruction(rd, RegOrUimm5::Register(rs1), csr, Funct3::CSRRW)
}

/// `CSRRS` (atomic read and set bits in CSR) instruction atomically reads the value of the CSR register `csr`,
/// zero-extends the value to XLEN bits, and writes it to the register `rd`. The initial value in the register `rs1` is
/// treated as a bit mask that specifies bit positions to be set in the CSR. Any bit that is high in `rs1` will cause
/// the corresponding bit to be set in the CSR, if that CSR bit is writable. Other bits in the CSR are not explicitly
/// written. If `rs1` = [X0], then the instruction will not write to the CSR at all, and so shall not cause any of the
/// side effects that might otherwise occur on a CSR write, nor raise illegal instruction exceptions on accesses to
/// read-only CSRs. `CSRRS` always reads the addressed CSR and cause any read side effects regardless of `rs1` and `rd`
/// values. Note that if `rs1` specifies a register holding a zero value other than [X0], the instruction will still
/// attempt to write the unmodified value back to the CSR and will cause any attendant side effects.<br/><br/>
/// Similar instructions and pseudoinstructions for accessing CSRs:
/// [CSRRW](csrrw), [CSRR](csrr), [CSRS](csrs), [CSRRC](csrrc), [CSRC](csrc),
/// [CSRRWI](csrrwi), [CSRRSI](csrrsi), [CSRSI](csrsi), [CSRRCI](csrrci), [CSRCI](csrci)
pub fn csrrs(rd: Register, rs1: Register, csr: Imm12) -> u32 {
    csr_instruction(rd, RegOrUimm5::Register(rs1), csr, Funct3::CSRRS)
}

/// `CSRR` (atomic read and set bits in CSR) pseudoinstruction atomically reads the value of the CSR register `csr`,
/// zero-extends the value to XLEN bits, and writes it to the register `rd`. `CSRR` always reads the addressed CSR and
/// cause any read side effects regardless of `rd` value.<br/><br/>
/// `CSRR rd, csr` is encoded as [CSRRS](csrrs) `rd, x0, csr`.<br/><br/>
/// Similar instructions and pseudoinstructions for accessing CSRs:
/// [CSRRW](csrrw), [CSRRS](csrrs), [CSRS](csrs), [CSRRC](csrrc), [CSRC](csrc),
/// [CSRRWI](csrrwi), [CSRRSI](csrrsi), [CSRSI](csrsi), [CSRRCI](csrrci), [CSRCI](csrci)
pub fn csrr(rd: Register, csr: Imm12) -> u32 {
    csrrs(rd, X0, csr)
}

/// `CSRS` (atomic set bits in CSR) pseudoinstruction atomically sets bits in the CSR register `csr` using the register
/// `rs1` as a bit mask that specifies bit positions to be set in the CSR. Any bit that is high in `rs1` will cause the
/// corresponding bit to be set in the CSR, if that CSR bit is writable. Other bits in the CSR are not explicitly
/// written. If `rs1` = [X0], then the instruction will not write to the CSR at all, and so shall not cause any of the
/// side effects that might otherwise occur on a CSR write, nor raise illegal instruction exceptions on accesses to
/// read-only CSRs. `CSRS` always reads the addressed CSR and cause any read side effects regardless of `rs1` value.
/// Note that if `rs1` specifies a register holding a zero value other than [X0], the instruction will still attempt to
/// write the unmodified value back to the CSR and will cause any attendant side effects.<br/><br/>
/// `CSRS rs1, csr` is encoded as [CSRS](csrs) `x0, rs1, csr`.<br/><br/>
/// Similar instructions and pseudoinstructions for accessing CSRs:
/// [CSRRW](csrrw), [CSRRS](csrrs), [CSRR](csrr), [CSRRC](csrrc), [CSRC](csrc),
/// [CSRRWI](csrrwi), [CSRRSI](csrrsi), [CSRSI](csrsi), [CSRRCI](csrrci), [CSRCI](csrci)
pub fn csrs(rs1: Register, csr: Imm12) -> u32 {
    csrrs(X0, rs1, csr)
}

/// `CSRRC` (atomic read and clear bits in CSR) instruction atomically reads the value of the CSR register `csr`,
/// zero-extends the value to XLEN bits, and writes it to the register `rd`. The value in the register `rs1` is treated
/// as a bit mask that specifies bit positions to be cleared in the CSR. Any bit that is high in `rs1` will cause the
/// corresponding bit to be cleared in the CSR, if that CSR bit is writable. Other bits in the CSR are not explicitly
/// written. If `rs1` = [X0], then the instruction will not write to the CSR at all, and so shall not cause any of the
/// side effects that might otherwise occur on a CSR write, nor raise illegal instruction exceptions on accesses to
/// read-only CSRs. `CSRRC` always reads the addressed CSR and cause any read side effects regardless of `rs1` and `rd`
/// values. Note that if `rs1` specifies a register holding a zero value other than [X0], the instruction will still
/// attempt to write the unmodified value back to the CSR and will cause any attendant side effects.<br/><br/>
/// Similar instructions and pseudoinstructions for accessing CSRs:
/// [CSRRW](csrrw), [CSRRS](csrrs), [CSRR](csrr), [CSRS](csrs), [CSRC](csrc),
/// [CSRRWI](csrrwi), [CSRRSI](csrrsi), [CSRSI](csrsi), [CSRRCI](csrrci), [CSRCI](csrci)
pub fn csrrc(rd: Register, rs1: Register, csr: Imm12) -> u32 {
    csr_instruction(rd, RegOrUimm5::Register(rs1), csr, Funct3::CSRRC)
}

/// `CSRC` (atomic clear bits in CSR) pseudoinstruction atomically clears bits in the CSR register `csr` using the value
/// in the register `rs1` as a bit mask that specifies bit positions to be cleared in the CSR. Any bit that is high in
/// `rs1` will cause the corresponding bit to be cleared in the CSR, if that CSR bit is writable. Other bits in the CSR
/// are not explicitly written. If `rs1` = [X0], then the instruction will not write to the CSR at all, and so shall not
/// cause any of the side effects that might otherwise occur on a CSR write, nor raise illegal instruction exceptions on
/// accesses to read-only CSRs. `CSRC` always reads the addressed CSR and cause any read side effects regardless of
/// `rs1` value. Note that if `rs1` specifies a register holding a zero value other than [X0], the instruction will
/// still attempt to write the unmodified value back to the CSR and will cause any attendant side effects.<br/><br/>
/// `CSRC rs1, csr` is encoded as [CSRRC](csrrc)&nbsp;`x0, rs1, csr`.<br/><br/>
/// Similar instructions and pseudoinstructions for accessing CSRs:
/// [CSRRW](csrrw), [CSRRS](csrrs), [CSRR](csrr), [CSRS](csrs), [CSRRC](csrrc),
/// [CSRRWI](csrrwi), [CSRRSI](csrrsi), [CSRSI](csrsi), [CSRRCI](csrrci), [CSRCI](csrci)
pub fn csrc(rs1: Register, csr: Imm12) -> u32 {
    csrrc(X0, rs1, csr)
}

/// `CSRRWI` (atomic read/write CSR with immediate) instruction atomically swaps values in CSRs and general-purpose
/// registers. `CSRRWI` reads the old value of the CSR register `csr`, zero-extends the value to XLEN bits, then writes
/// it to the register `rd`. An XLEN-bit value obtained by zero-extending a 5-bit unsigned immediate `uimm` is written
/// to the CSR. `CSRRWI` always reads the addressed CSR and cause any read side effects regardless of
/// `uimm` and `rd` values.<br/><br/>
/// Similar instructions and pseudoinstructions for accessing CSRs:
/// [CSRRW](csrrw), [CSRRS](csrrs), [CSRR](csrr), [CSRS](csrs), [CSRRC](csrrc),
/// [CSRC](csrc), [CSRRSI](csrrsi), [CSRSI](csrsi), [CSRRCI](csrrci), [CSRCI](csrci)
pub fn csrrwi(rd: Register, uimm: Uimm5, csr: Imm12) -> u32 {
    csr_instruction(rd, RegOrUimm5::Uimm5(uimm), csr, Funct3::CSRRWI)
}

/// `CSRRSI` (atomic read and set bits in CSR with immediate) instruction atomically reads the value of the CSR register
/// `csr`, zero-extends the value to XLEN bits, and writes it to the register `rd`. An XLEN-bit value obtained by
/// zero-extending a 5-bit unsigned immediate `uimm` is treated as a bit mask that specifies bit positions to be set in
/// the CSR. Any bit that is high in `uimm` will cause the corresponding bit to be set in the CSR, if that CSR bit is
/// writable. Other bits in the CSR are not explicitly written. `CSRRSI` always reads the addressed CSR and cause any
/// read side effects regardless of `uimm` and `rd` values.<br/><br/>
/// Similar instructions and pseudoinstructions for accessing CSRs:
/// [CSRRW](csrrw), [CSRRS](csrrs), [CSRR](csrr), [CSRS](csrs), [CSRRC](csrrc),
/// [CSRC](csrc), [CSRRWI](csrrwi), [CSRSI](csrsi), [CSRRCI](csrrci), [CSRCI](csrci)
pub fn csrrsi(rd: Register, uimm: Uimm5, csr: Imm12) -> u32 {
    csr_instruction(rd, RegOrUimm5::Uimm5(uimm), csr, Funct3::CSRRSI)
}

/// `CSRSI` (atomic set bits in CSR with immediate) pseudoinstruction atomically sets bits in the CSR register `csr`
/// using an XLEN-bit value obtained by zero-extending a 5-bit unsigned immediate `uimm` as a bit mask that specifies
/// bit positions to be set in the CSR. Any bit that is high in `uimm` will cause the corresponding bit to be set in the
/// CSR, if that CSR bit is writable. Other bits in the CSR are not explicitly written. `CSRSI` always reads the
/// addressed CSR and cause any read side effects regardless of `uimm` value.<br/><br/>
/// `CSRSI uimm, csr` is encoded as [CSRRSI](csrrsi) `x0, uimm, csr`.<br/><br/>
/// Similar instructions and pseudoinstructions for accessing CSRs:
/// [CSRRW](csrrw), [CSRRS](csrrs), [CSRR](csrr), [CSRS](csrs), [CSRRC](csrrc),
/// [CSRC](csrc), [CSRRWI](csrrwi), [CSRRSI](csrrsi),  [CSRRCI](csrrci), [CSRCI](csrci)
pub fn csrsi(uimm: Uimm5, csr: Imm12) -> u32 {
    csrrsi(X0, uimm, csr)
}

/// `CSRRCI` (atomic read and clear bits in CSR with immediate) instruction atomically reads the value of the CSR
/// register `csr`, zero-extends the value to XLEN bits, and writes it to the register `rd`. An XLEN-bit value obtained
/// by zero-extending a 5-bit unsigned immediate `uimm` is treated as a bit mask that specifies bit positions to be
/// cleared in the CSR. Any bit that is high in `rs1` will cause the corresponding bit to be cleared in the CSR, if that
/// CSR bit is writable. Other bits in the CSR are not explicitly written. `CSRRCI` always reads the addressed CSR and
/// cause any read side effects regardless of `uimm` and `rd` values.<br/><br/>
/// Similar instructions and pseudoinstructions for accessing CSRs:
/// [CSRRW](csrrw), [CSRRS](csrrs), [CSRR](csrr), [CSRS](csrs), [CSRRC](csrrc),
/// [CSRC](csrc), [CSRRWI](csrrwi), [CSRRSI](csrrsi), [CSRSI](csrsi), [CSRCI](csrci)
pub fn csrrci(rd: Register, uimm: Uimm5, csr: Imm12) -> u32 {
    csr_instruction(rd, RegOrUimm5::Uimm5(uimm), csr, Funct3::CSRRCI)
}

/// `CSRCI` (atomic clear bits in CSR with immediate) pseudoinstruction atomically clears bits in the CSR register
/// `csr`, using an XLEN-bit value obtained by zero-extending the 5-bit unsigned immediate `uimm` as a bit mask that
/// specifies bit positions to be cleared in the CSR. Any bit that is high in `uimm` will cause the corresponding bit to
/// be cleared in the CSR, if that CSR bit is writable. Other bits in the CSR are not explicitly written. `CSRCI` always
/// reads the addressed CSR and cause any read side effects regardless of `uimm`.<br/><br/>
/// `CSRCI uimm, csr` is encoded as [CSRRCI](csrrci) `x0, uimm, csr`.<br/><br/>
/// Similar instructions and pseudoinstructions for accessing CSRs:
/// [CSRRW](csrrw), [CSRRS](csrrs), [CSRR](csrr), [CSRS](csrs), [CSRRC](csrrc),
/// [CSRC](csrc), [CSRRWI](csrrwi), [CSRRSI](csrrsi), [CSRSI](csrsi), [CSRRCI](csrrci)
pub fn csrci(uimm: Uimm5, csr: Imm12) -> u32 {
    csrrci(X0, uimm, csr)
}

/// CSRR-type instruction encoding.<br/>
/// ```text
/// Bit range     |    31:20    | 19:15  | 14:12  | 11:7 |  6:0   |
/// Field name    |     csr     |  rs1   | funct3 |  rd  | opcode |
/// Bit count     |     12      |   5    |   3    |  5   |   7    |
/// Description   | source/dest | source | CSRRW  | dest | SYSTEM |
///               | source/dest | source | CSRRS  | dest | SYSTEM |
///               | source/dest | source | CSRRC  | dest | SYSTEM |
///               | source/dest |  uimm  | CSRRWI | dest | SYSTEM |
///               | source/dest |  uimm  | CSRRSI | dest | SYSTEM |
///               | source/dest |  uimm  | CSRRCI | dest | SYSTEM |
/// ```
fn csr_instruction(rd: Register, mask: RegOrUimm5, csr: Imm12, funct3: Funct3) -> u32 {
    i_instruction(Opcode::SYSTEM, rd, funct3, mask, csr)
}
