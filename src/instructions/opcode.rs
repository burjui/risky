use bitvec::order::Lsb0;
use bitvec::{slice::BitSlice, view::BitView};

pub(crate) struct Opcode(u8);

impl Opcode {
    pub(crate) const LUI: Self = Opcode(0b0110111);
    pub(crate) const AUIPC: Self = Opcode(0b0010111);
    pub(crate) const JAL: Self = Opcode(0b1101111);
    pub(crate) const JALR: Self = Opcode(0b1100111);
    pub(crate) const BRANCH: Self = Opcode(0b1100011);
    pub(crate) const LOAD: Self = Opcode(0b0000011);
    pub(crate) const STORE: Self = Opcode(0b0100011);
    pub(crate) const OP_IMM: Self = Opcode(0b0010011);
    pub(crate) const OP: Self = Opcode(0b0110011);
    pub(crate) const MISC_MEM: Self = Opcode(0b0001111);

    // M extension
    pub(crate) const SYSTEM: Self = Opcode(0b1110011);
}

impl Opcode {
    pub(crate) fn view_bits(&self) -> &BitSlice<u8, Lsb0> {
        &self.0.view_bits()[0..7]
    }
}
