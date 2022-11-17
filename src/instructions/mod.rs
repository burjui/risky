/*!
- RV32I base instruction set
- Zicsr standard extension
- M standard extension

Based on the following document:
> ["The RISC-V Instruction Set Manual, Volume I: User-Level ISA,
Document Version 20191214-draft"](https://github.com/riscv/riscv-isa-manual),
Editors Andrew Waterman and Krste Asanović, RISC-V International, December 2019.
*/

mod csr_mask;
mod fence_mask;
mod funct3;
mod funct7;
mod opcode;

use self::csr_mask::CsrMask;
use self::fence_mask::{FenceMask, FENCE_MASK_RW};
use self::funct3::Funct3;
use self::funct7::Funct7;
use self::opcode::Opcode;
use crate::registers::*;
use bitvec::slice::BitSlice;
use bitvec::view::BitView;
use bitvec::{field::BitField, order::Lsb0};

// RV32I Base Instruction Set

/// *(RV32I, U-format)*<br/>
/// `LUI` (load upper immediate) instruction is used to build 32-bit constants. `LUI` places `imm` in the top 20 bits
/// of the destination register `rd`, filling in the lowest 12 bits with zeros.
pub fn lui(rd: Register, imm: i32) -> u32 {
    u_instruction(Opcode::LUI, rd, imm)
}

/// *(RV32I, U-format)*<br/>
/// `AUIPC` (add upper immediate to pc) instruction is used to build pc-relative addresses.
/// `AUIPC` forms a 32-bit offset from `imm`, filling in the lowest 12 bits with zeros,
/// adds this offset to the address of the `AUIPC` instruction, then places the result in the register `rd`.
pub fn auipc(rd: Register, imm: i32) -> u32 {
    u_instruction(Opcode::AUIPC, rd, imm)
}

/// *(RV32I, J-format)*<br/>
/// `JAL` (jump and link) instruction uses the J-type format, where `imm` encodes a
/// signed offset in multiples of 2 bytes. The offset is sign-extended and added to the address of the
/// jump instruction to form the jump target address. Jumps can therefore target a ±1&nbsp;MiB range.
/// `JAL` stores the address of the instruction that follows the `JAL` (pc+4) into the register `rd`. The standard
/// software calling convention uses [X1] as the return address register and x5 as an alternate link register.
pub fn jal(rd: Register, imm: i32) -> u32 {
    j_instruction(Opcode::JAL, rd, imm)
}

/// *(RV32I, I-format)*<br/>
/// `JALR` (jump and link register) indirect jump instruction uses the I-type encoding. The target
/// address is obtained by adding the sign-extended 12-bit `imm` to the register `rs1`, then setting
/// the least-significant bit of the result to zero. The address of the instruction following the jump
/// (pc+4) is written to the register `rd`. Register [X0] can be used as the destination if the result is not
/// required.
pub fn jalr(rd: Register, rs1: Register, imm: i16) -> u32 {
    i_instruction(Opcode::JALR, rd, Funct3::JALR, ITypeRs1::Register(rs1), imm)
}

/// *(RV32I, B-format)*<br/>
/// `BEQ` branch instruction takes the branch if registers `rs1` = `rs2`. The 12-bit `imm` encodes signed
/// offsets in multiples of 2 bytes. The offset is sign-extended and added to the address of the branch
/// instruction to give the target address. The conditional branch range is ±4&nbsp;KiB.<br/><br/>
/// Similar instructions with other branch conditions:
/// [BNE](bne), [BLT](blt), [BLTU](bltu), [BGE](bge), [BGEU](bgeu).
pub fn beq(imm: i16, rs1: Register, rs2: Register) -> u32 {
    b_instruction(Opcode::BRANCH, imm, Funct3::BEQ, rs1, rs2)
}

/// *(RV32I, B-format)*<br/>
/// `BNE` branch instruction takes the branch if registers `rs1` ≠ `rs2`. The 12-bit `imm` encodes signed
/// offsets in multiples of 2 bytes. The offset is sign-extended and added to the address of the branch
/// instruction to give the target address. The conditional branch range is ±4&nbsp;KiB.<br/><br/>
/// Similar instructions with other branch conditions:
/// [BEQ](beq), [BLT](blt), [BLTU](bltu), [BGE](bge), [BGEU](bgeu).
pub fn bne(imm: i16, rs1: Register, rs2: Register) -> u32 {
    b_instruction(Opcode::BRANCH, imm, Funct3::BNE, rs1, rs2)
}

/// *(RV32I, B-format)*<br/>
/// `BLT` branch instruction takes the branch if registers `rs1` < `rs2`, using signed comparison.
/// The 12-bit `imm` encodes signed offsets in multiples of 2 bytes. The offset is sign-extended and
/// added to the address of the branch instruction to give the target address.
/// The conditional branch range is ±4&nbsp;KiB.<br/><br/>
/// Similar instructions with other branch conditions:
/// [BLTU](bltu), [BGE](bge), [BGEU](bgeu), [BEQ](beq), [BNE](bne).
pub fn blt(imm: i16, rs1: Register, rs2: Register) -> u32 {
    b_instruction(Opcode::BRANCH, imm, Funct3::BLT, rs1, rs2)
}

