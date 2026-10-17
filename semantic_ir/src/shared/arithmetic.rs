//! Stanford ARM64 Shared Pseudocode - Arithmetic Functions
//!
//! Source:
//! https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
//!
//! Stanford function: AddWithCarry
//! Library pseudocode for shared/functions/integer/AddWithCarry

use crate::shared::enums::ConditionFlags;

/// Result of AddWithCarry operation: (result, NZCV flags)
/// The type parameter N must implement IntegerWidth
pub type AddWithCarryResult<N> = (N, ConditionFlags);

/// Trait for integer widths supported by ARM arithmetic operations
pub trait IntegerWidth:
    Copy
    + core::fmt::Debug
    + core::ops::Add<Output = Self>
    + core::ops::Sub<Output = Self>
    + core::ops::BitAnd<Output = Self>
    + core::ops::BitOr<Output = Self>
    + core::ops::BitXor<Output = Self>
    + core::ops::Not<Output = Self>
    + core::ops::Shl<u32, Output = Self>
    + core::ops::Shr<u32, Output = Self>
    + PartialEq
    + Eq
    + From<u8>
    + Into<u128>
{
    const WIDTH: u32;
    const MAX: Self;
    const MIN: Self;
    const MSB_MASK: Self;
    const SIGNED_MAX: i128;
    const SIGNED_MIN: i128;

    fn from_u128(val: u128) -> Self;
    fn to_u128(self) -> u128;
    fn to_i128(self) -> i128;
    fn leading_zeros(self) -> u32;
    fn trailing_zeros(self) -> u32;
    fn count_ones(self) -> u32;
    fn rotate_left(self, n: u32) -> Self;
    fn rotate_right(self, n: u32) -> Self;
}

macro_rules! impl_integer_width {
    ($type:ty, $width:expr, $signed_max:expr, $signed_min:expr) => {
        impl IntegerWidth for $type {
            const WIDTH: u32 = $width;
            const MAX: Self = <$type>::MAX;
            const MIN: Self = <$type>::MIN;
            const MSB_MASK: Self = 1 << ($width - 1);
            const SIGNED_MAX: i128 = $signed_max;
            const SIGNED_MIN: i128 = $signed_min;

            fn from_u128(val: u128) -> Self {
                val as $type
            }

            fn to_u128(self) -> u128 {
                self as u128
            }

            fn to_i128(self) -> i128 {
                let val = self as u128;
                // Sign-extend based on the MSB of the N-bit value
                if (val & (1u128 << (Self::WIDTH - 1))) != 0 {
                    // Negative: sign-extend by subtracting 2^WIDTH
                    // For WIDTH=128, 1i128 << 128 overflows, so compute 2^128 as i128::MIN (which is -2^127... no)
                    // Actually for u128, the MSB is bit 127, and 2^128 doesn't fit in i128.
                    // But u128::MAX as i128 via sign-extension should be -1.
                    // So: if MSB set, result = (val as i128) - (1 << WIDTH)
                    // For WIDTH=128: (val as i128) - 2^128. But 2^128 doesn't fit in i128.
                    // Alternative: cast to i128 directly (which treats val as unsigned), then if MSB was set, subtract 2^WIDTH.
                    // Since 2^128 = 0 in i128 (overflow), we can't do that.
                    // Instead: for WIDTH < 128, use the formula. For WIDTH = 128, the sign bit is 127,
                    // and the signed interpretation of u128 is just the two's complement of the same bits.
                    // In Rust, `val as i128` for u128 reinterprets the bits as signed (two's complement).
                    // So for WIDTH=128, `val as i128` IS the correct sign-extended value!
                    // For WIDTH < 128, we need to manually sign-extend.
                    if Self::WIDTH == 128 {
                        val as i128
                    } else {
                        (val as i128) - (1i128 << Self::WIDTH)
                    }
                } else {
                    // Positive: zero-extend (same as cast)
                    val as i128
                }
            }

            fn leading_zeros(self) -> u32 {
                <$type>::leading_zeros(self)
            }

            fn trailing_zeros(self) -> u32 {
                <$type>::trailing_zeros(self)
            }

            fn count_ones(self) -> u32 {
                <$type>::count_ones(self)
            }

            fn rotate_left(self, n: u32) -> Self {
                <$type>::rotate_left(self, n)
            }

            fn rotate_right(self, n: u32) -> Self {
                <$type>::rotate_right(self, n)
            }
        }
    };
}

impl_integer_width!(u8, 8, 127, -128);
impl_integer_width!(u16, 16, 32767, -32768);
impl_integer_width!(u32, 32, 2147483647, -2147483648);
impl_integer_width!(u64, 64, 9223372036854775807, -9223372036854775808);
impl_integer_width!(u128, 128, 170141183460469231731687303715884105727, -170141183460469231731687303715884105728);

