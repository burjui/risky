//! Defines [Imm12] and relevant trait implementations

use core::fmt;
use std::{
    error::Error,
    fmt::Display,
    ops::Neg,
};

use crate::util::{
    i16_fits_n_bits,
    i32_fits_n_bits,
    i64_fits_n_bits,
    isize_fits_n_bits,
    u16_fits_n_bits,
    u32_fits_n_bits,
    u64_fits_n_bits,
    usize_fits_n_bits,
};

/// 12-bit signed immediate value
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Imm12(pub(crate) i16);

impl Imm12 {
    const NBITS: usize = 12;

    /// Zero
    pub const ZERO: Self = Self(0);

    pub(crate) const ONE: Self = Self(1);
    pub(crate) const MINUS_ONE: Self = Self(-1);

    /// Creates an `Imm12` from an [i8] constant
    #[must_use]
    pub const fn from_i8<const VALUE: i8>() -> Self {
        Self(VALUE as i16)
    }

    /// Creates an `Imm12` from an [u8] constant
    #[must_use]
    pub const fn from_u8<const VALUE: u8>() -> Self {
        Self(VALUE as i16)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Imm12` from an [i16] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i16<const VALUE: i16>() -> Self
    where
        internal::Assert<{ i16_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits12BIts,
    {
        Self(VALUE)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Imm12` from an [i32] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i32<const VALUE: i32>() -> Self
    where
        internal::Assert<{ i32_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits12BIts,
    {
        Self(VALUE as i16)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Imm12` from an [i64] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i64<const VALUE: i64>() -> Self
    where
        internal::Assert<{ i64_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits12BIts,
    {
        Self(VALUE as i16)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Imm12` from an [isize] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_isize<const VALUE: isize>() -> Self
    where
        internal::Assert<{ isize_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits12BIts,
    {
        Self(VALUE as i16)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Imm12` from an [u16] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u16<const VALUE: u16>() -> Self
    where
        internal::Assert<{ u16_fits_n_bits(VALUE, Self::NBITS - 1) }>: internal::Fits12BIts,
    {
        Self(VALUE as i16)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Imm12` from an [u32] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u32<const VALUE: u32>() -> Self
    where
        internal::Assert<{ u32_fits_n_bits(VALUE, Self::NBITS - 1) }>: internal::Fits12BIts,
    {
        Self(VALUE as i16)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Imm12` from an [u64] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u64<const VALUE: u64>() -> Self
    where
        internal::Assert<{ u64_fits_n_bits(VALUE, Self::NBITS - 1) }>: internal::Fits12BIts,
    {
        Self(VALUE as i16)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates an `Imm12` from an [usize] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_usize<const VALUE: usize>() -> Self
    where
        internal::Assert<{ usize_fits_n_bits(VALUE, Self::NBITS - 1) }>: internal::Fits12BIts,
    {
        Self(VALUE as i16)
    }

    pub(crate) const fn to_u32(self) -> u32 {
        self.0 as u32
    }
}

#[cfg(feature = "nightly")]
#[test]
fn constructors() {
    let _ = Imm12::from_i8::<-128>();
    let _ = Imm12::from_i8::<127>();
    let _ = Imm12::from_u8::<255>();
    let _ = Imm12::from_i16::<-2048>();
    let _ = Imm12::from_i16::<2047>();
    let _ = Imm12::from_u16::<2047>();
    let _ = Imm12::from_i32::<-2048>();
    let _ = Imm12::from_i32::<2047>();
    let _ = Imm12::from_u32::<2047>();
    let _ = Imm12::from_i64::<-2048>();
    let _ = Imm12::from_i64::<2047>();
    let _ = Imm12::from_u64::<2047>();
    let _ = Imm12::from_isize::<-2048>();
    let _ = Imm12::from_isize::<2047>();
    let _ = Imm12::from_usize::<2047>();
}

impl Display for Imm12 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&(self.0 as i32), f)
    }
}

#[test]
fn display() -> Result<(), Imm12ConvError> {
    assert_eq!(Imm12::try_from(-2048)?.to_string(), "-2048");
    assert_eq!(Imm12::try_from(2047)?.to_string(), "2047");
    Ok(())
}

impl Neg for Imm12 {
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
    assert_eq!(-Imm12(2047), Imm12(-2047));
    assert_eq!(-Imm12(-2047), Imm12(2047));
    assert_eq!(-Imm12(-2048), Imm12(-2048));
}

impl From<i8> for Imm12 {
    fn from(value: i8) -> Self {
        Self(value as i16)
    }
}

impl From<u8> for Imm12 {
    fn from(value: u8) -> Self {
        Self(value as i16)
    }
}

impl TryFrom<i16> for Imm12 {
    type Error = Imm12ConvError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if i16_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value))
        } else {
            Err(Imm12ConvError::I16(value))
        }
    }
}

impl TryFrom<i32> for Imm12 {
    type Error = Imm12ConvError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if i32_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as i16))
        } else {
            Err(Imm12ConvError::I32(value))
        }
    }
}

impl TryFrom<i64> for Imm12 {
    type Error = Imm12ConvError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i64_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as i16))
        } else {
            Err(Imm12ConvError::I64(value))
        }
    }
}

impl TryFrom<isize> for Imm12 {
    type Error = Imm12ConvError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if isize_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as i16))
        } else {
            Err(Imm12ConvError::Isize(value))
        }
    }
}

impl TryFrom<u16> for Imm12 {
    type Error = Imm12ConvError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if u16_fits_n_bits(value, Self::NBITS - 1) {
            Ok(Self(value as i16))
        } else {
            Err(Imm12ConvError::U16(value))
        }
    }
}

impl TryFrom<u32> for Imm12 {
    type Error = Imm12ConvError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if u32_fits_n_bits(value, Self::NBITS - 1) {
            Ok(Self(value as i16))
        } else {
            Err(Imm12ConvError::U32(value))
        }
    }
}

impl TryFrom<u64> for Imm12 {
    type Error = Imm12ConvError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if u64_fits_n_bits(value, Self::NBITS - 1) {
            Ok(Self(value as i16))
        } else {
            Err(Imm12ConvError::U64(value))
        }
    }
}

impl TryFrom<usize> for Imm12 {
    type Error = Imm12ConvError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if usize_fits_n_bits(value, Self::NBITS - 1) {
            Ok(Self(value as i16))
        } else {
            Err(Imm12ConvError::Usize(value))
        }
    }
}

#[test]
fn conversions() -> Result<(), Imm12ConvError> {
    assert_eq!(Imm12::try_from(-2048_i16)?, Imm12(-2048));
    assert_eq!(Imm12::try_from(2047_i16)?, Imm12(2047));
    assert!(matches!(
        Imm12::try_from(-2049_i16),
        Err(Imm12ConvError::I16(-2049))
    ));
    assert!(matches!(
        dbg!(Imm12::try_from(2048_i16)),
        Err(Imm12ConvError::I16(2048))
    ));

    assert_eq!(Imm12::try_from(2047_u16)?, Imm12(2047));
    assert!(matches!(
        Imm12::try_from(2048_u16),
        Err(Imm12ConvError::U16(2048))
    ));

    assert_eq!(Imm12::try_from(-2048_i32)?, Imm12(-2048));
    assert_eq!(Imm12::try_from(2047_i32)?, Imm12(2047));
    assert!(matches!(
        Imm12::try_from(-2049_i32),
        Err(Imm12ConvError::I32(-2049))
    ));
    assert!(matches!(
        Imm12::try_from(2048_i32),
        Err(Imm12ConvError::I32(2048))
    ));

    assert_eq!(Imm12::try_from(2047_u32)?, Imm12(2047));
    assert!(matches!(
        Imm12::try_from(2048_u32),
        Err(Imm12ConvError::U32(2048))
    ));

    assert_eq!(Imm12::try_from(-2048_i64)?, Imm12(-2048));
    assert_eq!(Imm12::try_from(2047_i64)?, Imm12(2047));
    assert!(matches!(
        Imm12::try_from(-2049_i64),
        Err(Imm12ConvError::I64(-2049))
    ));
    assert!(matches!(
        Imm12::try_from(2048_i64),
        Err(Imm12ConvError::I64(2048))
    ));

    assert_eq!(Imm12::try_from(2047_u64)?, Imm12(2047));
    assert!(matches!(
        Imm12::try_from(2048_u64),
        Err(Imm12ConvError::U64(2048))
    ));

    assert_eq!(Imm12::try_from(-2048_isize)?, Imm12(-2048));
    assert_eq!(Imm12::try_from(2047_isize)?, Imm12(2047));
    assert!(matches!(
        Imm12::try_from(-2049_isize),
        Err(Imm12ConvError::Isize(-2049))
    ));
    assert!(matches!(
        Imm12::try_from(2048_isize),
        Err(Imm12ConvError::Isize(2048))
    ));

    assert_eq!(Imm12::try_from(2047_usize)?, Imm12(2047));
    assert!(matches!(
        Imm12::try_from(2048_usize),
        Err(Imm12ConvError::Usize(2048))
    ));

    Ok(())
}

/// [Imm12] conversion error
#[derive(Debug)]
pub enum Imm12ConvError {
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

impl Display for Imm12ConvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid {}-bit signed immediate: ", Imm12::NBITS)?;
        match self {
            Imm12ConvError::I16(value) => write!(f, "{} (0x{:04x})", value, value),
            Imm12ConvError::I32(value) => write!(f, "{} (0x{:08x})", value, value),
            Imm12ConvError::I64(value) => write!(f, "{} (0x{:016x})", value, value),
            Imm12ConvError::Isize(value) => write!(f, "{}", value),
            Imm12ConvError::U16(value) => write!(f, "{} (0x{:04x})", value, value),
            Imm12ConvError::U32(value) => write!(f, "{} (0x{:08x})", value, value),
            Imm12ConvError::U64(value) => write!(f, "{} (0x{:016x})", value, value),
            Imm12ConvError::Usize(value) => write!(f, "{}", value),
        }
    }
}
#[test]
fn conv_error_impl_display() {
    assert_eq!(
        Imm12::try_from(-2049_i16).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: -2049 (0xf7ff)",
            Imm12::NBITS
        )
    );
    assert_eq!(
        Imm12::try_from(2048_i16).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: 2048 (0x0800)",
            Imm12::NBITS
        )
    );

    assert_eq!(
        Imm12::try_from(2048_u16).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: 2048 (0x0800)",
            Imm12::NBITS
        )
    );

    assert_eq!(
        Imm12::try_from(-2049_i32).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: -2049 (0xfffff7ff)",
            Imm12::NBITS
        )
    );
    assert_eq!(
        Imm12::try_from(2048_i32).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: 2048 (0x00000800)",
            Imm12::NBITS
        )
    );

    assert_eq!(
        Imm12::try_from(2048_u32).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: 2048 (0x00000800)",
            Imm12::NBITS
        )
    );

    assert_eq!(
        Imm12::try_from(-2049_i64).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: -2049 (0xfffffffffffff7ff)",
            Imm12::NBITS
        )
    );
    assert_eq!(
        Imm12::try_from(2048_i64).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: 2048 (0x0000000000000800)",
            Imm12::NBITS
        )
    );

    assert_eq!(
        Imm12::try_from(2048_u64).unwrap_err().to_string(),
        format!(
            "invalid {}-bit signed immediate: 2048 (0x0000000000000800)",
            Imm12::NBITS
        )
    );

    assert_eq!(
        Imm12::try_from(-2049_isize).unwrap_err().to_string(),
        format!("invalid {}-bit signed immediate: -2049", Imm12::NBITS)
    );
    assert_eq!(
        Imm12::try_from(2048_isize).unwrap_err().to_string(),
        format!("invalid {}-bit signed immediate: 2048", Imm12::NBITS)
    );

    assert_eq!(
        Imm12::try_from(2048_usize).unwrap_err().to_string(),
        format!("invalid {}-bit signed immediate: 2048", Imm12::NBITS)
    );
}

impl Error for Imm12ConvError {}

#[test]
fn conv_error_impl_error() -> Result<(), Box<dyn Error>> {
    assert_eq!(Imm12::try_from(0)?, Imm12(0));
    Ok(())
}
mod internal {
    pub enum Assert<const CHECK: bool> {}
    pub trait Fits12BIts {}
    impl Fits12BIts for Assert<true> {}
}
