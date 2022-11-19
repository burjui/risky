use bitvec::order::Lsb0;
use bitvec::{slice::BitSlice, view::BitView};
use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub struct FenceMask(u8);

// TODO: Implement as e.g. parse_fence_mask("rw") when const fns become able to do this
pub(crate) const FENCE_MASK_RW: FenceMask = FenceMask(0b0011);

impl FenceMask {
    pub(crate) fn view_bits(&self) -> &BitSlice<u8, Lsb0> {
        &self.0.view_bits()[0..4]
    }
}

impl<'a> TryFrom<&'a str> for FenceMask {
    type Error = FenceMaskFlagError<'a>;

    fn try_from(mask_spec: &'a str) -> Result<Self, Self::Error> {
        let mut mask = 0;
        let mask_bits = &mut mask.view_bits_mut::<Lsb0>()[0..4];
        for flag in mask_spec.chars() {
            let Some(index) = "wroi".find(flag) else {
                return Err(FenceMaskFlagError::invalid(mask_spec, flag));
            };

            if mask_bits[index] {
                return Err(FenceMaskFlagError::duplicate(mask_spec, flag));
            } else {
                mask_bits.set(index, true)
            }
        }
        Ok(Self(mask))
    }
}

#[derive(Debug, PartialEq)]
pub struct FenceMaskFlagError<'a> {
    mask: &'a str,
    flag: char,
    kind: FenceMaskFlagErrorKind,
}

impl<'a> FenceMaskFlagError<'a> {
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

impl Display for FenceMaskFlagError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"malformed fence mask "{}": {} flag '{}'"#,
            self.mask, self.kind, self.flag
        )
    }
}

impl Error for FenceMaskFlagError<'_> {}

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
        Err(FenceMaskFlagError::duplicate("rwr", 'r'))
    );

    assert_eq!(
        FenceMask::try_from("iorwx"),
        Err(FenceMaskFlagError::invalid("iorwx", 'x'))
    );
}

#[test]
fn fence_mask_flag_error() {
    assert_eq!(
        FenceMaskFlagError::invalid("iorwx", 'x').to_string(),
        r#"malformed fence mask "iorwx": invalid flag 'x'"#
    );
    assert_eq!(
        FenceMaskFlagError::duplicate("rwr", 'r').to_string(),
        r#"malformed fence mask "rwr": duplicate flag 'r'"#
    );
}
