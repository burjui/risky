//! [`JImm`] implementation

use core::fmt;
use std::{
    error::Error,
    fmt::{Debug, Display},
    num::TryFromIntError,
    ops::Neg,
};

use crate::util::{
    i32_fits_n_bits, i64_fits_n_bits, isize_fits_n_bits, u32_fits_n_bits, u64_fits_n_bits,
    usize_fits_n_bits,
};

/// 21-bit signed immediate value used in the [jal](crate::rv32i::jal) instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JImm(pub(crate) i32);

impl JImm {
    const NBITS: usize = 21;

    /// The smallest value that can be represented by this integer type (−2²⁰)
    pub const MIN: Self = Self(-0x10_0000);
    /// The largest value that can be represented by this integer type (2²⁰ - 2)
    pub const MAX: Self = Self(0xFFFFE);

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

    #[allow(clippy::cast_sign_loss)]
    pub(crate) const fn into_u32(self) -> u32 {
        self.0 as u32
    }
}

#[test]
fn constructors() {
    assert_eq!(JImm::from_i8::<-128>(), JImm(-128));
    assert_eq!(JImm::from_i8::<127>(), JImm(126));
    assert_eq!(JImm::from_u8::<255>(), JImm(254));
    assert_eq!(JImm::from_i16::<-32768>(), JImm(-32768));
    assert_eq!(JImm::from_i16::<32767>(), JImm(32766));
    assert_eq!(JImm::from_u16::<65535>(), JImm(65534));
}

#[cfg(feature = "nightly")]
#[test]
fn const_constructors() {
    assert_eq!(JImm::from_i32::<-1048576>(), JImm(-1048576));
    assert_eq!(JImm::from_i32::<1048575>(), JImm(1048574));
    assert_eq!(JImm::from_u32::<1048575>(), JImm(1048574));
    assert_eq!(JImm::from_i64::<-1048576>(), JImm(-1048576));
    assert_eq!(JImm::from_i64::<1048575>(), JImm(1048574));
    assert_eq!(JImm::from_u64::<1048575>(), JImm(1048574));
    assert_eq!(JImm::from_isize::<-1048576>(), JImm(-1048576));
    assert_eq!(JImm::from_isize::<1048575>(), JImm(1048574));
    assert_eq!(JImm::from_usize::<1048575>(), JImm(1048574));
}

#[test]
fn into_u32() {
    assert_eq!(JImm(-1_048_576).into_u32(), 0xFFF0_0000);
}

impl Display for JImm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[test]
fn display() -> Result<(), JImmConvError> {
    assert_eq!(JImm::try_from(-1_048_575)?.to_string(), "-1048576");
    assert_eq!(JImm::try_from(1_048_575)?.to_string(), "1048574");
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
    assert_eq!(-JImm(1_048_574), JImm(-1_048_574));
    assert_eq!(-JImm(-1_048_574), JImm(1_048_574));
    assert_eq!(-JImm(-1_048_576), JImm(-1_048_576));
}

impl From<i8> for JImm {
    fn from(value: i8) -> Self {
        Self(i32::from(value) & !1)
    }
}

impl From<i16> for JImm {
    fn from(value: i16) -> Self {
        Self(i32::from(value) & !1)
    }
}

impl From<u8> for JImm {
    fn from(value: u8) -> Self {
        Self(i32::from(value) & !1)
    }
}

impl From<u16> for JImm {
    fn from(value: u16) -> Self {
        Self(i32::from(value) & !1)
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

    #[allow(clippy::cast_possible_truncation)]
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

    #[allow(clippy::cast_possible_truncation)]
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

    #[allow(clippy::cast_possible_wrap)]
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

    #[allow(clippy::cast_possible_truncation)]
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

    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if usize_fits_n_bits(value, Self::NBITS - 1) {
            Ok(Self(value as i32 & !1))
        } else {
            Err(JImmConvError::Usize(value))
        }
    }
}

impl TryFrom<JImm> for i8 {
    type Error = TryFromIntError;

