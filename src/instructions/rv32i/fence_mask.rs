use bitvec::order::Lsb0;
use bitvec::{slice::BitSlice, view::BitView};
use std::{error::Error, fmt::Display};

/// Represents a mask for the [FENCE](super::fence) instruction
#[derive(Debug, PartialEq, Eq)]
pub struct FenceMask(u8);

// TODO: Implement as e.g. parse_fence_mask("rw") when const fns become able to do this

impl FenceMask {
    pub(crate) const RW: FenceMask = FenceMask(0b0011);

    pub(crate) fn view_bits(&self) -> &BitSlice<u8, Lsb0> {
        &self.0.view_bits()[0..4]
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

/// [Fence mask](FenceMask) parse error
#[derive(Debug, PartialEq)]
pub struct FenceMaskParseError<'a> {
    mask: &'a str,
    flag: char,
    kind: FenceMaskFlagErrorKind,
}

impl<'a> FenceMaskParseError<'a> {
    fn invalid(mask: &'a str, flag: char) -> Self {
        Self::new(mask, flag, FenceMaskFlagErrorKind::Invalid)
    }

    fn duplicate(mask: &'a str, flag: char) -> Self {
        Self::new(mask, flag, FenceMaskFlagErrorKind::Duplicate)
    }

    fn new(mask: &'a str, flag: char, kind: FenceMaskFlagErrorKind) -> Self {
        Self { mask, flag, kind }
    }
}

impl Display for FenceMaskParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"malformed fence mask "{}": {} flag '{}'"#,
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

#[test]
fn fence_mask() {
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
fn fence_mask_flag_error() {
    assert_eq!(
        FenceMaskParseError::invalid("iorwx", 'x').to_string(),
        r#"malformed fence mask "iorwx": invalid flag 'x'"#
    );
    assert_eq!(
        FenceMaskParseError::duplicate("rwr", 'r').to_string(),
        r#"malformed fence mask "rwr": duplicate flag 'r'"#
    );
}