/// *(RV32I, B-format)*<br/>
/// `BLTU` branch instruction takes the branch if registers `rs1` < `rs2`, using unsigned comparison.
/// The 12-bit `imm` encodes signed offsets in multiples of 2 bytes. The offset is sign-extended and
/// added to the address of the branch instruction to give the target address.
/// The conditional branch range is ±4&nbsp;KiB..<br/><br/>
/// Note that signed array bounds may be checked with a single BLTU instruction,
/// since any negative index will compare greater than any nonnegative bound.<br/><br/>
/// Similar instructions with other branch conditions:
/// [BLT](blt), [BGE](bge), [BGEU](bgeu), [BEQ](beq), [BNE](bne)
pub fn bltu(imm: i16, rs1: Register, rs2: Register) -> u32 {
    b_instruction(Opcode::BRANCH, imm, Funct3::BLTU, rs1, rs2)
}

/// *(RV32I, B-format)*<br/>
/// `BGE` branch instruction takes the branch if registers `rs1` ≥ `rs2`, using signed comparison.
/// The 12-bit `imm` encodes signed offsets in multiples of 2 bytes. The offset is sign-extended and
/// added to the address of the branch instruction to give the target address.
/// The conditional branch range is ±4&nbsp;KiB.<br/><br/>
/// Similar instructions with other branch conditions:
/// [BGEU](bgeu), [BLT](blt), [BLTU](bltu), [BEQ](beq), [BNE](bne).
pub fn bge(imm: i16, rs1: Register, rs2: Register) -> u32 {
    b_instruction(Opcode::BRANCH, imm, Funct3::BGE, rs1, rs2)
}

/// *(RV32I, B-format)*<br/>
/// `BGEU` branch instruction takes the branch if registers `rs1` ≥ `rs2`, using unsigned comparison.
/// The 12-bit `imm` encodes signed offsets in multiples of 2 bytes. The offset is sign-extended and
/// added to the address of the branch instruction to give the target address.
/// The conditional branch range is ±4&nbsp;KiB.<br/><br/>
/// Similar instructions with other branch conditions:
/// [BGE](bge), [BLT](blt), [BLTU](bltu), [BEQ](beq), [BNE](bne).
pub fn bgeu(imm: i16, rs1: Register, rs2: Register) -> u32 {
    b_instruction(Opcode::BRANCH, imm, Funct3::BGEU, rs1, rs2)
}

/// *(RV32I, I-format)*<br/>
/// `LB` instruction copies a 8-bit value from memory to the register `rd`, sign-extending it to
/// 32&nbsp;bits. The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset `imm`.
pub fn lb(rd: Register, rs1: Register, imm: i16) -> u32 {
    i_instruction(Opcode::LOAD, rd, Funct3::LB, ITypeRs1::Register(rs1), imm)
}

/// *(RV32I, I-format)*<br/>
/// `LBU` instruction copies a 8-bit from memory to the register `rd`, zero-extending it to
/// 32&nbsp;bits. The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset `imm`.
pub fn lbu(rd: Register, rs1: Register, imm: i16) -> u32 {
    i_instruction(Opcode::LOAD, rd, Funct3::LBU, ITypeRs1::Register(rs1), imm)
}

/// *(RV32I, I-format)*<br/>
/// `LH` instruction copies a 16-bit value from memory to the register `rd`, sign-extending it to
/// 32&nbsp;bits. The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset `imm`.
pub fn lh(rd: Register, rs1: Register, imm: i16) -> u32 {
    i_instruction(Opcode::LOAD, rd, Funct3::LH, ITypeRs1::Register(rs1), imm)
}

/// *(RV32I, I-format)*<br/>
/// `LHU` instruction copies a 16-bit value from memory to the register `rd`, zero-extending it to
/// 32&nbsp;bits. The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset `imm`.
pub fn lhu(rd: Register, rs1: Register, imm: i16) -> u32 {
    i_instruction(Opcode::LOAD, rd, Funct3::LHU, ITypeRs1::Register(rs1), imm)
}

/// *(RV32I, I-format)*<br/>
/// `LW` instruction copies a 32-bit value from memory to the register `rd`.
/// The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset `imm`.
pub fn lw(rd: Register, rs1: Register, imm: i16) -> u32 {
    i_instruction(Opcode::LOAD, rd, Funct3::LW, ITypeRs1::Register(rs1), imm)
}

/// *(RV32I, S-format)*<br/>
/// `SB` instruction copies a 8-bit value from the low bits of register `rs2` to memory.
/// The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset `imm`.
pub fn sb(rs1: Register, imm: i16, rs2: Register) -> u32 {
    s_instruction(Opcode::STORE, imm, Funct3::SB, rs1, rs2)
}

/// *(RV32I, S-format)*<br/>
/// `SH` instruction copies a 16-bit value from the low bits of register `rs2` to memory.
/// The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset `imm`.
pub fn sh(rs1: Register, imm: i16, rs2: Register) -> u32 {
    s_instruction(Opcode::STORE, imm, Funct3::SH, rs1, rs2)
}

/// *(RV32I, S-format)*<br/>
/// `SW` instruction copies a 32-bit value from the register `rs2` to memory.
/// The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset `imm`.
pub fn sw(rs1: Register, imm: i16, rs2: Register) -> u32 {
    s_instruction(Opcode::STORE, imm, Funct3::SW, rs1, rs2)
}

