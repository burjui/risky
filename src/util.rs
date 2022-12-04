use std::mem::size_of;
#[cfg(test)]
use std::panic::catch_unwind;

#[must_use]
pub const fn u8_fits_n_bits(value: u8, nbits: usize) -> bool {
    let max_nbits = nbits_of(&value);
    assert!(nbits > 0 && nbits <= max_nbits);

    value <= max_of_type(&value) >> (max_nbits - nbits)
}

#[test]
fn u8_fits_n_bits_algorithm() -> Result<(), std::num::TryFromIntError> {
    assert!(u8_fits_n_bits(2, 2));
    assert!(!u8_fits_n_bits(2, 1));

    assert!(u8_fits_n_bits(u8::MAX, usize::try_from(u8::BITS)?));
    assert!(!u8_fits_n_bits(u8::MAX, usize::try_from(u8::BITS)? - 1));

    Ok(())
}

#[test]
fn u8_fits_n_bits_preconditions() {
    assert!(catch_unwind(|| u8_fits_n_bits(0, 0)).is_err());
    assert!(catch_unwind(|| u8_fits_n_bits(0, usize::try_from(u8::BITS).unwrap() + 1)).is_err());
}

#[must_use]
pub const fn u16_fits_n_bits(value: u16, nbits: usize) -> bool {
    let max_nbits = nbits_of(&value);
    assert!(nbits > 0 && nbits <= max_nbits);

    value <= max_of_type(&value) >> (max_nbits - nbits)
}

#[test]
fn u16_fits_n_bits_algorithm() -> Result<(), std::num::TryFromIntError> {
    assert!(u16_fits_n_bits(2, 2));
    assert!(!u16_fits_n_bits(2, 1));

    assert!(u16_fits_n_bits(u16::MAX, usize::try_from(u16::BITS)?));
    assert!(!u16_fits_n_bits(u16::MAX, usize::try_from(u16::BITS)? - 1));

    Ok(())
}

#[test]
fn u16_fits_n_bits_preconditions() {
    assert!(catch_unwind(|| u16_fits_n_bits(0, 0)).is_err());
    assert!(catch_unwind(|| u16_fits_n_bits(0, usize::try_from(u16::BITS).unwrap() + 1)).is_err());
}

#[must_use]
pub const fn u32_fits_n_bits(value: u32, nbits: usize) -> bool {
    let max_nbits = nbits_of(&value);
    assert!(nbits > 0 && nbits <= max_nbits);

    value <= max_of_type(&value) >> (max_nbits - nbits)
}

#[test]
fn u32_fits_n_bits_algorithm() -> Result<(), std::num::TryFromIntError> {
    assert!(u32_fits_n_bits(2, 2));
    assert!(!u32_fits_n_bits(2, 1));

    assert!(u32_fits_n_bits(u32::MAX, usize::try_from(u32::BITS)?));
    assert!(!u32_fits_n_bits(u32::MAX, usize::try_from(u32::BITS)? - 1));

    Ok(())
}

#[test]
fn u32_fits_n_bits_preconditions() {
    assert!(catch_unwind(|| u32_fits_n_bits(0, 0)).is_err());
    assert!(catch_unwind(|| u32_fits_n_bits(0, usize::try_from(u32::BITS).unwrap() + 1)).is_err());
}

#[must_use]
pub const fn u64_fits_n_bits(value: u64, nbits: usize) -> bool {
    let max_nbits = nbits_of(&value);
    assert!(nbits > 0 && nbits <= max_nbits);

    value <= max_of_type(&value) >> (max_nbits - nbits)
}

#[test]
fn u64_fits_n_bits_algorithm() -> Result<(), std::num::TryFromIntError> {
    assert!(u64_fits_n_bits(2, 2));
    assert!(!u64_fits_n_bits(2, 1));

    assert!(u64_fits_n_bits(u64::MAX, usize::try_from(u64::BITS)?));
    assert!(!u64_fits_n_bits(u64::MAX, usize::try_from(u64::BITS)? - 1));

    Ok(())
}

#[test]
fn u64_fits_n_bits_preconditions() {
    assert!(catch_unwind(|| u64_fits_n_bits(0, 0)).is_err());
    assert!(catch_unwind(|| u64_fits_n_bits(0, usize::try_from(u64::BITS).unwrap() + 1)).is_err());
}

#[must_use]
pub const fn usize_fits_n_bits(value: usize, nbits: usize) -> bool {
    let max_nbits = nbits_of(&value);
    assert!(nbits > 0 && nbits <= max_nbits);

    value <= max_of_type(&value) >> (max_nbits - nbits)
}

