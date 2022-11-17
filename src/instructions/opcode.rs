use bitvec::order::Lsb0;
use bitvec::{slice::BitSlice, view::BitView};

pub(crate) struct Opcode(u8);

impl Opcode {
    pub(crate) const LUI: Opcode = Opcode(0b0110111);
    pub(crate) const AUIPC: Opcode = Opcode(0b0010111);
    pub(crate) const JAL: Opcode = Opcode(0b1101111);
    pub(crate) const JALR: Opcode = Opcode(0b1100111);
    pub(crate) const BRANCH: Opcode = Opcode(0b1100011);
    pub(crate) const LOAD: Opcode = Opcode(0b0000011);
    pub(crate) const STORE: Opcode = Opcode(0b0100011);
    pub(crate) const OP_IMM: Opcode = Opcode(0b0010011);
    pub(crate) const OP: Opcode = Opcode(0b0110011);
    pub(crate) const MISC_MEM: Opcode = Opcode(0b0001111);

    // M extension
    pub(crate) const SYSTEM: Opcode = Opcode(0b1110011);
}

impl Opcode {
    pub(crate) fn view_bits(&self) -> &BitSlice<u8, Lsb0> {
        &self.0.view_bits()[0..7]
    }
}
