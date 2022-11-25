use std::ops::Range;

pub(crate) const fn i16_value_range(nbits: usize) -> Range<i16> {
    -(1 << (nbits - 1))..(1 << (nbits - 1))
}

pub(crate) const fn i32_value_range(nbits: usize) -> Range<i32> {
    -(1 << (nbits - 1))..(1 << (nbits - 1))
}

pub(crate) const fn u8_max_value(nbits: usize) -> u8 {
    (1 << nbits) - 1
}
