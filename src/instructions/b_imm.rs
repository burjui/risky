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

use crate::util::i16_value_range;

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

impl TryFrom<i16> for BImm {
    type Error = BImmConvError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if i16_value_range(Self::BIT_RANGE.end).contains(&value) {
            Ok(Self(value as u32))
        } else {
            Err(BImmConvError(value))
        }
    }
}

/// [BImm] conversion error
#[derive(Debug)]
pub struct BImmConvError(i16);

impl Display for BImmConvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid 13-bit signed immediate: {} 0x{:08x}",
            self.0, self.0
        )
    }
}

impl Error for BImmConvError {}
