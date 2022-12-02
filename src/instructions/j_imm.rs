//! Defines [JImm] and relevant trait implementations

use core::fmt;
use std::{
    error::Error,
    fmt::Display,
    ops::Neg,
};

use crate::util::{
    i32_fits_n_bits,
    i64_fits_n_bits,
    isize_fits_n_bits,
    u32_fits_n_bits,
    u64_fits_n_bits,
    usize_fits_n_bits,
};

/// 21-bit signed immediate value used in the [jal](crate::instructions::rv32i::jal) instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JImm(i32);

impl JImm {
    const NBITS: usize = 21;

    /// Creates an `JImm` from an [i8] constant
    #[must_use]
    pub const fn from_i8<const VALUE: i8>() -> Self {
        Self((VALUE as i32) & !1)
    }

    /// Creates an `JImm` from an [i16] constant
    #[must_use]
    pub const fn from_i16<const VALUE: i16>() -> Self {
        Self((VALUE as i32) & !1)
    }

    /// Creates an `JImm` from an [u8] constant
    #[must_use]
    pub const fn from_u8<const VALUE: u8>() -> Self {
        Self((VALUE as i32) & !1)
    }

    /// Creates an `JImm` from an [u16] constant
    #[must_use]
    pub const fn from_u16<const VALUE: u16>() -> Self {
        Self((VALUE as i32) & !1)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `JImm` from an [i32] constant, failing to compile if the constant does not fit 21-bit signed range
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i32<const VALUE: i32>() -> Self
    where
        internal::Assert<{ i32_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits21BIts,
    {
        Self(VALUE & !1)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `JImm` from an [i64] constant, failing to compile if the constant does not fit 21-bit signed range
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i64<const VALUE: i64>() -> Self
    where
        internal::Assert<{ i64_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits21BIts,
    {
        Self((VALUE as i32) & !1)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `JImm` from an [isize] constant, failing to compile if the constant does not fit 21-bit signed range
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_isize<const VALUE: isize>() -> Self
    where
        internal::Assert<{ isize_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits21BIts,
    {
        Self(VALUE as i32 & !1)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `JImm` from an [u32] constant, failing to compile if the constant does not fit 21-bit signed range
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u32<const VALUE: u32>() -> Self
    where
        internal::Assert<{ u32_fits_n_bits(VALUE, Self::NBITS - 1) }>: internal::Fits21BIts,
    {
        Self(VALUE as i32 & !1)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `JImm` from an [u64] constant, failing to compile if the constant does not fit 21-bit signed range
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u64<const VALUE: u64>() -> Self
    where
        internal::Assert<{ u64_fits_n_bits(VALUE, Self::NBITS - 1) }>: internal::Fits21BIts,
    {
        Self(VALUE as i32 & !1)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `JImm` from an [usize] constant, failing to compile if the constant does not fit 21-bit signed range
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_usize<const VALUE: usize>() -> Self
    where
        internal::Assert<{ usize_fits_n_bits(VALUE, Self::NBITS - 1) }>: internal::Fits21BIts,
    {
        Self(VALUE as i32 & !1)
    }

    pub(crate) const fn to_u32(self) -> u32 {
        self.0 as u32
    }
}

#[cfg(feature = "nightly")]
#[test]
fn constructors() {
    let _ = JImm::from_i32::<-1048576>();
    let _ = JImm::from_i32::<1048575>();
    let _ = JImm::from_u32::<1048575>();
    let _ = JImm::from_i64::<-1048576>();
    let _ = JImm::from_i64::<1048575>();
    let _ = JImm::from_u64::<1048575>();
    let _ = JImm::from_isize::<-1048576>();
    let _ = JImm::from_isize::<1048575>();
    let _ = JImm::from_usize::<1048575>();
}

impl Display for JImm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[test]
fn display() -> Result<(), JImmConvError> {
    assert_eq!(JImm::try_from(-1048575)?.to_string(), "-1048576");
    assert_eq!(JImm::try_from(1048575)?.to_string(), "1048574");
    Ok(())
}

impl Neg for JImm {
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
    assert_eq!(-JImm(1048574), JImm(-1048574));
    assert_eq!(-JImm(-1048574), JImm(1048574));
    assert_eq!(-JImm(-1048576), JImm(-1048576));
}

impl From<i8> for JImm {
    fn from(value: i8) -> Self {
        Self((value as i32) & !1)
    }
}

impl From<i16> for JImm {
    fn from(value: i16) -> Self {
        Self((value as i32) & !1)
    }
}

impl From<u8> for JImm {
    fn from(value: u8) -> Self {
        Self((value as i32) & !1)
    }
}

impl From<u16> for JImm {
    fn from(value: u16) -> Self {
        Self((value as i32) & !1)
    }
}

impl TryFrom<i32> for JImm {
    type Error = JImmConvError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if i32_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value & !1))
        } else {
            Err(JImmConvError::I32(value))
        }
    }
}

impl TryFrom<i64> for JImm {
    type Error = JImmConvError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i64_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as i32 & !1))
        } else {
            Err(JImmConvError::I64(value))
        }
    }
}

impl TryFrom<isize> for JImm {
    type Error = JImmConvError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if isize_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as i32 & !1))
        } else {
            Err(JImmConvError::Isize(value))
        }
    }
}

