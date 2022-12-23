#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "nightly", allow(incomplete_features))]
#![cfg_attr(feature = "nightly", feature(generic_const_exprs))]
#![allow(clippy::module_name_repetitions)]

pub mod abi;
pub mod common;
pub mod decoding;
pub(crate) mod encoding;
pub mod instruction;
pub mod m_ext;
pub mod registers;
pub mod rv32i;
pub(crate) mod util;
pub mod zicsr_ext;
