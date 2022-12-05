/*!
RV32 Register definitions.

Based on the following document:
> ["RISC-V ABIs Specification, Document Version 1.0"](https://github.com/riscv-non-isa/riscv-elf-psabi-doc),
Editors Kito Cheng and Jessica Clarke, RISC-V International, November 2022.
*/

use core::fmt;
use std::{
    error::Error,
    fmt::{Debug, Display, Write},
};

use crate::util::{
    u16_fits_n_bits, u32_fits_n_bits, u64_fits_n_bits, u8_fits_n_bits, usize_fits_n_bits,
};

/// Number of `RISC-V` registers
pub const NUMBER_OF_REGISTERS: usize = 32;

/// Hard-wired zero, ignores writes
pub const X0: Register = Register(0);
/// General-purpose register 1
pub const X1: Register = Register(1);
/// General-purpose register 2
pub const X2: Register = Register(2);
/// General-purpose register 3
pub const X3: Register = Register(3);
/// General-purpose register 4
pub const X4: Register = Register(4);
/// General-purpose register 5
pub const X5: Register = Register(5);
/// General-purpose register 6
pub const X6: Register = Register(6);
/// General-purpose register 7
pub const X7: Register = Register(7);
/// General-purpose register 8
pub const X8: Register = Register(8);
/// General-purpose register 9
pub const X9: Register = Register(9);
/// General-purpose register 10
pub const X10: Register = Register(10);
/// General-purpose register 11
pub const X11: Register = Register(11);
/// General-purpose register 12
pub const X12: Register = Register(12);
/// General-purpose register 13
pub const X13: Register = Register(13);
/// General-purpose register 14
pub const X14: Register = Register(14);
/// General-purpose register 15
pub const X15: Register = Register(15);
/// General-purpose register 16
pub const X16: Register = Register(16);
/// General-purpose register 17
pub const X17: Register = Register(17);
/// General-purpose register 18
pub const X18: Register = Register(18);
/// General-purpose register 19
pub const X19: Register = Register(19);
/// General-purpose register 20
pub const X20: Register = Register(20);
/// General-purpose register 21
pub const X21: Register = Register(21);
/// General-purpose register 22
pub const X22: Register = Register(22);
/// General-purpose register 23
pub const X23: Register = Register(23);
/// General-purpose register 24
pub const X24: Register = Register(24);
/// General-purpose register 25
pub const X25: Register = Register(25);
/// General-purpose register 26
pub const X26: Register = Register(26);
/// General-purpose register 27
pub const X27: Register = Register(27);
/// General-purpose register 28
pub const X28: Register = Register(28);
/// General-purpose register 29
pub const X29: Register = Register(29);
/// General-purpose register 30
pub const X30: Register = Register(30);
/// General-purpose register 31
pub const X31: Register = Register(31);

/// Represents a `RISC-V` register
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Register(u8);

impl Register {
    const NBITS: usize = 5;

