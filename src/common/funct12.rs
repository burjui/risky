use core::fmt;
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Funct12(pub(crate) u16);

impl Funct12 {
    // RV32I
    pub(crate) const ECALL: Self = Self(0);
    pub(crate) const EBREAK: Self = Self(1);
}

impl Debug for Funct12 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Funct12(0b{:012b})", self.0)
    }
}

#[test]
fn debug() {
    assert_eq!(format!("{:?}", Funct12::EBREAK), "Funct12(0b000000000001)")
}

impl Display for Funct12 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0b{:012b}", self.0)
    }
}

#[test]
fn display() {
    assert_eq!(Funct12::EBREAK.to_string(), "0b000000000001")
}