#[test]
fn usize_fits_n_bits_algorithm() -> Result<(), std::num::TryFromIntError> {
    assert!(usize_fits_n_bits(2, 2));
    assert!(!usize_fits_n_bits(2, 1));

    assert!(usize_fits_n_bits(usize::MAX, usize::try_from(usize::BITS)?));
    assert!(!usize_fits_n_bits(
        usize::MAX,
        usize::try_from(usize::BITS)? - 1
    ));

    Ok(())
}

#[test]
fn usize_fits_n_bits_preconditions() {
    assert!(catch_unwind(|| usize_fits_n_bits(0, 0)).is_err());
    assert!(
        catch_unwind(|| usize_fits_n_bits(0, usize::try_from(usize::BITS).unwrap() + 1)).is_err()
    );
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
    let max_nbits = nbits_of(&value);
    assert!(nbits > 0 && nbits <= max_nbits);

    let shift_amount = max_nbits - nbits;
    value >= min_of_type(&value) >> shift_amount && value <= max_of_type(&value) >> shift_amount
}

#[test]
fn i16_fits_n_bits_algorithm() -> Result<(), std::num::TryFromIntError> {
    assert!(i16_fits_n_bits(i16::MAX, usize::try_from(i16::BITS)?));
    assert!(!i16_fits_n_bits(i16::MAX, usize::try_from(i16::BITS)? - 1));

    assert!(i16_fits_n_bits(i16::MIN, usize::try_from(i16::BITS)?));
    assert!(!i16_fits_n_bits(i16::MIN, usize::try_from(i16::BITS)? - 1));

    Ok(())
}

#[must_use]
pub const fn i32_fits_n_bits(value: i32, nbits: usize) -> bool {
    let max_nbits = nbits_of(&value);
    assert!(nbits > 0 && nbits <= max_nbits);

    let shift_amount = max_nbits - nbits;
    value >= min_of_type(&value) >> shift_amount && value <= max_of_type(&value) >> shift_amount
}

#[test]
fn i32_fits_n_bits_algorithm() -> Result<(), std::num::TryFromIntError> {
    assert!(i32_fits_n_bits(i32::MAX, usize::try_from(i32::BITS)?));
    assert!(!i32_fits_n_bits(i32::MAX, usize::try_from(i32::BITS)? - 1));

    assert!(i32_fits_n_bits(i32::MIN, usize::try_from(i32::BITS)?));
    assert!(!i32_fits_n_bits(i32::MIN, usize::try_from(i32::BITS)? - 1));

    Ok(())
}

#[must_use]
pub const fn i64_fits_n_bits(value: i64, nbits: usize) -> bool {
    let max_nbits = nbits_of(&value);
    assert!(nbits > 0 && nbits <= max_nbits);

    let shift_amount = max_nbits - nbits;
    value >= min_of_type(&value) >> shift_amount && value <= max_of_type(&value) >> shift_amount
}

#[test]
fn i64_fits_n_bits_algorithm() -> Result<(), std::num::TryFromIntError> {
    assert!(i64_fits_n_bits(i64::MAX, usize::try_from(i64::BITS)?));
    assert!(!i64_fits_n_bits(i64::MAX, usize::try_from(i64::BITS)? - 1));

    assert!(i64_fits_n_bits(i64::MIN, usize::try_from(i64::BITS)?));
    assert!(!i64_fits_n_bits(i64::MIN, usize::try_from(i64::BITS)? - 1));

    Ok(())
}

#[must_use]
pub const fn isize_fits_n_bits(value: isize, nbits: usize) -> bool {
    let max_nbits = nbits_of(&value);
    assert!(nbits > 0 && nbits <= max_nbits);

    let shift_amount = max_nbits - nbits;
    value >= min_of_type(&value) >> shift_amount && value <= max_of_type(&value) >> shift_amount
}

#[test]
fn isize_fits_n_bits_algorithm() -> Result<(), std::num::TryFromIntError> {
    assert!(isize_fits_n_bits(isize::MAX, usize::try_from(isize::BITS)?));
    assert!(!isize_fits_n_bits(
        isize::MAX,
        usize::try_from(isize::BITS)? - 1
    ));

    assert!(isize_fits_n_bits(isize::MIN, usize::try_from(isize::BITS)?));
    assert!(!isize_fits_n_bits(
        isize::MIN,
        usize::try_from(isize::BITS)? - 1
    ));

    Ok(())
}

const fn nbits_of<T: internal::Integer>(_: &T) -> usize {
    size_of::<T>() * 8
}

#[test]
fn max_of_type_algorithm() {
    assert_eq!(max_of_type(&0_i32), i32::MAX);
    assert_eq!(max_of_type(&0_u32), u32::MAX);
}

const fn min_of_type<T: internal::Integer>(_: &T) -> T {
    T::MIN
}

#[test]
fn min_of_type_algorithm() {
    assert_eq!(min_of_type(&0_i32), i32::MIN);
    assert_eq!(min_of_type(&0_u32), u32::MIN);
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