    fn try_from(value: JImm) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl TryFrom<JImm> for i16 {
    type Error = TryFromIntError;

    fn try_from(value: JImm) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl From<JImm> for i32 {
    fn from(value: JImm) -> Self {
        value.0
    }
}

impl From<JImm> for i64 {
    fn from(value: JImm) -> Self {
        i64::from(value.0)
    }
}

impl TryFrom<JImm> for isize {
    type Error = TryFromIntError;

    fn try_from(value: JImm) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl TryFrom<JImm> for u8 {
    type Error = TryFromIntError;

    fn try_from(value: JImm) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl TryFrom<JImm> for u16 {
    type Error = TryFromIntError;

    fn try_from(value: JImm) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl TryFrom<JImm> for u32 {
    type Error = TryFromIntError;

    fn try_from(value: JImm) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl TryFrom<JImm> for u64 {
    type Error = TryFromIntError;

    fn try_from(value: JImm) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl TryFrom<JImm> for usize {
    type Error = TryFromIntError;

    fn try_from(value: JImm) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

#[test]
fn conversions_from_integers() -> Result<(), JImmConvError> {
    assert_eq!(JImm::from(-128_i8), JImm(-128));
    assert_eq!(JImm::from(127_i8), JImm(126));
    assert_eq!(JImm::from(255_u8), JImm(254));
    assert_eq!(JImm::from(-32768_i16), JImm(-32768));
    assert_eq!(JImm::from(32767_i16), JImm(32766));
    assert_eq!(JImm::from(65535_u16), JImm(65534));
    assert_eq!(JImm::try_from(-1_048_576_i32)?, JImm(-1_048_576));
    assert_eq!(JImm::try_from(1_048_575_i32)?, JImm(1_048_574));
    assert_eq!(JImm::try_from(1_048_575_u32)?, JImm(1_048_574));
    assert_eq!(JImm::try_from(-1_048_576_i64)?, JImm(-1_048_576));
    assert_eq!(JImm::try_from(1_048_575_i64)?, JImm(1_048_574));
    assert_eq!(JImm::try_from(1_048_575_u64)?, JImm(1_048_574));
    assert_eq!(JImm::try_from(-1_048_576_isize)?, JImm(-1_048_576));
    assert_eq!(JImm::try_from(1_048_575_isize)?, JImm(1_048_574));
    assert_eq!(JImm::try_from(1_048_575_usize)?, JImm(1_048_574));

    assert!(matches!(
        JImm::try_from(-1_048_577_i32),
        Err(JImmConvError::I32(-1_048_577))
    ));
    assert!(matches!(
        JImm::try_from(1_048_576_i32),
        Err(JImmConvError::I32(1_048_576))
    ));
    assert!(matches!(
        JImm::try_from(-1_048_577_i64),
        Err(JImmConvError::I64(-1_048_577))
    ));
    assert!(matches!(
        JImm::try_from(1_048_576_i64),
        Err(JImmConvError::I64(1_048_576))
    ));

    Ok(())
}

#[test]
fn conversions_to_integers() -> Result<(), Box<dyn Error>> {
    assert_eq!(i8::try_from(JImm::from(i8::MIN))?, i8::MIN);
    assert_eq!(i8::try_from(JImm::from(i8::MAX))?, i8::MAX - 1);
    assert!(i8::try_from(JImm::try_from(i16::from(i8::MIN) - 1)?).is_err());
    assert!(i8::try_from(JImm::try_from(i16::from(i8::MAX) + 1)?).is_err());

    assert_eq!(i16::try_from(JImm::from(i16::MIN))?, i16::MIN);
    assert_eq!(i16::try_from(JImm::from(i16::MAX))?, i16::MAX - 1);
    assert!(i16::try_from(JImm::try_from(i32::from(i16::MIN) - 1)?).is_err());
    assert!(i16::try_from(JImm::try_from(i32::from(i16::MAX) + 1)?).is_err());

    assert_eq!(i32::from(JImm::MIN), -0x100000);
    assert_eq!(i32::from(JImm::MAX), 0xFFFFE);

    assert_eq!(i64::from(JImm::MIN), -0x100000);
    assert_eq!(i64::from(JImm::MAX), 0xFFFFE);

    assert_eq!(isize::try_from(JImm::MIN)?, -0x100000);
    assert_eq!(isize::try_from(JImm::MAX)?, 0xFFFFE);

    assert_eq!(u8::try_from(JImm::from(u8::MAX))?, u8::MAX - 1);
    assert!(u8::try_from(JImm::try_from(u16::from(u8::MAX) + 1)?).is_err());
    assert!(u8::try_from(JImm::try_from(-1)?).is_err());

    assert_eq!(u16::try_from(JImm::from(u16::MAX))?, u16::MAX - 1);
    assert!(u16::try_from(JImm::try_from(u32::from(u16::MAX) + 1)?).is_err());
    assert!(u16::try_from(JImm::try_from(-1)?).is_err());

    assert_eq!(u32::try_from(JImm::MAX)?, 0xFFFFE);
    assert!(u32::try_from(JImm::try_from(-1)?).is_err());

    assert_eq!(u64::try_from(JImm::MAX)?, 0xFFFFE);
    assert!(u64::try_from(JImm::try_from(-1)?).is_err());

    assert_eq!(isize::try_from(JImm::MAX)?, 0xFFFFE);
    assert!(usize::try_from(JImm::try_from(-1)?).is_err());

    Ok(())
}

/// [`JImm`] conversion error
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

impl Debug for JImmConvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JImmConvError::I32(value) => write!(f, "JImmConvError::I32({value})"),
            JImmConvError::I64(value) => write!(f, "JImmConvError::I64({value})"),
            JImmConvError::Isize(value) => write!(f, "JImmConvError::Isize({value})"),
            JImmConvError::U32(value) => write!(f, "JImmConvError::U32({value})"),
            JImmConvError::U64(value) => write!(f, "JImmConvError::U64({value})"),
            JImmConvError::Usize(value) => write!(f, "JImmConvError::Usize({value})"),
        }
    }
}

#[test]
fn conv_error_impl_debug() {
    assert_eq!(
        format!("{:?}", JImmConvError::I32(i32::from(JImm::MIN) - 1)),
        "JImmConvError::I32(-1048577)"
    );
    assert_eq!(
        format!("{:?}", JImmConvError::I64(i64::from(JImm::MIN) - 1)),
        "JImmConvError::I64(-1048577)"
    );
    assert_eq!(
        format!(
            "{:?}",
            JImmConvError::Isize(isize::try_from(JImm::MIN).unwrap() - 1)
        ),
        "JImmConvError::Isize(-1048577)"
    );

    assert_eq!(
        format!(
            "{:?}",
            JImmConvError::U32(u32::try_from(JImm::MAX).unwrap() + 1)
        ),
        "JImmConvError::U32(1048575)"
    );
    assert_eq!(
        format!(
            "{:?}",
            JImmConvError::U64(u64::try_from(JImm::MAX).unwrap() + 1)
        ),
        "JImmConvError::U64(1048575)"
    );
    assert_eq!(
        format!(
            "{:?}",
            JImmConvError::Usize(usize::try_from(JImm::MAX).unwrap() + 1)
        ),
        "JImmConvError::Usize(1048575)"
    );
}

impl Display for JImmConvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid {}-bit signed immediate: ", JImm::NBITS)?;
        match self {
            JImmConvError::I32(value) => write!(f, "{value} (0x{value:08x})"),
            JImmConvError::I64(value) => write!(f, "{value} (0x{value:016x})"),
            JImmConvError::Isize(value) => write!(f, "{value}"),
            JImmConvError::U32(value) => write!(f, "{value} (0x{value:08x})"),
            JImmConvError::U64(value) => write!(f, "{value} (0x{value:016x})"),
            JImmConvError::Usize(value) => write!(f, "{value}"),
        }
    }
}

#[test]
fn conv_error_impl_display() {
    assert_eq!(
        JImm::try_from(-1_048_577_i32).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: -1048577 (0xffefffff)",
            JImm::NBITS
        )
    );
    assert_eq!(
        JImm::try_from(1_048_576_i32).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: 1048576 (0x00100000)",
            JImm::NBITS
        )
    );

    assert_eq!(
        JImm::try_from(1_048_576_u32).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: 1048576 (0x00100000)",
            JImm::NBITS
        )
    );

    assert_eq!(
        JImm::try_from(-1_048_577_i64).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: -1048577 (0xffffffffffefffff)",
            JImm::NBITS
        )
    );
    assert_eq!(
        JImm::try_from(1_048_576_i64).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: 1048576 (0x0000000000100000)",
            JImm::NBITS
        )
    );
    assert_eq!(
        JImm::try_from(1_048_576_u64).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: 1048576 (0x0000000000100000)",
            JImm::NBITS
        )
    );

    assert_eq!(
        JImm::try_from(-1_048_577_isize).unwrap_err().to_string(),
        format!("invalid {}-bit signed immediate: -1048577", JImm::NBITS)
    );
    assert_eq!(
        JImm::try_from(1_048_576_isize).unwrap_err().to_string(),
        format!("invalid {}-bit signed immediate: 1048576", JImm::NBITS)
    );
    assert_eq!(
        JImm::try_from(1_048_576_usize).unwrap_err().to_string(),
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