/// *(RV32I, I-format)*<br/>
/// `ADDI` (add immediate) instruction adds the sign-extended 12-bit immediate `imm` to the register `rs1`.
/// Arithmetic overflow is ignored and the result is simply the low XLEN bits of the result.
/// Note, `ADDI rd, rs1, 0` is equivalent to pseudoinstruction [MV](mv)&nbsp;`rd, rs1`,
/// and `ADDI x0, x0, 0` is equivalent to pseudoinstruction [NOP](nop).
pub fn addi(rd: Register, rs1: Register, imm: i16) -> u32 {
    i_instruction(
        Opcode::OP_IMM,
        rd,
        Funct3::ADDI,
        ITypeRs1::Register(rs1),
        imm,
    )
}

/// *(RV32I, I-format)*<br/>
/// `MV` pseudoinstruction copies the register `rs1` to the register `rd`.<br/><br/>
/// `MV rd, rs1` is encoded as [ADDI](addi)&nbsp;`rd, rs1, 0`.
pub fn mv(rd: Register, rs1: Register) -> u32 {
    addi(rd, rs1, 0)
}

/// *(RV32I, I-format)*<br/>
/// `NOP` instruction does not change any architecturally visible state, except for advancing the
/// pc and incrementing any applicable performance counters.<br/><br/>
/// `NOP` is encoded as [ADDI](addi)&nbsp;`x0, x0, 0`.
pub fn nop() -> u32 {
    addi(X0, X0, 0)
}

/// *(RV32I, I-format)*<br/>
/// `SLTI` (set less than immediate) places the value 1 in the register `rd` if register `rs1` is less than the
/// sign-extended immediate when both are treated as signed numbers, else 0 is written to `rd`.
pub fn slti(rd: Register, rs1: Register, imm: i16) -> u32 {
    i_instruction(
        Opcode::OP_IMM,
        rd,
        Funct3::SLTI,
        ITypeRs1::Register(rs1),
        imm,
    )
}

/// *(RV32I, I-format)*<br/>
/// `SLTIU` (set less than immediate unsigned) places the value 1 in the register `rd` if register `rs1` is less than
/// the sign-extended immediate when both are treated as unsigned numbers, else 0 is written to `rd`. Note,
/// `SLTIU rd, rs1, 1` sets `rd` to 1 if `rs1` = 0, otherwise sets `rd` to 0, and is equivalent to pseudoinstruction
/// [SEQZ](seqz)&nbsp;`rd, rs`).
pub fn sltiu(rd: Register, rs1: Register, imm: i16) -> u32 {
    i_instruction(
        Opcode::OP_IMM,
        rd,
        Funct3::SLTIU,
        ITypeRs1::Register(rs1),
        imm,
    )
}

/// *(RV32I, I-format)*<br/>
/// `SEQZ` (set equal to zero) pseudoinstruction places the value 1 in register `rd` if register `rs1` = 0,
/// else 0 is written to `rd`.<br/><br/>
/// `SEQZ rd, rs1` is encoded as [SLTIU](sltiu)&nbsp;`rd, rs1, 1`.
pub fn seqz(rd: Register, rs1: Register) -> u32 {
    sltiu(rd, rs1, 1)
}

/// *(RV32I, I-format)*<br/>
/// `XORI` performs XOR bitwise logical operation on register `rs1` and the sign-extended 12-bit immediate `imm` and
/// places the result in register `rd`. Note, `XORI rd, rs1, -1` performs a bitwise logical inversion of the register
/// `rs1` and is equivalent to pseudoinstruction [NOT](not)&nbsp;`rd, rs`.
pub fn xori(rd: Register, rs1: Register, imm: i16) -> u32 {
    i_instruction(
        Opcode::OP_IMM,
        rd,
        Funct3::XORI,
        ITypeRs1::Register(rs1),
        imm,
    )
}

/// *(RV32I, I-format)*<br/>
/// `NOT` pseudoinstruction performs bitwise logical inversion of register `rs1` and places the result in the
/// register `rd`.<br/><br/>
/// `NOT rd, rs1` is encoded as [XORI](xori)&nbsp;`rd, rs1, -1`.
pub fn not(rd: Register, rs1: Register) -> u32 {
    xori(rd, rs1, -1)
}

/// *(RV32I, I-format)*<br/>
/// `ORI` performs OR bitwise logical operation on register `rs1` and the sign-extended 12-bit immediate `imm`
/// and places the result in the register `rd`.
pub fn ori(rd: Register, rs1: Register, imm: i16) -> u32 {
    i_instruction(
        Opcode::OP_IMM,
        rd,
        Funct3::ORI,
        ITypeRs1::Register(rs1),
        imm,
    )
}

/// *(RV32I, I-format)*<br/>
/// `ANDI` performs AND bitwise logical operation on register `rs1` and the sign-extended 12-bit immediate `imm`
/// and places the result in the register `rd`.
pub fn andi(rd: Register, rs1: Register, imm: i16) -> u32 {
    i_instruction(
        Opcode::OP_IMM,
        rd,
        Funct3::ANDI,
        ITypeRs1::Register(rs1),
        imm,
    )
}

/// *(RV32I, I-format)*<br/>
/// `SLLI` (shift left logical immediate) instruction performs a left shift of register `rs1` by
/// a constant `shamt` encoded in the lower 5 bits of the I-immediate field, shifting zeros into the lower bits,
/// and places the result in the register `rd`.
pub fn slli(rd: Register, rs1: Register, shamt: u8) -> u32 {
    i_instruction(
        Opcode::OP_IMM,
        rd,
        Funct3::SLLI,
        ITypeRs1::Register(rs1),
        i16::from(shamt & 0x1F),
    )
}

