//! Defines [BImm] and relevant trait implementations

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

use crate::util::{
    i16_fits_n_bits,
    i32_fits_n_bits,
    i64_fits_n_bits,
};

/// 13-bit signed immediate value used in branch instructions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BImm(u32);

impl BImm {
    const BIT_RANGE: Range<usize> = 0..13;

    pub(crate) fn view_bits(&self) -> &BitSlice<u32, Lsb0> {
        &self.0.view_bits()[Self::BIT_RANGE]
    }
}

impl Display for BImm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&(self.0 as i16), f)
    }
}

#[test]
fn display() -> Result<(), BImmConvError> {
    assert_eq!(BImm::try_from(-600)?.to_string(), "-600");
    Ok(())
}

impl From<i8> for BImm {
    fn from(value: i8) -> Self {
        Self(value as u32)
    }
}

impl TryFrom<i16> for BImm {
    type Error = BImmConvError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if i16_fits_n_bits(value, Self::BIT_RANGE.end) {
            Ok(Self(value as u32))
        } else {
            Err(BImmConvError::I16(value))
        }
    }
}

impl TryFrom<i32> for BImm {
    type Error = BImmConvError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if i32_fits_n_bits(value, Self::BIT_RANGE.end) {
            Ok(Self(value as u32))
        } else {
            Err(BImmConvError::I32(value))
        }
    }
}

impl TryFrom<i64> for BImm {
    type Error = BImmConvError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i64_fits_n_bits(value, Self::BIT_RANGE.end) {
            Ok(Self(value as u32))
        } else {
            Err(BImmConvError::I64(value))
        }
    }
}

/// [BImm] conversion error
#[derive(Debug)]
pub enum BImmConvError {
    ///
    I16(i16),
    ///
    I32(i32),
    ///
    I64(i64),
}

impl Display for BImmConvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid 13-bit signed immediate: ")?;
        match self {
            BImmConvError::I16(value) => write!(f, "{} 0x{:04x}", value, value),
            BImmConvError::I32(value) => write!(f, "{} 0x{:08x}", value, value),
            BImmConvError::I64(value) => write!(f, "{} 0x{:016x}", value, value),
        }
    }
}

impl Error for BImmConvError {}
