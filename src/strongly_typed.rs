//! Fallible versions of instruction encodings that return Result

pub mod m_ext;
pub mod rv32i;
pub mod zicsr_ext;

pub mod abi;
mod encoding;
pub mod immediates;
pub mod registers;
