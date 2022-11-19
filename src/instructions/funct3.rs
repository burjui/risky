use bitvec::order::Lsb0;
use bitvec::slice::BitSlice;
use bitvec::view::BitView;

pub(crate) struct Funct3(u8);

impl Funct3 {
    // RV32I
    pub(crate) const JALR: Self = Funct3(0b000);
    pub(crate) const BEQ: Self = Funct3(0b000);
    pub(crate) const BNE: Self = Funct3(0b001);
    pub(crate) const BLT: Self = Funct3(0b100);
    pub(crate) const BGE: Self = Funct3(0b101);
    pub(crate) const BLTU: Self = Funct3(0b110);
    pub(crate) const BGEU: Self = Funct3(0b111);
    pub(crate) const LB: Self = Funct3(0b000);
    pub(crate) const LH: Self = Funct3(0b001);
    pub(crate) const LW: Self = Funct3(0b010);
    pub(crate) const LBU: Self = Funct3(0b100);
    pub(crate) const LHU: Self = Funct3(0b101);
    pub(crate) const SB: Self = Funct3(0b000);
    pub(crate) const SH: Self = Funct3(0b001);
    pub(crate) const SW: Self = Funct3(0b010);
    pub(crate) const ADDI: Self = Funct3(0b000);
    pub(crate) const SLTI: Self = Funct3(0b010);
    pub(crate) const SLTIU: Self = Funct3(0b011);
    pub(crate) const XORI: Self = Funct3(0b100);
    pub(crate) const ORI: Self = Funct3(0b110);
    pub(crate) const ANDI: Self = Funct3(0b111);
    pub(crate) const SLLI: Self = Funct3(0b001);
    pub(crate) const SRLI: Self = Funct3(0b101);
    pub(crate) const SRAI: Self = Funct3(0b101);
    pub(crate) const ADD: Self = Funct3(0b000);
    pub(crate) const SUB: Self = Funct3(0b000);
    pub(crate) const SLL: Self = Funct3(0b001);
    pub(crate) const SLT: Self = Funct3(0b010);
    pub(crate) const SLTU: Self = Funct3(0b011);
    pub(crate) const XOR: Self = Funct3(0b100);
    pub(crate) const SRL: Self = Funct3(0b101);
    pub(crate) const SRA: Self = Funct3(0b101);
    pub(crate) const OR: Self = Funct3(0b110);
    pub(crate) const AND: Self = Funct3(0b111);
    pub(crate) const FENCE: Self = Funct3(0b000);
    pub(crate) const ECALL: Self = Funct3(0b000);
    pub(crate) const EBREAK: Self = Funct3(0b000);

    // Zicsr standard extension
    pub(crate) const CSRRW: Self = Funct3(0b001);
    pub(crate) const CSRRS: Self = Funct3(0b010);
    pub(crate) const CSRRC: Self = Funct3(0b011);
    pub(crate) const CSRRWI: Self = Funct3(0b101);
    pub(crate) const CSRRSI: Self = Funct3(0b110);
    pub(crate) const CSRRCI: Self = Funct3(0b111);

    // M standard extension
    pub(crate) const MUL: Self = Funct3(0b000);
    pub(crate) const MULH: Self = Funct3(0b001);
    pub(crate) const MULHSU: Self = Funct3(0b010);
    pub(crate) const MULHU: Self = Funct3(0b011);
    pub(crate) const DIV: Self = Funct3(0b100);
    pub(crate) const DIVU: Self = Funct3(0b101);
    pub(crate) const REM: Self = Funct3(0b110);
    pub(crate) const REMU: Self = Funct3(0b111);

    pub(crate) fn view_bits(&self) -> &BitSlice<u8, Lsb0> {
        &self.0.view_bits()[0..3]
    }
}
