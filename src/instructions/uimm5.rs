//! Defines [Uimm5] and relevant trait implementations

use core::fmt;
use std::{
    error::Error,
    fmt::Display,
};

use crate::util::{
    u16_fits_n_bits,
    u32_fits_n_bits,
    u64_fits_n_bits,
    u8_fits_n_bits,
};

mod internal {
    pub enum Assert<const CHECK: bool> {}
    pub trait Fits5BIts {}
    impl Fits5BIts for Assert<true> {}
}

/// 5-bit unsigned immediate value
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uimm5(u8);

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

    pub(crate) const fn to_u32(self) -> u32 {
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

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if u64_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as u8))
        } else {
            Err(Uimm5ConvError::U64(value))
        }
    }
}

#[test]
fn conversions() -> Result<(), Uimm5ConvError> {
    assert_eq!(Uimm5::try_from(0b11111u8)?, Uimm5(0b11111));
    assert_eq!(Uimm5::try_from(0b11111u16)?, Uimm5(0b11111));
    assert_eq!(Uimm5::try_from(0b11111u32)?, Uimm5(0b11111));
    assert_eq!(Uimm5::try_from(0b11111u64)?, Uimm5(0b11111));

    assert!(matches!(
        Uimm5::try_from(0b100000u8),
        Err(Uimm5ConvError::U8(0b100000))
    ));
    assert!(matches!(
        Uimm5::try_from(0b100000u16),
        Err(Uimm5ConvError::U16(0b100000))
    ));
    assert!(matches!(
        Uimm5::try_from(0b100000u32),
        Err(Uimm5ConvError::U32(0b100000))
    ));
    assert!(matches!(
        Uimm5::try_from(0b100000u64),
        Err(Uimm5ConvError::U64(0b100000))
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
}

impl Display for Uimm5ConvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid {}-bit unsigned immediate: ", Uimm5::NBITS)?;
        match self {
            Uimm5ConvError::U8(value) => write!(f, "{} (0x{:02x})", value, value),
            Uimm5ConvError::U16(value) => write!(f, "{} (0x{:04x})", value, value),
            Uimm5ConvError::U32(value) => write!(f, "{} (0x{:08x})", value, value),
            Uimm5ConvError::U64(value) => write!(f, "{} (0x{:016x})", value, value),
        }
    }
}

#[test]
fn conv_error_impl_display() {
    assert_eq!(
        Uimm5::try_from(0b100000u8).unwrap_err().to_string(),
        format!("invalid {}-bit unsigned immediate: 32 (0x20)", Uimm5::NBITS)
    );
    assert_eq!(
        Uimm5::try_from(0b100000u16).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: 32 (0x0020)",
            Uimm5::NBITS
        )
    );
    assert_eq!(
        Uimm5::try_from(0b100000u32).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: 32 (0x00000020)",
            Uimm5::NBITS
        )
    );
    assert_eq!(
        Uimm5::try_from(0b100000u64).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: 32 (0x0000000000000020)",
            Uimm5::NBITS
        )
    );
}

impl Error for Uimm5ConvError {}

#[test]
fn conv_error_impl_error() -> Result<(), Box<dyn Error>> {
    assert_eq!(Uimm5::try_from(0u8)?, Uimm5(0));
    Ok(())
}
