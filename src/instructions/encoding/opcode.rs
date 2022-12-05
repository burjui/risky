pub(crate) struct Opcode(u8);

impl Opcode {
    pub(crate) const LUI: Self = Self(0b011_0111);
    pub(crate) const AUIPC: Self = Self(0b001_0111);
    pub(crate) const JAL: Self = Self(0b110_1111);
    pub(crate) const JALR: Self = Self(0b110_0111);
    pub(crate) const BRANCH: Self = Self(0b110_0011);
    pub(crate) const LOAD: Self = Self(0b000_0011);
    pub(crate) const STORE: Self = Self(0b010_0011);
    pub(crate) const OP_IMM: Self = Self(0b001_0011);
    pub(crate) const OP: Self = Self(0b011_0011);
    pub(crate) const MISC_MEM: Self = Self(0b000_1111);

    // M extension
    pub(crate) const SYSTEM: Self = Self(0b111_0011);

    pub(crate) const fn into_u32(self) -> u32 {
        self.0 as u32
    }
}

#[test]
fn into_u32() {
    assert_eq!(Opcode::SYSTEM.into_u32(), 0b111_0011);
}
