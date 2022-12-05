//! [`BImm`] implementation

use core::fmt;
use std::{
    error::Error,
    fmt::{Debug, Display},
    ops::Neg,
};

use crate::util::{
    i16_fits_n_bits, i32_fits_n_bits, i64_fits_n_bits, isize_fits_n_bits, u16_fits_n_bits,
    u32_fits_n_bits, u64_fits_n_bits, usize_fits_n_bits,
};

/// 13-bit signed immediate value used in branch instructions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BImm(i16);

impl BImm {
    const NBITS: usize = 13;

    /// Creates an `BImm` from an [i8] constant
    #[must_use]
    pub const fn from_i8<const VALUE: i8>() -> Self {
        Self(VALUE as i16 & !1)
    }

    /// Creates an `BImm` from an [u8] constant
    #[must_use]
    pub const fn from_u8<const VALUE: u8>() -> Self {
        Self(VALUE as i16 & !1)
    }

    #[doc = include_str!("../../../../doc/nightly_warning.html")]
    ///
    /// Creates an `BImm` from an [i16] constant, failing to compile if the constant does not fit 13-bit signed range
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i16<const VALUE: i16>() -> Self
    where
        internal::Assert<{ i16_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits13BIts,
    {
        Self(VALUE & !1)
    }

    #[doc = include_str!("../../../../doc/nightly_warning.html")]
    ///
    /// Creates an `BImm` from an [i32] constant, failing to compile if the constant does not fit 13-bit signed range
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i32<const VALUE: i32>() -> Self
    where
        internal::Assert<{ i32_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits13BIts,
    {
        Self(VALUE as i16 & !1)
    }

    #[doc = include_str!("../../../../doc/nightly_warning.html")]
    ///
    /// Creates an `BImm` from an [i64] constant, failing to compile if the constant does not fit 13-bit signed range
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i64<const VALUE: i64>() -> Self
    where
        internal::Assert<{ i64_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits13BIts,
    {
        Self(VALUE as i16 & !1)
    }

    #[doc = include_str!("../../../../doc/nightly_warning.html")]
    ///
    /// Creates an `BImm` from an [isize] constant, failing to compile if the constant does not fit 13-bit signed range
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_isize<const VALUE: isize>() -> Self
    where
        internal::Assert<{ isize_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits13BIts,
    {
        Self(VALUE as i16 & !1)
    }

    #[doc = include_str!("../../../../doc/nightly_warning.html")]
    ///
    /// Creates an `BImm` from an [u16] constant, failing to compile if the constant does not fit 13-bit signed range
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u16<const VALUE: u16>() -> Self
    where
        internal::Assert<{ u16_fits_n_bits(VALUE, Self::NBITS - 1) }>: internal::Fits13BIts,
    {
        Self(VALUE as i16 & !1)
    }

    #[doc = include_str!("../../../../doc/nightly_warning.html")]
    ///
    /// Creates an `BImm` from an [u32] constant, failing to compile if the constant does not fit 13-bit signed range
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u32<const VALUE: u32>() -> Self
    where
        internal::Assert<{ u32_fits_n_bits(VALUE, Self::NBITS - 1) }>: internal::Fits13BIts,
    {
        Self(VALUE as i16 & !1)
    }

    #[doc = include_str!("../../../../doc/nightly_warning.html")]
    ///
    /// Creates an `BImm` from an [u64] constant, failing to compile if the constant does not fit 13-bit signed range
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u64<const VALUE: u64>() -> Self
    where
        internal::Assert<{ u64_fits_n_bits(VALUE, Self::NBITS - 1) }>: internal::Fits13BIts,
    {
        Self(VALUE as i16 & !1)
    }

    #[doc = include_str!("../../../../doc/nightly_warning.html")]
    ///
    /// Creates an `BImm` from an [usize] constant, failing to compile if the constant does not fit 13-bit signed range
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_usize<const VALUE: usize>() -> Self
    where
        internal::Assert<{ usize_fits_n_bits(VALUE, Self::NBITS - 1) }>: internal::Fits13BIts,
    {
        Self(VALUE as i16 & !1)
    }

    #[allow(clippy::cast_sign_loss)]
    pub(crate) const fn into_u32(self) -> u32 {
        self.0 as u32
    }
}

#[test]
fn constructors() {
    assert_eq!(BImm::from_i8::<-128>(), BImm(-128));
    assert_eq!(BImm::from_i8::<127>(), BImm(126));
    assert_eq!(BImm::from_u8::<255>(), BImm(254));
}

#[cfg(feature = "nightly")]
#[test]
fn const_constructors() {
    assert_eq!(BImm::from_i16::<-4096>(), BImm(-4096));
    assert_eq!(BImm::from_i16::<4095>(), BImm(4094));
    assert_eq!(BImm::from_u16::<4095>(), BImm(4094));
    assert_eq!(BImm::from_i32::<-4096>(), BImm(-4096));
    assert_eq!(BImm::from_i32::<4095>(), BImm(4094));
    assert_eq!(BImm::from_u32::<4095>(), BImm(4094));
    assert_eq!(BImm::from_i64::<-4096>(), BImm(-4096));
    assert_eq!(BImm::from_i64::<4095>(), BImm(4094));
    assert_eq!(BImm::from_u64::<4095>(), BImm(4094));
    assert_eq!(BImm::from_isize::<-4096>(), BImm(-4096));
    assert_eq!(BImm::from_isize::<4095>(), BImm(4094));
    assert_eq!(BImm::from_usize::<4095>(), BImm(4094));
}

#[test]
fn into_u32() {
    assert_eq!(BImm(-4096).into_u32(), 0xFFFFF000);
}

impl Display for BImm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&i32::from(self.0), f)
    }
}

#[test]
fn display() -> Result<(), BImmConvError> {
    assert_eq!(BImm::try_from(-4096)?.to_string(), "-4096");
    assert_eq!(BImm::try_from(4095)?.to_string(), "4094");
    Ok(())
}

impl Neg for BImm {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self.0 == (-(1 << (Self::NBITS - 1))) {
            self
        } else {
            Self(-self.0)
        }
    }
}