impl TryFrom<u32> for JImm {
    type Error = JImmConvError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if u32_fits_n_bits(value, Self::NBITS - 1) {
            Ok(Self(value as i32 & !1))
        } else {
            Err(JImmConvError::U32(value))
        }
    }
}

impl TryFrom<u64> for JImm {
    type Error = JImmConvError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if u64_fits_n_bits(value, Self::NBITS - 1) {
            Ok(Self(value as i32 & !1))
        } else {
            Err(JImmConvError::U64(value))
        }
    }
}

impl TryFrom<usize> for JImm {
    type Error = JImmConvError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if usize_fits_n_bits(value, Self::NBITS - 1) {
            Ok(Self(value as i32 & !1))
        } else {
            Err(JImmConvError::Usize(value))
        }
    }
}

#[test]
fn conversions() -> Result<(), JImmConvError> {
    assert_eq!(JImm::from(-128_i8), JImm(-128));
    assert_eq!(JImm::from(127_i8), JImm(126));
    assert_eq!(JImm::from(-32768_i16), JImm(-32768));
    assert_eq!(JImm::from(32767_i16), JImm(32766));
    assert_eq!(JImm::try_from(-1048576_i32)?, JImm(-1048576));
    assert_eq!(JImm::try_from(1048575_i32)?, JImm(1048574));
    assert_eq!(JImm::try_from(-1048576_i64)?, JImm(-1048576));
    assert_eq!(JImm::try_from(1048575_i64)?, JImm(1048574));

    assert!(matches!(
        JImm::try_from(-1048577_i32),
        Err(JImmConvError::I32(-1048577))
    ));
    assert!(matches!(
        JImm::try_from(1048576_i32),
        Err(JImmConvError::I32(1048576))
    ));
    assert!(matches!(
        JImm::try_from(-1048577_i64),
        Err(JImmConvError::I64(-1048577))
    ));
    assert!(matches!(
        JImm::try_from(1048576_i64),
        Err(JImmConvError::I64(1048576))
    ));

    Ok(())
}

/// [JImm] conversion error
#[derive(Debug)]
pub enum JImmConvError {
    ///
    I32(i32),
    ///
    I64(i64),
    ///
    Isize(isize),
    ///
    U32(u32),
    ///
    U64(u64),
    ///
    Usize(usize),
}

impl Display for JImmConvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid {}-bit signed immediate: ", JImm::NBITS)?;
        match self {
            JImmConvError::I32(value) => write!(f, "{} (0x{:08x})", value, value),
            JImmConvError::I64(value) => write!(f, "{} (0x{:016x})", value, value),
            JImmConvError::Isize(value) => write!(f, "{}", value),
            JImmConvError::U32(value) => write!(f, "{} (0x{:08x})", value, value),
            JImmConvError::U64(value) => write!(f, "{} (0x{:016x})", value, value),
            JImmConvError::Usize(value) => write!(f, "{}", value),
        }
    }
}

#[test]
fn conv_error_impl_display() {
    assert_eq!(
        JImm::try_from(-1048577_i32).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: -1048577 (0xffefffff)",
            JImm::NBITS
        )
    );
    assert_eq!(
        JImm::try_from(1048576_i32).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: 1048576 (0x00100000)",
            JImm::NBITS
        )
    );

    assert_eq!(
        JImm::try_from(1048576_u32).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: 1048576 (0x00100000)",
            JImm::NBITS
        )
    );

    assert_eq!(
        JImm::try_from(-1048577_i64).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: -1048577 (0xffffffffffefffff)",
            JImm::NBITS
        )
    );
    assert_eq!(
        JImm::try_from(1048576_i64).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: 1048576 (0x0000000000100000)",
            JImm::NBITS
        )
    );
    assert_eq!(
        JImm::try_from(1048576_u64).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: 1048576 (0x0000000000100000)",
            JImm::NBITS
        )
    );

    assert_eq!(
        JImm::try_from(-1048577_isize).unwrap_err().to_string(),
        format!("invalid {}-bit signed immediate: -1048577", JImm::NBITS)
    );
    assert_eq!(
        JImm::try_from(1048576_isize).unwrap_err().to_string(),
        format!("invalid {}-bit signed immediate: 1048576", JImm::NBITS)
    );
    assert_eq!(
        JImm::try_from(1048576_usize).unwrap_err().to_string(),
        format!("invalid {}-bit signed immediate: 1048576", JImm::NBITS)
    );
}

impl Error for JImmConvError {}

#[test]
fn conv_error_impl_error() -> Result<(), Box<dyn Error>> {
    assert_eq!(JImm::try_from(0)?, JImm(0));
    Ok(())
}
mod internal {
    pub enum Assert<const CHECK: bool> {}
    pub trait Fits21BIts {}
    impl Fits21BIts for Assert<true> {}
}
