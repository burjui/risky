//! RV32I base instruction set

mod fence_mask;
mod fence_mode;

use super::formats::funct3::Funct3;
use super::formats::funct7::Funct7;
pub use super::formats::j_imm::*;
use super::formats::opcode::Opcode;
use super::formats::{
    b_instruction, i_instruction, j_instruction, r_instruction, s_instruction, u_instruction,
    ITypeRs1,
};
use crate::registers::*;
use bitvec::order::Lsb0;
use bitvec::view::BitView;
pub use fence_mask::*;
use fence_mode::FenceMode;

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
/// `JAL` (jump and link) instruction uses the J-type format, where `imm` encodes a 20-bit signed offset, with the
/// lowest bit ignored. The offset is sign-extended and added to the address of the jump instruction to form the jump
/// target address. Jumps can therefore target a ±1&nbsp;MiB range. `JAL` stores the address of the instruction that
/// follows the `JAL` (pc+4) into the register `rd`. The standard software calling convention uses [X1] as the return
/// address register and x5 as an alternate link register.
pub fn jal(rd: Register, imm: JImm) -> u32 {
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
    fence_instruction(FenceMode::FENCE, predecessor, successor)
}

/// *(RV32I, I-format specialized)*<br/>
/// `FENCE.TCO` orders all load operations in its predecessor set before all memory operations in its successor set,
/// and all store operations in its predecessor set before all store operations in its successor set. This leaves
/// non-AMO store operations in the `FENCE.TSO`'s predecessor set unordered with non-AMO loads in its successor set.
/// <br/><br/>
/// `FENCE.TSO` instruction is encoded as a [FENCE](fence) instruction with `fm` = 1000
/// (refer to the instruction manual for this field), `predecessor` = "rw", and `successor` = "rw".
pub fn fence_tso() -> u32 {
    fence_instruction(FenceMode::FENCE_TSO, FenceMask::RW, FenceMask::RW)
}

/// *(RV32I, I-format specialized)*<br/>
/// FENCE-type instruction encoding.<br/>
/// ```text
/// Bit range     | 31:28 | 27 26 25 24 | 23 22 21 20 | 19:15 | 14:12  | 11:7 |  6:0     |
/// Field name    |  fm   | PI PO PR PW | SI SO SR SW |  rs1  | funct3 |  rd  | opcode   |
/// Bit count     |  4    | 1  1  1  1  | 1  1  1  1  |   5   |   3    |  5   |   7      |
/// Description   |  FM   | predecessor |  successor  |   0   | FENCE  |  0   | MISC_MEM |
/// ```
fn fence_instruction(fm: FenceMode, predecessor: FenceMask, successor: FenceMask) -> u32 {
    let mut imm = 0u16;
    let imm_bits = imm.view_bits_mut::<Lsb0>();
    imm_bits[0..4].clone_from_bitslice(predecessor.view_bits());
    imm_bits[4..8].clone_from_bitslice(successor.view_bits());
    imm_bits[8..12].clone_from_bitslice(fm.view_bits());
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
