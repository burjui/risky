//! Defines [`RegOrUimm5`]
use super::uimm5::Uimm5;
use crate::registers::Register;

/// Register or a 5-bit unsigned immediate used in S instruction format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegOrUimm5 {
    ///
    Register(Register),
    ///
    Uimm5(Uimm5),
}

impl RegOrUimm5 {
    pub(crate) const fn into_u32(self) -> u32 {
        match self {
            RegOrUimm5::Register(reg) => reg.into_u32(),
            RegOrUimm5::Uimm5(imm) => imm.into_u32(),
        }
    }
}

#[test]
fn into_u32() {
    assert_eq!(RegOrUimm5::Register(crate::registers::X31).into_u32(), 31);
    assert_eq!(RegOrUimm5::Uimm5(31.try_into().unwrap()).into_u32(), 31);
}
