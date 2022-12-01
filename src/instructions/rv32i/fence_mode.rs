pub(crate) struct FenceMode(u8);

impl FenceMode {
    pub(crate) const FENCE: Self = Self(0b0000);
    pub(crate) const FENCE_TSO: Self = Self(0b1000);

    pub(crate) const fn into_u32(self) -> u32 {
        self.0 as u32
    }
}
