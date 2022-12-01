//! Defines [FenceMask] and relevant trait implementations

use core::fmt;
use std::{
    error::Error,
    fmt::{
        Display,
        Write,
    },
};

use crate::util::{
    u16_fits_n_bits,
    u32_fits_n_bits,
    u64_fits_n_bits,
    u8_fits_n_bits,
};

mod internal {
    pub enum Assert<const CHECK: bool> {}
    pub trait Fits4BIts {}
    impl Fits4BIts for Assert<true> {}
}

/// 4-bit mask for the [fence](super::fence) instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FenceMask(u8);

impl FenceMask {
    const NBITS: usize = 4;
    const BIT_CHARS: &'static str = "wroi";

    pub(crate) const RW: Self = Self(0b0011);

    #[doc = include_str!("../../../doc/nightly_warning.html")]
    ///
    /// Creates an `FenceMask` from an [u8] constant, the lower 4 bits of which specify the flags to be set:
    ///
    /// | Bit | Flag          |
    /// |-----|---------------|
    /// | 0   | memory writes |
    /// | 1   | memory reads  |
    /// | 2   | device output |
    /// | 3   | device input  |
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u8<const VALUE: u8>() -> Self
    where
        internal::Assert<{ u8_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits4BIts,
    {
        Self(VALUE)
    }

    #[doc = include_str!("../../../doc/nightly_warning.html")]
    ///
    /// Creates an `FenceMask` from an [u16] constant, the lower 4 bits of which specify the flags to be set:
    ///
    /// | Bit | Flag          |
    /// |-----|---------------|
    /// | 0   | memory writes |
    /// | 1   | memory reads  |
    /// | 2   | device output |
    /// | 3   | device input  |
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u16<const VALUE: u16>() -> Self
    where
        internal::Assert<{ u16_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits4BIts,
    {
        Self(VALUE as u8)
    }

    #[doc = include_str!("../../../doc/nightly_warning.html")]
    ///
    /// Creates an `FenceMask` from an [u32] constant, the lower 4 bits of which specify the flags to be set:
    ///
    /// | Bit | Flag          |
    /// |-----|---------------|
    /// | 0   | memory writes |
    /// | 1   | memory reads  |
    /// | 2   | device output |
    /// | 3   | device input  |
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u32<const VALUE: u32>() -> Self
    where
        internal::Assert<{ u32_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits4BIts,
    {
        Self(VALUE as u8)
    }

    #[doc = include_str!("../../../doc/nightly_warning.html")]
    ///
    /// Creates an `FenceMask` from an [u64] constant, the lower 4 bits of which specify the flags to be set:
    ///
    /// | Bit | Flag          |
    /// |-----|---------------|
    /// | 0   | memory writes |
    /// | 1   | memory reads  |
    /// | 2   | device output |
    /// | 3   | device input  |
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u64<const VALUE: u64>() -> Self
    where
        internal::Assert<{ u64_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits4BIts,
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
    let _ = FenceMask::from_u8::<0b1111>();
    let _ = FenceMask::from_u16::<0b1111>();
    let _ = FenceMask::from_u32::<0b1111>();
    let _ = FenceMask::from_u64::<0b1111>();
}

impl Display for FenceMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, c) in Self::BIT_CHARS.char_indices() {
            if (self.0 >> i) & 1 == 1 {
                f.write_char(c)?;
            }
        }
        Ok(())
    }
}

#[test]
fn display() -> Result<(), FenceMaskConvError> {
    assert_eq!(
        FenceMask::try_from(0b1111u8)?.to_string(),
        FenceMask::BIT_CHARS
    );
    Ok(())
}

impl TryFrom<u8> for FenceMask {
    type Error = FenceMaskConvError;

    /// Creates a FenceMask from [u8], the lower 4 bits of which specify the flags to be set:
    ///
    /// | Bit | Flag          |
    /// |-----|---------------|
    /// | 0   | memory writes |
    /// | 1   | memory reads  |
    /// | 2   | device output |
    /// | 3   | device input  |
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if u8_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value))
        } else {
            Err(FenceMaskConvError::U8(value))
        }
    }
}