/// *(RV32I, I-format)*<br/>
/// `SRLI` (shift right logical immediate) instruction performs a right shift of register `rs1` by
/// a constant `shamt` encoded in the lower 5 bits of the I-immediate field, shifting zeros into the upper bits,
/// and places the result in the register `rd`.
pub fn srli(rd: Register, rs1: Register, shamt: u8) -> u32 {
    i_instruction(
        Opcode::OP_IMM,
        rd,
        Funct3::SRLI,
        ITypeRs1::Register(rs1),
        i16::from(shamt & 0x1F),
    )
}

/// *(RV32I, I-format)*<br/>
/// `SRAI` (shift right arithmetic immediate) instruction performs a left shift of register `rs1` by
/// a constant `shamt` encoded in the lower 5 bits of the I-immediate field, sign-extends the result
/// and places the result in the register `rd`.
pub fn srai(rd: Register, rs1: Register, shamt: u8) -> u32 {
    i_instruction(
        Opcode::OP_IMM,
        rd,
        Funct3::SRAI,
        ITypeRs1::Register(rs1),
        i16::from(shamt & 0x1F) | 0b0100000 << 5,
    )
}

/// *(RV32I, R-format)*<br/>
/// `ADD` instruction performs the addition of registers `rs1` and `rs2`
/// and places the result in the register `rd`. Overflows are ignored and the low XLEN bits of results are written
/// to the destination `rd`.
pub fn add(rd: Register, rs1: Register, rs2: Register) -> u32 {
    r_instruction(Opcode::OP, rd, Funct3::ADD, rs1, rs2, Funct7::ADD)
}

/// *(RV32I, R-format)*<br/>
/// `SUB` instruction performs the subtraction of registers `rs1` and `rs2`
/// and places the result in the register `rd`. Overflows are ignored and the low XLEN bits of results are written
/// to the destination `rd`.
pub fn sub(rd: Register, rs1: Register, rs2: Register) -> u32 {
    r_instruction(Opcode::OP, rd, Funct3::SUB, rs1, rs2, Funct7::SUB)
}

/// *(RV32I, R-format)*<br/>
/// `SLL` instruction (shift logical left) performs logical left shift on the value in register `rs1` by the shift
/// amount held in the lower 5 bits of register `rs2` and places the result in the register `rd`.
pub fn sll(rd: Register, rs1: Register, rs2: Register) -> u32 {
    r_instruction(Opcode::OP, rd, Funct3::SLL, rs1, rs2, Funct7::SLL)
}

/// *(RV32I, R-format)*<br/>
/// `SRL` instruction (shift logical right) performs logical right shift on the value in register `rs1` by the shift
/// amount held in the lower 5 bits of register `rs2` and places the result in the register `rd`.
pub fn srl(rd: Register, rs1: Register, rs2: Register) -> u32 {
    r_instruction(Opcode::OP, rd, Funct3::SRL, rs1, rs2, Funct7::SRL)
}

/// *(RV32I, R-format)*<br/>
/// `SRA` instruction (shift arithmetic right) performs right shift on the value in register `rs1` by the shift amount
/// held in the lower 5 bits of register `rs2`, sign-extends the result and places the it in the register `rd`.
pub fn sra(rd: Register, rs1: Register, rs2: Register) -> u32 {
    r_instruction(Opcode::OP, rd, Funct3::SRA, rs1, rs2, Funct7::SRA)
}

/// *(RV32I, R-format)*<br/>
/// `SLT` instruction (set less than) perform signed compare,
/// writing 1 to the register `rd` if registers `rs1` < `rs2`, 0 otherwise.
pub fn slt(rd: Register, rs1: Register, rs2: Register) -> u32 {
    r_instruction(Opcode::OP, rd, Funct3::SLT, rs1, rs2, Funct7::SLT)
}

/// *(RV32I, R-format)*<br/>
/// `SLTU` (set less than unsigned) perform unsigned compare,
/// writing 1 to the register `rd` if registers `rs1` < `rs2`, 0 otherwise.
/// Note, `SLTU rd, x0, rs2` sets `rd` to 1 if `rs2` ≠ 0, otherwise sets `rd` to 0, and is equivalent to
/// pseudoinstruction [SNEZ](snez)&nbsp;`rd, rs`.
pub fn sltu(rd: Register, rs1: Register, rs2: Register) -> u32 {
    r_instruction(Opcode::OP, rd, Funct3::SLTU, rs1, rs2, Funct7::SLTU)
}

/// *(RV32I, R-format)*<br/>
/// `SNEZ` pseudoinstruction sets `rd` to 1 if `rs2` ≠ 0, otherwise sets `rd` to 0.<br/><br/>
/// `SNEZ rd, rs2` is encoded as [SLTU](sltu)&nbsp;`rd, x0, rs2`.
pub fn snez(rd: Register, rs2: Register) -> u32 {
    sltu(rd, X0, rs2)
}

/// *(RV32I, R-format)*<br/>
/// `XOR` instruction performs XOR logical operation on registers `rs1` and `rs2`
/// and places the result in the register `rd`.
pub fn xor(rd: Register, rs1: Register, rs2: Register) -> u32 {
    r_instruction(Opcode::OP, rd, Funct3::XOR, rs1, rs2, Funct7::XOR)
}

/// *(RV32I, R-format)*<br/>
/// `OR` instruction performs OR logical operation on registers `rs1` and `rs2`
/// and places the result in the register `rd`.
pub fn or(rd: Register, rs1: Register, rs2: Register) -> u32 {
    r_instruction(Opcode::OP, rd, Funct3::OR, rs1, rs2, Funct7::OR)
}

