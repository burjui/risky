//! RV32I base instruction set

mod fence_mask;
mod fence_mode;

pub use fence_mask::*;
use fence_mode::FenceMode;

use super::formats::{
    b_instruction, funct3::Funct3, funct7::Funct7, i_instruction, j_instruction, opcode::Opcode,
    r_instruction, s_instruction, u_instruction, RegOrUimm5,
};
pub use super::{b_imm::*, imm12::*, j_imm::*, uimm5::*};
pub use crate::registers::*;

/// "Load Upper Immediate" instruction is primarily used to build 32-bit constants. It places `imm` in the top
/// 20 bits of the destination register `rd`, filling in the lowest 12 bits with zeros.<br/><br/>
/// It is supposed to be paired with the following instructions for:
/// - Just building a constant: [addi]
/// - Loading from memory: [lb], [lbu], [lh], [lhu], [lw]
/// - Storing to memory: [sb], [sh], [sw]
/// - Indirect jumps: [jalr]
pub fn lui(rd: Register, imm: i32) -> u32 {
    u_instruction(Opcode::LUI, rd, imm)
}

/// "Add Upper Immediate to PC" instruction is used to build pc-relative addresses.
/// It forms a 32-bit offset from `imm`, filling in the lowest 12 bits with zeros, adds this offset to the address of
/// the `auipc` instruction, then places the result in the register `rd`.<br/><br/>
/// For a pc-relative jump, use [jal].
pub fn auipc(rd: Register, imm: i32) -> u32 {
    u_instruction(Opcode::AUIPC, rd, imm)
}

/// "Jump And Link" instruction performs a pc-relative jump, where `imm` encodes a 21-bit signed offset,
/// with the lowest bit ignored. The offset is sign-extended and added to the address of the jump instruction (pc) to
/// form the jump target address. Jumps can therefore target a ±1&nbsp;MiB range. `jal` stores the address of the
/// instruction that follows the `jal` (pc+4) into the register `rd`. The standard software calling convention uses [X1]
/// as the return address register and [X5] as an alternate link register.<br/><br/>
/// For an indirect jump, use [jalr].<br/>
/// For just retrieving the value of `pc`, use [auipc].
pub fn jal(rd: Register, imm: JImm) -> u32 {
    j_instruction(Opcode::JAL, rd, imm)
}

/// "Jump And Link from Register" instruction performs an indirect jump. The target address is obtained by
/// adding the sign-extended 12-bit `imm` to the register `rs1`, then setting the least-significant bit of the result to
/// zero. The address of the instruction following the `jalr` (pc+4) is written to the register `rd`. Register [X0] can
/// be used as the destination if the result is not required.<br/><br/>
/// For a pc-relative jump, use [jal].<br/>
/// For just retrieving the value of `pc`, use [auipc].
pub fn jalr(rd: Register, rs1: Register, imm: Imm12) -> u32 {
    i_instruction(
        Opcode::JALR,
        rd,
        Funct3::JALR,
        RegOrUimm5::Register(rs1),
        imm,
    )
}

/// "Branch if EQual" instruction takes the branch if `rs1` = `rs2`, using the 13-bit immediate `imm` as an
/// offset from the address of the branch instruction (pc). The lower bit if `imm` is ignored, so the effective offset
/// is always in multiples of 2. The branch range is ±4&nbsp;KiB.<br/><br/>
/// Other branch instructions with different conditions: [bne], [blt], [bltu], [bge], [bgeu].
pub fn beq(imm: BImm, rs1: Register, rs2: Register) -> u32 {
    b_instruction(Opcode::BRANCH, imm, Funct3::BEQ, rs1, rs2)
}

/// "Branch if Not Equal" instruction takes the branch if `rs1` ≠ `rs2`, using the 13-bit immediate `imm` as
/// an offset from the address of the branch instruction (pc). The lower bit if `imm` is ignored, so the effective
/// offset is always in multiples of 2. The branch range is ±4&nbsp;KiB.<br/><br/>
/// Other branch instructions with different conditions: [beq], [blt], [bltu], [bge], [bgeu].
pub fn bne(imm: BImm, rs1: Register, rs2: Register) -> u32 {
    b_instruction(Opcode::BRANCH, imm, Funct3::BNE, rs1, rs2)
}