impl TryFrom<u16> for FenceMask {
    type Error = FenceMaskConvError;

    /// Creates a FenceMask from [u16], the lower 4 bits of which specify the flags to be set:
    ///
    /// | Bit | Flag          |
    /// |-----|---------------|
    /// | 0   | memory writes |
    /// | 1   | memory reads  |
    /// | 2   | device output |
    /// | 3   | device input  |
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if u16_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as u8))
        } else {
            Err(FenceMaskConvError::U16(value))
        }
    }
}

impl TryFrom<u32> for FenceMask {
    type Error = FenceMaskConvError;

    /// Creates a FenceMask from [u32], the lower 4 bits of which specify the flags to be set:
    ///
    /// | Bit | Flag          |
    /// |-----|---------------|
    /// | 0   | memory writes |
    /// | 1   | memory reads  |
    /// | 2   | device output |
    /// | 3   | device input  |
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if u32_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as u8))
        } else {
            Err(FenceMaskConvError::U32(value))
        }
    }
}

impl TryFrom<u64> for FenceMask {
    type Error = FenceMaskConvError;

    /// Creates a FenceMask from [u64], the lower 4 bits of which specify the flags to be set:
    ///
    /// | Bit | Flag          |
    /// |-----|---------------|
    /// | 0   | memory writes |
    /// | 1   | memory reads  |
    /// | 2   | device output |
    /// | 3   | device input  |
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if u64_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as u8))
        } else {
            Err(FenceMaskConvError::U64(value))
        }
    }
}

impl<'a> TryFrom<&'a str> for FenceMask {
    type Error = FenceMaskParseError<'a>;

    /// Creates a FenceMask from an [str], the characters of which specify the flags to be set:
    ///
    /// | Bit | Flag          |
    /// |-----|---------------|
    /// | w   | memory writes |
    /// | r   | memory reads  |
    /// | o   | device output |
    /// | i   | device input  |
    fn try_from(mask_spec: &'a str) -> Result<Self, Self::Error> {
        let mut mask = 0;
        for flag in mask_spec.chars() {
            let Some(index) = "wroi".find(flag) else {
                return Err(FenceMaskParseError::invalid(mask_spec, flag));
            };

            if (mask >> index) & 1 == 1 {
                return Err(FenceMaskParseError::duplicate(mask_spec, flag));
            } else {
                mask |= 1 << index;
            }
        }
        Ok(Self(mask))
    }
}

#[test]
fn conversions() -> Result<(), FenceMaskConvError> {
    assert_eq!(FenceMask::try_from(0b1111u8)?, FenceMask(0b1111));
    assert_eq!(FenceMask::try_from(0b1111u16)?, FenceMask(0b1111));
    assert_eq!(FenceMask::try_from(0b1111u32)?, FenceMask(0b1111));
    assert_eq!(FenceMask::try_from(0b1111u64)?, FenceMask(0b1111));

    assert!(matches!(
        FenceMask::try_from(0b10000u8),
        Err(FenceMaskConvError::U8(0b10000))
    ));
    assert!(matches!(
        FenceMask::try_from(0b10000u16),
        Err(FenceMaskConvError::U16(0b10000))
    ));
    assert!(matches!(
        FenceMask::try_from(0b10000u32),
        Err(FenceMaskConvError::U32(0b10000))
    ));
    assert!(matches!(
        FenceMask::try_from(0b10000u64),
        Err(FenceMaskConvError::U64(0b10000))
    ));

    assert_eq!(FenceMask::try_from(""), Ok(FenceMask(0b0000)));
    assert_eq!(FenceMask::try_from("r"), Ok(FenceMask(0b0010)));
    assert_eq!(FenceMask::try_from("w"), Ok(FenceMask(0b0001)));
    assert_eq!(FenceMask::try_from("i"), Ok(FenceMask(0b1000)));
    assert_eq!(FenceMask::try_from("o"), Ok(FenceMask(0b0100)));
    assert_eq!(FenceMask::try_from("riow"), Ok(FenceMask(0b1111)));

    assert_eq!(
        FenceMask::try_from("rwr"),
        Err(FenceMaskParseError::duplicate("rwr", 'r'))
    );

    assert_eq!(
        FenceMask::try_from("iorwx"),
        Err(FenceMaskParseError::invalid("iorwx", 'x'))
    );

    Ok(())
}

