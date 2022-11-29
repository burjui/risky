pub const fn u8_fits_n_bits(value: u8, nbits: usize) -> bool {
    let max_value = if nbits < 8 { (1 << nbits) - 1 } else { u8::MAX };
    value <= max_value
}

pub const fn u16_fits_n_bits(value: u16, nbits: usize) -> bool {
    let max_value = if nbits < 16 {
        (1 << nbits) - 1
    } else {
        u16::MAX
    };
    value <= max_value
}

pub const fn u32_fits_n_bits(value: u32, nbits: usize) -> bool {
    let max_value = if nbits < 32 {
        (1 << nbits) - 1
    } else {
        u32::MAX
    };
    value <= max_value
}

pub const fn u64_fits_n_bits(value: u64, nbits: usize) -> bool {
    let max_value = if nbits < 64 {
        (1 << nbits) - 1
    } else {
        u64::MAX
    };
    value <= max_value
}

// pub const fn i8_fits_n_bits(value: i8, nbits: usize) -> bool {
//     let (min, max) = if nbits < 7 {
//         (-(1 << (nbits - 1)), (1 << (nbits - 1)) - 1)
//     } else {
//         (i8::MIN, i8::MAX)
//     };
//     value >= min && value <= max
// }

pub const fn i16_fits_n_bits(value: i16, nbits: usize) -> bool {
    let (min, max) = if nbits < 15 {
        (-(1 << (nbits - 1)), (1 << (nbits - 1)) - 1)
    } else {
        (i16::MIN, i16::MAX)
    };
    value >= min && value <= max
}

pub const fn i32_fits_n_bits(value: i32, nbits: usize) -> bool {
    let (min, max) = if nbits < 31 {
        (-(1 << (nbits - 1)), (1 << (nbits - 1)) - 1)
    } else {
        (i32::MIN, i32::MAX)
    };
    value >= min && value <= max
}

pub const fn i64_fits_n_bits(value: i64, nbits: usize) -> bool {
    let (min, max) = if nbits < 63 {
        (-(1 << (nbits - 1)), (1 << (nbits - 1)) - 1)
    } else {
        (i64::MIN, i64::MAX)
    };
    value >= min && value <= max
}
