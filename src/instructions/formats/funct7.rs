pub(crate) struct Funct7(u8);

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

    pub(crate) const fn to_u32(&self) -> u32 {
        self.0 as u32
    }
}
