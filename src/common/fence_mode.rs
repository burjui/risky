//! Defines [`FenceMode`]

/// "Fence" instruction mode
///
/// Relevant instructions:
/// [`fence`](crate::instructions::rv32i::fence), [`fence_tso`](crate::instructions::rv32i::fence_tso)
pub struct FenceMode(u8);

impl FenceMode {
    /// [`fence`](crate::instructions::rv32i::fence) mode
    pub const FENCE: Self = Self(0b0000);
    /// [`fence_tso`](crate::instructions::rv32i::fence_tso) mode
    pub const FENCE_TSO: Self = Self(0b1000);

    pub(crate) const fn into_u32(self) -> u32 {
        self.0 as u32
    }
}

#[test]
fn into_u32() {
    assert_eq!(FenceMode::FENCE_TSO.into_u32(), 0b1000);
}