    #[doc = include_str!("../doc/nightly_warning.html")]
    ///
    /// Creates an `Register` from an [u8] constant, if `r` < [NUMBER_OF_REGISTERS], otherwise fails to compile
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u8<const VALUE: u8>() -> Self
    where
        internal::Assert<{ u8_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits5BIts,
    {
        Self(VALUE)
    }

    #[doc = include_str!("../doc/nightly_warning.html")]
    ///
    /// Creates an `Register` from an [u16] constant, if `r` < [NUMBER_OF_REGISTERS], otherwise fails to compile
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u16<const VALUE: u16>() -> Self
    where
        internal::Assert<{ u16_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits5BIts,
    {
        Self(VALUE as u8)
    }

    #[doc = include_str!("../doc/nightly_warning.html")]
    ///
    /// Creates an `Register` from an [u32] constant, if `r` < [NUMBER_OF_REGISTERS], otherwise fails to compile
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u32<const VALUE: u32>() -> Self
    where
        internal::Assert<{ u32_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits5BIts,
    {
        Self(VALUE as u8)
    }

    #[doc = include_str!("../doc/nightly_warning.html")]
    ///
    /// Creates an `Register` from an [u64] constant, if `r` < [NUMBER_OF_REGISTERS], otherwise fails to compile
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u64<const VALUE: u64>() -> Self
    where
        internal::Assert<{ u64_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits5BIts,
    {
        Self(VALUE as u8)
    }

    #[doc = include_str!("../doc/nightly_warning.html")]
    ///
    /// Creates an `Register` from an [usize] constant, if `r` < [NUMBER_OF_REGISTERS], otherwise fails to compile
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_usize<const VALUE: usize>() -> Self
    where
        internal::Assert<{ usize_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits5BIts,
    {
        Self(VALUE as u8)
    }

    #[doc = include_str!("../doc/nightly_warning.html")]
    ///
    /// Creates an `Register` from an [i8] constant, if `r` < [NUMBER_OF_REGISTERS], otherwise fails to compile
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i8<const VALUE: i8>() -> Self
    where
        internal::Assert<{ (VALUE as usize) < NUMBER_OF_REGISTERS }>: internal::Fits5BIts,
    {
        Self(VALUE as u8)
    }

    #[doc = include_str!("../doc/nightly_warning.html")]
    ///
    /// Creates an `Register` from an [i16] constant, if `r` < [NUMBER_OF_REGISTERS], otherwise fails to compile
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i16<const VALUE: i16>() -> Self
    where
        internal::Assert<{ (VALUE as usize) < NUMBER_OF_REGISTERS }>: internal::Fits5BIts,
    {
        Self(VALUE as u8)
    }

    #[doc = include_str!("../doc/nightly_warning.html")]
    ///
    /// Creates an `Register` from an [i32] constant, if `r` < [NUMBER_OF_REGISTERS], otherwise fails to compile
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i32<const VALUE: i32>() -> Self
    where
        internal::Assert<{ (VALUE as usize) < NUMBER_OF_REGISTERS }>: internal::Fits5BIts,
    {
        Self(VALUE as u8)
    }

    #[doc = include_str!("../doc/nightly_warning.html")]
    ///
    /// Creates an `Register` from an [i64] constant, if `r` < [NUMBER_OF_REGISTERS], otherwise fails to compile
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i64<const VALUE: i64>() -> Self
    where
        internal::Assert<{ (VALUE as usize) < NUMBER_OF_REGISTERS }>: internal::Fits5BIts,
    {
        Self(VALUE as u8)
    }

    #[doc = include_str!("../doc/nightly_warning.html")]
    ///
    /// Creates an `Register` from an [isize] constant, if `r` < [NUMBER_OF_REGISTERS], otherwise fails to compile
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_isize<const VALUE: isize>() -> Self
    where
        internal::Assert<{ (VALUE as usize) < NUMBER_OF_REGISTERS }>: internal::Fits5BIts,
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
    assert_eq!(Register::from_i8::<31>(), Register(31));
    assert_eq!(Register::from_u8::<31>(), Register(31));
    assert_eq!(Register::from_i16::<31>(), Register(31));
    assert_eq!(Register::from_u16::<31>(), Register(31));
    assert_eq!(Register::from_i32::<31>(), Register(31));
    assert_eq!(Register::from_u32::<31>(), Register(31));
    assert_eq!(Register::from_i64::<31>(), Register(31));
    assert_eq!(Register::from_u64::<31>(), Register(31));
    assert_eq!(Register::from_isize::<31>(), Register(31));
    assert_eq!(Register::from_usize::<31>(), Register(31));
}

#[test]
fn into_u32() {
    assert_eq!(X22.into_u32(), 22);
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('x')?;
        Display::fmt(&self.0, f)
    }
}

#[test]
fn register_display() -> Result<(), RegisterConvError> {
    for i in 0..NUMBER_OF_REGISTERS {
        assert_eq!(Register::try_from(i)?.to_string(), format!("x{i}"));
    }
    Ok(())
}

impl TryFrom<u8> for Register {
    type Error = RegisterConvError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if u8_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value))
        } else {
            Err(RegisterConvError::U8(value))
        }
    }
}

impl TryFrom<u16> for Register {
    type Error = RegisterConvError;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if u16_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as u8))
        } else {
            Err(RegisterConvError::U16(value))
        }
    }
}

impl TryFrom<u32> for Register {
    type Error = RegisterConvError;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if u32_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as u8))
        } else {
            Err(RegisterConvError::U32(value))
        }
    }
}

impl TryFrom<u64> for Register {
    type Error = RegisterConvError;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if u64_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as u8))
        } else {
            Err(RegisterConvError::U64(value))
        }
    }
}

impl TryFrom<usize> for Register {
    type Error = RegisterConvError;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if usize_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as u8))
        } else {
            Err(RegisterConvError::Usize(value))
        }
    }
}

impl TryFrom<i8> for Register {
    type Error = RegisterConvError;

    #[allow(clippy::cast_sign_loss)]
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if (value as usize) < NUMBER_OF_REGISTERS {
            Ok(Self(value as u8))
        } else {
            Err(RegisterConvError::I8(value))
        }
    }
}

impl TryFrom<i16> for Register {
    type Error = RegisterConvError;

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if (value as usize) < NUMBER_OF_REGISTERS {
            Ok(Self(value as u8))
        } else {
            Err(RegisterConvError::I16(value))
        }
    }
}

