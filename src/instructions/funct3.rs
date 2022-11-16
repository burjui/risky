use std::{error::Error, fmt::Display};

use bitvec::order::Lsb0;
use bitvec::slice::BitSlice;
use bitvec::view::BitView;

pub(crate) struct Funct3(u8);

impl Funct3 {
    // RV32I
    pub(crate) const JALR: Funct3 = Funct3(0b000);
    pub(crate) const BEQ: Funct3 = Funct3(0b000);
    pub(crate) const BNE: Funct3 = Funct3(0b001);
    pub(crate) const BLT: Funct3 = Funct3(0b100);
    pub(crate) const BGE: Funct3 = Funct3(0b101);
    pub(crate) const BLTU: Funct3 = Funct3(0b110);
    pub(crate) const BGEU: Funct3 = Funct3(0b111);
    pub(crate) const LB: Funct3 = Funct3(0b000);
    pub(crate) const LH: Funct3 = Funct3(0b001);
    pub(crate) const LW: Funct3 = Funct3(0b010);
    pub(crate) const LBU: Funct3 = Funct3(0b100);
    pub(crate) const LHU: Funct3 = Funct3(0b101);
    pub(crate) const SB: Funct3 = Funct3(0b000);
    pub(crate) const SH: Funct3 = Funct3(0b001);
    pub(crate) const SW: Funct3 = Funct3(0b010);
    pub(crate) const ADDI: Funct3 = Funct3(0b000);
    pub(crate) const SLTI: Funct3 = Funct3(0b010);
    pub(crate) const SLTIU: Funct3 = Funct3(0b011);
    pub(crate) const XORI: Funct3 = Funct3(0b100);
    pub(crate) const ORI: Funct3 = Funct3(0b110);
    pub(crate) const ANDI: Funct3 = Funct3(0b111);
    pub(crate) const SLLI: Funct3 = Funct3(0b001);
    pub(crate) const SRLI: Funct3 = Funct3(0b101);
    pub(crate) const SRAI: Funct3 = Funct3(0b101);
    pub(crate) const ADD: Funct3 = Funct3(0b000);
    pub(crate) const SUB: Funct3 = Funct3(0b000);
    pub(crate) const SLL: Funct3 = Funct3(0b001);
    pub(crate) const SLT: Funct3 = Funct3(0b010);
    pub(crate) const SLTU: Funct3 = Funct3(0b011);
    pub(crate) const XOR: Funct3 = Funct3(0b100);
    pub(crate) const SRL: Funct3 = Funct3(0b101);
    pub(crate) const SRA: Funct3 = Funct3(0b101);
    pub(crate) const OR: Funct3 = Funct3(0b110);
    pub(crate) const AND: Funct3 = Funct3(0b111);
    pub(crate) const FENCE: Funct3 = Funct3(0b000);
    pub(crate) const ECALL: Funct3 = Funct3(0b000);
    pub(crate) const EBREAK: Funct3 = Funct3(0b000);

    // Zicsr standard extension
    pub(crate) const CSRRW: Funct3 = Funct3(0b001);
    pub(crate) const CSRRS: Funct3 = Funct3(0b010);
    pub(crate) const CSRRC: Funct3 = Funct3(0b011);
    pub(crate) const CSRRWI: Funct3 = Funct3(0b101);
    pub(crate) const CSRRSI: Funct3 = Funct3(0b110);
    pub(crate) const CSRRCI: Funct3 = Funct3(0b111);

    // M standard extension
    pub(crate) const MUL: Funct3 = Funct3(0b000);
    pub(crate) const MULH: Funct3 = Funct3(0b001);
    pub(crate) const MULHSU: Funct3 = Funct3(0b010);
    pub(crate) const MULHU: Funct3 = Funct3(0b011);
    pub(crate) const DIV: Funct3 = Funct3(0b100);
    pub(crate) const DIVU: Funct3 = Funct3(0b101);
    pub(crate) const REM: Funct3 = Funct3(0b110);
    pub(crate) const REMU: Funct3 = Funct3(0b111);

    pub(crate) fn view_bits(&self) -> &BitSlice<u8, Lsb0> {
        &self.0.view_bits()[0..3]
    }
}

#[derive(Debug)]
pub(crate) struct Funct3Error(u8);

impl Display for Funct3Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid funct3 value, must be 3 bits wide: {} (0b{:08b})",
            self.0, self.0
        )
    }
}

impl Error for Funct3Error {}

impl TryFrom<u8> for Funct3 {
    type Error = Funct3Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 0b111 {
            Ok(Funct3(value))
        } else {
            Err(Funct3Error(value))
        }
    }
}
