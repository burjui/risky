use bitvec::{
    order::Lsb0,
    slice::BitSlice,
    view::BitView,
};

pub(crate) struct Funct7(u32);

impl Funct7 {
    // RV32I
    pub(crate) const ADD: Self = Self(0b0000000);
    pub(crate) const SLT: Self = Self(0b0000000);
    pub(crate) const SLTU: Self = Self(0b0000000);
    pub(crate) const AND: Self = Self(0b0000000);
    pub(crate) const OR: Self = Self(0b0000000);
    pub(crate) const XOR: Self = Self(0b0000000);
    pub(crate) const SUB: Self = Self(0b0100000);
    pub(crate) const SRA: Self = Self(0b0100000);
    pub(crate) const SLL: Self = Self(0b0000000);
    pub(crate) const SRL: Self = Self(0b0000000);
    pub(crate) const MULDIV: Self = Self(0b0000001);

    pub(crate) fn view_bits(&self) -> &BitSlice<u32, Lsb0> {
        &self.0.view_bits()[0..7]
    }
}
