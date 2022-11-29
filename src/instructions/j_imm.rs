//! Defines [JImm] and relevant trait implementations

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

use crate::util::i32_fits_n_bits;

/// 21-bit signed immediate value used in the [crate::instructions::rv32i::jal] instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JImm(u32);

impl JImm {
    const BIT_RANGE: Range<usize> = 0..21;

    pub(crate) fn view_bits(&self) -> &BitSlice<u32, Lsb0> {
        &self.0.view_bits()[Self::BIT_RANGE]
    }
}

impl Display for JImm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&(self.0 as i32), f)
    }
}

#[test]
fn display() -> Result<(), JImmConvError> {
    assert_eq!(JImm::try_from(-600)?.to_string(), "-600");
    Ok(())
}

impl TryFrom<i32> for JImm {
    type Error = JImmConvError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if i32_fits_n_bits(value, Self::BIT_RANGE.end) {
            Ok(Self(value as u32))
        } else {
            Err(JImmConvError(value))
        }
    }
}

/// [JImm] conversion error
#[derive(Debug)]
pub struct JImmConvError(i32);

impl Display for JImmConvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid 21-bit signed immediate: {} 0x{:08x}",
            self.0, self.0
        )
    }
}

impl Error for JImmConvError {}
