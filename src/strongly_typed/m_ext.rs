//! M standard extension

use super::{
    encoding::{funct3::Funct3, funct7::Funct7, opcode::Opcode, r_instruction, RegOrUimm5},
    registers::Register,
};

/// `mul` instruction performs an XLEN-bit×XLEN-bit multiplication of `rs1` by `rs2` and places the
/// lower XLEN bits in the destination register. If both the high and low bits of the same product
/// are required, then the recommended code sequence is:<br/>
/// <code>
/// [mulh] | [mulhsu] | [mulhu] rdh, rs1, rs2
/// mul rdl, rs1, rs2
/// </code><br/>
/// Source register specifiers must be in same order and `rdh` cannot be the same as `rs1` or `rs2`.
/// Microarchitectures can then fuse these into a single multiply operation instead of performing
/// two separate multiplies.
///
/// Other arithmetic instructions:
/// - M extension: [mulh], [mulhsu], [mulhu], [div], [divu], [rem], [remu]
/// - RV32I: [add](super::rv32i::add), [addi](super::rv32i::addi), [sub](super::rv32i::sub)
#[must_use]
#[inline]
pub const fn mul(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::MUL)
}

/// `mulh` instruction performs an XLEN-bit×XLEN-bit multiplication of `rs1` (signed) by `rs2`
/// (signed) and places the upper XLEN bits in the destination register. If both the high and low
/// bits of the same product are required, then the recommended code sequence is:<br/>
/// <code>
/// mulh | [mulhsu] | [mulhu] rdh, rs1, rs2
/// [mul] rdl, rs1, rs2
/// </code><br/>
/// Source register specifiers must be in same order and `rdh` cannot be the same as `rs1` or `rs2`.
/// Microarchitectures can then fuse these into a single multiply operation instead of performing
/// two separate multiplies.
///
/// Other arithmetic instructions:
/// - M extension: [mul], [mulhsu], [mulhu], [div], [divu], [rem], [remu]
/// - RV32I: [add](super::rv32i::add), [addi](super::rv32i::addi), [sub](super::rv32i::sub)
#[must_use]
#[inline]
pub const fn mulh(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::MULH)
}

/// `mulhsu` instruction performs an XLEN-bit×XLEN-bit multiplication of `rs1` (signed) by `rs2`
/// (unsigned) and places the upper XLEN bits in the destination register. If both the high and low
/// bits of the same product are required, then the recommended code sequence is:<br/>
/// <code>
/// [mulh] | mulhsu | [mulhu] rdh, rs1, rs2
/// [mul] rdl, rs1, rs2
/// </code><br/>
/// Source register specifiers must be in same order and `rdh` cannot be the same as `rs1` or `rs2`.
/// Microarchitectures can then fuse these into a single multiply operation instead of performing
/// two separate multiplies.
///
/// Other arithmetic instructions:
/// - M extension: [mul], [mulh], [mulhu], [div], [divu], [rem], [remu]
/// - RV32I: [add](super::rv32i::add), [addi](super::rv32i::addi), [sub](super::rv32i::sub)
#[must_use]
#[inline]
pub const fn mulhsu(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::MULHSU)
}

/// `mulhu` instruction performs an XLEN-bit×XLEN-bit multiplication of `rs1` (unsigned) by `rs2`
/// (unsigned) and places the upper XLEN bits in the destination register. If both the high and low
/// bits of the same product are required, then the recommended code sequence is:<br/>
/// <code>
/// [mulh] | [mulhsu] | mulhu rdh, rs1, rs2
/// [mul] rdl, rs1, rs2
/// </code><br/>
/// Source register specifiers must be in same order and `rdh` cannot be the same as `rs1` or `rs2`.
/// Microarchitectures can then fuse these into a single multiply operation instead of performing
/// two separate multiplies.
///
/// Other arithmetic instructions:
/// - M extension: [mul], [mulh], [mulhsu], [div], [divu], [rem], [remu]
/// - RV32I: [add](super::rv32i::add), [addi](super::rv32i::addi), [sub](super::rv32i::sub)
#[must_use]
#[inline]
pub const fn mulhu(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::MULHU)
}

/// `div` instruction performs XLEN-bit signed integer division of registers `rs1`÷`rs2`, rounding
/// towards zero, and places the result in the register `rd`. [rem] and [remu] provide the remainder
/// of the corresponding division operation. For both signed and unsigned division, it holds that
/// dividend = divisor × quotient + remainder. If both the quotient and remainder are required from
/// the same division, the recommended code sequence is:<br/>
/// <code>
/// div | [divu] rdq, rs1, rs2
/// [rem] | [remu] rdr, rs1, rs2
/// </code><br/>
/// `rdq` cannot be the same as `rs1` or `rs2`.
/// Microarchitectures can then fuse these into a single divide operation instead of performing two
/// separate divides.
///
/// Other arithmetic instructions:
/// - M extension: [divu], [rem], [remu], [mul], [mulh], [mulhsu], [mulhu]
/// - RV32I: [add](super::rv32i::add), [addi](super::rv32i::addi), [sub](super::rv32i::sub)
#[must_use]
#[inline]
pub const fn div(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::DIV)
}

