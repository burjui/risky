use core::fmt;
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FenceMode(pub(crate) u8);

impl FenceMode {
    pub(crate) const FENCE: Self = Self(0b0000);
    pub(crate) const FENCE_TSO: Self = Self(0b1000);

    pub(crate) const fn into_u32(self) -> u32 {
        self.0 as u32
    }
}

#[test]
fn into_u32() {
    assert_eq!(FenceMode::FENCE_TSO.into_u32(), 0b1000);
}

impl Debug for FenceMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FenceMode(0b{:04b})", self.0)
    }
}

#[test]
fn debug() {
    assert_eq!(format!("{:?}", FenceMode::FENCE_TSO), "FenceMode(0b1000)")
}

impl Display for FenceMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0b{:04b}", self.0)
    }
}

#[test]
fn display() {
    assert_eq!(FenceMode::FENCE_TSO.to_string(), "0b1000")
}