#[test]
fn neg() {
    assert_eq!(-BImm(4094), BImm(-4094));
    assert_eq!(-BImm(-4094), BImm(4094));
    assert_eq!(-BImm(-4096), BImm(-4096));
}

impl From<i8> for BImm {
    fn from(value: i8) -> Self {
        Self(i16::from(value) & !1)
    }
}

impl From<u8> for BImm {
    fn from(value: u8) -> Self {
        Self(i16::from(value) & !1)
    }
}

impl TryFrom<i16> for BImm {
    type Error = BImmConvError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if i16_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value & !1))
        } else {
            Err(BImmConvError::I16(value))
        }
    }
}

impl TryFrom<i32> for BImm {
    type Error = BImmConvError;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if i32_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as i16 & !1))
        } else {
            Err(BImmConvError::I32(value))
        }
    }
}

impl TryFrom<i64> for BImm {
    type Error = BImmConvError;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i64_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as i16 & !1))
        } else {
            Err(BImmConvError::I64(value))
        }
    }
}

impl TryFrom<isize> for BImm {
    type Error = BImmConvError;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if isize_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as i16 & !1))
        } else {
            Err(BImmConvError::Isize(value))
        }
    }
}

impl TryFrom<u16> for BImm {
    type Error = BImmConvError;

    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if u16_fits_n_bits(value, Self::NBITS - 1) {
            Ok(Self(value as i16 & !1))
        } else {
            Err(BImmConvError::U16(value))
        }
    }
}

impl TryFrom<u32> for BImm {
    type Error = BImmConvError;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if u32_fits_n_bits(value, Self::NBITS - 1) {
            Ok(Self(value as i16 & !1))
        } else {
            Err(BImmConvError::U32(value))
        }
    }
}

impl TryFrom<u64> for BImm {
    type Error = BImmConvError;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if u64_fits_n_bits(value, Self::NBITS - 1) {
            Ok(Self(value as i16 & !1))
        } else {
            Err(BImmConvError::U64(value))
        }
    }
}

impl TryFrom<usize> for BImm {
    type Error = BImmConvError;

    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if usize_fits_n_bits(value, Self::NBITS - 1) {
            Ok(Self(value as i16 & !1))
        } else {
            Err(BImmConvError::Usize(value))
        }
    }
}

