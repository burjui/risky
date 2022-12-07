use std::ops::Range;

/// Merges `bitfields` into a single value. Each bit field is a tuple of:
/// - dst range
/// - value
/// - src range
/// If any bit fields overlap, the behavior of `merge_bitfields` is unspecified.
#[inline(always)]
pub(crate) const fn merge_bitfields<const N: usize>(
    bitfields: &[(Range<u32>, u32, Range<u32>); N],
) -> u32 {
    let mut dst_bits_visited = 0u32;
    let mut dst = 0;
    let mut i = 0;
    while i < bitfields.len() {
        let (dst_range, src, src_range) = &bitfields[i];
        assert!(
            dst_range.end <= 32,
            "bit field range crosses 32-bit boundary"
        );
        assert!(
            (dst_range.end - dst_range.start) == (src_range.end - src_range.start),
            "bit range lengths do not match"
        );

        // Copy the bitfield
        let src_mask =
            shlz(0xFFFF_FFFF_u32, src_range.end) ^ shlz(0xFFFF_FFFF_u32, src_range.start);
        dst |= shlz(shrz(*src & src_mask, src_range.start), dst_range.start);

        // Check for bit field overlap
        let dst_mask =
            shlz(0xFFFF_FFFF_u32, dst_range.end) ^ shlz(0xFFFF_FFFF_u32, dst_range.start);
        assert!(
            dst_bits_visited & dst_mask == 0,
            "bit field overlap detected"
        );
        dst_bits_visited |= dst_mask;

        i += 1;
    }
    dst
}

#[test]
fn merge_bitfields_algorithm() {
    assert_eq!(
        merge_bitfields(&[(3..5, 0b11, 0..2), (7..10, 0b0100000, 4..7)]),
        0b01_0001_1000
    );
}

#[test]
#[should_panic]
fn mismatched_bit_ranges() {
    let _ = merge_bitfields(&[(3..5, 0, 0..3)]);
}

#[test]
#[should_panic]
fn bit_range_crossing_32bit_boundary() {
    let _ = merge_bitfields(&[(0..33, 0, 0..0)]);
}

pub(crate) const fn shlz(value: u32, nbits: u32) -> u32 {
    #[allow(clippy::cast_sign_loss)]
    let mask = (-1 + (nbits >= 32) as i32) as u32;
    value.wrapping_shl(nbits) & mask
}

pub(crate) const fn shrz(value: u32, nbits: u32) -> u32 {
    #[allow(clippy::cast_sign_loss)]
    let mask = (-1 + (nbits >= 32) as i32) as u32;
    value.wrapping_shr(nbits) & mask
}