/// "Branch if Less Than" instruction takes the branch if `rs1` < `rs2`, using signed comparison and the
/// 13-bit immediate `imm` as an offset from the address of the branch instruction (pc). The lower bit if `imm` is
/// ignored, so the effective offset is always in multiples of 2. The conditional branch range is ±4&nbsp;KiB.<br/><br/>
/// Other branch instructions with different conditions: [bltu], [bge], [bgeu], [beq], [bne].
pub fn blt(imm: BImm, rs1: Register, rs2: Register) -> u32 {
    b_instruction(Opcode::BRANCH, imm, Funct3::BLT, rs1, rs2)
}

/// "Branch if Less Than Unsigned" instruction takes the branch if `rs1` < `rs2`, using unsigned comparison
/// and the 13-bit immediate `imm` as an offset from the address of the branch instruction (pc). The lower bit if `imm`
/// is ignored, so the effective offset is always in multiples of 2. The branch range is ±4&nbsp;KiB.<br/><br/>
/// Note that signed array bounds may be checked with a single BLTU instruction,
/// since any negative index will compare greater than any nonnegative bound.<br/><br/>
/// Other branch instructions with different conditions: [blt], [bge], [bgeu], [beq], [bne]
pub fn bltu(imm: BImm, rs1: Register, rs2: Register) -> u32 {
    b_instruction(Opcode::BRANCH, imm, Funct3::BLTU, rs1, rs2)
}

/// "Branch if Greater or Equal" instruction takes the branch if `rs1` ≥ `rs2`, using signed comparison and
/// the 13-bit immediate `imm` as an offset from the address of the branch instruction (pc). The lower bit if `imm` is
/// ignored, so the effective offset is always in multiples of 2. The branch range is ±4&nbsp;KiB.<br/><br/>
/// Other branch instructions with different conditions: [bgeu], [blt], [bltu], [beq], [bne].
pub fn bge(imm: BImm, rs1: Register, rs2: Register) -> u32 {
    b_instruction(Opcode::BRANCH, imm, Funct3::BGE, rs1, rs2)
}

/// "Branch if Greater or Equal Unsigned" instruction takes the branch if `rs1` ≥ `rs2`, using unsigned
/// comparison and the 13-bit immediate `imm` as an offset from the address of the branch instruction (pc). The lower
/// bit if `imm` is ignored, so the effective offset is always in multiples of 2.
/// The branch range is ±4&nbsp;KiB.<br/><br/>
/// Other branch instructions with different conditions: [bge], [blt], [bltu], [beq], [bne].
pub fn bgeu(imm: BImm, rs1: Register, rs2: Register) -> u32 {
    b_instruction(Opcode::BRANCH, imm, Funct3::BGEU, rs1, rs2)
}

/// "Load Byte" instruction copies a 8-bit value from memory to the register `rd`, sign-extending it to
/// 32&nbsp;bits. The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset `imm`.
/// <br/><br/>
/// For loading from a constant address, pair `lb` with [lui].<br/><br/>
/// Other load instructions: [lbu], [lh], [lhu], [lw]<br/>
/// Store instructions: [sb], [sh], [sw]<br/>
/// Copying values between registers: [mv]
pub fn lb(rd: Register, rs1: Register, imm: Imm12) -> u32 {
    i_instruction(Opcode::LOAD, rd, Funct3::LB, RegOrUimm5::Register(rs1), imm)
}

/// "Load Byte Unsigned" instruction copies a 8-bit from memory to the register `rd`, zero-extending it to
/// 32&nbsp;bits. The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset `imm`.
/// <br/><br/>
/// For loading from a constant address, pair `lbu` with [lui].<br/><br/>
/// Other load instructions: [lb], [lh], [lhu], [lw]<br/>
/// Store instructions: [sb], [sh], [sw]<br/>
/// Copying values between registers: [mv]
pub fn lbu(rd: Register, rs1: Register, imm: Imm12) -> u32 {
    i_instruction(
        Opcode::LOAD,
        rd,
        Funct3::LBU,
        RegOrUimm5::Register(rs1),
        imm,
    )
}