/// `divu` instruction performs XLEN-bit unsigned integer division of registers `rs1`÷`rs2`,
/// rounding towards zero, and places the result in the register `rd`. [rem] and [remu] provide the
/// remainder of the corresponding division operation. For both signed and unsigned division, it
/// holds that dividend = divisor × quotient + remainder. If both the quotient and remainder are
/// required from the same division, the recommended code sequence is:<br/>
/// <code>
/// [div] | divu rdq, rs1, rs2
/// [rem] | [remu] rdr, rs1, rs2
/// </code><br/>
/// `rdq` cannot be the same as `rs1` or `rs2`. Microarchitectures can then fuse these into a single
/// divide operation instead of performing two separate divides.
///
/// Other arithmetic instructions:
/// - M extension: [div], [rem], [remu], [mul], [mulh], [mulhsu], [mulhu]
/// - RV32I: [add](super::rv32i::add), [addi](super::rv32i::addi), [sub](super::rv32i::sub)
#[must_use]
#[inline]
pub const fn divu(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::DIVU)
}

/// `rem` instruction performs XLEN-bit signed integer division of registers `rs1`÷`rs2`, and places
/// the remainder of that division in register `rd`. [div] and [divu] provide the quotient of the
/// corresponding division operation. For both signed and unsigned division, it holds that dividend
/// = divisor × quotient + remainder. If both the quotient and remainder are required from the same
/// division, the recommended code sequence is:<br/>
/// <code>
/// [div] | [divu] rdq, rs1, rs2
/// rem | [remu] rdr, rs1, rs2
/// </code><br/>
/// `rdq` cannot be the same as `rs1` or `rs2`. Microarchitectures can then fuse these into a single
/// divide operation instead of performing two separate divides.
///
/// Other arithmetic instructions:
/// - M extension: [remu], [div], [divu], [mul], [mulh], [mulhsu], [mulhu]
/// - RV32I: [add](super::rv32i::add), [addi](super::rv32i::addi), [sub](super::rv32i::sub)
#[must_use]
#[inline]
pub const fn rem(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::REM)
}

/// `remu` instruction performs XLEN-bit unsigned integer division of registers `rs1`÷`rs2`, and
/// places the remainder of that division in the register `rd`. [div] and [divu] provide the
/// quotient of the corresponding division operation. For both signed and unsigned division, it
/// holds that dividend = divisor × quotient + remainder. If both the quotient and remainder are
/// required from the same division, the recommended code sequence is:<br/>
/// <code>
/// [div] | [divu] rdq, rs1, rs2
/// [rem] | remu rdr, rs1, rs2
/// </code><br/>
/// `rdq` cannot be the same as `rs1` or `rs2`. Microarchitectures can then fuse these into a single
/// divide operation instead of performing two separate divides.
///
/// Other arithmetic instructions:
/// - M extension: [rem], [div], [divu], [mul], [mulh], [mulhsu], [mulhu]
/// - RV32I: [add](super::rv32i::add), [addi](super::rv32i::addi), [sub](super::rv32i::sub)
#[must_use]
#[inline]
pub const fn remu(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::REMU)
}

/// MUL-type instruction encoding.<br/>
/// ```text
/// Bit range     | 31:25  |   24:20    |    19:15     | 14:12  | 11:7 |  6:0   |
/// Bit count     |   7    |     5      |      5       |   3    |  5   |   7    |
/// Field name    | funct7 |    rs2     |     rs1      | funct3 |  rd  | opcode |
/// Description   | MULDIV | multiplier | multiplicand | MUL    | dest |   OP   |
///               | MULDIV | multiplier | multiplicand | MULH   | dest |   OP   |
///               | MULDIV | multiplier | multiplicand | MULHSU | dest |   OP   |
///               | MULDIV | multiplier | multiplicand | MULHU  | dest |   OP   |
///               | MULDIV |  divisor   |   dividend   | DIV    | dest |   OP   |
///               | MULDIV |  divisor   |   dividend   | DIVU   | dest |   OP   |
///               | MULDIV |  divisor   |   dividend   | REM    | dest |   OP   |
///               | MULDIV |  divisor   |   dividend   | REMU   | dest |   OP   |
/// ```
const fn muldiv_instruction(rd: Register, rs1: Register, rs2: Register, funct3: Funct3) -> u32 {
    r_instruction(
        Opcode::OP,
        rd,
        funct3,
        rs1,
        RegOrUimm5::Register(rs2),
        Funct7::MULDIV,
    )
}