/// Stanford pseudocode: AddWithCarry
///
/// ```text
/// func AddWithCarry{N}(x : bits(N), y : bits(N), carry_in : bit) => (bits(N), bits(4))
/// begin
///     let unsigned_sum : integer = UInt(x) + UInt(y) + UInt(carry_in);
///     let signed_sum   : integer = SInt(x) + SInt(y) + UInt(carry_in);
///     let result : bits(N) = unsigned_sum[N-1:0]; // same value as signed_sum[N-1:0]
///     let n : bit = result[N-1];
///     let z : bit = if IsZero(result) then '1' else '0';
///     let c : bit = if UInt(result) == unsigned_sum then '0' else '1';
///     let v : bit = if SInt(result) == signed_sum then '0' else '1';
///     return (result, n::z::c::v);
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// AddWithCarry
#[inline]
pub fn add_with_carry<N: IntegerWidth>(x: N, y: N, carry_in: bool) -> AddWithCarryResult<N> {
    let carry_val = if carry_in { N::from_u128(1) } else { N::from_u128(0) };

    // Unsigned arithmetic
    let unsigned_sum = x.to_u128() + y.to_u128() + carry_val.to_u128();
    let result_unsigned = N::from_u128(unsigned_sum);

    // Signed arithmetic
    let signed_sum = x.to_i128() + y.to_i128() + carry_val.to_i128();
    let result_signed = N::from_u128(signed_sum as u128);

    // The result is the same bit pattern for both unsigned and signed
    let result = result_unsigned;

    // N flag: MSB of result
    let n = (result & N::MSB_MASK) != N::from_u128(0);

    // Z flag: result is zero
    let z = result == N::from_u128(0);

    // C flag: unsigned overflow (carry out)
    let c = unsigned_sum > N::MAX.to_u128();

    // V flag: signed overflow
    let v = signed_sum > N::SIGNED_MAX || signed_sum < N::SIGNED_MIN;

    let flags = ConditionFlags::new(n, z, c, v);

    (result, flags)
}

/// Stanford pseudocode: SubWithCarry
///
/// ```text
/// func SubWithCarry{N}(x : bits(N), y : bits(N), carry_in : bit) => (bits(N), bits(4))
/// begin
///     let unsigned_diff : integer = UInt(x) - UInt(y) - UInt(NOT(carry_in));
///     let signed_diff   : integer = SInt(x) - SInt(y) - UInt(NOT(carry_in));
///     let result : bits(N) = unsigned_diff[N-1:0];
///     let n : bit = result[N-1];
///     let z : bit = if IsZero(result) then '1' else '0';
///     let c : bit = if UInt(result) == unsigned_diff then '0' else '1'; // borrow
///     let v : bit = if SInt(result) == signed_diff then '0' else '1';
///     return (result, n::z::c::v);
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// SubWithCarry
#[inline]
pub fn sub_with_carry<N: IntegerWidth>(x: N, y: N, carry_in: bool) -> AddWithCarryResult<N> {
    // In ARM, carry_in for subtraction is the "borrow" flag inverted
    // carry_in = 1 means no borrow, carry_in = 0 means borrow
    let borrow = if carry_in { 0 } else { 1 };

    // Unsigned arithmetic
    let unsigned_diff = x.to_u128().wrapping_sub(y.to_u128()).wrapping_sub(borrow);
    let result_unsigned = N::from_u128(unsigned_diff);

    // Signed arithmetic
    let signed_diff = x.to_i128().wrapping_sub(y.to_i128()).wrapping_sub(borrow as i128);
    let result_signed = N::from_u128(signed_diff as u128);

    let result = result_unsigned;

    // N flag: MSB of result
    let n = (result & N::MSB_MASK) != N::from_u128(0);

    // Z flag: result is zero
    let z = result == N::from_u128(0);

    // C flag: no borrow (unsigned underflow didn't happen)
    // In ARM, C=1 means no borrow, C=0 means borrow occurred
    let c = x.to_u128() >= y.to_u128() + borrow as u128;

    // V flag: signed overflow
    // Overflow occurs if (x >= 0 && y < 0 && result < 0) or (x < 0 && y >= 0 && result >= 0)
    // Simplified: (x_sign != y_sign) && (x_sign != result_sign)
    let x_sign = (x & N::MSB_MASK) != N::from_u128(0);
    let y_sign = (y & N::MSB_MASK) != N::from_u128(0);
    let result_sign = (result & N::MSB_MASK) != N::from_u128(0);
    let v = (x_sign != y_sign) && (x_sign != result_sign);

    let flags = ConditionFlags::new(n, z, c, v);

    (result, flags)
}

/// Add without carry (carry_in = 0)
#[inline]
pub fn add<N: IntegerWidth>(x: N, y: N) -> AddWithCarryResult<N> {
    add_with_carry(x, y, false)
}

/// Subtract without carry (carry_in = 1, i.e., no borrow)
#[inline]
pub fn sub<N: IntegerWidth>(x: N, y: N) -> AddWithCarryResult<N> {
    sub_with_carry(x, y, true)
}

