//! [`Uimm5`] implementation

use core::fmt;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::util::{
    i16_fits_n_bits_unsigned, i32_fits_n_bits_unsigned, i64_fits_n_bits_unsigned,
    i8_fits_n_bits_unsigned, isize_fits_n_bits_unsigned, u16_fits_n_bits, u32_fits_n_bits,
    u64_fits_n_bits, u8_fits_n_bits, usize_fits_n_bits,
};

/// 5-bit unsigned immediate value
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uimm5(pub(crate) u8);

impl Uimm5 {
    const NBITS: usize = 5;

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Uimm5` from an [u8] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u8<const VALUE: u8>() -> Self
    where
        internal::Assert<{ u8_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits5BIts,
    {
        Self(VALUE)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Uimm5` from an [u16] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u16<const VALUE: u16>() -> Self
    where
        internal::Assert<{ u16_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits5BIts,
    {
        Self(VALUE as u8)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Uimm5` from an [u32] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u32<const VALUE: u32>() -> Self
    where
        internal::Assert<{ u32_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits5BIts,
    {
        Self(VALUE as u8)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Uimm5` from an [u64] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u64<const VALUE: u64>() -> Self
    where
        internal::Assert<{ u64_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits5BIts,
    {
        Self(VALUE as u8)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Uimm5` from an [usize] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_usize<const VALUE: usize>() -> Self
    where
        internal::Assert<{ usize_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits5BIts,
    {
        Self(VALUE as u8)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Uimm5` from an [i8] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i8<const VALUE: i8>() -> Self
    where
        internal::Assert<{ i8_fits_n_bits_unsigned(VALUE, Self::NBITS) }>: internal::Fits5BIts,
    {
        Self(VALUE as u8)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Uimm5` from an [i16] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i16<const VALUE: i16>() -> Self
    where
        internal::Assert<{ i16_fits_n_bits_unsigned(VALUE, Self::NBITS) }>: internal::Fits5BIts,
    {
        Self(VALUE as u8)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Uimm5` from an [i32] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i32<const VALUE: i32>() -> Self
    where
        internal::Assert<{ i32_fits_n_bits_unsigned(VALUE, Self::NBITS) }>: internal::Fits5BIts,
    {
        Self(VALUE as u8)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Uimm5` from an [i64] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i64<const VALUE: i64>() -> Self
    where
        internal::Assert<{ i64_fits_n_bits_unsigned(VALUE, Self::NBITS) }>: internal::Fits5BIts,
    {
        Self(VALUE as u8)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Uimm5` from an [isize] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_isize<const VALUE: isize>() -> Self
    where
        internal::Assert<{ isize_fits_n_bits_unsigned(VALUE, Self::NBITS) }>: internal::Fits5BIts,
    {
        Self(VALUE as u8)
    }

    pub(crate) const fn into_u32(self) -> u32 {
        self.0 as u32
    }
}

#[cfg(feature = "nightly")]
#[test]
fn constructors() {
    let _ = Uimm5::from_u8::<0b11111>();
    let _ = Uimm5::from_u16::<0b11111>();
    let _ = Uimm5::from_u32::<0b11111>();
    let _ = Uimm5::from_u64::<0b11111>();
    let _ = Uimm5::from_usize::<0b11111>();
    let _ = Uimm5::from_i8::<0b11111>();
    let _ = Uimm5::from_i16::<0b11111>();
    let _ = Uimm5::from_i32::<0b11111>();
    let _ = Uimm5::from_i64::<0b11111>();
    let _ = Uimm5::from_isize::<0b11111>();
}

#[test]
fn into_u32() {
    assert_eq!(Uimm5(0xFF).into_u32(), 0xFF);
}

impl Display for Uimm5 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[test]
fn display() -> Result<(), Uimm5ConvError> {
    assert_eq!(Uimm5::try_from(0b11111u8)?.to_string(), "31");
    Ok(())
}

impl TryFrom<u8> for Uimm5 {
    type Error = Uimm5ConvError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if u8_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value))
        } else {
            Err(Uimm5ConvError::U8(value))
        }
    }
}

impl TryFrom<u16> for Uimm5 {
    type Error = Uimm5ConvError;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if u16_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as u8))
        } else {
            Err(Uimm5ConvError::U16(value))
        }
    }
}

