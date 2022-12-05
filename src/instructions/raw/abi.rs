/*!
ABI-related register definitions.

Based on the following official documents:

> ["RISC-V ABIs Specification, Document Version 1.0"](https://github.com/riscv-non-isa/riscv-elf-psabi-doc),
Editors Kito Cheng and Jessica Clarke, RISC-V International, November 2022.

> ["RISC-V Assembly Programmer’s Manual"](https://github.com/riscv-non-isa/riscv-asm-manual),
Editors Palmer Dabbelt <palmer@dabbelt.com>, Michael Clark <michaeljclark@mac.com> and Alex Bradbury <asb@lowrisc.org>, 2017.
*/

use crate::instructions::raw::registers::{
    X0, X1, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X2, X20, X21, X22, X23, X24, X25,
    X26, X27, X28, X29, X3, X30, X31, X4, X5, X6, X7, X8, X9,
};

/// Hard-wired zero, ignores writes ([X0])
pub const ZERO: u8 = X0;
/// Return address for jumps ([X1])
pub const RA: u8 = X1;
/// Stack pointer ([X2])
pub const SP: u8 = X2;
/// Global pointer ([X3])
pub const GP: u8 = X3;
/// Thread pointer ([X4])
pub const TP: u8 = X4;
/// Frame pointer or saved register ([X8], [S0])
pub const FP: u8 = X8;

/// Temporary register 0 ([X5])
pub const T0: u8 = X5;
/// Temporary register 1 ([X6])
pub const T1: u8 = X6;
/// Temporary register 2 ([X7])
pub const T2: u8 = X7;
/// Temporary register 3 ([X28])
pub const T3: u8 = X28;
/// Temporary register 4 ([X29])
pub const T4: u8 = X29;
/// Temporary register 5 ([X30])
pub const T5: u8 = X30;
/// Temporary register 6 ([X31])
pub const T6: u8 = X31;

/// Return value or function argument 0 ([X10])
pub const A0: u8 = X10;
/// Return value or function argument 1 ([X11])
pub const A1: u8 = X11;

/// Function argument 2 ([X12])
pub const A2: u8 = X12;
/// Function argument 3 ([X13])
pub const A3: u8 = X13;
/// Function argument 4 ([X14])
pub const A4: u8 = X14;
/// Function argument 5 ([X15])
pub const A5: u8 = X15;
/// Function argument 6 ([X16])
pub const A6: u8 = X16;
/// Function argument 7 ([X17])
pub const A7: u8 = X17;

/// Saved register 0 or frame pointer ([X8], [FP])
pub const S0: u8 = X8;
/// Saved register 1 ([X9])
pub const S1: u8 = X9;
/// Saved register 2 ([X18])
pub const S2: u8 = X18;
/// Saved register 3 ([X19])
pub const S3: u8 = X19;
/// Saved register 4 ([X20])
pub const S4: u8 = X20;
/// Saved register 5 ([X21])
pub const S5: u8 = X21;
/// Saved register 6 ([X22])
pub const S6: u8 = X22;
/// Saved register 7 ([X23])
pub const S7: u8 = X23;
/// Saved register 8 ([X24])
pub const S8: u8 = X24;
/// Saved register 9 ([X25])
pub const S9: u8 = X25;
/// Saved register 10 ([X26])
pub const S10: u8 = X26;
/// Saved register 11 ([X27])
pub const S11: u8 = X27;
