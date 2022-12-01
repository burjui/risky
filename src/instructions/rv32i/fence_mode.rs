pub(crate) struct FenceMode(pub(crate) u32);

impl FenceMode {
    pub(crate) const FENCE: Self = Self(0b0000);
    pub(crate) const FENCE_TSO: Self = Self(0b1000);
}