impl TryFrom<i32> for Register {
    type Error = RegisterConvError;

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if (value as usize) < NUMBER_OF_REGISTERS {
            Ok(Self(value as u8))
        } else {
            Err(RegisterConvError::I32(value))
        }
    }
}

impl TryFrom<i64> for Register {
    type Error = RegisterConvError;

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if (value as usize) < NUMBER_OF_REGISTERS {
            Ok(Self(value as u8))
        } else {
            Err(RegisterConvError::I64(value))
        }
    }
}

impl TryFrom<isize> for Register {
    type Error = RegisterConvError;

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if (value as usize) < NUMBER_OF_REGISTERS {
            Ok(Self(value as u8))
        } else {
            Err(RegisterConvError::Isize(value))
        }
    }
}

#[test]
fn conversions_from() -> Result<(), RegisterConvError> {
    assert_eq!(Register::try_from(31_i8)?, Register(31));
    assert_eq!(Register::try_from(31_u8)?, Register(31));
    assert_eq!(Register::try_from(31_i16)?, Register(31));
    assert_eq!(Register::try_from(31_u16)?, Register(31));
    assert_eq!(Register::try_from(31_i32)?, Register(31));
    assert_eq!(Register::try_from(31_u32)?, Register(31));
    assert_eq!(Register::try_from(31_i64)?, Register(31));
    assert_eq!(Register::try_from(31_u64)?, Register(31));
    assert_eq!(Register::try_from(31_isize)?, Register(31));
    assert_eq!(Register::try_from(31_usize)?, Register(31));

    assert!(matches!(
        Register::try_from(-1_i8),
        Err(RegisterConvError::I8(-1))
    ));
    assert!(matches!(
        Register::try_from(32_i8),
        Err(RegisterConvError::I8(32))
    ));
    assert!(matches!(
        Register::try_from(32_u8),
        Err(RegisterConvError::U8(32))
    ));
    assert!(matches!(
        Register::try_from(-1_i16),
        Err(RegisterConvError::I16(-1))
    ));
    assert!(matches!(
        Register::try_from(32_i16),
        Err(RegisterConvError::I16(32))
    ));
    assert!(matches!(
        Register::try_from(32_u16),
        Err(RegisterConvError::U16(32))
    ));
    assert!(matches!(
        Register::try_from(-1_i32),
        Err(RegisterConvError::I32(-1))
    ));
    assert!(matches!(
        Register::try_from(32_i32),
        Err(RegisterConvError::I32(32))
    ));
    assert!(matches!(
        Register::try_from(32_u32),
        Err(RegisterConvError::U32(32))
    ));
    assert!(matches!(
        Register::try_from(-1_i64),
        Err(RegisterConvError::I64(-1))
    ));
    assert!(matches!(
        Register::try_from(32_i64),
        Err(RegisterConvError::I64(32))
    ));
    assert!(matches!(
        Register::try_from(32_u64),
        Err(RegisterConvError::U64(32))
    ));
    assert!(matches!(
        Register::try_from(-1_isize),
        Err(RegisterConvError::Isize(-1))
    ));
    assert!(matches!(
        Register::try_from(32_isize),
        Err(RegisterConvError::Isize(32))
    ));
    assert!(matches!(
        Register::try_from(32_usize),
        Err(RegisterConvError::Usize(32))
    ));

    Ok(())
}

/// `Register` conversion error
#[derive(Debug)]
pub enum RegisterConvError {
    ///
    I8(i8),
    ///
    U8(u8),
    ///
    I16(i16),
    ///
    U16(u16),
    ///
    I32(i32),
    ///
    U32(u32),
    ///
    I64(i64),
    ///
    U64(u64),
    ///
    Isize(isize),
    ///
    Usize(usize),
}

// For line coverage
#[test]
fn debug() {
    format!("{:?}", RegisterConvError::I8(0));
}

impl Display for RegisterConvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegisterConvError::I8(value) => write!(f, "invalid register index: {value}"),
            RegisterConvError::U8(value) => write!(f, "invalid register index: {value}"),
            RegisterConvError::I16(value) => write!(f, "invalid register index: {value}"),
            RegisterConvError::U16(value) => write!(f, "invalid register index: {value}"),
            RegisterConvError::I32(value) => write!(f, "invalid register index: {value}"),
            RegisterConvError::U32(value) => write!(f, "invalid register index: {value}"),
            RegisterConvError::I64(value) => write!(f, "invalid register index: {value}"),
            RegisterConvError::U64(value) => write!(f, "invalid register index: {value}"),
            RegisterConvError::Isize(value) => write!(f, "invalid register index: {value}"),
            RegisterConvError::Usize(value) => write!(f, "invalid register index: {value}"),
        }
    }
}