/// "Load Half-word" instruction copies a 16-bit value from memory to the register `rd`, sign-extending it to
/// 32&nbsp;bits. The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset `imm`.
/// <br/><br/>
/// For loading from a constant address, pair `lh` with [lui].<br/><br/>
/// Other load instructions: [lhu], [lb], [lbu], [lw]<br/>
/// Store instructions: [sb], [sh], [sw]<br/>
/// Copying values between registers: [mv]
pub fn lh(rd: Register, rs1: Register, imm: Imm12) -> u32 {
    i_instruction(Opcode::LOAD, rd, Funct3::LH, RegOrUimm5::Register(rs1), imm)
}

/// "Load Half-word Unsigned" instruction copies a 16-bit value from memory to the register `rd`, zero-extending it to
/// 32&nbsp;bits. The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset `imm`.
/// <br/><br/>
/// For loading from a constant address, pair `lhu` with [lui].<br/><br/>
/// Other load instructions: [lh], [lb], [lbu], [lw]<br/>
/// Store instructions: [sb], [sh], [sw]<br/>
/// Copying values between registers: [mv]
pub fn lhu(rd: Register, rs1: Register, imm: Imm12) -> u32 {
    i_instruction(
        Opcode::LOAD,
        rd,
        Funct3::LHU,
        RegOrUimm5::Register(rs1),
        imm,
    )
}

/// "Load Word" instruction copies a 32-bit value from memory to the register `rd`.
/// The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset `imm`.
/// <br/><br/>
/// For loading from a constant address, pair `lw` with [lui].<br/><br/>
/// Other load instructions: [lb], [lbu], [lh], [lhu]<br/>
/// Store instructions: [sb], [sh], [sw]<br/>
/// Copying values between registers: [mv]
pub fn lw(rd: Register, rs1: Register, imm: Imm12) -> u32 {
    i_instruction(Opcode::LOAD, rd, Funct3::LW, RegOrUimm5::Register(rs1), imm)
}

/// "Store Byte" instruction copies a 8-bit value from the low bits of register `rs2` to memory.
/// The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset `imm`.
/// <br/><br/>
/// For storing to a constant address, pair `sb` with [lui].<br/><br/>
/// Other store instructions: [sh], [sw]<br/>
/// Load instructions: [lb], [lbu], [lh], [lhu], [lw]<br/>
/// Copying values between registers: [mv]
pub fn sb(rs1: Register, imm: Imm12, rs2: Register) -> u32 {
    s_instruction(Opcode::STORE, imm, Funct3::SB, rs1, rs2)
}

/// "Store Half-word" instruction copies a 16-bit value from the low bits of register `rs2` to memory.
/// The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset `imm`.
/// <br/><br/>
/// For storing to a constant address, pair `sh` with [lui].<br/><br/>
/// Other store instructions: [sb], [sw]<br/>
/// Load instructions: [lb], [lbu], [lh], [lhu], [lw]<br/>
/// Copying values between registers: [mv]
pub fn sh(rs1: Register, imm: Imm12, rs2: Register) -> u32 {
    s_instruction(Opcode::STORE, imm, Funct3::SH, rs1, rs2)
}

/// "Store Word" instruction copies a 32-bit value from the register `rs2` to memory.
/// The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset `imm`.
/// <br/><br/>
/// For storing to a constant address, pair `sw` with [lui].<br/><br/>
/// Other store instructions: [sb], [sh]<br/>
/// Load instructions: [lb], [lbu], [lh], [lhu], [lw]<br/>
/// Copying values between registers: [mv]
pub fn sw(rs1: Register, imm: Imm12, rs2: Register) -> u32 {
    s_instruction(Opcode::STORE, imm, Funct3::SW, rs1, rs2)
}

/// "ADD Immediate" instruction adds the sign-extended 12-bit immediate `imm` to the register `rs1`.
/// Arithmetic overflow is ignored and the result is simply the low XLEN bits of the result.
/// Note, `ADDI rd, rs1, 0` is equivalent to pseudoinstruction [MV](mv)&nbsp;`rd, rs1`,
/// and `ADDI x0, x0, 0` is equivalent to pseudoinstruction [NOP](nop).<br/><br/>
/// Other arithmetic instructions:
/// - RV32I: [add], [sub]
/// - M extension: [mul](super::mul), [mulh](super::mulh), [mulhsu](super::mulhsu), [mulhu](super::mulhu),
/// [div](super::div), [divu](super::divu), [rem](super::rem), [remu](super::remu)
pub fn addi(rd: Register, rs1: Register, imm: Imm12) -> u32 {
    i_instruction(
        Opcode::OP_IMM,
        rd,
        Funct3::ADDI,
        RegOrUimm5::Register(rs1),
        imm,
    )
}

