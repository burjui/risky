//! Defines [`FenceMode`]

/// "Fence" instruction mode
///
/// Relevant instructions:
/// [`fence`](crate::instructions::rv32i::fence), [`fence_tso`](crate::instructions::rv32i::fence_tso)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FenceMode(pub(crate) u8);

impl FenceMode {
    /// [`fence`](crate::instructions::rv32i::fence) mode
    pub const FENCE: Self = Self(0b0000);
    /// [`fence_tso`](crate::instructions::rv32i::fence_tso) mode
    pub const FENCE_TSO: Self = Self(0b1000);

    pub(crate) const fn into_u32(self) -> u32 {
        self.0 as u32
    }
}

impl From<FenceMode> for i8 {
    #[allow(clippy::cast_lossless, clippy::cast_possible_wrap)]
    fn from(value: FenceMode) -> Self {
        value.0 as i8
    }
}

impl From<FenceMode> for i16 {
    fn from(value: FenceMode) -> Self {
        i16::from(value.0)
    }
}

impl From<FenceMode> for i32 {
    fn from(value: FenceMode) -> Self {
        i32::from(value.0)
    }
}

impl From<FenceMode> for i64 {
    fn from(value: FenceMode) -> Self {
        i64::from(value.0)
    }
}

impl From<FenceMode> for isize {
    #[allow(clippy::cast_possible_wrap)]
    fn from(value: FenceMode) -> Self {
        value.0 as isize
    }
}

impl From<FenceMode> for u8 {
    fn from(value: FenceMode) -> Self {
        value.0
    }
}

impl From<FenceMode> for u16 {
    fn from(value: FenceMode) -> Self {
        u16::from(value.0)
    }
}

impl From<FenceMode> for u32 {
    fn from(value: FenceMode) -> Self {
        u32::from(value.0)
    }
}

impl From<FenceMode> for u64 {
    fn from(value: FenceMode) -> Self {
        u64::from(value.0)
    }
}

impl From<FenceMode> for usize {
    fn from(value: FenceMode) -> Self {
        usize::from(value.0)
    }
}

#[test]
fn into_u32() {
    assert_eq!(FenceMode::FENCE_TSO.into_u32(), 0b1000);
}
