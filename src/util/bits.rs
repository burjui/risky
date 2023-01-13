use std::ops::RangeInclusive;

/// Combines `bitfields` into a single value. Each bit field is a tuple of:
/// - dst range
/// - value
/// - src range
#[inline(always)]
#[allow(clippy::inline_always)]
pub(crate) const fn combine_bitfields<const N: usize>(
    bitfields: &[(RangeInclusive<usize>, u32, RangeInclusive<usize>); N],
) -> u32 {
    let mut dst_bits_visited = 0u32;
    let mut dst = 0;
    let mut i = 0;
    while i < bitfields.len() {
        let (dst_range, src, src_range) = &bitfields[i];
        assert!(
            *src_range.end() < 32 && *dst_range.end() < 32,
            "bit range crosses 32-bit boundary"
        );
        assert!(
            (*dst_range.end() - *dst_range.start()) == *src_range.end() - *src_range.start(),
            "bit range lengths do not match"
        );

        // Copy the bitfield
        dst |= ((*src & !(0xFFFF_FFFF_u32 << *src_range.end() << 1)) >> *src_range.start())
            << *dst_range.start();

        // Check for overlaps
        let dst_mask =
            (0xFFFF_FFFF_u32 << *dst_range.end() << 1) ^ (0xFFFF_FFFF_u32 << *dst_range.start());
        assert!(
            dst_bits_visited & dst_mask == 0,
            "bit field overlap detected",
        );
        dst_bits_visited |= dst_mask;

        i += 1;
    }
    dst
}

#[test]
fn merge_bitfields_algorithm() {
    assert_eq!(
        combine_bitfields(&[(3..=4, 0b11, 0..=1), (7..=9, 0b010_0000, 4..=6),]),
        0b01_0001_1000
    );
}

#[test]
#[should_panic]
fn merge_bitfields_mismatched_ranges() {
    let _ = combine_bitfields(&[(3..=4, 0, 0..=2)]);
}

#[test]
#[should_panic]
fn merge_bitfields_crossing_32bit_boundary() {
    let _ = combine_bitfields(&[(0..=32, 0, 0..=32)]);
}

#[test]
#[should_panic]
fn merge_bitfields_overlap() {
    let _ = combine_bitfields(&[(0..=4, 0, 0..=4), (2..=2, 0, 2..=2)]);
}

pub(crate) const fn bitfield<const START: u32, const END_INCLUSIVE: u32>(src: u32) -> u32 {
    assert!(END_INCLUSIVE > START && END_INCLUSIVE < 32);
    (src & !(0xFFFF_FFFF << END_INCLUSIVE << 1)) >> START
}