/// "MoVe" pseudoinstruction copies the register `rs1` to the register `rd`.<br/><br/>
/// `MV rd, rs1` is encoded as [ADDI](addi)&nbsp;`rd, rs1, 0`.<br/><br/>
/// For copying values to and from memory, use the following instructions:
/// - Loading from memory: [lb], [lbu], [lh], [lhu], [lw]
/// - Storing to memory: [sb], [sh], [sw]
pub fn mv(rd: Register, rs1: Register) -> u32 {
    addi(rd, rs1, Imm12::ZERO)
}

/// "No OPeration" instruction does not change any architecturally visible state, except for advancing the
/// pc and incrementing any applicable performance counters.<br/><br/>
/// `NOP` is encoded as [ADDI](addi)&nbsp;`x0, x0, 0`.
pub fn nop() -> u32 {
    addi(X0, X0, Imm12::ZERO)
}

/// "Set Less Than Immediate" instruction places the value 1 in the register `rd` if register `rs1` is less than the
/// sign-extended immediate when both are treated as signed numbers, else 0 is written to `rd`.<br/><br/>
/// Other comparison instructions: [slt], [sltu], [sltiu], [seqz], [snez]
pub fn slti(rd: Register, rs1: Register, imm: Imm12) -> u32 {
    i_instruction(
        Opcode::OP_IMM,
        rd,
        Funct3::SLTI,
        RegOrUimm5::Register(rs1),
        imm,
    )
}

/// "Set Less Than Immediate Unsigned" (set less than immediate unsigned) places the value 1 in the register `rd` if register `rs1` is less than
/// the sign-extended immediate when both are treated as unsigned numbers, else 0 is written to `rd`. Note,
/// `SLTIU rd, rs1, 1` sets `rd` to 1 if `rs1` = 0, otherwise sets `rd` to 0, and is equivalent to pseudoinstruction
/// [SEQZ](seqz)&nbsp;`rd, rs`).<br/><br/>
/// Other comparison instructions: [slt], [sltu], [slti], [seqz], [snez]
pub fn sltiu(rd: Register, rs1: Register, imm: Imm12) -> u32 {
    i_instruction(
        Opcode::OP_IMM,
        rd,
        Funct3::SLTIU,
        RegOrUimm5::Register(rs1),
        imm,
    )
}

/// "Set EQual to Zero" pseudoinstruction places the value 1 in register `rd` if register `rs1` = 0,
/// else 0 is written to `rd`.<br/><br/>
/// `SEQZ rd, rs1` is encoded as [SLTIU](sltiu)&nbsp;`rd, rs1, 1`.<br/><br/>
/// Other comparison instructions: [slt], [sltu], [slti], [sltiu], [snez]
pub fn seqz(rd: Register, rs1: Register) -> u32 {
    sltiu(rd, rs1, Imm12::ONE)
}

/// "XOR with Immediate" performs XOR bitwise logical operation on register `rs1` and the sign-extended 12-bit immediate `imm` and
/// places the result in register `rd`. Note, `XORI rd, rs1, -1` performs a bitwise logical inversion of the register
/// `rs1` and is equivalent to pseudoinstruction [NOT](not)&nbsp;`rd, rs`.<br/><br/>
/// Other logical operations: [xor], [or], [ori], [and], [andi], [not]
pub fn xori(rd: Register, rs1: Register, imm: Imm12) -> u32 {
    i_instruction(
        Opcode::OP_IMM,
        rd,
        Funct3::XORI,
        RegOrUimm5::Register(rs1),
        imm,
    )
}

