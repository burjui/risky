//! Defines Funct3

/// 3-bit function field
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Funct3(pub(crate) u8);

impl Funct3 {
    // RV32I
    ///
    pub const JALR: Self = Self(0b000);
    ///
    pub const BEQ: Self = Self(0b000);
    ///
    pub const BNE: Self = Self(0b001);
    ///
    pub const BLT: Self = Self(0b100);
    ///
    pub const BGE: Self = Self(0b101);
    ///
    pub const BLTU: Self = Self(0b110);
    ///
    pub const BGEU: Self = Self(0b111);
    ///
    pub const LB: Self = Self(0b000);
    ///
    pub const LH: Self = Self(0b001);
    ///
    pub const LW: Self = Self(0b010);
    ///
    pub const LBU: Self = Self(0b100);
    ///
    pub const LHU: Self = Self(0b101);
    ///
    pub const SB: Self = Self(0b000);
    ///
    pub const SH: Self = Self(0b001);
    ///
    pub const SW: Self = Self(0b010);
    ///
    pub const ADDI: Self = Self(0b000);
    ///
    pub const SLTI: Self = Self(0b010);
    ///
    pub const SLTIU: Self = Self(0b011);
    ///
    pub const XORI: Self = Self(0b100);
    ///
    pub const ORI: Self = Self(0b110);
    ///
    pub const ANDI: Self = Self(0b111);
    ///
    pub const SLLI: Self = Self(0b001);
    ///
    pub const ADD_SUB: Self = Self(0b000);
    ///
    pub const SLL: Self = Self(0b001);
    ///
    pub const SLT: Self = Self(0b010);
    ///
    pub const SLTU: Self = Self(0b011);
    ///
    pub const XOR: Self = Self(0b100);
    ///
    pub const SRL_SRA: Self = Self(0b101);
    ///
    pub const OR: Self = Self(0b110);
    ///
    pub const AND: Self = Self(0b111);
    ///
    pub const FENCE: Self = Self(0b000);
    ///
    pub const PRIV: Self = Self(0b000);

    // Zicsr standard extension
    ///
    pub const CSRRW: Self = Self(0b001);
    ///
    pub const CSRRS: Self = Self(0b010);
    ///
    pub const CSRRC: Self = Self(0b011);
    ///
    pub const CSRRWI: Self = Self(0b101);
    ///
    pub const CSRRSI: Self = Self(0b110);
    ///
    pub const CSRRCI: Self = Self(0b111);

    // M standard extension
    ///
    pub const MUL: Self = Self(0b000);
    ///
    pub const MULH: Self = Self(0b001);
    ///
    pub const MULHSU: Self = Self(0b010);
    ///
    pub const MULHU: Self = Self(0b011);
    ///
    pub const DIV: Self = Self(0b100);
    ///
    pub const DIVU: Self = Self(0b101);
    ///
    pub const REM: Self = Self(0b110);
    ///
    pub const REMU: Self = Self(0b111);

    pub(crate) const fn into_u32(self) -> u32 {
        self.0 as u32
    }
}

#[test]
fn into_u32() {
    assert_eq!(Funct3::PRIV.into_u32(), 0b000);
}
