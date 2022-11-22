use core::fmt;
use std::{error::Error, fmt::Display, ops::Range};

use bitvec::{order::Lsb0, slice::BitSlice, view::BitView};

/// 12-bit signed immediate value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl TryFrom<i16> for Imm12 {
    type Error = IImmConvError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if (-(1 << 12) - 1..1 << 12).contains(&value) {
            Ok(Self(value as u32))
        } else {
            Err(IImmConvError(value))
        }
    }
}

/// [Imm12] conversion error
#[derive(Debug)]
pub struct IImmConvError(i16);

impl Display for IImmConvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid 12-bit signed immediate: {} (0x{:04x})",
            self.0, self.0
        )
    }
}

impl Error for IImmConvError {}
