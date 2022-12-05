/*!
RV32 Register definitions.

Based on the following document:
> ["RISC-V ABIs Specification, Document Version 1.0"](https://github.com/riscv-non-isa/riscv-elf-psabi-doc),
Editors Kito Cheng and Jessica Clarke, RISC-V International, November 2022.
*/

/// Number of `RISC-V` registers
pub const NUMBER_OF_REGISTERS: usize = 32;

/// Hard-wired zero, ignores writes
pub const X0: u8 = 0;
/// General-purpose register 1
pub const X1: u8 = 1;
/// General-purpose register 2
pub const X2: u8 = 2;
/// General-purpose register 3
pub const X3: u8 = 3;
/// General-purpose register 4
pub const X4: u8 = 4;
/// General-purpose register 5
pub const X5: u8 = 5;
/// General-purpose register 6
pub const X6: u8 = 6;
/// General-purpose register 7
pub const X7: u8 = 7;
/// General-purpose register 8
pub const X8: u8 = 8;
/// General-purpose register 9
pub const X9: u8 = 9;
/// General-purpose register 10
pub const X10: u8 = 10;
/// General-purpose register 11
pub const X11: u8 = 11;
/// General-purpose register 12
pub const X12: u8 = 12;
/// General-purpose register 13
pub const X13: u8 = 13;
/// General-purpose register 14
pub const X14: u8 = 14;
/// General-purpose register 15
pub const X15: u8 = 15;
/// General-purpose register 16
pub const X16: u8 = 16;
/// General-purpose register 17
pub const X17: u8 = 17;
/// General-purpose register 18
pub const X18: u8 = 18;
/// General-purpose register 19
pub const X19: u8 = 19;
/// General-purpose register 20
pub const X20: u8 = 20;
/// General-purpose register 21
pub const X21: u8 = 21;
/// General-purpose register 22
pub const X22: u8 = 22;
/// General-purpose register 23
pub const X23: u8 = 23;
/// General-purpose register 24
pub const X24: u8 = 24;
/// General-purpose register 25
pub const X25: u8 = 25;
/// General-purpose register 26
pub const X26: u8 = 26;
/// General-purpose register 27
pub const X27: u8 = 27;
/// General-purpose register 28
pub const X28: u8 = 28;
/// General-purpose register 29
pub const X29: u8 = 29;
/// General-purpose register 30
pub const X30: u8 = 30;
/// General-purpose register 31
pub const X31: u8 = 31;
