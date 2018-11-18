extern crate int;

use int::UInt;

pub trait Base2 {
    /// `floor(log(2, v))`
    ///
    /// Examples
    ///
    /// ```rust
    /// use base2::Base2;
    /// assert_eq!(0_u8.floor_log2(), 0);
    /// assert_eq!(1_u16.floor_log2(), 0);
    /// assert_eq!(2_u32.floor_log2(), 1);
    /// assert_eq!(3_u64.floor_log2(), 1);
    /// assert_eq!(4_u128.floor_log2(), 2);
    /// assert_eq!(8_usize.floor_log2(), 3);
    /// ```
    fn floor_log2(self) -> u8;

    /// `2^p`
    ///
    /// Examples
    ///
    /// ```rust
    /// use base2::Base2;
    /// assert_eq!(u8::exp2(0), 1);
    /// assert_eq!(u16::exp2(1), 2);
    /// assert_eq!(u32::exp2(2), 4);
    /// assert_eq!(u64::exp2(3), 8);
    /// assert_eq!(u128::exp2(4), 16);
    /// assert_eq!(usize::exp2(5), 32);
    /// ```
    fn exp2(p: u8) -> Self;

    /// A mask with a `p` number of bits.
    ///
    /// ```
    /// use base2::Base2;
    /// assert_eq!(u8::mask(0), 0);
    /// assert_eq!(u16::mask(1), 1);
    /// assert_eq!(u32::mask(2), 0b11);
    /// assert_eq!(u64::mask(3), 0b111);
    /// assert_eq!(u128::mask(4), 0b1111);
    /// assert_eq!(usize::mask(5), 0b11111);
    /// assert_eq!(u8::mask(8), 0b1111_1111);
    /// ```
    fn mask(p: u8) -> Self;
}

impl<T: UInt> Base2 for T {
    fn floor_log2(mut self) -> u8 {
        let mut result = 0;
        loop {
            self >>= 1;
            if self == Self::_0 { break; }
            result += 1;
        }
        result
    }
    fn exp2(p: u8) -> Self { Self::_1 << p }
    fn mask(p: u8) -> Self {
        if p >= Self::BIT_COUNT { Self::MAX_VALUE } else { Self::exp2(p) - Self::_1 }
    }
}

