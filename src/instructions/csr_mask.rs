use bitvec::order::Lsb0;
use bitvec::{slice::BitSlice, view::BitView};
use std::{error::Error, fmt::Display};

pub struct CsrMask(u8);

impl CsrMask {
    pub(crate) fn view_bits(&self) -> &BitSlice<u8, Lsb0> {
        &self.0.view_bits()[0..5]
    }
}

#[derive(Debug)]
pub struct CsrMaskError(u8);

impl Display for CsrMaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid uimm5 value, must be 5 bits wide: {} (0b{:08b})",
            self.0, self.0
        )
    }
}

impl Error for CsrMaskError {}

impl TryFrom<u8> for CsrMask {
    type Error = CsrMaskError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 0b11111 {
            Ok(CsrMask(value))
        } else {
            Err(CsrMaskError(value))
        }
    }
}