/// *(RV32I, R-format)*<br/>
/// `AND` instruction performs AND logical operation on registers `rs1` and `rs2`
/// and places the result in the register `rd`.
pub fn and(rd: Register, rs1: Register, rs2: Register) -> u32 {
    r_instruction(Opcode::OP, rd, Funct3::AND, rs1, rs2, Funct7::AND)
}

/// *(RV32 Zicsr, I-format)*<br/>
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
pub fn csrrw(rd: Register, rs1: Register, csr: u16) -> u32 {
    csr_instruction(rd, ITypeRs1::Register(rs1), csr, Funct3::CSRRW)
}

/// *(RV32 Zicsr, I-format)*<br/>
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
pub fn csrrs(rd: Register, rs1: Register, csr: u16) -> u32 {
    csr_instruction(rd, ITypeRs1::Register(rs1), csr, Funct3::CSRRS)
}

/// *(RV32 Zicsr, I-format)*<br/>
/// `CSRR` (atomic read and set bits in CSR) pseudoinstruction atomically reads the value of the CSR register `csr`,
/// zero-extends the value to XLEN bits, and writes it to the register `rd`. `CSRR` always reads the addressed CSR and
/// cause any read side effects regardless of `rd` value.<br/><br/>
/// `CSRR rd, csr` is encoded as [CSRRS](csrrs) `rd, x0, csr`.<br/><br/>
/// Similar instructions and pseudoinstructions for accessing CSRs:
/// [CSRRW](csrrw), [CSRRS](csrrs), [CSRS](csrs), [CSRRC](csrrc), [CSRC](csrc),
/// [CSRRWI](csrrwi), [CSRRSI](csrrsi), [CSRSI](csrsi), [CSRRCI](csrrci), [CSRCI](csrci)
pub fn csrr(rd: Register, csr: u16) -> u32 {
    csrrs(rd, X0, csr)
}

/// *(RV32 Zicsr, I-format)*<br/>
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
pub fn csrs(rs1: Register, csr: u16) -> u32 {
    csrrs(X0, rs1, csr)
}

/// *(RV32 Zicsr, I-format)*<br/>
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
pub fn csrrc(rd: Register, rs1: Register, csr: u16) -> u32 {
    csr_instruction(rd, ITypeRs1::Register(rs1), csr, Funct3::CSRRC)
}

/// *(RV32 Zicsr, I-format)*<br/>
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
pub fn csrc(rs1: Register, csr: u16) -> u32 {
    csrrc(X0, rs1, csr)
}

/// *(RV32 Zicsr, I-format)*<br/>
/// `CSRRWI` (atomic read/write CSR with immediate) instruction atomically swaps values in CSRs and general-purpose
/// registers. `CSRRWI` reads the old value of the CSR register `csr`, zero-extends the value to XLEN bits, then writes
/// it to the register `rd`. An XLEN-bit value obtained by zero-extending a 5-bit unsigned immediate `uimm` is written
/// to the CSR. `CSRRWI` always reads the addressed CSR and cause any read side effects regardless of
/// `uimm` and `rd` values.<br/><br/>
/// Similar instructions and pseudoinstructions for accessing CSRs:
/// [CSRRW](csrrw), [CSRRS](csrrs), [CSRR](csrr), [CSRS](csrs), [CSRRC](csrrc),
/// [CSRC](csrc), [CSRRSI](csrrsi), [CSRSI](csrsi), [CSRRCI](csrrci), [CSRCI](csrci)
pub fn csrrwi(rd: Register, uimm: CsrMask, csr: u16) -> u32 {
    csr_instruction(rd, ITypeRs1::Uimm5(uimm), csr, Funct3::CSRRWI)
}

/// *(RV32I, I-format)*<br/>
/// `CSRRSI` (atomic read and set bits in CSR with immediate) instruction atomically reads the value of the CSR register
/// `csr`, zero-extends the value to XLEN bits, and writes it to the register `rd`. An XLEN-bit value obtained by
/// zero-extending a 5-bit unsigned immediate `uimm` is treated as a bit mask that specifies bit positions to be set in
/// the CSR. Any bit that is high in `uimm` will cause the corresponding bit to be set in the CSR, if that CSR bit is
/// writable. Other bits in the CSR are not explicitly written. `CSRRSI` always reads the addressed CSR and cause any
/// read side effects regardless of `uimm` and `rd` values.<br/><br/>
/// Similar instructions and pseudoinstructions for accessing CSRs:
/// [CSRRW](csrrw), [CSRRS](csrrs), [CSRR](csrr), [CSRS](csrs), [CSRRC](csrrc),
/// [CSRC](csrc), [CSRRWI](csrrwi), [CSRSI](csrsi), [CSRRCI](csrrci), [CSRCI](csrci)
pub fn csrrsi(rd: Register, uimm: CsrMask, csr: u16) -> u32 {
    csr_instruction(rd, ITypeRs1::Uimm5(uimm), csr, Funct3::CSRRSI)
}