#[test]
fn conv_error_impl_display() {
    assert_eq!(
        Register::try_from(-1_i8).unwrap_err().to_string(),
        "invalid register index: -1"
    );
    assert_eq!(
        Register::try_from(32_i8).unwrap_err().to_string(),
        "invalid register index: 32"
    );
    assert_eq!(
        Register::try_from(32_u8).unwrap_err().to_string(),
        "invalid register index: 32"
    );
    assert_eq!(
        Register::try_from(-1_i16).unwrap_err().to_string(),
        "invalid register index: -1"
    );
    assert_eq!(
        Register::try_from(32_i16).unwrap_err().to_string(),
        "invalid register index: 32"
    );
    assert_eq!(
        Register::try_from(32_u16).unwrap_err().to_string(),
        "invalid register index: 32"
    );
    assert_eq!(
        Register::try_from(-1_i32).unwrap_err().to_string(),
        "invalid register index: -1"
    );
    assert_eq!(
        Register::try_from(32_i32).unwrap_err().to_string(),
        "invalid register index: 32"
    );
    assert_eq!(
        Register::try_from(32_u32).unwrap_err().to_string(),
        "invalid register index: 32"
    );
    assert_eq!(
        Register::try_from(-1_i64).unwrap_err().to_string(),
        "invalid register index: -1"
    );
    assert_eq!(
        Register::try_from(32_i64).unwrap_err().to_string(),
        "invalid register index: 32"
    );
    assert_eq!(
        Register::try_from(32_u64).unwrap_err().to_string(),
        "invalid register index: 32"
    );
    assert_eq!(
        Register::try_from(-1_isize).unwrap_err().to_string(),
        "invalid register index: -1"
    );
    assert_eq!(
        Register::try_from(32_isize).unwrap_err().to_string(),
        "invalid register index: 32"
    );
    assert_eq!(
        Register::try_from(32_usize).unwrap_err().to_string(),
        "invalid register index: 32"
    );
}

impl Error for RegisterConvError {}

#[test]
fn conv_error_impl_error() -> Result<(), Box<dyn Error>> {
    assert_eq!(Register::try_from(0)?, Register(0));
    Ok(())
}

impl From<Register> for u8 {
    fn from(r: Register) -> Self {
        r.0
    }
}

impl From<Register> for u16 {
    fn from(r: Register) -> Self {
        From::from(r.0)
    }
}

impl From<Register> for u32 {
    fn from(r: Register) -> Self {
        From::from(r.0)
    }
}

impl From<Register> for u64 {
    fn from(r: Register) -> Self {
        From::from(r.0)
    }
}

impl From<Register> for usize {
    fn from(value: Register) -> Self {
        usize::from(value.0)
    }
}

impl From<Register> for i8 {
    #[allow(clippy::cast_possible_wrap)]
    fn from(r: Register) -> Self {
        r.0 as i8 // safe because r.0 < NUMBER_OF_REGISTERS
    }
}

impl From<Register> for i16 {
    fn from(r: Register) -> Self {
        From::from(r.0)
    }
}

impl From<Register> for i32 {
    fn from(r: Register) -> Self {
        From::from(r.0)
    }
}

impl From<Register> for i64 {
    fn from(r: Register) -> Self {
        From::from(r.0)
    }
}

impl From<Register> for isize {
    fn from(r: Register) -> Self {
        From::from(r.0)
    }
}

#[test]
fn conversions_into() -> Result<(), RegisterConvError> {
    assert_eq!(Into::<i8>::into(Register::try_from(31_i8)?), 31);
    assert_eq!(Into::<u8>::into(Register::try_from(31_u8)?), 31);
    assert_eq!(Into::<i16>::into(Register::try_from(31_i16)?), 31);
    assert_eq!(Into::<u16>::into(Register::try_from(31_u16)?), 31);
    assert_eq!(Into::<i32>::into(Register::try_from(31_i32)?), 31);
    assert_eq!(Into::<u32>::into(Register::try_from(31_u32)?), 31);
    assert_eq!(Into::<i64>::into(Register::try_from(31_i64)?), 31);
    assert_eq!(Into::<u64>::into(Register::try_from(31_u64)?), 31);
    assert_eq!(Into::<isize>::into(Register::try_from(31_isize)?), 31);
    assert_eq!(Into::<usize>::into(Register::try_from(31_usize)?), 31);
    Ok(())
}

mod internal {
    pub enum Assert<const CHECK: bool> {}
    pub trait Fits5BIts {}
    impl Fits5BIts for Assert<true> {}
}
