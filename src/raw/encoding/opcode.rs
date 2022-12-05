pub(crate) const LUI: u8 = 0b011_0111;
pub(crate) const AUIPC: u8 = 0b001_0111;
pub(crate) const JAL: u8 = 0b110_1111;
pub(crate) const JALR: u8 = 0b110_0111;
pub(crate) const BRANCH: u8 = 0b110_0011;
pub(crate) const LOAD: u8 = 0b000_0011;
pub(crate) const STORE: u8 = 0b010_0011;
pub(crate) const OP_IMM: u8 = 0b001_0011;
pub(crate) const OP: u8 = 0b011_0011;
pub(crate) const MISC_MEM: u8 = 0b000_1111;

// M extension
pub(crate) const SYSTEM: u8 = 0b111_0011;
