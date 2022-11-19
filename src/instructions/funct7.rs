use bitvec::order::Lsb0;
use bitvec::{slice::BitSlice, view::BitView};

pub(crate) struct Funct7(u8);

impl Funct7 {
    // RV32I
    pub(crate) const ADD: Self = Funct7(0b0000000);
    pub(crate) const SLT: Self = Funct7(0b0000000);
    pub(crate) const SLTU: Self = Funct7(0b0000000);
    pub(crate) const AND: Self = Funct7(0b0000000);
    pub(crate) const OR: Self = Funct7(0b0000000);
    pub(crate) const XOR: Self = Funct7(0b0000000);
    pub(crate) const SUB: Self = Funct7(0b0100000);
    pub(crate) const SRA: Self = Funct7(0b0100000);
    pub(crate) const SLL: Self = Funct7(0b0000000);
    pub(crate) const SRL: Self = Funct7(0b0000000);
    pub(crate) const MULDIV: Self = Funct7(0b0000001);

    pub(crate) fn view_bits(&self) -> &BitSlice<u8, Lsb0> {
        &self.0.view_bits()[0..7]
    }
}
