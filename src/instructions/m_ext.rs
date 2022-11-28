//! M standard extension

use super::formats::{
    funct3::Funct3,
    funct7::Funct7,
    opcode::Opcode,
    r_instruction,
    RegOrUimm5,
};
pub use crate::registers::*;

/// `MUL` instruction performs an XLEN-bit×XLEN-bit multiplication of `rs1` by `rs2` and places the lower XLEN bits
/// in the destination register. If both the high and low bits of the same
/// product are required, then the recommended code sequence is:<br/>
/// [`MULH`](mulh) | [`MULHSU`](mulhsu) | [`MULHU`](mulhu) `rdh, rs1, rs2`<br/>
/// `MUL rdl, rs1, rs2`<br/>
/// Source register specifiers must be in same order and `rdh` cannot be the same as `rs1` or
/// `rs2`. Microarchitectures can then fuse these into a single multiply operation instead of performing
/// two separate multiplies.<br/><br/>
/// Other arithmetic instructions:
/// - M extension: [mulh], [mulhsu], [mulhu], [div], [divu], [rem], [remu]
/// - RV32I: [add](super::add), [sub](super::sub), [addi](super::addi)
pub fn mul(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::MUL)
}

/// `MULH` instruction performs an XLEN-bit×XLEN-bit multiplication of `rs1` (signed) by `rs2` (signed) and
/// places the upper XLEN bits in the destination register. If both the high and low bits of the same
/// product are required, then the recommended code sequence is:<br/>
/// `MULH` | [`MULHSU`](mulhsu) | [`MULHU`](mulhu) `rdh, rs1, rs2`<br/>
/// [`MUL`](mul)` rdl, rs1, rs2`<br/>
/// Source register specifiers must be in same order and `rdh` cannot be the same as `rs1` or
/// `rs2`. Microarchitectures can then fuse these into a single multiply operation instead of performing
/// two separate multiplies.<br/><br/>
/// Other arithmetic instructions:
/// - M extension: [mul], [mulhsu], [mulhu], [div], [divu], [rem], [remu]
/// - RV32I: [add](super::add), [sub](super::sub), [addi](super::addi)
pub fn mulh(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::MULH)
}

/// `MULHSU` instruction performs an XLEN-bit×XLEN-bit multiplication of `rs1` (signed) by `rs2` (unsigned) and
/// places the upper XLEN bits in the destination register. If both the high and low bits of the same
/// product are required, then the recommended code sequence is:<br/>
/// [`MULH`](mulh) | `MULHSU` | [`MULHU`](mulhu) `rdh, rs1, rs2`<br/>
/// [`MUL`](mul)` rdl, rs1, rs2`<br/>
/// Source register specifiers must be in same order and `rdh` cannot be the same as `rs1` or
/// `rs2`. Microarchitectures can then fuse these into a single multiply operation instead of performing
/// two separate multiplies.<br/><br/>
/// Other arithmetic instructions:
/// - M extension: [mul], [mulh], [mulhu], [div], [divu], [rem], [remu]
/// - RV32I: [add](super::add), [sub](super::sub), [addi](super::addi)
pub fn mulhsu(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::MULHSU)
}

/// `MULHU` instruction performs an XLEN-bit×XLEN-bit multiplication of `rs1` (unsigned) by `rs2` (unsigned) and
/// places the upper XLEN bits in the destination register. If both the high and low bits of the same
/// product are required, then the recommended code sequence is:<br/>
/// [`MULH`](mulh) | [`MULHSU`](mulhsu) | `MULHU` `rdh, rs1, rs2`<br/>
/// [`MUL`](mul)` rdl, rs1, rs2`<br/>
/// Source register specifiers must be in same order and `rdh` cannot be the same as `rs1` or
/// `rs2`. Microarchitectures can then fuse these into a single multiply operation instead of performing
/// two separate multiplies.<br/><br/>
/// Other arithmetic instructions:
/// - M extension: [mul], [mulh], [mulhsu], [div], [divu], [rem], [remu]
/// - RV32I: [add](super::add), [sub](super::sub), [addi](super::addi)
pub fn mulhu(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::MULHU)
}

/// `DIV` instruction performs XLEN-bit signed integer division of registers `rs1`÷`rs2`, rounding towards zero,
/// and places the result in the register `rd`.
/// [REM](rem) and [REMU](remu) provide the remainder of the corresponding division operation.
/// For both signed and unsigned division, it holds that dividend = divisor × quotient + remainder.
/// If both the quotient and remainder are required from the same division, the recommended code
/// sequence is:<br/>
/// `DIV` | [DIVU](divu) `rdq, rs1, rs2`<br/>
/// [REM](rem) | [REMU](remu) `rdr, rs1, rs2`<br/>
/// `rdq` cannot be the same as `rs1` or `rs2`.
/// Microarchitectures can then fuse these into a single divide operation instead of performing two
/// separate divides.<br/><br/>
/// Other arithmetic instructions:
/// - M extension: [divu], [rem], [remu], [mul], [mulh], [mulhsu], [mulhu]
/// - RV32I: [add](super::add), [sub](super::sub), [addi](super::addi)
pub fn div(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::DIV)
}