/// Saturating signed add
///
/// Stanford pseudocode: SQAdd, UQAdd
/// Source: https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
pub fn sq_add<N: IntegerWidth>(x: N, y: N) -> N {
    let x_signed = x.to_i128();
    let y_signed = y.to_i128();
    let result = x_signed.saturating_add(y_signed);
    // Saturate at the target type's signed limits
    let saturated = result.clamp(N::SIGNED_MIN, N::SIGNED_MAX);
    N::from_u128(saturated as u128)
}

/// Saturating unsigned add
pub fn uq_add<N: IntegerWidth>(x: N, y: N) -> N {
    let result = x.to_u128().saturating_add(y.to_u128());
    // Saturate at the target type's unsigned limits
    let saturated = result.clamp(N::MIN.to_u128(), N::MAX.to_u128());
    N::from_u128(saturated)
}

/// Saturating signed subtract
pub fn sq_sub<N: IntegerWidth>(x: N, y: N) -> N {
    let x_signed = x.to_i128();
    let y_signed = y.to_i128();
    let result = x_signed.saturating_sub(y_signed);
    N::from_u128(result as u128)
}

/// Saturating unsigned subtract
pub fn uq_sub<N: IntegerWidth>(x: N, y: N) -> N {
    let result = x.to_u128().saturating_sub(y.to_u128());
    N::from_u128(result)
}

/// Doubling multiply-add (for QDMULH-type operations)
/// Returns (result_high, overflow_flag)
pub fn sq_dmulh<N: IntegerWidth>(x: N, y: N) -> (N, bool) {
    // For N-bit operands, result is 2N bits, we return top N bits
    // This is a simplified implementation
    let x_signed = x.to_i128();
    let y_signed = y.to_i128();
    let result = x_signed * y_signed * 2; // doubling
    let overflow = result > N::SIGNED_MAX || result < N::SIGNED_MIN;
    let result_high = (result >> (N::WIDTH - 1)) as u128;
    (N::from_u128(result_high), overflow)
}

/// Absolute difference (for SADDLV, etc.)
pub fn abs_diff<N: IntegerWidth>(x: N, y: N) -> N {
    let x_u = x.to_u128();
    let y_u = y.to_u128();
    let diff = if x_u > y_u { x_u - y_u } else { y_u - x_u };
    N::from_u128(diff)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_with_carry_basic() {
        let (result, flags) = add_with_carry::<u64>(10, 20, false);
        assert_eq!(result, 30);
        assert!(!flags.n); // positive
        assert!(!flags.z); // non-zero
        assert!(!flags.c); // no carry
        assert!(!flags.v); // no overflow
    }

    #[test]
    fn test_add_with_carry_carry_out() {
        let (result, flags) = add_with_carry::<u8>(200, 100, false);
        assert_eq!(result, 44); // 300 % 256
        assert!(flags.c); // carry out
    }

    #[test]
    fn test_add_with_carry_overflow() {
        // 127 + 1 = 128 (overflow for i8)
        let (result, flags) = add_with_carry::<u8>(127, 1, false);
        assert_eq!(result, 128);
        assert!(flags.n); // negative (MSB set)
        assert!(flags.v); // signed overflow
    }

    #[test]
    fn test_sub_with_carry_basic() {
        let (result, flags) = sub_with_carry::<u64>(20, 10, true); // no borrow
        assert_eq!(result, 10);
        assert!(flags.c); // C=1 means no borrow
    }

    #[test]
    fn test_sub_with_carry_borrow() {
        let (result, flags) = sub_with_carry::<u8>(10, 20, true); // no borrow in
        assert_eq!(result, 246); // 10 - 20 = -10 = 246 (mod 256)
        assert!(!flags.c); // borrow occurred (C=0)
    }

    #[test]
    fn test_sub_with_carry_overflow() {
        // -128 - 1 = -129 (overflow for i8)
        // In u8: 128 - 1 = 127, but signed interpretation overflows
        let (result, flags) = sub_with_carry::<u8>(128, 1, true);
        assert_eq!(result, 127);
        assert!(!flags.n); // positive result
        assert!(flags.v); // signed overflow: (-128) - 1 = -129
    }

    #[test]
    fn test_sq_add_saturating() {
        // sq_add is signed saturating add - test using unsigned types with signed interpretation
        // 100 + 50 = 150, which saturates at i8::MAX (127)
        assert_eq!(sq_add::<u8>(100, 50), 127);
        // 50 + 20 = 70, no saturation
        assert_eq!(sq_add::<u8>(50, 20), 70);
        // Test saturation at i8::MAX (127)
        assert_eq!(sq_add::<u8>(100, 50), 127); // saturates at i8::MAX
    }

    #[test]
    fn test_uq_add_saturating() {
        // uq_add is unsigned saturating add
        assert_eq!(uq_add::<u8>(200, 100), 255); // saturates at u8::MAX
        assert_eq!(uq_add::<u8>(100, 50), 150); // no saturation
    }
}