/// "NOT" pseudoinstruction performs bitwise logical inversion of register `rs1` and places the result in the
/// register `rd`.<br/><br/>
/// `NOT rd, rs1` is encoded as [XORI](xori)&nbsp;`rd, rs1, -1`.<br/><br/>
/// Other logical operations: [xor], [xori], [or], [ori], [and], [andi]
pub fn not(rd: Register, rs1: Register) -> u32 {
    xori(rd, rs1, Imm12::MINUS_ONE)
}

/// "OR with Immediate" instruction performs OR bitwise logical operation on register `rs1` and the sign-extended 12-bit immediate `imm`
/// and places the result in the register `rd`.<br/><br/>
/// Other logical operations: [or], [xor], [xori], [and], [andi], [not]
pub fn ori(rd: Register, rs1: Register, imm: Imm12) -> u32 {
    i_instruction(
        Opcode::OP_IMM,
        rd,
        Funct3::ORI,
        RegOrUimm5::Register(rs1),
        imm,
    )
}

/// "AND with Immediate" instruction performs AND bitwise logical operation on register `rs1` and the sign-extended 12-bit immediate `imm`
/// and places the result in the register `rd`.<br/><br/>
/// Other logical operations: [and], [xor], [xori], [or], [ori], [not]
pub fn andi(rd: Register, rs1: Register, imm: Imm12) -> u32 {
    i_instruction(
        Opcode::OP_IMM,
        rd,
        Funct3::ANDI,
        RegOrUimm5::Register(rs1),
        imm,
    )
}

/// "Shift Left Logical by Immediate" instruction performs a left shift of register `rs1` by
/// a constant `shamt` encoded in the lower 5 bits of the I-immediate field, shifting zeros into the lower bits,
/// and places the result in the register `rd`.<br/><br/>
/// Other shift instructions: [sll], [srl], [srli], [sra], [srai]
pub fn slli(rd: Register, rs1: Register, shamt: Uimm5) -> u32 {
    r_instruction(
        Opcode::OP_IMM,
        rd,
        Funct3::SLLI,
        rs1,
        RegOrUimm5::Uimm5(shamt),
        Funct7::SLL,
    )
}

/// "Shift Right Logical by Immediate" instruction performs a right shift of register `rs1` by
/// a constant `shamt` encoded in the lower 5 bits of the I-immediate field, shifting zeros into the upper bits,
/// and places the result in the register `rd`.<br/><br/>
/// Other shift instructions: [srl], [sra], [srai], [slli], [sll],
pub fn srli(rd: Register, rs1: Register, shamt: Uimm5) -> u32 {
    r_instruction(
        Opcode::OP_IMM,
        rd,
        Funct3::SRLI,
        rs1,
        RegOrUimm5::Uimm5(shamt),
        Funct7::SRL,
    )
}

/// "Shift Right Arithmetic by Immediate" instruction performs a left shift of register `rs1` by
/// a constant `shamt` encoded in the lower 5 bits of the I-immediate field, sign-extends the result
/// and places the result in the register `rd`.<br/><br/>
/// Other shift instructions: [sra], [srl], [srli], [sll], [slli]
pub fn srai(rd: Register, rs1: Register, shamt: Uimm5) -> u32 {
    r_instruction(
        Opcode::OP_IMM,
        rd,
        Funct3::SRAI,
        rs1,
        RegOrUimm5::Uimm5(shamt),
        Funct7::SRA,
    )
}

/// "ADD" instruction performs the addition of registers `rs1` and `rs2`
/// and places the result in the register `rd`. Overflows are ignored and the low XLEN bits of results are written
/// to the destination `rd`.<br/><br/>
/// Other arithmetic instructions:
/// - RV32I: [addi], [sub]
/// - M extension: [mul](super::mul), [mulh](super::mulh), [mulhsu](super::mulhsu), [mulhu](super::mulhu),
/// [div](super::div), [divu](super::divu), [rem](super::rem), [remu](super::remu)
pub fn add(rd: Register, rs1: Register, rs2: Register) -> u32 {
    r_instruction(
        Opcode::OP,
        rd,
        Funct3::ADD,
        rs1,
        RegOrUimm5::Register(rs2),
        Funct7::ADD,
    )
}

