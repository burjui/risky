use core::fmt;
use std::error::Error;
use std::fmt::{Display, Formatter};

use bitvec::order::Lsb0;
use bitvec::slice::BitSlice;
use bitvec::view::BitView;

/// Represents a 20-bit signed J-immediate used in the [JAL](crate::instructions::rv32i::jal) instruction.
pub struct JImm(u32);

impl JImm {
    pub(crate) fn view_bits(&self) -> &BitSlice<u32, Lsb0> {
        &self.0.view_bits()[0..21]
    }
}

/// [JImm] parse error
#[derive(Debug)]
pub struct JImmParseError(i32);

impl Display for JImmParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid J-immediate, must be 20 bits wide (lower bit is ignored): 0x{:08x}",
            self.0
        )
    }
}

impl Error for JImmParseError {}

impl TryFrom<i32> for JImm {
    type Error = JImmParseError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if (-(1 << 20) - 1..(1 << 20)).contains(&value) {
            Ok(Self(value as u32))
        } else {
            Err(JImmParseError(value))
        }
    }
}
