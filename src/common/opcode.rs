use core::fmt;
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Opcode(pub(crate) u8);

impl Opcode {
    pub const LUI: Self = Self(0b011_0111);
    pub const AUIPC: Self = Self(0b001_0111);
    pub const JAL: Self = Self(0b110_1111);
    pub const JALR: Self = Self(0b110_0111);
    pub const BRANCH: Self = Self(0b110_0011);
    pub const LOAD: Self = Self(0b000_0011);
    pub const STORE: Self = Self(0b010_0011);
    pub const OP_IMM: Self = Self(0b001_0011);
    pub const OP: Self = Self(0b011_0011);
    pub const MISC_MEM: Self = Self(0b000_1111);
    pub const SYSTEM: Self = Self(0b111_0011);

    pub(crate) const fn into_u32(self) -> u32 {
        self.0 as u32
    }
}

#[test]
fn into_u32() {
    assert_eq!(Opcode::SYSTEM.into_u32(), 0b111_0011);
}

impl Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Opcode(0b{:07b})", self.0)
    }
}

#[test]
fn debug() {
    assert_eq!(format!("{:?}", Opcode::BRANCH), "Opcode(0b1100011)")
}

impl Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0b{:07b}", self.0)
    }
}

#[test]
fn display() {
    assert_eq!(Opcode::BRANCH.to_string(), "0b1100011")
}
