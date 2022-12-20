//! Defines Funct7

/// 7-bit function field
pub struct Funct7(u8);

impl Funct7 {
    // RV32I
    ///
    pub const ADD: Self = Self(0b000_0000);
    ///
    pub const SLT: Self = Self(0b000_0000);
    ///
    pub const SLTU: Self = Self(0b000_0000);
    ///
    pub const AND: Self = Self(0b000_0000);
    ///
    pub const OR: Self = Self(0b000_0000);
    ///
    pub const XOR: Self = Self(0b000_0000);
    ///
    pub const SUB: Self = Self(0b010_0000);
    ///
    pub const SRA: Self = Self(0b010_0000);
    ///
    pub const SLL: Self = Self(0b000_0000);
    ///
    pub const SRL: Self = Self(0b000_0000);
    ///
    pub const MULDIV: Self = Self(0b000_0001);

    pub(crate) const fn into_u32(self) -> u32 {
        self.0 as u32
    }
}

#[test]
fn into_u32() {
    assert_eq!(Funct7::ADD.into_u32(), 0b000_0000);
}
