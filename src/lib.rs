/// `floor(log(2, v))`
///
/// Examples
///
/// ```rust
/// assert_eq!(base2::floor_log2(0), 0);
/// assert_eq!(base2::floor_log2(1), 0);
/// assert_eq!(base2::floor_log2(2), 1);
/// assert_eq!(base2::floor_log2(3), 1);
/// assert_eq!(base2::floor_log2(4), 2);
/// assert_eq!(base2::floor_log2(8), 3);
/// ```
pub fn floor_log2(mut v: u32) -> u32 {
    let mut result = 0;
    loop {
        v /= 2;
        if v == 0 { break; }
        result += 1;
    }
    result
}

/// `2^p`
///
/// Examples
///
/// ```rust
/// assert_eq!(base2::pow2(0), 1);
/// assert_eq!(base2::pow2(1), 2);
/// assert_eq!(base2::pow2(2), 4);
/// assert_eq!(base2::pow2(3), 8);
/// ```
pub fn pow2(p: u8) -> u32 {
    1 << p
}

/// A mask with a `p` number of bits.
///
/// ```
/// assert_eq!(base2::mask(0), 0);
/// assert_eq!(base2::mask(1), 1);
/// assert_eq!(base2::mask(2), 0b11);
/// assert_eq!(base2::mask(3), 0b111);
/// assert_eq!(base2::mask(4), 0b1111);
/// assert_eq!(base2::mask(5), 0b11111);
/// ```
pub fn mask(p: u8) -> u32 {
    pow2(p) - 1
}