impl TryFrom<u32> for Uimm5 {
    type Error = Uimm5ConvError;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if u32_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as u8))
        } else {
            Err(Uimm5ConvError::U32(value))
        }
    }
}

impl TryFrom<u64> for Uimm5 {
    type Error = Uimm5ConvError;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if u64_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as u8))
        } else {
            Err(Uimm5ConvError::U64(value))
        }
    }
}

impl TryFrom<usize> for Uimm5 {
    type Error = Uimm5ConvError;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if usize_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as u8))
        } else {
            Err(Uimm5ConvError::Usize(value))
        }
    }
}

impl TryFrom<i8> for Uimm5 {
    type Error = Uimm5ConvError;

    #[allow(clippy::cast_sign_loss)]
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if i8_fits_n_bits_unsigned(value, Self::NBITS) {
            Ok(Self(value as u8))
        } else {
            Err(Uimm5ConvError::I8(value))
        }
    }
}

impl TryFrom<i16> for Uimm5 {
    type Error = Uimm5ConvError;

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if i16_fits_n_bits_unsigned(value, Self::NBITS) {
            Ok(Self(value as u8))
        } else {
            Err(Uimm5ConvError::I16(value))
        }
    }
}

impl TryFrom<i32> for Uimm5 {
    type Error = Uimm5ConvError;

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if i32_fits_n_bits_unsigned(value, Self::NBITS) {
            Ok(Self(value as u8))
        } else {
            Err(Uimm5ConvError::I32(value))
        }
    }
}

impl TryFrom<i64> for Uimm5 {
    type Error = Uimm5ConvError;

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i64_fits_n_bits_unsigned(value, Self::NBITS) {
            Ok(Self(value as u8))
        } else {
            Err(Uimm5ConvError::I64(value))
        }
    }
}

impl TryFrom<isize> for Uimm5 {
    type Error = Uimm5ConvError;

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if isize_fits_n_bits_unsigned(value, Self::NBITS) {
            Ok(Self(value as u8))
        } else {
            Err(Uimm5ConvError::Isize(value))
        }
    }
}

#[test]
fn conversions() -> Result<(), Uimm5ConvError> {
    assert_eq!(Uimm5::try_from(0b11111u8)?, Uimm5(0b11111));
    assert_eq!(Uimm5::try_from(0b11111u16)?, Uimm5(0b11111));
    assert_eq!(Uimm5::try_from(0b11111u32)?, Uimm5(0b11111));
    assert_eq!(Uimm5::try_from(0b11111u64)?, Uimm5(0b11111));
    assert_eq!(Uimm5::try_from(0b11111usize)?, Uimm5(0b11111));
    assert_eq!(Uimm5::try_from(0b11111i8)?, Uimm5(0b11111));
    assert_eq!(Uimm5::try_from(0b11111i16)?, Uimm5(0b11111));
    assert_eq!(Uimm5::try_from(0b11111i32)?, Uimm5(0b11111));
    assert_eq!(Uimm5::try_from(0b11111i64)?, Uimm5(0b11111));
    assert_eq!(Uimm5::try_from(0b11111isize)?, Uimm5(0b11111));

    assert!(matches!(Uimm5::try_from(-1i8), Err(Uimm5ConvError::I8(-1))));
    assert!(matches!(
        Uimm5::try_from(-1i16),
        Err(Uimm5ConvError::I16(-1))
    ));
    assert!(matches!(
        Uimm5::try_from(-1i32),
        Err(Uimm5ConvError::I32(-1))
    ));
    assert!(matches!(
        Uimm5::try_from(-1i64),
        Err(Uimm5ConvError::I64(-1))
    ));
    assert!(matches!(
        Uimm5::try_from(-1isize),
        Err(Uimm5ConvError::Isize(-1))
    ));

    assert!(matches!(
        Uimm5::try_from(0b10_0000_u8),
        Err(Uimm5ConvError::U8(0b10_0000))
    ));
    assert!(matches!(
        Uimm5::try_from(0b10_0000_u16),
        Err(Uimm5ConvError::U16(0b10_0000))
    ));
    assert!(matches!(
        Uimm5::try_from(0b10_0000_u32),
        Err(Uimm5ConvError::U32(0b10_0000))
    ));
    assert!(matches!(
        Uimm5::try_from(0b10_0000_u64),
        Err(Uimm5ConvError::U64(0b10_0000))
    ));
    assert!(matches!(
        Uimm5::try_from(0b10_0000_usize),
        Err(Uimm5ConvError::Usize(0b10_0000))
    ));

    Ok(())
}