/// "SUBtract" instruction performs the subtraction of registers `rs1` and `rs2`
/// and places the result in the register `rd`. Overflows are ignored and the low XLEN bits of results are written
/// to the destination `rd`.<br/><br/>
/// Other arithmetic instructions:
/// - RV32I: [addi], [add]
/// - M extension: [mul](super::mul), [mulh](super::mulh), [mulhsu](super::mulhsu), [mulhu](super::mulhu),
/// [div](super::div), [divu](super::divu), [rem](super::rem), [remu](super::remu)
pub fn sub(rd: Register, rs1: Register, rs2: Register) -> u32 {
    r_instruction(
        Opcode::OP,
        rd,
        Funct3::SUB,
        rs1,
        RegOrUimm5::Register(rs2),
        Funct7::SUB,
    )
}

/// "Shift Logical Left" instruction performs logical left shift on the value in register `rs1` by the shift
/// amount held in the lower 5 bits of register `rs2` and places the result in the register `rd`.<br/><br/>
/// Other shift instructions: [slli], [srl], [sra], [srli], [srai]
pub fn sll(rd: Register, rs1: Register, rs2: Register) -> u32 {
    r_instruction(
        Opcode::OP,
        rd,
        Funct3::SLL,
        rs1,
        RegOrUimm5::Register(rs2),
        Funct7::SLL,
    )
}

/// "Shift Logical Right" instruction performs logical right shift on the value in register `rs1` by the shift
/// amount held in the lower 5 bits of register `rs2` and places the result in the register `rd`.<br/><br/>
/// Other shift instructions: [srli], [sra], [srai], [sll], [slli]
pub fn srl(rd: Register, rs1: Register, rs2: Register) -> u32 {
    r_instruction(
        Opcode::OP,
        rd,
        Funct3::SRL,
        rs1,
        RegOrUimm5::Register(rs2),
        Funct7::SRL,
    )
}

/// "Shift Arithmetic Right" instruction performs right shift on the value in register `rs1` by the shift amount
/// held in the lower 5 bits of register `rs2`, sign-extends the result and places the it in the register `rd`.<br/><br/>
/// Other shift instructions: [srai], [srl], [srli], [sll], [slli]
pub fn sra(rd: Register, rs1: Register, rs2: Register) -> u32 {
    r_instruction(
        Opcode::OP,
        rd,
        Funct3::SRA,
        rs1,
        RegOrUimm5::Register(rs2),
        Funct7::SRA,
    )
}

/// "Set Less Than" instruction performs signed compare,
/// writing 1 to the register `rd` if registers `rs1` < `rs2`, 0 otherwise.<br/><br/>
/// Other comparison instructions: [sltu], [slti], [sltiu], [seqz], [snez]
pub fn slt(rd: Register, rs1: Register, rs2: Register) -> u32 {
    r_instruction(
        Opcode::OP,
        rd,
        Funct3::SLT,
        rs1,
        RegOrUimm5::Register(rs2),
        Funct7::SLT,
    )
}

/// "Set Less Than Unsigned" instruction performs unsigned compare,
/// writing 1 to the register `rd` if registers `rs1` < `rs2`, 0 otherwise.
/// Note, `SLTU rd, x0, rs2` sets `rd` to 1 if `rs2` ≠ 0, otherwise sets `rd` to 0, and is equivalent to
/// pseudoinstruction [SNEZ](snez)&nbsp;`rd, rs`.<br/><br/>
/// Other comparison instructions: [slt], [slti], [sltiu], [seqz], [snez]
pub fn sltu(rd: Register, rs1: Register, rs2: Register) -> u32 {
    r_instruction(
        Opcode::OP,
        rd,
        Funct3::SLTU,
        rs1,
        RegOrUimm5::Register(rs2),
        Funct7::SLTU,
    )
}

/// "Set Not Equal to Zero" pseudoinstruction sets `rd` to 1 if `rs2` ≠ 0, otherwise sets `rd` to 0.<br/><br/>
/// `SNEZ rd, rs2` is encoded as [SLTU](sltu)&nbsp;`rd, x0, rs2`.<br/><br/>
/// Other comparison instructions: [slt], [sltu], [slti], [sltiu], [seqz]
pub fn snez(rd: Register, rs2: Register) -> u32 {
    sltu(rd, X0, rs2)
}

