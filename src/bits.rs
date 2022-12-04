use std::ops::Range;

/// Merges `bitfields` into a single value. Each bit field is a tuple of:
/// - dst range
/// - value
/// - src range
/// If any bit fields overlap, the behavior of `merge_bitfields` is unspecified.
#[inline]
pub(crate) const fn merge_bitfields<const N: usize>(
    bitfields: &[(Range<u32>, u32, Range<u32>); N],
) -> u32 {
    let mut dst = 0;
    let mut i = 0;
    while i < bitfields.len() {
        let (dst_range, src, src_range) = &bitfields[i];
        assert!(
            (dst_range.end - dst_range.start) == (src_range.end - src_range.start),
            "bit range lengths do not match"
        );
        let mask_left = shlz(0xFFFF_FFFF_u32, src_range.end);
        let mask_right = shrz(0xFFFF_FFFF_u32, 32 - src_range.start);
        let mask = !(mask_left | mask_right);
        dst |= shlz(shrz(*src & mask, src_range.start), dst_range.start);
        i += 1;
    }
    dst
}

const fn shlz(value: u32, nbits: u32) -> u32 {
    #[allow(clippy::cast_sign_loss)]
    let mask = (-1 + (nbits >= 32) as i32) as u32;
    value.wrapping_shl(nbits) & mask
}

const fn shrz(value: u32, nbits: u32) -> u32 {
    #[allow(clippy::cast_sign_loss)]
    let mask = (-1 + (nbits >= 32) as i32) as u32;
    value.wrapping_shr(nbits) & mask
}
