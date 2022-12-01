use std::ops::Range;

#[inline]
pub(crate) const fn set_bits<const N: usize>(
    mut dst: u32,
    bits: [(Range<u32>, u32, Range<u32>); N],
) -> u32 {
    let mut i = 0;
    while i < bits.len() {
        let (dst_range, src, src_range) = &bits[i];
        if (dst_range.end - dst_range.start) != (src_range.end - src_range.start) {
            panic!("bit range lengths do not match");
        }
        let mask = shlz(0xFFFFFFFF_u32, dst_range.end) | shrz(0xFFFFFFFF_u32, 32 - dst_range.start);
        dst = dst & mask | (shlz(shrz(*src, src_range.start), dst_range.start)) & !mask;
        i += 1;
    }
    dst
}

const fn shlz(value: u32, nbits: u32) -> u32 {
    let mask = (-1 + (nbits >= 32) as i32) as u32;
    value.wrapping_shl(nbits) & mask
}

const fn shrz(value: u32, nbits: u32) -> u32 {
    let mask = (-1 + (nbits >= 32) as i32) as u32;
    value.wrapping_shr(nbits) & mask
}

#[test]
fn bits_set() -> std::io::Result<()> {
    fn pretty_write_bits(value: u32, w: &mut impl std::io::Write) -> std::io::Result<()> {
        for i in 0..8 {
            write!(w, " {:04b}", (value >> (28 - i * 4)) & 0xF)?;
        }
        writeln!(w)
    }

    let value = 0b1100_0111_0001_1100_0111_0001_1100_0111_u32;
    let value = set_bits(value, [(13..21, 0b1110_0011_1010_1010_0110, 6..14)]);
    if value != 0b1100_0111_0001_1101_0101_0001_1100_0111_u32 {
        pretty_write_bits(value, &mut std::io::stdout())?;
        panic!();
    } else {
        Ok(())
    }
}
