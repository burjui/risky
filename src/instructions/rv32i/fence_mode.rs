use bitvec::order::Lsb0;
use bitvec::view::BitView;

pub(crate) struct FenceMode(u8);

impl FenceMode {
    pub(crate) const FENCE: Self = Self(0b0000);
    pub(crate) const FENCE_TSO: Self = Self(0b1000);

    pub(crate) fn view_bits(&self) -> &bitvec::slice::BitSlice<u8, Lsb0> {
        &self.0.view_bits()[0..4]
    }
}
