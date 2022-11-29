pub const fn u8_fits_n_bits(value: u8, nbits: usize) -> bool {
    value < (1 << nbits)
}

pub const fn u16_fits_n_bits(value: u16, nbits: usize) -> bool {
    value < (1 << nbits)
}

pub const fn u32_fits_n_bits(value: u32, nbits: usize) -> bool {
    value < (1 << nbits)
}

pub const fn u64_fits_n_bits(value: u64, nbits: usize) -> bool {
    value < (1 << nbits)
}

pub const fn i16_fits_n_bits(value: i16, nbits: usize) -> bool {
    value >= -(1 << (nbits - 1)) && value < (1 << (nbits - 1))
}

pub const fn i32_fits_n_bits(value: i32, nbits: usize) -> bool {
    value >= -(1 << (nbits - 1)) && value < (1 << (nbits - 1))
}

pub const fn i64_fits_n_bits(value: i64, nbits: usize) -> bool {
    value >= -(1 << (nbits - 1)) && value < (1 << (nbits - 1))
}