/// *(RV32 Zicsr, I-format)*<br/>
/// `CSRSI` (atomic set bits in CSR with immediate) pseudoinstruction atomically sets bits in the CSR register `csr`
/// using an XLEN-bit value obtained by zero-extending a 5-bit unsigned immediate `uimm` as a bit mask that specifies
/// bit positions to be set in the CSR. Any bit that is high in `uimm` will cause the corresponding bit to be set in the
/// CSR, if that CSR bit is writable. Other bits in the CSR are not explicitly written. `CSRSI` always reads the
/// addressed CSR and cause any read side effects regardless of `uimm` value.<br/><br/>
/// `CSRSI uimm, csr` is encoded as [CSRRSI](csrrsi) `x0, uimm, csr`.<br/><br/>
/// Similar instructions and pseudoinstructions for accessing CSRs:
/// [CSRRW](csrrw), [CSRRS](csrrs), [CSRR](csrr), [CSRS](csrs), [CSRRC](csrrc),
/// [CSRC](csrc), [CSRRWI](csrrwi), [CSRRSI](csrrsi),  [CSRRCI](csrrci), [CSRCI](csrci)
pub fn csrsi(uimm: CsrMask, csr: u16) -> u32 {
    csrrsi(X0, uimm, csr)
}

/// *(RV32 Zicsr, I-format)*<br/>
/// `CSRRCI` (atomic read and clear bits in CSR with immediate) instruction atomically reads the value of the CSR
/// register `csr`, zero-extends the value to XLEN bits, and writes it to the register `rd`. An XLEN-bit value obtained
/// by zero-extending a 5-bit unsigned immediate `uimm` is treated as a bit mask that specifies bit positions to be
/// cleared in the CSR. Any bit that is high in `rs1` will cause the corresponding bit to be cleared in the CSR, if that
/// CSR bit is writable. Other bits in the CSR are not explicitly written. `CSRRCI` always reads the addressed CSR and
/// cause any read side effects regardless of `uimm` and `rd` values.<br/><br/>
/// Similar instructions and pseudoinstructions for accessing CSRs:
/// [CSRRW](csrrw), [CSRRS](csrrs), [CSRR](csrr), [CSRS](csrs), [CSRRC](csrrc),
/// [CSRC](csrc), [CSRRWI](csrrwi), [CSRRSI](csrrsi), [CSRSI](csrsi), [CSRCI](csrci)
pub fn csrrci(rd: Register, uimm: CsrMask, csr: u16) -> u32 {
    csr_instruction(rd, ITypeRs1::Uimm5(uimm), csr, Funct3::CSRRCI)
}

/// *(RV32 Zicsr, I-format)*<br/>
/// `CSRCI` (atomic clear bits in CSR with immediate) pseudoinstruction atomically clears bits in the CSR register
/// `csr`, using an XLEN-bit value obtained by zero-extending the 5-bit unsigned immediate `uimm` as a bit mask that
/// specifies bit positions to be cleared in the CSR. Any bit that is high in `uimm` will cause the corresponding bit to
/// be cleared in the CSR, if that CSR bit is writable. Other bits in the CSR are not explicitly written. `CSRCI` always
/// reads the addressed CSR and cause any read side effects regardless of `uimm`.<br/><br/>
/// `CSRCI uimm, csr` is encoded as [CSRRCI](csrrci) `x0, uimm, csr`.<br/><br/>
/// Similar instructions and pseudoinstructions for accessing CSRs:
/// [CSRRW](csrrw), [CSRRS](csrrs), [CSRR](csrr), [CSRS](csrs), [CSRRC](csrrc),
/// [CSRC](csrc), [CSRRWI](csrrwi), [CSRRSI](csrrsi), [CSRSI](csrsi), [CSRRCI](csrrci)
pub fn csrci(uimm: CsrMask, csr: u16) -> u32 {
    csrrci(X0, uimm, csr)
}

/// *(RV32 Zicsr, I-format specialized)*<br/>
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
fn csr_instruction(rd: Register, mask: ITypeRs1, csr: u16, funct3: Funct3) -> u32 {
    i_instruction(
        Opcode::SYSTEM,
        rd,
        funct3,
        mask,
        i16::try_from(csr).unwrap(),
    )
}

/// *(RV32I, I-format specialized)*<br/>
/// `FENCE` instruction orders all memory operations in its `predecessor` set before all memory operations in
/// its `successor` set. Both sets are defined here as string slices, with individual characters representing
/// the corresponsing memory operations:
/// * `i` — device input
/// * `o` — device output
/// * `r` — memory reads
/// * `w` — memory writes
///
/// Note, [FENCE.TSO](fence_tso) instruction is encoded as a `FENCE` instruction with `fm` = 1000
/// (refer to the instruction manual for this field), `predecessor` = "rw", and `successor` = "rw".
pub fn fence(predecessor: FenceMask, successor: FenceMask) -> u32 {
    fence_instruction(0b0000, predecessor, successor)
}

/// *(RV32I, I-format specialized)*<br/>
/// `FENCE.TSO` instruction is encoded as a [FENCE](fence) instruction with `fm` = 1000
/// (refer to the instruction manual for this field), `predecessor` = "rw", and `successor` = "rw".
pub fn fence_tso() -> u32 {
    fence_instruction(0b1000, FENCE_MASK_RW, FENCE_MASK_RW)
}

