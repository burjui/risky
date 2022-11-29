//! Defines [Uimm5] and relevant trait implementations

use core::fmt;
use std::{
    error::Error,
    fmt::Display,
    ops::Range,
};

use bitvec::{
    order::Lsb0,
    view::BitView,
};

use crate::util::u8_max_value;

/// 5-bit unsigned immediate value
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uimm5(u32);

impl Uimm5 {
    const BIT_RANGE: Range<usize> = 0..5;

    pub(crate) fn view_bits(&self) -> &bitvec::slice::BitSlice<u32, Lsb0> {
        &self.0.view_bits()[Self::BIT_RANGE]
    }
}

impl Display for Uimm5 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[test]
fn display() -> Result<(), Uimm5ConvError> {
    assert_eq!(Uimm5::try_from(0b11111)?.to_string(), "31");
    Ok(())
}

impl TryFrom<u8> for Uimm5 {
    type Error = Uimm5ConvError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= u8_max_value(Self::BIT_RANGE.end) {
            Ok(Self(value as u32))
        } else {
            Err(Uimm5ConvError(value))
        }
    }
}

/// Uimm5 conversion error
#[derive(Debug)]
pub struct Uimm5ConvError(u8);

impl Display for Uimm5ConvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid 5-bit unsigned immediate: {} (0x{:02x})",
            self.0, self.0
        )
    }
}

impl Error for Uimm5ConvError {}
