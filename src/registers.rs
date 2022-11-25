/*!
RV32 Register definitions.

Based on the following document:
> ["RISC-V ABIs Specification, Document Version 1.0"](https://github.com/riscv-non-isa/riscv-elf-psabi-doc),
Editors Kito Cheng and Jessica Clarke, RISC-V International, November 2022.
*/

use std::fmt::{
    Display,
    Write,
};

use bitvec::{
    order::Lsb0,
    slice::BitSlice,
    view::BitView,
};

/// Number of `RISC-V` registers
pub const NUMBER_OF_REGISTERS: usize = 32;

/// Represents a `RISC-V` register
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Register(u32);

impl Register {
    /// Returns a register corresponding to `index`, if `index` < [NUMBER_OF_REGISTERS]
    pub const fn new(index: usize) -> Result<Self, &'static str> {
        if index < NUMBER_OF_REGISTERS {
            Ok(Self(index as u32))
        } else {
            Err("register index is greater than 31")
        }
    }

    pub(crate) fn view_bits(&self) -> &BitSlice<u32, Lsb0> {
        &self.0.view_bits()[0..5]
    }
}

impl TryFrom<u8> for Register {
    type Error = &'static str;

    fn try_from(index: u8) -> Result<Self, Self::Error> {
        Register::new(usize::from(index))
    }
}

impl TryFrom<usize> for Register {
    type Error = &'static str;

    fn try_from(index: usize) -> Result<Self, Self::Error> {
        Register::new(index)
    }
}

impl From<Register> for u8 {
    fn from(r: Register) -> Self {
        r.0 as u8
    }
}

impl From<Register> for usize {
    fn from(value: Register) -> Self {
        value.0 as usize // valid since NUMBER_OF_REGISTERS is small
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('x')?;
        Display::fmt(&self.0, f)
    }
}

/// Returns a register corresponding to `index`, if `r` < [NUMBER_OF_REGISTERS].
#[track_caller]
pub const fn x(index: usize) -> Result<Register, &'static str> {
    Register::new(index)
}

/// Hard-wired zero, ignores writes
pub const X0: Register = Register(0);
/// General-purpose register
/// General-purpose register 1
pub const X1: Register = Register(1);
/// General-purpose register 2
pub const X2: Register = Register(2);
/// General-purpose register 3
pub const X3: Register = Register(3);
/// General-purpose register 4
pub const X4: Register = Register(4);
/// General-purpose register 5
pub const X5: Register = Register(5);
/// General-purpose register 6
pub const X6: Register = Register(6);
/// General-purpose register 7
pub const X7: Register = Register(7);
/// General-purpose register 8
pub const X8: Register = Register(8);
/// General-purpose register 9
pub const X9: Register = Register(9);
/// General-purpose register 10
pub const X10: Register = Register(10);
/// General-purpose register 11
pub const X11: Register = Register(11);
/// General-purpose register 12
pub const X12: Register = Register(12);
/// General-purpose register 13
pub const X13: Register = Register(13);
/// General-purpose register 14
pub const X14: Register = Register(14);
/// General-purpose register 15
pub const X15: Register = Register(15);
/// General-purpose register 16
pub const X16: Register = Register(16);
/// General-purpose register 17
pub const X17: Register = Register(17);
/// General-purpose register 18
pub const X18: Register = Register(18);
/// General-purpose register 19
pub const X19: Register = Register(19);
/// General-purpose register 20
pub const X20: Register = Register(20);
/// General-purpose register 21
pub const X21: Register = Register(21);
/// General-purpose register 22
pub const X22: Register = Register(22);
/// General-purpose register 23
pub const X23: Register = Register(23);
/// General-purpose register 24
pub const X24: Register = Register(24);
/// General-purpose register 25
pub const X25: Register = Register(25);
/// General-purpose register 26
pub const X26: Register = Register(26);
/// General-purpose register 27
pub const X27: Register = Register(27);
/// General-purpose register 28
pub const X28: Register = Register(28);
/// General-purpose register 29
pub const X29: Register = Register(29);
/// General-purpose register 30
pub const X30: Register = Register(30);
/// General-purpose register 31
pub const X31: Register = Register(31);
