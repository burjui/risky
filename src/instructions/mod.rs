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
pub mod zicsr_ext;

pub use m_ext::*;
pub use rv32i::*;
pub use zicsr_ext::*;

mod formats;