/// *(RV32I, I-format specialized)*<br/>
/// FENCE-type instruction encoding.<br/>
/// ```text
/// Bit range     | 31:28 | 27 26 25 24 | 23 22 21 20 | 19:15 | 14:12  | 11:7 |  6:0     |
/// Field name    |  fm   | PI PO PR PW | SI SO SR SW |  rs1  | funct3 |  rd  | opcode   |
/// Bit count     |  4    | 1  1  1  1  | 1  1  1  1  |   5   |   3    |  5   |   7      |
/// Description   |  FM   | predecessor |  successor  |   0   | FENCE  |  0   | MISC_MEM |
/// ```
fn fence_instruction(fm: u8, predecessor: FenceMask, successor: FenceMask) -> u32 {
    let mut imm = 0u16;
    let imm_bits = imm.view_bits_mut::<Lsb0>();
    imm_bits[0..4].clone_from_bitslice(predecessor.view_bits());
    imm_bits[4..8].clone_from_bitslice(successor.view_bits());
    imm_bits[8..12].store(fm);
    i_instruction(
        Opcode::MISC_MEM,
        X0,
        Funct3::FENCE,
        ITypeRs1::Register(X0),
        imm as i16,
    )
}

/// *(RV32I, I-format)*<br/>
/// `ECALL` instruction is used to make a service request to the execution environment. The EEI
/// (execution environment interface) will define how parameters for the service request are passed,
/// but usually these will be in defined locations in the integer register file.
pub fn ecall() -> u32 {
    i_instruction(
        Opcode::SYSTEM,
        X0,
        Funct3::ECALL,
        ITypeRs1::Register(X0),
        0b0000_0000_0000,
    )
}

/// *(RV32I, I-format)*<br/>
/// `EBREAK` instruction is used to return control to a debugging environment.
/// `EBREAK` was primarily designed to be used by a debugger to cause execution to stop and fall back into the debugger.
/// `EBREAK` is also used by the standard gcc compiler to mark code paths/ that should not be executed.
/// Another use of `EBREAK` is to support "semihosting", where the execution environment in cludes a debugger that can
/// provide services over an alternate system call interface built around the `EBREAK` instruction. Because the RISC-V
/// base ISAs do not provide more than one `EBREAK` instruction, RISC-V semihosting uses a special sequence of
/// instructions to distinguish a semihosting `EBREAK` from a debugger inserted `EBREAK`.
/// ```asm
/// slli x0, x0, 0x1f   # Entry NOP
/// ebreak              # Break to debugger
/// srai x0, x0, 7      # NOP encoding the semihosting call number 7
/// ```
pub fn ebreak() -> u32 {
    i_instruction(
        Opcode::SYSTEM,
        X0,
        Funct3::EBREAK,
        ITypeRs1::Register(X0),
        0b0000_0000_0001,
    )
}

// RV32M Standard Extension

/// *(RV32M, R-format)*<br/>
/// `MUL` instruction performs an XLEN-bit×XLEN-bit multiplication of `rs1` by `rs2` and places the lower XLEN bits
/// in the destination register. If both the high and low bits of the same
/// product are required, then the recommended code sequence is:<br/>
/// [`MULH`](mulh) | [`MULHSU`](mulhsu) | [`MULHU`](mulhu) `rdh, rs1, rs2`<br/>
/// `MUL rdl, rs1, rs2`<br/>
/// Source register specifiers must be in same order and `rdh` cannot be the same as `rs1` or
/// `rs2`. Microarchitectures can then fuse these into a single multiply operation instead of performing
/// two separate multiplies.
pub fn mul(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::MUL)
}

/// *(RV32M, R-format)*<br/>
/// `MULH` instruction performs an XLEN-bit×XLEN-bit multiplication of `rs1` (signed) by `rs2` (signed) and
/// places the upper XLEN bits in the destination register. If both the high and low bits of the same
/// product are required, then the recommended code sequence is:<br/>
/// `MULH` | [`MULHSU`](mulhsu) | [`MULHU`](mulhu) `rdh, rs1, rs2`<br/>
/// [`MUL`](mul)` rdl, rs1, rs2`<br/>
/// Source register specifiers must be in same order and `rdh` cannot be the same as `rs1` or
/// `rs2`. Microarchitectures can then fuse these into a single multiply operation instead of performing
/// two separate multiplies.
pub fn mulh(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::MULH)
}

/// *(RV32M, R-format)*<br/>
/// `MULHSU` instruction performs an XLEN-bit×XLEN-bit multiplication of `rs1` (signed) by `rs2` (unsigned) and
/// places the upper XLEN bits in the destination register. If both the high and low bits of the same
/// product are required, then the recommended code sequence is:<br/>
/// [`MULH`](mulh) | `MULHSU` | [`MULHU`](mulhu) `rdh, rs1, rs2`<br/>
/// [`MUL`](mul)` rdl, rs1, rs2`<br/>
/// Source register specifiers must be in same order and `rdh` cannot be the same as `rs1` or
/// `rs2`. Microarchitectures can then fuse these into a single multiply operation instead of performing
/// two separate multiplies.
pub fn mulhsu(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::MULHSU)
}

/// *(RV32M, R-format)*<br/>
/// `MULHU` instruction performs an XLEN-bit×XLEN-bit multiplication of `rs1` (unsigned) by `rs2` (unsigned) and
/// places the upper XLEN bits in the destination register. If both the high and low bits of the same
/// product are required, then the recommended code sequence is:<br/>
/// [`MULH`](mulh) | [`MULHSU`](mulhsu) | `MULHU` `rdh, rs1, rs2`<br/>
/// [`MUL`](mul)` rdl, rs1, rs2`<br/>
/// Source register specifiers must be in same order and `rdh` cannot be the same as `rs1` or
/// `rs2`. Microarchitectures can then fuse these into a single multiply operation instead of performing
/// two separate multiplies.
pub fn mulhu(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::MULHU)
}

