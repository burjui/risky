/*!
- RV32I base instruction set
- Zicsr standard extension
- M standard extension

Based on the following document:
> ["The RISC-V Instruction Set Manual, Volume I: User-Level ISA,
Document Version 20191214-draft"](https://github.com/riscv/riscv-isa-manual),
Editors Andrew Waterman and Krste Asanović, RISC-V International, December 2019.
*/

pub mod m_ext;
pub mod rv32i;
pub mod ziscr_ext;

mod csr_mask;
mod fence_mask;
mod fence_mode;
mod formats;
mod funct3;
mod funct7;
mod opcode;
