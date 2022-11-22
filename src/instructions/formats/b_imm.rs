use core::fmt;
use std::error::Error;
use std::fmt::Display;

use bitvec::order::Lsb0;
use bitvec::slice::BitSlice;
use bitvec::view::BitView;

/// 13-bit signed J-immediate used in the branch instructions
pub struct BImm(u32);

impl BImm {
    pub(crate) fn view_bits(&self) -> &BitSlice<u32, Lsb0> {
        &self.0.view_bits()[0..13]
    }
}

impl TryFrom<i16> for BImm {
    type Error = BImmConvError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if (-(1 << 13) - 1..(1 << 13)).contains(&value) {
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
            "invalid 13-bit signed immediate:{} 0x{:08x}",
            self.0, self.0
        )
    }
}

impl Error for BImmConvError {}
