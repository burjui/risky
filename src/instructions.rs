/*!
- RV32I base instruction set
- Zicsr standard extension
- M standard extension

Based on the following document:
> ["The RISC-V Instruction Set Manual, Volume I: User-Level ISA,
Document Version 20191214-draft"](https://github.com/riscv/riscv-isa-manual),
Editors Andrew Waterman and Krste Asanović, RISC-V International, December 2019.
*/

pub mod b_imm;
pub mod imm12;
pub mod j_imm;
pub mod m_ext;
pub mod rv32i;
pub mod uimm5;
pub mod zicsr_ext;

pub use b_imm::*;
pub use imm12::*;
pub use j_imm::*;
pub use m_ext::*;
pub use rv32i::*;
pub use uimm5::*;
pub use zicsr_ext::*;

mod formats;
