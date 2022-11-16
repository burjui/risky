use bitvec::order::Lsb0;
use bitvec::{slice::BitSlice, view::BitView};
use std::{error::Error, fmt::Display};

pub(crate) struct Funct7(u8);

impl Funct7 {
    // RV32I
    pub(crate) const ADD: Funct7 = Funct7(0b0000000);
    pub(crate) const SLT: Funct7 = Funct7(0b0000000);
    pub(crate) const SLTU: Funct7 = Funct7(0b0000000);
    pub(crate) const AND: Funct7 = Funct7(0b0000000);
    pub(crate) const OR: Funct7 = Funct7(0b0000000);
    pub(crate) const XOR: Funct7 = Funct7(0b0000000);
    pub(crate) const SUB: Funct7 = Funct7(0b0100000);
    pub(crate) const SRA: Funct7 = Funct7(0b0100000);
    pub(crate) const SLL: Funct7 = Funct7(0b0000000);
    pub(crate) const SRL: Funct7 = Funct7(0b0000000);
    pub(crate) const MULDIV: Funct7 = Funct7(0b0000001);

    pub(crate) fn view_bits(&self) -> &BitSlice<u8, Lsb0> {
        &self.0.view_bits()[0..7]
    }
}

#[derive(Debug)]
pub(crate) struct Funct7Error(u8);

impl Display for Funct7Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid funct7 value, must be 7 bits wide: {} (0b{:08b})",
            self.0, self.0
        )
    }
}

impl Error for Funct7Error {}

impl TryFrom<u8> for Funct7 {
    type Error = Funct7Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 0b1111111 {
            Ok(Funct7(value))
        } else {
            Err(Funct7Error(value))
        }
    }
}
