//! [`FenceMask`] implementation

use core::fmt;
use std::{
    error::Error,
    fmt::{Debug, Display, Write},
    hash::Hash,
};

use crate::util::{u16_fits_n_bits, u32_fits_n_bits, u64_fits_n_bits, u8_fits_n_bits};

/// 4-bit mask for the [fence](crate::instructions::strongly_typed::rv32i::fence) instruction
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FenceMask(u8);

impl FenceMask {
    const NBITS: usize = 4;
    const BIT_CHARS: &'static str = "wroi";

    pub(crate) const RW: Self = Self(0b0011);

    #[doc = include_str!("../../../../doc/nightly_warning.html")]
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

    #[doc = include_str!("../../../../doc/nightly_warning.html")]
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

    #[doc = include_str!("../../../../doc/nightly_warning.html")]
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

    #[doc = include_str!("../../../../doc/nightly_warning.html")]
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

    pub(crate) const fn into_u32(self) -> u32 {
        self.0 as u32
    }
}

#[cfg(feature = "nightly")]
#[test]
fn constructors() {
    assert_eq!(FenceMask::from_u8::<0b1111>(), FenceMask(0b1111));
    assert_eq!(FenceMask::from_u16::<0b1111>(), FenceMask(0b1111));
    assert_eq!(FenceMask::from_u32::<0b1111>(), FenceMask(0b1111));
    assert_eq!(FenceMask::from_u64::<0b1111>(), FenceMask(0b1111));
}

#[test]
fn into_u32() {
    assert_eq!(FenceMask::RW.into_u32(), 0b0011);
}

#[test]
fn derived_impls() {
    // Clone, Copy
    #[allow(clippy::clone_on_copy)]
    let rw = FenceMask::RW.clone();

    fn _dummy(_: impl Eq) {}
    _dummy(FenceMask::RW);

    // Hash
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    rw.hash(&mut hasher);
}

impl Debug for FenceMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FenceMask(0b{:08b})", self.0)
    }
}

// Satisfy grcov
#[test]
fn debug() {
    format!("{:?}", FenceMask::RW);
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
    assert_eq!(FenceMask(0).to_string(), "");
    assert_eq!(FenceMask(0b1111).to_string(), FenceMask::BIT_CHARS);
    Ok(())
}

impl TryFrom<u8> for FenceMask {
    type Error = FenceMaskConvError;

    /// Creates a `FenceMask` from [u8], the lower 4 bits of which specify the flags to be set:
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

    /// Creates a `FenceMask` from [u16], the lower 4 bits of which specify the flags to be set:
    ///
    /// | Bit | Flag          |
    /// |-----|---------------|
    /// | 0   | memory writes |
    /// | 1   | memory reads  |
    /// | 2   | device output |
    /// | 3   | device input  |
    #[allow(clippy::cast_possible_truncation)]
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

    /// Creates a `FenceMask` from [u32], the lower 4 bits of which specify the flags to be set:
    ///
    /// | Bit | Flag          |
    /// |-----|---------------|
    /// | 0   | memory writes |
    /// | 1   | memory reads  |
    /// | 2   | device output |
    /// | 3   | device input  |
    #[allow(clippy::cast_possible_truncation)]
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

    /// Creates a `FenceMask` from [u64], the lower 4 bits of which specify the flags to be set:
    ///
    /// | Bit | Flag          |
    /// |-----|---------------|
    /// | 0   | memory writes |
    /// | 1   | memory reads  |
    /// | 2   | device output |
    /// | 3   | device input  |
    #[allow(clippy::cast_possible_truncation)]
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

    /// Creates a `FenceMask` from an [str], the characters of which specify the flags to be set:
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
            }

            mask |= 1 << index;
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

    assert!(matches!(FenceMask::try_from(""), Ok(FenceMask(0b0000))));
    assert!(matches!(FenceMask::try_from("r"), Ok(FenceMask(0b0010))));
    assert!(matches!(FenceMask::try_from("w"), Ok(FenceMask(0b0001))));
    assert!(matches!(FenceMask::try_from("i"), Ok(FenceMask(0b1000))));
    assert!(matches!(FenceMask::try_from("o"), Ok(FenceMask(0b0100))));
    assert!(matches!(FenceMask::try_from("riow"), Ok(FenceMask(0b1111))));

    assert!(matches!(
        FenceMask::try_from("rwr"),
        Err(FenceMaskParseError {
            mask: "rwr",
            flag: 'r',
            kind: FenceMaskFlagErrorKind::Duplicate
        })
    ));

    assert!(matches!(
        FenceMask::try_from("iorwx"),
        Err(FenceMaskParseError {
            mask: "iorwx",
            flag: 'x',
            kind: FenceMaskFlagErrorKind::Invalid
        })
    ));

    Ok(())
}

/// `FenceMask` conversion error
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

// Satisfy grcov
#[test]
fn conv_error_impl_debug() {
    format!("{:?}", FenceMaskConvError::U8(0));
}

impl Display for FenceMaskConvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid {}-bit unsigned immediate: ", FenceMask::NBITS)?;
        match self {
            FenceMaskConvError::U8(value) => write!(f, "{value} (0x{value:02x})"),
            FenceMaskConvError::U16(value) => write!(f, "{value} (0x{value:04x})"),
            FenceMaskConvError::U32(value) => write!(f, "{value} (0x{value:08x})"),
            FenceMaskConvError::U64(value) => write!(f, "{value} (0x{value:016x})"),
        }
    }
}

#[test]
fn conv_error_impl_display() {
    assert_eq!(
        FenceMask::try_from(0b10_0000_u8).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: 32 (0x20)",
            FenceMask::NBITS
        )
    );
    assert_eq!(
        FenceMask::try_from(0b10_0000_u16).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: 32 (0x0020)",
            FenceMask::NBITS
        )
    );
    assert_eq!(
        FenceMask::try_from(0b10_0000_u32).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: 32 (0x00000020)",
            FenceMask::NBITS
        )
    );
    assert_eq!(
        FenceMask::try_from(0b10_0000_u64).unwrap_err().to_string(),
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
#[derive(Debug)]
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

// Satisfy grcov
#[test]
fn parse_error_impl_debug() {
    format!("{:?}", FenceMaskParseError::duplicate("x", 'x'));
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

#[derive(Debug)]
enum FenceMaskFlagErrorKind {
    Invalid,
    Duplicate,
}

// Satisfy grcov
#[test]
fn error_kind_debug() {
    format!("{:?}", FenceMaskFlagErrorKind::Invalid);
}

impl Display for FenceMaskFlagErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Invalid => "invalid",
            Self::Duplicate => "duplicate",
        })
    }
}

#[test]
fn error_kind_display() {
    assert_eq!(FenceMaskFlagErrorKind::Invalid.to_string(), "invalid");
    assert_eq!(FenceMaskFlagErrorKind::Duplicate.to_string(), "duplicate");
}

mod internal {
    pub enum Assert<const CHECK: bool> {}
    pub trait Fits4BIts {}
    impl Fits4BIts for Assert<true> {}
}
