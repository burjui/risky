use bitvec::order::Lsb0;
use bitvec::{slice::BitSlice, view::BitView};

pub(crate) struct Opcode(u32);

impl Opcode {
    pub(crate) const LUI: Self = Self(0b0110111);
    pub(crate) const AUIPC: Self = Self(0b0010111);
    pub(crate) const JAL: Self = Self(0b1101111);
    pub(crate) const JALR: Self = Self(0b1100111);
    pub(crate) const BRANCH: Self = Self(0b1100011);
    pub(crate) const LOAD: Self = Self(0b0000011);
    pub(crate) const STORE: Self = Self(0b0100011);
    pub(crate) const OP_IMM: Self = Self(0b0010011);
    pub(crate) const OP: Self = Self(0b0110011);
    pub(crate) const MISC_MEM: Self = Self(0b0001111);

    // M extension
    pub(crate) const SYSTEM: Self = Self(0b1110011);
}

impl Opcode {
    pub(crate) fn view_bits(&self) -> &BitSlice<u32, Lsb0> {
        &self.0.view_bits()[0..7]
    }
}
