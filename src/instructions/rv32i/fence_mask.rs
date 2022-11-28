use core::fmt;
use std::{error::Error, fmt::Display, ops::Range};

use bitvec::{order::Lsb0, slice::BitSlice, view::BitView};

use crate::util::u8_max_value;

/// 4-bit mask for the [fence](super::fence) instruction
#[derive(Debug, PartialEq, Eq)]
pub struct FenceMask(u8);

// TODO: Implement as e.g. parse_fence_mask("rw") when const fns become able to do this

impl FenceMask {
    const BIT_RANGE: Range<usize> = 0..4;

    /// Creates a FenceMask from an [str], the characters of which specify the flags to be set:
    ///
    pub fn try_from_str(mask_spec: &str) -> Result<Self, FenceMaskParseError<'_>> {
        Self::try_from(mask_spec)
    }

    pub(crate) const RW: FenceMask = FenceMask(0b0011);

    pub(crate) fn view_bits(&self) -> &BitSlice<u8, Lsb0> {
        &self.0.view_bits()[Self::BIT_RANGE]
    }
}

impl<'a> TryFrom<&'a str> for FenceMask {
    type Error = FenceMaskParseError<'a>;

    fn try_from(mask_spec: &'a str) -> Result<Self, Self::Error> {
        let mut mask = 0;
        let mask_bits = &mut mask.view_bits_mut::<Lsb0>()[0..4];
        for flag in mask_spec.chars() {
            let Some(index) = "wroi".find(flag) else {
                return Err(FenceMaskParseError::invalid(mask_spec, flag));
            };

            if mask_bits[index] {
                return Err(FenceMaskParseError::duplicate(mask_spec, flag));
            } else {
                mask_bits.set(index, true)
            }
        }
        Ok(Self(mask))
    }
}

impl TryFrom<u8> for FenceMask {
    type Error = FenceMaskConvError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= u8_max_value(Self::BIT_RANGE.end) {
            Ok(Self(value))
        } else {
            Err(FenceMaskConvError(value))
        }
    }
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

/// [FenceMask] conversion error
#[derive(Debug, PartialEq, Eq)]
pub struct FenceMaskConvError(u8);

impl Display for FenceMaskConvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid 4-bit fence mask: {} (0x{:02x})", self.0, self.0)
    }
}

#[test]
fn fence_mask_str() {
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
}

#[test]
fn fence_mask_u8() {
    assert_eq!(FenceMask::try_from(0b0000), Ok(FenceMask(0b0000)));
    assert_eq!(FenceMask::try_from(0b0010), Ok(FenceMask(0b0010)));
    assert_eq!(FenceMask::try_from(0b0001), Ok(FenceMask(0b0001)));
    assert_eq!(FenceMask::try_from(0b1000), Ok(FenceMask(0b1000)));
    assert_eq!(FenceMask::try_from(0b0100), Ok(FenceMask(0b0100)));
    assert_eq!(FenceMask::try_from(0b1111), Ok(FenceMask(0b1111)));

    assert_eq!(
        FenceMask::try_from(0b10000),
        Err(FenceMaskConvError(0b10000))
    );
}

#[test]
fn fence_mask_parse_error() {
    assert_eq!(
        FenceMaskParseError::invalid("iorwx", 'x').to_string(),
        r#"invalid fence mask "iorwx": invalid flag 'x'"#
    );
    assert_eq!(
        FenceMaskParseError::duplicate("rwr", 'r').to_string(),
        r#"invalid fence mask "rwr": duplicate flag 'r'"#
    );
}
