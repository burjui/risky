use core::fmt;
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Funct7(pub(crate) u8);

impl Funct7 {
    // RV32I
    pub(crate) const ADD: Self = Self(0b000_0000);
    pub(crate) const SLT: Self = Self(0b000_0000);
    pub(crate) const SLTU: Self = Self(0b000_0000);
    pub(crate) const AND: Self = Self(0b000_0000);
    pub(crate) const OR: Self = Self(0b000_0000);
    pub(crate) const XOR: Self = Self(0b000_0000);
    pub(crate) const SUB: Self = Self(0b010_0000);
    pub(crate) const SRA: Self = Self(0b010_0000);
    pub(crate) const SLL: Self = Self(0b000_0000);
    pub(crate) const SRL: Self = Self(0b000_0000);

    // M standard extension
    pub(crate) const MULDIV: Self = Self(0b000_0001);

    pub(crate) const fn into_u32(self) -> u32 {
        self.0 as u32
    }
}

#[test]
fn into_u32() {
    assert_eq!(Funct7::ADD.into_u32(), 0b000_0000);
}

impl Debug for Funct7 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Funct7(0b{:07b})", self.0)
    }
}

#[test]
fn debug() {
    assert_eq!(format!("{:?}", Funct7::SRA), "Funct7(0b0100000)")
}

impl Display for Funct7 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0b{:07b}", self.0)
    }
}

#[test]
fn display() {
    assert_eq!(Funct7::SRA.to_string(), "0b0100000")
}
