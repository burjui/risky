//! [`Csr`] type and standard CSR definitions

use core::fmt;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::util::{
    i16_fits_n_bits_unsigned, i32_fits_n_bits_unsigned, i64_fits_n_bits_unsigned,
    isize_fits_n_bits_unsigned, u16_fits_n_bits, u32_fits_n_bits, u64_fits_n_bits,
    usize_fits_n_bits,
};

/// 12-bit unsigned value representing a CSR (Control and Status Register)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Csr(pub(crate) u16);

impl Csr {
    const NBITS: usize = 12;

    /// Creates an `Csr` from an [u8] constant
    #[must_use]
    pub const fn from_u8<const VALUE: u8>() -> Self {
        Self(VALUE as u16)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Csr` from an [u16] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u16<const VALUE: u16>() -> Self
    where
        internal::Assert<{ u16_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits12BIts,
    {
        Self(VALUE)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Csr` from an [u32] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u32<const VALUE: u32>() -> Self
    where
        internal::Assert<{ u32_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits12BIts,
    {
        Self(VALUE as u16)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Csr` from an [u64] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u64<const VALUE: u64>() -> Self
    where
        internal::Assert<{ u64_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits12BIts,
    {
        Self(VALUE as u16)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Csr` from an [usize] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_usize<const VALUE: usize>() -> Self
    where
        internal::Assert<{ usize_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits12BIts,
    {
        Self(VALUE as u16)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Csr` from an [i8] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i8<const VALUE: i8>() -> Self
    where
        internal::Assert<{ VALUE >= 0 }>: internal::Fits12BIts,
    {
        Self(VALUE as u16)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Csr` from an [i16] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i16<const VALUE: i16>() -> Self
    where
        internal::Assert<{ i16_fits_n_bits_unsigned(VALUE, Self::NBITS) }>: internal::Fits12BIts,
    {
        Self(VALUE as u16)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Csr` from an [i32] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i32<const VALUE: i32>() -> Self
    where
        internal::Assert<{ i32_fits_n_bits_unsigned(VALUE, Self::NBITS) }>: internal::Fits12BIts,
    {
        Self(VALUE as u16)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Csr` from an [i64] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i64<const VALUE: i64>() -> Self
    where
        internal::Assert<{ i64_fits_n_bits_unsigned(VALUE, Self::NBITS) }>: internal::Fits12BIts,
    {
        Self(VALUE as u16)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Csr` from an [isize] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_isize<const VALUE: isize>() -> Self
    where
        internal::Assert<{ isize_fits_n_bits_unsigned(VALUE, Self::NBITS) }>: internal::Fits12BIts,
    {
        Self(VALUE as u16)
    }

    pub(crate) const fn into_u32(self) -> u32 {
        self.0 as u32
    }
}

#[cfg(feature = "nightly")]
#[test]
fn constructors() {
    let _ = Csr::from_u8::<0xFF>();
    let _ = Csr::from_u16::<0xFFF>();
    let _ = Csr::from_u32::<0xFFF>();
    let _ = Csr::from_u64::<0xFFF>();
    let _ = Csr::from_usize::<0xFFF>();
    let _ = Csr::from_i8::<0b1111111>();
    let _ = Csr::from_i16::<0xFFF>();
    let _ = Csr::from_i32::<0xFFF>();
    let _ = Csr::from_i64::<0xFFF>();
    let _ = Csr::from_isize::<0xFFF>();
}

#[test]
fn into_u32() {
    assert_eq!(Csr(0xFF).into_u32(), 0xFF);
}

impl Display for Csr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[test]
fn display() -> Result<(), CsrConvError> {
    assert_eq!(Csr::try_from(0xFFF_u16)?.to_string(), "4095");
    Ok(())
}

impl From<u8> for Csr {
    fn from(value: u8) -> Self {
        Self(u16::from(value))
    }
}

impl TryFrom<u16> for Csr {
    type Error = CsrConvError;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if u16_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value))
        } else {
            Err(CsrConvError::U16(value))
        }
    }
}

impl TryFrom<u32> for Csr {
    type Error = CsrConvError;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if u32_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as u16))
        } else {
            Err(CsrConvError::U32(value))
        }
    }
}

impl TryFrom<u64> for Csr {
    type Error = CsrConvError;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if u64_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as u16))
        } else {
            Err(CsrConvError::U64(value))
        }
    }
}

impl TryFrom<usize> for Csr {
    type Error = CsrConvError;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if usize_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as u16))
        } else {
            Err(CsrConvError::Usize(value))
        }
    }
}

impl TryFrom<i8> for Csr {
    type Error = CsrConvError;

    #[allow(clippy::cast_sign_loss)]
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Self(value as u16))
        } else {
            Err(CsrConvError::I8(value))
        }
    }
}

impl TryFrom<i16> for Csr {
    type Error = CsrConvError;

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if i16_fits_n_bits_unsigned(value, Self::NBITS) {
            Ok(Self(value as u16))
        } else {
            Err(CsrConvError::I16(value))
        }
    }
}

impl TryFrom<i32> for Csr {
    type Error = CsrConvError;

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if i32_fits_n_bits_unsigned(value, Self::NBITS) {
            Ok(Self(value as u16))
        } else {
            Err(CsrConvError::I32(value))
        }
    }
}

impl TryFrom<i64> for Csr {
    type Error = CsrConvError;

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i64_fits_n_bits_unsigned(value, Self::NBITS) {
            Ok(Self(value as u16))
        } else {
            Err(CsrConvError::I64(value))
        }
    }
}

impl TryFrom<isize> for Csr {
    type Error = CsrConvError;

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if isize_fits_n_bits_unsigned(value, Self::NBITS) {
            Ok(Self(value as u16))
        } else {
            Err(CsrConvError::Isize(value))
        }
    }
}

#[test]
fn conversions() -> Result<(), CsrConvError> {
    assert_eq!(Csr::from(0xFF_u8), Csr(0xFF));
    assert_eq!(Csr::try_from(0xFFF_u16)?, Csr(0xFFF));
    assert_eq!(Csr::try_from(0xFFF_u32)?, Csr(0xFFF));
    assert_eq!(Csr::try_from(0xFFF_u64)?, Csr(0xFFF));
    assert_eq!(Csr::try_from(0xFFF_usize)?, Csr(0xFFF));
    assert_eq!(Csr::try_from(0b1111111_i8)?, Csr(0b1111111));
    assert_eq!(Csr::try_from(0xFFF_i16)?, Csr(0xFFF));
    assert_eq!(Csr::try_from(0xFFF_i32)?, Csr(0xFFF));
    assert_eq!(Csr::try_from(0xFFF_i64)?, Csr(0xFFF));
    assert_eq!(Csr::try_from(0xFFF_isize)?, Csr(0xFFF));

    assert!(matches!(Csr::try_from(-1_i8), Err(CsrConvError::I8(-1))));
    assert!(matches!(Csr::try_from(-1_i16), Err(CsrConvError::I16(-1))));
    assert!(matches!(Csr::try_from(-1_i32), Err(CsrConvError::I32(-1))));
    assert!(matches!(Csr::try_from(-1_i64), Err(CsrConvError::I64(-1))));
    assert!(matches!(
        Csr::try_from(-1_isize),
        Err(CsrConvError::Isize(-1))
    ));

    assert!(matches!(
        Csr::try_from(0x1000_u16),
        Err(CsrConvError::U16(4096))
    ));
    assert!(matches!(
        Csr::try_from(0x1000_u32),
        Err(CsrConvError::U32(4096))
    ));
    assert!(matches!(
        Csr::try_from(0x1000_u64),
        Err(CsrConvError::U64(4096))
    ));
    assert!(matches!(
        Csr::try_from(0x1000_usize),
        Err(CsrConvError::Usize(4096))
    ));

    Ok(())
}

/// Csr conversion error
#[derive(Debug)]
pub enum CsrConvError {
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
    format!("{:?}", CsrConvError::U8(32));
}

impl Display for CsrConvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid {}-bit unsigned immediate: ", Csr::NBITS)?;
        match self {
            CsrConvError::U8(value) => write!(f, "{value} (0x{value:02x})"),
            CsrConvError::U16(value) => write!(f, "{value} (0x{value:04x})"),
            CsrConvError::U32(value) => write!(f, "{value} (0x{value:08x})"),
            CsrConvError::U64(value) => write!(f, "{value} (0x{value:016x})"),
            CsrConvError::Usize(value) => write!(f, "{value}"),
            CsrConvError::I8(value) => write!(f, "{value} (0x{value:02x})"),
            CsrConvError::I16(value) => write!(f, "{value} (0x{value:04x})"),
            CsrConvError::I32(value) => write!(f, "{value} (0x{value:08x})"),
            CsrConvError::I64(value) => write!(f, "{value} (0x{value:016x})"),
            CsrConvError::Isize(value) => write!(f, "{value}"),
        }
    }
}

#[test]
fn conv_error_impl_display() {
    assert_eq!(
        Csr::try_from(-1_i8).unwrap_err().to_string(),
        format!("invalid {}-bit unsigned immediate: -1 (0xff)", Csr::NBITS)
    );
    assert_eq!(
        Csr::try_from(-1_i16).unwrap_err().to_string(),
        format!("invalid {}-bit unsigned immediate: -1 (0xffff)", Csr::NBITS)
    );
    assert_eq!(
        Csr::try_from(-1_i32).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: -1 (0xffffffff)",
            Csr::NBITS
        )
    );
    assert_eq!(
        Csr::try_from(-1_i64).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: -1 (0xffffffffffffffff)",
            Csr::NBITS
        )
    );
    assert_eq!(
        Csr::try_from(-1_isize).unwrap_err().to_string(),
        format!("invalid {}-bit unsigned immediate: -1", Csr::NBITS)
    );

    assert_eq!(
        Csr::try_from(0x1000_u16).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: 4096 (0x1000)",
            Csr::NBITS
        )
    );
    assert_eq!(
        Csr::try_from(0x1000_u16).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: 4096 (0x1000)",
            Csr::NBITS
        )
    );
    assert_eq!(
        Csr::try_from(0x1000_u32).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: 4096 (0x00001000)",
            Csr::NBITS
        )
    );
    assert_eq!(
        Csr::try_from(0x1000_u64).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: 4096 (0x0000000000001000)",
            Csr::NBITS
        )
    );
    assert_eq!(
        Csr::try_from(0x1000_usize).unwrap_err().to_string(),
        format!("invalid {}-bit unsigned immediate: 4096", Csr::NBITS)
    );
}

impl Error for CsrConvError {}

#[test]
fn conv_error_impl_error() -> Result<(), Box<dyn Error>> {
    assert_eq!(Csr::try_from(0u8)?, Csr(0));
    Ok(())
}

mod internal {
    pub enum Assert<const CHECK: bool> {}
    pub trait Fits12BIts {}
    impl Fits12BIts for Assert<true> {}
}
