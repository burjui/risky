use core::fmt;
use std::error::Error;
use std::fmt::Display;

use bitvec::order::Lsb0;
use bitvec::slice::BitSlice;
use bitvec::view::BitView;

/// 20-bit signed J-immediate used in the [JAL](crate::instructions::rv32i::jal) instruction
pub struct JImm(u32);

impl JImm {
    pub(crate) fn view_bits(&self) -> &BitSlice<u32, Lsb0> {
        &self.0.view_bits()[0..21]
    }
}

impl TryFrom<i32> for JImm {
    type Error = JImmConvError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if (-(1 << 20) - 1..(1 << 20)).contains(&value) {
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
            "invalid 20-bit signed immediate:{} 0x{:08x}",
            self.0, self.0
        )
    }
}

impl Error for JImmConvError {}
