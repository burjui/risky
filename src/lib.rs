#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "nightly", allow(incomplete_features))]
#![cfg_attr(feature = "nightly", feature(generic_const_exprs))]
#![allow(clippy::module_name_repetitions)]

pub mod instructions;

pub(crate) mod bits;
pub(crate) mod util;