/// Uimm5 conversion error
#[derive(Debug)]
pub enum Uimm5ConvError {
    ///
    U8(u8),
    ///
    U16(u16),
    ///
    U32(u32),
    ///
    U64(u64),
    ///
    Usize(usize),
    ///
    I8(i8),
    ///
    I16(i16),
    ///
    I32(i32),
    ///
    I64(i64),
    ///
    Isize(isize),
}

// Satisfy grcov
#[test]
fn conv_error_impl_debug() {
    format!("{:?}", Uimm5ConvError::U8(32));
}

impl Display for Uimm5ConvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid {}-bit unsigned immediate: ", Uimm5::NBITS)?;
        match self {
            Uimm5ConvError::U8(value) => write!(f, "{value} (0x{value:02x})"),
            Uimm5ConvError::U16(value) => write!(f, "{value} (0x{value:04x})"),
            Uimm5ConvError::U32(value) => write!(f, "{value} (0x{value:08x})"),
            Uimm5ConvError::U64(value) => write!(f, "{value} (0x{value:016x})"),
            Uimm5ConvError::Usize(value) => write!(f, "{value}"),
            Uimm5ConvError::I8(value) => write!(f, "{value} (0x{value:02x})"),
            Uimm5ConvError::I16(value) => write!(f, "{value} (0x{value:04x})"),
            Uimm5ConvError::I32(value) => write!(f, "{value} (0x{value:08x})"),
            Uimm5ConvError::I64(value) => write!(f, "{value} (0x{value:016x})"),
            Uimm5ConvError::Isize(value) => write!(f, "{value}"),
        }
    }
}

#[test]
fn conv_error_impl_display() {
    assert_eq!(
        Uimm5::try_from(-1_i8).unwrap_err().to_string(),
        format!("invalid {}-bit unsigned immediate: -1 (0xff)", Uimm5::NBITS)
    );
    assert_eq!(
        Uimm5::try_from(-1_i16).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: -1 (0xffff)",
            Uimm5::NBITS
        )
    );
    assert_eq!(
        Uimm5::try_from(-1_i32).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: -1 (0xffffffff)",
            Uimm5::NBITS
        )
    );
    assert_eq!(
        Uimm5::try_from(-1_i64).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: -1 (0xffffffffffffffff)",
            Uimm5::NBITS
        )
    );
    assert_eq!(
        Uimm5::try_from(-1_isize).unwrap_err().to_string(),
        format!("invalid {}-bit unsigned immediate: -1", Uimm5::NBITS)
    );

    assert_eq!(
        Uimm5::try_from(0b10_0000_u8).unwrap_err().to_string(),
        format!("invalid {}-bit unsigned immediate: 32 (0x20)", Uimm5::NBITS)
    );
    assert_eq!(
        Uimm5::try_from(0b10_0000_u16).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: 32 (0x0020)",
            Uimm5::NBITS
        )
    );
    assert_eq!(
        Uimm5::try_from(0b10_0000_u32).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: 32 (0x00000020)",
            Uimm5::NBITS
        )
    );
    assert_eq!(
        Uimm5::try_from(0b10_0000_u64).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: 32 (0x0000000000000020)",
            Uimm5::NBITS
        )
    );
    assert_eq!(
        Uimm5::try_from(0b10_0000_usize).unwrap_err().to_string(),
        format!("invalid {}-bit unsigned immediate: 32", Uimm5::NBITS)
    );
}

impl Error for Uimm5ConvError {}

#[test]
fn conv_error_impl_error() -> Result<(), Box<dyn Error>> {
    assert_eq!(Uimm5::try_from(0u8)?, Uimm5(0));
    Ok(())
}

mod internal {
    pub enum Assert<const CHECK: bool> {}
    pub trait Fits5BIts {}
    impl Fits5BIts for Assert<true> {}
}
