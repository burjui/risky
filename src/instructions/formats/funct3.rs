use bitvec::order::Lsb0;
use bitvec::slice::BitSlice;
use bitvec::view::BitView;

pub(crate) struct Funct3(u32);

impl Funct3 {
    // RV32I
    pub(crate) const JALR: Self = Self(0b000);
    pub(crate) const BEQ: Self = Self(0b000);
    pub(crate) const BNE: Self = Self(0b001);
    pub(crate) const BLT: Self = Self(0b100);
    pub(crate) const BGE: Self = Self(0b101);
    pub(crate) const BLTU: Self = Self(0b110);
    pub(crate) const BGEU: Self = Self(0b111);
    pub(crate) const LB: Self = Self(0b000);
    pub(crate) const LH: Self = Self(0b001);
    pub(crate) const LW: Self = Self(0b010);
    pub(crate) const LBU: Self = Self(0b100);
    pub(crate) const LHU: Self = Self(0b101);
    pub(crate) const SB: Self = Self(0b000);
    pub(crate) const SH: Self = Self(0b001);
    pub(crate) const SW: Self = Self(0b010);
    pub(crate) const ADDI: Self = Self(0b000);
    pub(crate) const SLTI: Self = Self(0b010);
    pub(crate) const SLTIU: Self = Self(0b011);
    pub(crate) const XORI: Self = Self(0b100);
    pub(crate) const ORI: Self = Self(0b110);
    pub(crate) const ANDI: Self = Self(0b111);
    pub(crate) const SLLI: Self = Self(0b001);
    pub(crate) const SRLI: Self = Self(0b101);
    pub(crate) const SRAI: Self = Self(0b101);
    pub(crate) const ADD: Self = Self(0b000);
    pub(crate) const SUB: Self = Self(0b000);
    pub(crate) const SLL: Self = Self(0b001);
    pub(crate) const SLT: Self = Self(0b010);
    pub(crate) const SLTU: Self = Self(0b011);
    pub(crate) const XOR: Self = Self(0b100);
    pub(crate) const SRL: Self = Self(0b101);
    pub(crate) const SRA: Self = Self(0b101);
    pub(crate) const OR: Self = Self(0b110);
    pub(crate) const AND: Self = Self(0b111);
    pub(crate) const FENCE: Self = Self(0b000);
    pub(crate) const ECALL: Self = Self(0b000);
    pub(crate) const EBREAK: Self = Self(0b000);

    // Zicsr standard extension
    pub(crate) const CSRRW: Self = Self(0b001);
    pub(crate) const CSRRS: Self = Self(0b010);
    pub(crate) const CSRRC: Self = Self(0b011);
    pub(crate) const CSRRWI: Self = Self(0b101);
    pub(crate) const CSRRSI: Self = Self(0b110);
    pub(crate) const CSRRCI: Self = Self(0b111);

    // M standard extension
    pub(crate) const MUL: Self = Self(0b000);
    pub(crate) const MULH: Self = Self(0b001);
    pub(crate) const MULHSU: Self = Self(0b010);
    pub(crate) const MULHU: Self = Self(0b011);
    pub(crate) const DIV: Self = Self(0b100);
    pub(crate) const DIVU: Self = Self(0b101);
    pub(crate) const REM: Self = Self(0b110);
    pub(crate) const REMU: Self = Self(0b111);

    pub(crate) fn view_bits(&self) -> &BitSlice<u32, Lsb0> {
        &self.0.view_bits()[0..3]
    }
}
