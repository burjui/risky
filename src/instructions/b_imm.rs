//! Defines [BImm] and relevant trait implementations

use core::fmt;
use std::{
    error::Error,
    fmt::Display,
};

use bitvec::{
    order::Lsb0,
    slice::BitSlice,
    view::BitView,
};

use crate::util::{
    i16_fits_n_bits,
    i32_fits_n_bits,
    i64_fits_n_bits,
};

mod internal {
    pub enum Assert<const CHECK: bool> {}
    pub trait Fits13BIts {}
    impl Fits13BIts for Assert<true> {}
}

/// 13-bit signed immediate value used in branch instructions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BImm(u32);

impl BImm {
    const NBITS: usize = 13;

    /// Zero
    pub const ZERO: Self = Self(0);

    /// Creates an `BImm` from an [i8] constant
    #[must_use]
    pub const fn from_i8<const VALUE: i8>() -> Self {
        Self(VALUE as u32)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `BImm` from an [i16] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i16<const VALUE: i16>() -> Self
    where
        internal::Assert<{ i16_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits13BIts,
    {
        Self(VALUE as u32)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `BImm` from an [i32] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i32<const VALUE: i32>() -> Self
    where
        internal::Assert<{ i32_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits13BIts,
    {
        Self(VALUE as u32)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `BImm` from an [i64] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i64<const VALUE: i64>() -> Self
    where
        internal::Assert<{ i64_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits13BIts,
    {
        Self(VALUE as u32)
    }

    pub(crate) fn view_bits(&self) -> &BitSlice<u32, Lsb0> {
        &self.0.view_bits()[0..Self::NBITS]
    }
}

#[cfg(feature = "nightly")]
#[test]
fn constructors() {
    let _ = BImm::from_i8::<-128>();
    let _ = BImm::from_i8::<127>();
    let _ = BImm::from_i16::<-4096>();
    let _ = BImm::from_i16::<4095>();
    let _ = BImm::from_i32::<-4096>();
    let _ = BImm::from_i32::<4095>();
    let _ = BImm::from_i64::<-4096>();
    let _ = BImm::from_i64::<4095>();
}

#[test]
fn view_bits() {
    use bitvec::field::BitField;

    let imm = BImm(-4096_i32 as u32);
    assert_eq!(imm.view_bits().len(), BImm::NBITS);
    assert_eq!(imm.view_bits().load_le::<i16>(), -4096);
}

impl Display for BImm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&(self.0 as i32), f)
    }
}

#[test]
fn display() -> Result<(), BImmConvError> {
    assert_eq!(BImm::try_from(-4096)?.to_string(), "-4096");
    assert_eq!(BImm::try_from(4095)?.to_string(), "4095");
    Ok(())
}

impl From<i8> for BImm {
    fn from(value: i8) -> Self {
        Self(value as u32)
    }
}

impl TryFrom<i16> for BImm {
    type Error = BImmConvError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if i16_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as u32))
        } else {
            Err(BImmConvError::I16(value))
        }
    }
}

impl TryFrom<i32> for BImm {
    type Error = BImmConvError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if i32_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as u32))
        } else {
            Err(BImmConvError::I32(value))
        }
    }
}

impl TryFrom<i64> for BImm {
    type Error = BImmConvError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i64_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as u32))
        } else {
            Err(BImmConvError::I64(value))
        }
    }
}
#[test]
fn conversions() -> Result<(), BImmConvError> {
    assert_eq!(BImm::from(-128_i8), BImm(-128_i32 as u32));
    assert_eq!(BImm::from(127_i8), BImm(127_i32 as u32));
    assert_eq!(BImm::try_from(-4096_i16)?, BImm(-4096_i32 as u32));
    assert_eq!(BImm::try_from(4095_i16)?, BImm(4095_i32 as u32));
    assert_eq!(BImm::try_from(-4096_i32)?, BImm(-4096_i32 as u32));
    assert_eq!(BImm::try_from(4095_i32)?, BImm(4095_i32 as u32));
    assert_eq!(BImm::try_from(-4096_i64)?, BImm(-4096_i32 as u32));
    assert_eq!(BImm::try_from(4095_i64)?, BImm(4095_i32 as u32));

    assert!(matches!(
        BImm::try_from(-1048577_i32),
        Err(BImmConvError::I32(-1048577))
    ));
    assert!(matches!(
        BImm::try_from(1048576_i32),
        Err(BImmConvError::I32(1048576))
    ));
    assert!(matches!(
        BImm::try_from(-1048577_i64),
        Err(BImmConvError::I64(-1048577))
    ));
    assert!(matches!(
        BImm::try_from(1048576_i64),
        Err(BImmConvError::I64(1048576))
    ));

    Ok(())
}

/// [BImm] conversion error
#[derive(Debug)]
pub enum BImmConvError {
    ///
    I16(i16),
    ///
    I32(i32),
    ///
    I64(i64),
}

impl Display for BImmConvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid {}-bit signed immediate: ", BImm::NBITS)?;
        match self {
            BImmConvError::I16(value) => write!(f, "{} (0x{:04x})", value, value),
            BImmConvError::I32(value) => write!(f, "{} (0x{:08x})", value, value),
            BImmConvError::I64(value) => write!(f, "{} (0x{:016x})", value, value),
        }
    }
}
#[test]
fn conv_error_impl_display() {
    assert_eq!(
        BImm::try_from(-4097_i16).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: -4097 (0xefff)",
            BImm::NBITS
        )
    );
    assert_eq!(
        BImm::try_from(4096_i16).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: 4096 (0x1000)",
            BImm::NBITS
        )
    );
    assert_eq!(
        BImm::try_from(-4097_i32).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: -4097 (0xffffefff)",
            BImm::NBITS
        )
    );
    assert_eq!(
        BImm::try_from(4096_i32).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: 4096 (0x00001000)",
            BImm::NBITS
        )
    );
    assert_eq!(
        BImm::try_from(-4097_i64).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: -4097 (0xffffffffffffefff)",
            BImm::NBITS
        )
    );
    assert_eq!(
        BImm::try_from(4096_i64).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: 4096 (0x0000000000001000)",
            BImm::NBITS
        )
    );
}

impl Error for BImmConvError {}

#[test]
fn conv_error_impl_error() -> Result<(), Box<dyn Error>> {
    assert_eq!(BImm::try_from(0)?, BImm(0));
    Ok(())
}
