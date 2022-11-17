use bitvec::order::Lsb0;
use bitvec::{slice::BitSlice, view::BitView};

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
