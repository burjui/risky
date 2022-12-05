#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(feature = "nightly", allow(incomplete_features))]
#![cfg_attr(feature = "nightly", feature(generic_const_exprs))]
#![allow(clippy::module_name_repetitions)]

pub mod raw;
pub mod strongly_typed;

pub(crate) mod bits;
pub(crate) mod util;