/// *(RV32M, R-format)*<br/>
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
/// separate divides.
pub fn div(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::DIV)
}

/// *(RV32M, R-format)*<br/>
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
/// separate divides.
pub fn divu(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::DIVU)
}

/// *(RV32M, R-format)*<br/>
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
/// separate divides.
pub fn rem(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::REM)
}

/// *(RV32M, R-format)*<br/>
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
/// separate divides.
pub fn remu(rd: Register, rs1: Register, rs2: Register) -> u32 {
    muldiv_instruction(rd, rs1, rs2, Funct3::REMU)
}

/// *(RV32M, R-format)*<br/>
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
    r_instruction(Opcode::OP, rd, funct3, rs1, rs2, Funct7::MULDIV)
}

// Implementation

fn r_instruction(
    opcode: Opcode,
    rd: Register,
    funct3: Funct3,
    rs1: Register,
    rs2: Register,
    funct7: Funct7,
) -> u32 {
    let mut instruction = 0;
    let bits = instruction.view_bits_mut::<Lsb0>();
    bits[0..=6].clone_from_bitslice(opcode.view_bits());
    bits[7..=11].clone_from_bitslice(rd.view_bits());
    bits[12..=14].clone_from_bitslice(funct3.view_bits());
    bits[15..=19].clone_from_bitslice(rs1.view_bits());
    bits[20..=24].clone_from_bitslice(rs2.view_bits());
    bits[25..=31].clone_from_bitslice(funct7.view_bits());
    instruction
}

enum ITypeRs1 {
    Register(Register),
    Uimm5(CsrMask),
}

impl ITypeRs1 {
    fn view_bits(&self) -> &BitSlice<u8, Lsb0> {
        match self {
            ITypeRs1::Register(register) => register.view_bits(),
            ITypeRs1::Uimm5(uimm) => uimm.view_bits(),
        }
    }
}

fn i_instruction(
    opcode: Opcode,
    rd: Register,
    funct3: Funct3,
    // `rs1` has to be u8, because there are specialized variants of I-format
    // that use rs1 as immediate
    rs1: ITypeRs1,
    imm: i16, // TODO make it u16 or an enum for specialized variants
) -> u32 {
    let mut instruction = 0;
    let bits = instruction.view_bits_mut::<Lsb0>();
    bits[0..=6].clone_from_bitslice(opcode.view_bits());
    bits[7..=11].clone_from_bitslice(rd.view_bits());
    bits[12..=14].clone_from_bitslice(funct3.view_bits());
    bits[15..=19].clone_from_bitslice(rs1.view_bits());
    bits[20..=31].store(imm);
    instruction
}

fn s_instruction(opcode: Opcode, imm: i16, funct3: Funct3, rs1: Register, rs2: Register) -> u32 {
    let mut instruction = 0;
    let bits = instruction.view_bits_mut::<Lsb0>();
    let imm = imm as u32;
    let imm_bits = imm.view_bits::<Lsb0>();
    bits[0..=6].clone_from_bitslice(opcode.view_bits());
    bits[7..=11].copy_from_bitslice(&imm_bits[0..=4]);
    bits[12..=14].clone_from_bitslice(funct3.view_bits());
    bits[15..=19].clone_from_bitslice(rs1.view_bits());
    bits[20..=24].clone_from_bitslice(rs2.view_bits());
    bits[25..=31].copy_from_bitslice(&imm_bits[5..=11]);
    instruction
}

fn b_instruction(opcode: Opcode, imm: i16, funct3: Funct3, rs1: Register, rs2: Register) -> u32 {
    let mut instruction = 0;
    let bits = instruction.view_bits_mut::<Lsb0>();
    let imm = imm as u16;
    let imm_bits = imm.view_bits::<Lsb0>();
    bits[0..=6].clone_from_bitslice(opcode.view_bits());
    bits.set(7, imm_bits[11]);
    bits[8..=11].clone_from_bitslice(&imm_bits[1..=4]);
    bits[12..=14].clone_from_bitslice(funct3.view_bits());
    bits[15..=19].clone_from_bitslice(rs1.view_bits());
    bits[20..=24].clone_from_bitslice(rs2.view_bits());
    bits[25..=30].clone_from_bitslice(&imm_bits[5..=10]);
    bits.set(31, imm_bits[12]);
    instruction
}

fn u_instruction(opcode: Opcode, rd: Register, imm: i32) -> u32 {
    let mut instruction = 0;
    let bits = instruction.view_bits_mut::<Lsb0>();
    bits[0..=6].clone_from_bitslice(opcode.view_bits());
    bits[7..=11].clone_from_bitslice(rd.view_bits());
    bits[12..=31].store(imm);
    instruction
}

fn j_instruction(opcode: Opcode, rd: Register, imm: i32) -> u32 {
    let mut instruction = 0;
    let bits = instruction.view_bits_mut::<Lsb0>();
    let imm = imm as u32;
    let imm_bits = imm.view_bits::<Lsb0>();
    bits[0..=6].clone_from_bitslice(opcode.view_bits());
    bits[7..=11].clone_from_bitslice(rd.view_bits());
    bits[12..=19].copy_from_bitslice(&imm_bits[12..=19]);
    bits.set(20, imm_bits[11]);
    bits[21..=30].copy_from_bitslice(&imm_bits[1..=10]);
    bits.set(31, imm_bits[20]);
    instruction
}