#[test]
fn conversions() -> Result<(), BImmConvError> {
    assert_eq!(BImm::from(-128_i8), BImm(-128));
    assert_eq!(BImm::from(127_i8), BImm(126));
    assert_eq!(BImm::from(255_u8), BImm(254));

    assert_eq!(BImm::try_from(-4096_i16)?, BImm(-4096));
    assert_eq!(BImm::try_from(4095_i16)?, BImm(4094));
    assert!(matches!(
        BImm::try_from(-4097_i16),
        Err(BImmConvError::I16(-4097))
    ));
    assert!(matches!(
        BImm::try_from(4096_i16),
        Err(BImmConvError::I16(4096))
    ));

    assert_eq!(BImm::try_from(4095_u16)?, BImm(4094));
    assert!(matches!(
        BImm::try_from(4096_u16),
        Err(BImmConvError::U16(4096))
    ));

    assert_eq!(BImm::try_from(-4096_i32)?, BImm(-4096));
    assert_eq!(BImm::try_from(4095_i32)?, BImm(4094));
    assert!(matches!(
        BImm::try_from(-4097_i32),
        Err(BImmConvError::I32(-4097))
    ));
    assert!(matches!(
        BImm::try_from(4096_i32),
        Err(BImmConvError::I32(4096))
    ));

    assert_eq!(BImm::try_from(4095_u32)?, BImm(4094));
    assert!(matches!(
        BImm::try_from(4096_u32),
        Err(BImmConvError::U32(4096))
    ));

    assert_eq!(BImm::try_from(-4096_i64)?, BImm(-4096));
    assert_eq!(BImm::try_from(4095_i64)?, BImm(4094));
    assert!(matches!(
        BImm::try_from(-4097_i64),
        Err(BImmConvError::I64(-4097))
    ));
    assert!(matches!(
        BImm::try_from(4096_i64),
        Err(BImmConvError::I64(4096))
    ));

    assert_eq!(BImm::try_from(4095_u64)?, BImm(4094));
    assert!(matches!(
        BImm::try_from(4096_u64),
        Err(BImmConvError::U64(4096))
    ));

    assert_eq!(BImm::try_from(-4096_isize)?, BImm(-4096));
    assert_eq!(BImm::try_from(4095_isize)?, BImm(4094));
    assert!(matches!(
        BImm::try_from(-4097_isize),
        Err(BImmConvError::Isize(-4097))
    ));
    assert!(matches!(
        BImm::try_from(4096_isize),
        Err(BImmConvError::Isize(4096))
    ));

    assert_eq!(BImm::try_from(4095_usize)?, BImm(4094));
    assert!(matches!(
        BImm::try_from(4096_usize),
        Err(BImmConvError::Usize(4096))
    ));

    Ok(())
}

/// [`BImm`] conversion error
#[derive(Debug)]
pub enum BImmConvError {
    ///
    I16(i16),
    ///
    I32(i32),
    ///
    I64(i64),
    ///
    Isize(isize),
    ///
    U16(u16),
    ///
    U32(u32),
    ///
    U64(u64),
    ///
    Usize(usize),
}

// Satisfy grcov
#[test]
fn conv_error_impl_debug() {
    format!("{:?}", BImmConvError::I16(0));
}

impl Display for BImmConvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid {}-bit signed immediate: ", BImm::NBITS)?;
        match self {
            BImmConvError::I16(value) => write!(f, "{value} (0x{value:04x})"),
            BImmConvError::I32(value) => write!(f, "{value} (0x{value:08x})"),
            BImmConvError::I64(value) => write!(f, "{value} (0x{value:016x})"),
            BImmConvError::Isize(value) => write!(f, "{value}"),
            BImmConvError::U16(value) => write!(f, "{value} (0x{value:04x})"),
            BImmConvError::U32(value) => write!(f, "{value} (0x{value:08x})"),
            BImmConvError::U64(value) => write!(f, "{value} (0x{value:016x})"),
            BImmConvError::Usize(value) => write!(f, "{value}"),
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
        BImm::try_from(4096_u16).unwrap_err().to_string(),
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
        BImm::try_from(4096_u32).unwrap_err().to_string(),
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

    assert_eq!(
        BImm::try_from(4096_u64).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: 4096 (0x0000000000001000)",
            BImm::NBITS
        )
    );

    assert_eq!(
        BImm::try_from(-4097_isize).unwrap_err().to_string(),
        format!("invalid {}-bit signed immediate: -4097", BImm::NBITS)
    );
    assert_eq!(
        BImm::try_from(4096_isize).unwrap_err().to_string(),
        format!("invalid {}-bit signed immediate: 4096", BImm::NBITS)
    );

    assert_eq!(
        BImm::try_from(4096_usize).unwrap_err().to_string(),
        format!("invalid {}-bit signed immediate: 4096", BImm::NBITS)
    );
}

impl Error for BImmConvError {}

#[test]
fn conv_error_impl_error() -> Result<(), Box<dyn Error>> {
    assert_eq!(BImm::try_from(0)?, BImm(0));
    Ok(())
}

mod internal {
    pub enum Assert<const CHECK: bool> {}
    pub trait Fits13BIts {}
    impl Fits13BIts for Assert<true> {}
}
