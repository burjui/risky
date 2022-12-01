use std::mem::size_of;

#[must_use]
pub const fn u8_fits_n_bits(value: u8, nbits: usize) -> bool {
    let max_value = if nbits < nbits_of(&value) {
        (1 << nbits) - 1
    } else {
        min_of_type(&value)
    };
    value <= max_value
}

#[must_use]
pub const fn u16_fits_n_bits(value: u16, nbits: usize) -> bool {
    let max_value = if nbits < nbits_of(&value) {
        (1 << nbits) - 1
    } else {
        min_of_type(&value)
    };
    value <= max_value
}

#[must_use]
pub const fn u32_fits_n_bits(value: u32, nbits: usize) -> bool {
    let max_value = if nbits < nbits_of(&value) {
        (1 << nbits) - 1
    } else {
        min_of_type(&value)
    };
    value <= max_value
}

#[must_use]
pub const fn u64_fits_n_bits(value: u64, nbits: usize) -> bool {
    let max_value = if nbits < nbits_of(&value) {
        (1 << nbits) - 1
    } else {
        min_of_type(&value)
    };
    value <= max_value
}

#[must_use]
pub const fn usize_fits_n_bits(value: usize, nbits: usize) -> bool {
    let max_value = if nbits < nbits_of(&value) {
        (1 << nbits) - 1
    } else {
        min_of_type(&value)
    };
    value <= max_value
}

// #[must_use]
// pub const fn i8_fits_n_bits(value: i8, nbits: usize) -> bool {
//     let (min, max) = if nbits < nbits_of(&value) - 1 {
//         (-(1 << (nbits - 1)), (1 << (nbits - 1)) - 1)
//     } else {
//         (min_of_type(&value), max_of_type(&value))
//     };
//     value >= min && value <= max
// }

#[must_use]
pub const fn i16_fits_n_bits(value: i16, nbits: usize) -> bool {
    let (min, max) = if nbits < nbits_of(&value) - 1 {
        (-(1 << (nbits - 1)), (1 << (nbits - 1)) - 1)
    } else {
        (min_of_type(&value), max_of_type(&value))
    };
    value >= min && value <= max
}

#[must_use]
pub const fn i32_fits_n_bits(value: i32, nbits: usize) -> bool {
    let (min, max) = if nbits < nbits_of(&value) - 1 {
        (-(1 << (nbits - 1)), (1 << (nbits - 1)) - 1)
    } else {
        (min_of_type(&value), max_of_type(&value))
    };
    value >= min && value <= max
}

#[must_use]
pub const fn i64_fits_n_bits(value: i64, nbits: usize) -> bool {
    let (min, max) = if nbits < nbits_of(&value) - 1 {
        (-(1 << (nbits - 1)), (1 << (nbits - 1)) - 1)
    } else {
        (min_of_type(&value), max_of_type(&value))
    };
    value >= min && value <= max
}

#[must_use]
pub const fn isize_fits_n_bits(value: isize, nbits: usize) -> bool {
    let (min, max) = if nbits < nbits_of(&value) - 1 {
        (-(1 << (nbits - 1)), (1 << (nbits - 1)) - 1)
    } else {
        (min_of_type(&value), max_of_type(&value))
    };
    value >= min && value <= max
}

const fn nbits_of<T: internal::Integer>(_: &T) -> usize {
    size_of::<T>() * 8
}

const fn min_of_type<T: internal::Integer>(_: &T) -> T {
    T::MIN
}

const fn max_of_type<T: internal::Integer>(_: &T) -> T {
    T::MAX
}

mod internal {
    pub trait Integer: Copy {
        const MIN: Self;
        const MAX: Self;
    }

    impl Integer for i8 {
        const MIN: Self = i8::MIN;
        const MAX: Self = i8::MAX;
    }

    impl Integer for u8 {
        const MIN: Self = u8::MIN;
        const MAX: Self = u8::MAX;
    }

    impl Integer for i16 {
        const MIN: Self = i16::MIN;
        const MAX: Self = i16::MAX;
    }

    impl Integer for u16 {
        const MIN: Self = u16::MIN;
        const MAX: Self = u16::MAX;
    }

    impl Integer for i32 {
        const MIN: Self = i32::MIN;
        const MAX: Self = i32::MAX;
    }

    impl Integer for u32 {
        const MIN: Self = u32::MIN;
        const MAX: Self = u32::MAX;
    }

    impl Integer for i64 {
        const MIN: Self = i64::MIN;
        const MAX: Self = i64::MAX;
    }

    impl Integer for u64 {
        const MIN: Self = u64::MIN;
        const MAX: Self = u64::MAX;
    }

    impl Integer for isize {
        const MIN: Self = isize::MIN;
        const MAX: Self = isize::MAX;
    }

    impl Integer for usize {
        const MIN: Self = usize::MIN;
        const MAX: Self = usize::MAX;
    }
}

