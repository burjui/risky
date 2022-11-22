use core::fmt;
use std::fmt::Display;

use bitvec::{order::Lsb0, view::BitView};

/// 5-bit unsigned immediate value
pub struct Uimm5(u32);

impl Uimm5 {
    pub(crate) fn view_bits(&self) -> &bitvec::slice::BitSlice<u32, Lsb0> {
        &self.0.view_bits()[0..5]
    }
}

impl TryFrom<u8> for Uimm5 {
    type Error = Uimm5ConvError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 0b11111 {
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
