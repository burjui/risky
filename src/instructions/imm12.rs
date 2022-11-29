//! Defines [Imm12] and relevant trait implementations

use core::fmt;
use std::{
    error::Error,
    fmt::Display,
    ops::Range,
};

use bitvec::{
    order::Lsb0,
    slice::BitSlice,
    view::BitView,
};

use crate::util::i16_value_range;

/// 12-bit signed immediate value
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Imm12(u32);

impl Imm12 {
    const BIT_RANGE: Range<usize> = 0..12;

    /// Zero
    pub const ZERO: Self = Self(0);
    /// One
    pub const ONE: Self = Self(1);

    pub(crate) const MINUS_ONE: Self = Self(-1i16 as u32);

    pub(crate) fn view_bits(&self) -> &BitSlice<u32, Lsb0> {
        &self.0.view_bits()[Self::BIT_RANGE]
    }

    pub(crate) fn view_bits_mut(&mut self) -> &mut BitSlice<u32, Lsb0> {
        &mut self.0.view_bits_mut()[Self::BIT_RANGE]
    }
}

impl Display for Imm12 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&(self.0 as i16), f)
    }
}

#[test]
fn display() -> Result<(), Imm12ConvError> {
    assert_eq!(Imm12::try_from(-600)?.to_string(), "-600");
    Ok(())
}

impl TryFrom<i16> for Imm12 {
    type Error = Imm12ConvError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if i16_value_range(Self::BIT_RANGE.end).contains(&value) {
            Ok(Self(value as u32))
        } else {
            Err(Imm12ConvError(value))
        }
    }
}

/// [Imm12] conversion error
#[derive(Debug)]
pub struct Imm12ConvError(i16);

impl Display for Imm12ConvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid 12-bit signed immediate: {} (0x{:04x})",
            self.0, self.0
        )
    }
}

impl Error for Imm12ConvError {}