/// FenceMask conversion error
#[derive(Debug)]
pub enum FenceMaskConvError {
    ///
    U8(u8),
    ///
    U16(u16),
    ///
    U32(u32),
    ///
    U64(u64),
}

impl Display for FenceMaskConvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid {}-bit unsigned immediate: ", FenceMask::NBITS)?;
        match self {
            FenceMaskConvError::U8(value) => write!(f, "{} (0x{:02x})", value, value),
            FenceMaskConvError::U16(value) => write!(f, "{} (0x{:04x})", value, value),
            FenceMaskConvError::U32(value) => write!(f, "{} (0x{:08x})", value, value),
            FenceMaskConvError::U64(value) => write!(f, "{} (0x{:016x})", value, value),
        }
    }
}

#[test]
fn conv_error_impl_display() {
    assert_eq!(
        FenceMask::try_from(0b100000u8).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: 32 (0x20)",
            FenceMask::NBITS
        )
    );
    assert_eq!(
        FenceMask::try_from(0b100000u16).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: 32 (0x0020)",
            FenceMask::NBITS
        )
    );
    assert_eq!(
        FenceMask::try_from(0b100000u32).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: 32 (0x00000020)",
            FenceMask::NBITS
        )
    );
    assert_eq!(
        FenceMask::try_from(0b100000u64).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: 32 (0x0000000000000020)",
            FenceMask::NBITS
        )
    );
}

impl Error for FenceMaskConvError {}

#[test]
fn conv_error_impl_error() -> Result<(), Box<dyn Error>> {
    assert_eq!(FenceMask::try_from(0u8)?, FenceMask(0));
    Ok(())
}

/// [Fence mask](FenceMask) parse error
#[derive(Debug, PartialEq)]
pub struct FenceMaskParseError<'a> {
    mask: &'a str,
    flag: char,
    kind: FenceMaskFlagErrorKind,
}

impl<'a> FenceMaskParseError<'a> {
    fn invalid(mask: &'a str, flag: char) -> Self {
        Self {
            mask,
            flag,
            kind: FenceMaskFlagErrorKind::Invalid,
        }
    }

    fn duplicate(mask: &'a str, flag: char) -> Self {
        Self {
            mask,
            flag,
            kind: FenceMaskFlagErrorKind::Duplicate,
        }
    }
}

impl Display for FenceMaskParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"invalid fence mask "{}": {} flag '{}'"#,
            self.mask, self.kind, self.flag
        )
    }
}

impl Error for FenceMaskParseError<'_> {}

#[test]
fn parse_error_impl_display() {
    assert_eq!(
        FenceMaskParseError::invalid("iorwx", 'x').to_string(),
        r#"invalid fence mask "iorwx": invalid flag 'x'"#
    );
    assert_eq!(
        FenceMaskParseError::duplicate("rwr", 'r').to_string(),
        r#"invalid fence mask "rwr": duplicate flag 'r'"#
    );
}

#[test]
fn parse_error_impl_error() -> Result<(), Box<dyn Error>> {
    assert_eq!(FenceMask::try_from("")?, FenceMask(0));
    Ok(())
}

#[derive(Debug, PartialEq)]
enum FenceMaskFlagErrorKind {
    Invalid,
    Duplicate,
}

impl Display for FenceMaskFlagErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Invalid => "invalid",
            Self::Duplicate => "duplicate",
        })
    }
}