/// `DIVU` instruction performs XLEN-bit unsigned integer division of registers `rs1`÷`rs2`, rounding towards zero,
/// and places the result in the register `rd`.
/// [REM](rem) and [REMU](remu) provide the remainder of the corresponding division operation.
/// For both signed and unsigned division, it holds that dividend = divisor × quotient + remainder.
/// If both the quotient and remainder are required from the same division, the recommended code
/// sequence is:<br/>
/// [DIV](div) | `DIVU` `rdq, rs1, rs2`<br/>
/// [REM](rem) | [REMU](remu) `rdr, rs1, rs2`<br/>
/// `rdq` cannot be the same as `rs1` or `rs2`.
/// Microarchitectures can then fuse these into a single divide operation instead of performing two
/// separate divides.<br/><br/>
/// Other arithmetic instructions:
/// - M extension: [div], [rem], [remu], [mul], [mulh], [mulhsu], [mulhu]
/// - RV32I: [add](super::add), [sub](super::sub), [addi](super::addi)
pub fn divu(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::DIVU)
}

/// `REM` instruction performs XLEN-bit signed integer division of registers `rs1`÷`rs2`,
/// and places the remainder of that division in register `rd`.
/// [DIV](div) and [DIVU](divu) provide the quotient of the corresponding division operation.
/// For both signed and unsigned division, it holds that dividend = divisor × quotient + remainder.
/// If both the quotient and remainder are required from the same division, the recommended code
/// sequence is:<br/>
/// [DIV](div) | [DIVU](divu) `rdq, rs1, rs2`<br/>
/// `REM` | [REMU](remu) `rdr, rs1, rs2`<br/>
/// `rdq` cannot be the same as `rs1` or `rs2`.
/// Microarchitectures can then fuse these into a single divide operation instead of performing two
/// separate divides.<br/><br/>
/// Other arithmetic instructions:
/// - M extension: [remu], [div], [divu], [mul], [mulh], [mulhsu], [mulhu]
/// - RV32I: [add](super::add), [sub](super::sub), [addi](super::addi)
pub fn rem(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::REM)
}

/// `REMU` instruction performs XLEN-bit unsigned integer division of registers `rs1`÷`rs2`,
/// and places the remainder of that division in the register `rd`.
/// [DIV](div) and [DIVU](divu) provide the quotient of the corresponding division operation.
/// For both signed and unsigned division, it holds that dividend = divisor × quotient + remainder.
/// If both the quotient and remainder are required from the same division, the recommended code
/// sequence is:<br/>
/// [DIV](div) | [DIVU](divu) `rdq, rs1, rs2`<br/>
/// [REM](rem) | `REMU` `rdr, rs1, rs2`<br/>
/// `rdq` cannot be the same as `rs1` or `rs2`.
/// Microarchitectures can then fuse these into a single divide operation instead of performing two
/// separate divides.<br/><br/>
/// Other arithmetic instructions:
/// - M extension: [rem], [div], [divu], [mul], [mulh], [mulhsu], [mulhu]
/// - RV32I: [add](super::add), [sub](super::sub), [addi](super::addi)
pub fn remu(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::REMU)
}

/// MUL-type instruction encoding.<br/>
/// ```text
/// Bit range     | 31:25  |   24:20    |    19:15     | 14:12  | 11:7 |  6:0   |
/// Field name    | funct7 |    rs2     |     rs1      | funct3 |  rd  | opcode |
/// Bit count     |   7    |     5      |      5       |   3    |  5   |   7    |
/// Description   | MULDIV | multiplier | multiplicand | MUL    | dest |   OP   |
///               | MULDIV | multiplier | multiplicand | MULH   | dest |   OP   |
///               | MULDIV | multiplier | multiplicand | MULHSU | dest |   OP   |
///               | MULDIV | multiplier | multiplicand | MULHU  | dest |   OP   |
///               | MULDIV |  divisor   |   dividend   | DIV    | dest |   OP   |
///               | MULDIV |  divisor   |   dividend   | DIVU   | dest |   OP   |
///               | MULDIV |  divisor   |   dividend   | REM    | dest |   OP   |
///               | MULDIV |  divisor   |   dividend   | REMU   | dest |   OP   |
/// ```
fn muldiv_instruction(rd: Register, rs1: Register, rs2: Register, funct3: Funct3) -> u32 {
    r_instruction(
        Opcode::OP,
        rd,
        funct3,
        rs1,
        RegOrUimm5::Register(rs2),
        Funct7::MULDIV,
    )
}