/// "XOR" instruction performs XOR logical operation on registers `rs1` and `rs2`
/// and places the result in the register `rd`.<br/><br/>
/// Other logical operations: [xori], [or], [ori], [and], [andi], [not]
pub fn xor(rd: Register, rs1: Register, rs2: Register) -> u32 {
    r_instruction(
        Opcode::OP,
        rd,
        Funct3::XOR,
        rs1,
        RegOrUimm5::Register(rs2),
        Funct7::XOR,
    )
}

/// "OR" instruction performs OR logical operation on registers `rs1` and `rs2`
/// and places the result in the register `rd`.<br/><br/>
/// Other logical operations: [ori], [xor], [xori], [and], [andi], [not]
pub fn or(rd: Register, rs1: Register, rs2: Register) -> u32 {
    r_instruction(
        Opcode::OP,
        rd,
        Funct3::OR,
        rs1,
        RegOrUimm5::Register(rs2),
        Funct7::OR,
    )
}

/// "AND" instruction performs AND logical operation on registers `rs1` and `rs2`
/// and places the result in the register `rd`.<br/><br/>
/// Other logical operations: [andi], [xor], [xori], [or], [ori], [not]
pub fn and(rd: Register, rs1: Register, rs2: Register) -> u32 {
    r_instruction(
        Opcode::OP,
        rd,
        Funct3::AND,
        rs1,
        RegOrUimm5::Register(rs2),
        Funct7::AND,
    )
}

/// "FENCE" instruction orders all memory operations in its `predecessor` set before all memory operations in
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
    fence_instruction(FenceMode::FENCE, predecessor, successor)
}

/// "FENCE.TCO" orders all load operations in its predecessor set before all memory operations in its successor set,
/// and all store operations in its predecessor set before all store operations in its successor set. This leaves
/// non-AMO store operations in the `FENCE.TSO`'s predecessor set unordered with non-AMO loads in its successor set.
/// <br/><br/>
/// `FENCE.TSO` instruction is encoded as a [FENCE](fence) instruction with `fm` = 1000
/// (refer to the instruction manual for this field), `predecessor` = "rw", and `successor` = "rw".
pub fn fence_tso() -> u32 {
    fence_instruction(FenceMode::FENCE_TSO, FenceMask::RW, FenceMask::RW)
}

/// FENCE-type instruction encoding.<br/>
/// ```text
/// Bit range     | 31:28 | 27 26 25 24 | 23 22 21 20 | 19:15 | 14:12  | 11:7 |  6:0     |
/// Field name    |  fm   | PI PO PR PW | SI SO SR SW |  rs1  | funct3 |  rd  | opcode   |
/// Bit count     |  4    | 1  1  1  1  | 1  1  1  1  |   5   |   3    |  5   |   7      |
/// Description   |  FM   | predecessor |  successor  |   0   | FENCE  |  0   | MISC_MEM |
/// ```
fn fence_instruction(fm: FenceMode, predecessor: FenceMask, successor: FenceMask) -> u32 {
    let mut imm = Imm12::ZERO;
    let imm_bits = imm.view_bits_mut();
    imm_bits[0..4].clone_from_bitslice(predecessor.view_bits());
    imm_bits[4..8].clone_from_bitslice(successor.view_bits());
    imm_bits[8..12].clone_from_bitslice(fm.view_bits());
    i_instruction(
        Opcode::MISC_MEM,
        X0,
        Funct3::FENCE,
        RegOrUimm5::Register(X0),
        imm,
    )
}

/// "Environment CALL" instruction is used to make a service request to the execution environment. The EEI
/// (execution environment interface) will define how parameters for the service request are passed,
/// but usually these will be in defined locations in the integer register file.<br/><br/>
/// Other environment instructions: [ebreak].
pub fn ecall() -> u32 {
    i_instruction(
        Opcode::SYSTEM,
        X0,
        Funct3::ECALL,
        RegOrUimm5::Register(X0),
        Imm12::ZERO,
    )
}

/// "Environment BREAK" instruction is used to return control to a debugging environment.
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
/// <br/>
/// <br/>
///
/// Other environment instructions: [ecall]
pub fn ebreak() -> u32 {
    i_instruction(
        Opcode::SYSTEM,
        X0,
        Funct3::EBREAK,
        RegOrUimm5::Register(X0),
        Imm12::ONE,
    )
}