// #![cfg_attr(feature = "nightly", feature(const_trait_impl))]
//
// #[cfg(feature = "nightly")]
// #[must_use]
// pub const fn unsigned_fits_nbits<T: ~const internal::Integer>(value: &T, nbits: usize) -> bool {
//     let max_value = if nbits < nbits_of(value) {
//         (T::ONE.shl(nbits)).sub(T::ONE)
//     } else {
//         min_of_type(value)
//     };
//     value.lteq(max_value)
// }

// #[cfg(feature = "nightly")]
// mod internal {
//     #[const_trait]
//     pub trait Integer: Copy {
//         const MIN: Self;
//         const MAX: Self;
//         const ONE: Self;

//         fn shl(self, nbits: usize) -> Self;
//         fn sub(self, value: Self) -> Self;
//         fn lteq(self, value: Self) -> bool;
//     }

//     impl const Integer for i8 {
//         const MIN: Self = i8::MIN;
//         const MAX: Self = i8::MAX;
//         const ONE: Self = 1;
//         fn shl(self, nbits: usize) -> Self {
//             self << nbits
//         }
//         fn sub(self, value: Self) -> Self {
//             self - value
//         }
//         fn lteq(self, value: Self) -> bool {
//             self < value
//         }
//     }

//     impl const Integer for u8 {
//         const MIN: Self = u8::MIN;
//         const MAX: Self = u8::MAX;
//         const ONE: Self = 1;
//         fn shl(self, nbits: usize) -> Self {
//             self << nbits
//         }
//         fn sub(self, value: Self) -> Self {
//             self - value
//         }
//         fn lteq(self, value: Self) -> bool {
//             self < value
//         }
//     }

//     impl const Integer for i16 {
//         const MIN: Self = i16::MIN;
//         const MAX: Self = i16::MAX;
//         const ONE: Self = 1;
//         fn shl(self, nbits: usize) -> Self {
//             self << nbits
//         }
//         fn sub(self, value: Self) -> Self {
//             self - value
//         }
//         fn lteq(self, value: Self) -> bool {
//             self < value
//         }
//     }

//     impl const Integer for u16 {
//         const MIN: Self = u16::MIN;
//         const MAX: Self = u16::MAX;
//         const ONE: Self = 1;
//         fn shl(self, nbits: usize) -> Self {
//             self << nbits
//         }
//         fn sub(self, value: Self) -> Self {
//             self - value
//         }
//         fn lteq(self, value: Self) -> bool {
//             self < value
//         }
//     }

//     impl const Integer for i32 {
//         const MIN: Self = i32::MIN;
//         const MAX: Self = i32::MAX;
//         const ONE: Self = 1;
//         fn shl(self, nbits: usize) -> Self {
//             self << nbits
//         }
//         fn sub(self, value: Self) -> Self {
//             self - value
//         }
//         fn lteq(self, value: Self) -> bool {
//             self < value
//         }
//     }

//     impl const Integer for u32 {
//         const MIN: Self = u32::MIN;
//         const MAX: Self = u32::MAX;
//         const ONE: Self = 1;
//         fn shl(self, nbits: usize) -> Self {
//             self << nbits
//         }
//         fn sub(self, value: Self) -> Self {
//             self - value
//         }
//         fn lteq(self, value: Self) -> bool {
//             self < value
//         }
//     }

//     impl const Integer for i64 {
//         const MIN: Self = i64::MIN;
//         const MAX: Self = i64::MAX;
//         const ONE: Self = 1;
//         fn shl(self, nbits: usize) -> Self {
//             self << nbits
//         }
//         fn sub(self, value: Self) -> Self {
//             self - value
//         }
//         fn lteq(self, value: Self) -> bool {
//             self < value
//         }
//     }

//     impl const Integer for u64 {
//         const MIN: Self = u64::MIN;
//         const MAX: Self = u64::MAX;
//         const ONE: Self = 1;
//         fn shl(self, nbits: usize) -> Self {
//             self << nbits
//         }
//         fn sub(self, value: Self) -> Self {
//             self - value
//         }
//         fn lteq(self, value: Self) -> bool {
//             self < value
//         }
//     }

//     impl const Integer for isize {
//         const MIN: Self = isize::MIN;
//         const MAX: Self = isize::MAX;
//         const ONE: Self = 1;
//         fn shl(self, nbits: usize) -> Self {
//             self << nbits
//         }
//         fn sub(self, value: Self) -> Self {
//             self - value
//         }
//         fn lteq(self, value: Self) -> bool {
//             self < value
//         }
//     }

//     impl const Integer for usize {
//         const MIN: Self = usize::MIN;
//         const MAX: Self = usize::MAX;
//         const ONE: Self = 1;
//         fn shl(self, nbits: usize) -> Self {
//             self << nbits
//         }
//         fn sub(self, value: Self) -> Self {
//             self - value
//         }
//         fn lteq(self, value: Self) -> bool {
//             self < value
//         }
//     }
// }
