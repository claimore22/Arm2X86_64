//! Stanford ARM64 Shared Pseudocode - Bit Operations
//!
//! Source:
//! https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
//!
//! Stanford functions: UInt, SInt, IsZero, HighestSetBit, LowestSetBit,
//! SignExtend, ZeroExtend, Align, Replicate, Reverse, BitReverse, etc.

use crate::shared::enums::{ConditionFlags, Endianness};
use core::fmt;

/// Trait for bit-vector operations on integer types
pub trait BitVector: Copy + fmt::Debug + PartialEq + Eq {
    fn width() -> u32;

    /// Get bit at index
    fn bit(self, index: u32) -> bool;

    /// Set bit at index
    fn set_bit(self, index: u32, value: bool) -> Self;

    /// Get bit range [hi:lo] inclusive
    fn bits(self, hi: u32, lo: u32) -> Self;

    /// Concatenate: self || other (self is high bits)
    fn concat(self, other: Self, other_width: u32) -> Self;

    /// Replicate value n times
    fn replicate(self, count: u32) -> Self;

    /// Reverse bytes/halfwords/words
    fn reverse(self, element_size: u32) -> Self;

    /// Count leading zeros
    fn clz(self) -> u32;

    /// Count trailing zeros
    fn ctz(self) -> u32;

    /// Population count
    fn popcount(self) -> u32;

    /// Rotate left
    fn rotl(self, n: u32) -> Self;

    /// Rotate right
    fn rotr(self, n: u32) -> Self;

    /// Shift left logical
    fn shl(self, n: u32) -> Self;

    /// Shift right logical
    fn shr(self, n: u32) -> Self;

    /// Shift right arithmetic
    fn sar(self, n: u32) -> Self;

    /// Convert to unsigned integer
    fn to_uint(self) -> u128;

    /// Convert to signed integer
    fn to_sint(self) -> i128;

    /// Create from unsigned integer
    fn from_uint(val: u128) -> Self;

    /// Create from signed integer
    fn from_sint(val: i128) -> Self;

    /// Check if zero
    fn is_zero(self) -> bool;

    /// Check if all ones
    fn is_ones(self) -> bool;

    /// Get MSB index (highest set bit)
    fn highest_set_bit(self) -> Option<u32>;

    /// Get LSB index (lowest set bit)
    fn lowest_set_bit(self) -> Option<u32>;
}

macro_rules! impl_bitvector {
    ($type:ty, $width:expr) => {
        impl BitVector for $type {
            fn width() -> u32 {
                $width
            }

            fn bit(self, index: u32) -> bool {
                assert!(index < $width);
                (self & (1 << index)) != 0
            }

            fn set_bit(self, index: u32, value: bool) -> Self {
                assert!(index < $width);
                if value {
                    self | (1 << index)
                } else {
                    self & !(1 << index)
                }
            }

            fn bits(self, hi: u32, lo: u32) -> Self {
                assert!(hi < $width && lo <= hi);
                let mask = if hi == $width - 1 {
                    !0
                } else {
                    (1 << (hi + 1)) - 1
                };
                (self & mask) >> lo
            }

            fn concat(self, other: Self, other_width: u32) -> Self {
                (self << other_width) | other
            }

            fn replicate(self, count: u32) -> Self {
                let mut result = Self::from_uint(0);
                let total_width = Self::width();
                let element_size = total_width / count;
                for i in 0..count {
                    let shift = i * element_size;
                    if shift >= Self::width() {
                        // Shifting by >= width gives 0
                        continue;
                    }
                    result = result | (self << shift);
                }
                result
            }

            fn reverse(self, element_size: u32) -> Self {
                let mut result = Self::from_uint(0);
                let elements = $width / element_size;
                for i in 0..elements {
                    let element = self.bits((i + 1) * element_size - 1, i * element_size);
                    let pos = (elements - 1 - i) * element_size;
                    result = result | (element << pos);
                }
                result
            }

            fn clz(self) -> u32 {
                Self::leading_zeros(self)
            }

            fn ctz(self) -> u32 {
                Self::trailing_zeros(self)
            }

            fn popcount(self) -> u32 {
                Self::count_ones(self)
            }

            fn rotl(self, n: u32) -> Self {
                Self::rotate_left(self, n)
            }

            fn rotr(self, n: u32) -> Self {
                Self::rotate_right(self, n)
            }

            fn shl(self, n: u32) -> Self {
                self << n
            }

            fn shr(self, n: u32) -> Self {
                self >> n
            }

            fn sar(self, n: u32) -> Self {
                ((self as i128) >> n) as Self
            }

            fn to_uint(self) -> u128 {
                self as u128
            }

            fn to_sint(self) -> i128 {
                self as i128
            }

            fn from_uint(val: u128) -> Self {
                val as Self
            }

            fn from_sint(val: i128) -> Self {
                val as Self
            }

            fn is_zero(self) -> bool {
                self == 0
            }

            fn is_ones(self) -> bool {
                self == Self::from_uint(!0u128 >> (128 - $width))
            }

            fn highest_set_bit(self) -> Option<u32> {
                if self == 0 {
                    None
                } else {
                    Some($width - 1 - self.leading_zeros())
                }
            }

            fn lowest_set_bit(self) -> Option<u32> {
                if self == 0 {
                    None
                } else {
                    Some(self.trailing_zeros())
                }
            }
        }
    };
}

impl_bitvector!(u8, 8);
impl_bitvector!(u16, 16);
impl_bitvector!(u32, 32);
impl_bitvector!(u64, 64);
impl_bitvector!(u128, 128);

/// Stanford pseudocode: UInt
///
/// Convert bit vector to unsigned integer
///
/// ```text
/// func UInt(x : bits(N)) => integer
/// begin
///     return integer value of x as unsigned
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// UInt
#[inline]
pub fn uint<T: BitVector>(x: T) -> u128 {
    x.to_uint()
}

/// Stanford pseudocode: SInt
///
/// Convert bit vector to signed integer (two's complement)
///
/// ```text
/// func SInt(x : bits(N)) => integer
/// begin
///     return integer value of x as signed
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// SInt
#[inline]
pub fn sint<T: BitVector>(x: T) -> i128 {
    x.to_sint()
}

/// Stanford pseudocode: IsZero
///
/// Check if bit vector is all zeros
///
/// ```text
/// func IsZero(x : bits(N)) => boolean
/// begin
///     return x == Zeros(N);
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// IsZero
#[inline]
pub fn is_zero<T: BitVector>(x: T) -> bool {
    x.is_zero()
}

/// Stanford pseudocode: IsOnes
///
/// Check if bit vector is all ones
///
/// ```text
/// func IsOnes(x : bits(N)) => boolean
/// begin
///     return x == Ones(N);
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// IsOnes
#[inline]
pub fn is_ones<T: BitVector>(x: T) -> bool {
    x.is_ones()
}

/// Stanford pseudocode: HighestSetBit
///
/// Find the index of the highest set bit
///
/// ```text
/// func HighestSetBit(x : bits(N)) => integer
/// begin
///     for i = N-1 downto 0
///         if x[i] == '1' then return i;
///     return -1;
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// HighestSetBit
#[inline]
pub fn highest_set_bit<T: BitVector>(x: T) -> Option<u32> {
    x.highest_set_bit()
}

/// Stanford pseudocode: LowestSetBit
///
/// Find the index of the lowest set bit
///
/// ```text
/// func LowestSetBit(x : bits(N)) => integer
/// begin
///     for i = 0 to N-1
///         if x[i] == '1' then return i;
///     return -1;
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// LowestSetBit
#[inline]
pub fn lowest_set_bit<T: BitVector>(x: T) -> Option<u32> {
    x.lowest_set_bit()
}

/// Stanford pseudocode: CountLeadingZeros
///
/// Count leading zeros in a bit vector
///
/// ```text
/// func CountLeadingZeros(x : bits(N)) => integer
/// begin
///     return number of leading zeros in x;
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// CountLeadingZeros
#[inline]
pub fn count_leading_zeros<T: BitVector>(x: T) -> u32 {
    x.clz()
}

/// Stanford pseudocode: CountTrailingZeros
///
/// Count trailing zeros in a bit vector
///
/// ```text
/// func CountTrailingZeros(x : bits(N)) => integer
/// begin
///     return number of trailing zeros in x;
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// CountTrailingZeros
#[inline]
pub fn count_trailing_zeros<T: BitVector>(x: T) -> u32 {
    x.ctz()
}

/// Stanford pseudocode: PopulationCount
///
/// Count the number of set bits
///
/// ```text
/// func PopulationCount(x : bits(N)) => integer
/// begin
///     return number of set bits in x;
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// PopulationCount
#[inline]
pub fn population_count<T: BitVector>(x: T) -> u32 {
    x.popcount()
}

/// Stanford pseudocode: SignExtend
///
/// Sign-extend a bit vector to a wider type
///
/// ```text
/// func SignExtend(x : bits(N)) => bits(M)
/// begin
///     return SignExtend{x}(M);
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// SignExtend
#[inline]
pub fn sign_extend<T: BitVector, U: BitVector>(x: T) -> U {
    let width_x = T::width();
    let width_u = U::width();
    if width_x >= width_u {
        return U::from_uint(x.to_uint());
    }
    let sign_bit = (x.to_uint() >> (width_x - 1)) & 1;
    if sign_bit == 1 {
        let mask = !0u128 << width_x;
        U::from_uint(x.to_uint() | mask)
    } else {
        U::from_uint(x.to_uint())
    }
}

/// Stanford pseudocode: ZeroExtend
///
/// Zero-extend a bit vector to a wider type
///
/// ```text
/// func ZeroExtend(x : bits(N)) => bits(M)
/// begin
///     return ZeroExtend{x}(M);
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// ZeroExtend
#[inline]
pub fn zero_extend<T: BitVector, U: BitVector>(x: T) -> U {
    U::from_uint(x.to_uint())
}

/// Stanford pseudocode: Align
///
/// Align address down to alignment boundary
///
/// ```text
/// func Align(address : bits(64), alignment : integer) => bits(64)
/// begin
///     let align_mask : bits(64) = (alignment - 1);
///     return address AND NOT(align_mask);
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// Align
#[inline]
pub fn align(address: u64, alignment: u64) -> u64 {
    let align_mask = alignment - 1;
    address & !align_mask
}

/// Align down to size boundary
#[inline]
pub fn align_down_size(address: u64, size: u64) -> u64 {
    align(address, size)
}

/// Align up to alignment boundary
#[inline]
pub fn align_up(address: u64, alignment: u64) -> u64 {
    let align_mask = alignment - 1;
    (address + align_mask) & !align_mask
}

/// Stanford pseudocode: ReplicateBit
///
/// Replicate a single bit N times
///
/// ```text
/// func ReplicateBit(bit : bit, N : integer) => bits(N)
/// begin
///     if bit == '1' then return Ones(N);
///     else return Zeros(N);
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// ReplicateBit
#[inline]
pub fn replicate_bit<T: BitVector>(bit: bool) -> T {
    if bit {
        T::from_uint(!0u128 >> (128 - T::width()))
    } else {
        T::from_uint(0)
    }
}

/// Stanford pseudocode: Reverse
///
/// Reverse bytes/halfwords/words in a bit vector
///
/// ```text
/// func Reverse(x : bits(N), element_size : integer) => bits(N)
/// begin
///     // Reverse the order of elements of size element_size
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// Reverse
#[inline]
pub fn reverse<T: BitVector>(word: T, element_size: u32) -> T {
    word.reverse(element_size)
}

/// Stanford pseudocode: BitReverse
///
/// Reverse the order of bits in a bit vector
///
/// ```text
/// func BitReverse(x : bits(N)) => bits(N)
/// begin
///     return Reverse(x, 1);
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// BitReverse
#[inline]
pub fn bit_reverse<T: BitVector>(x: T) -> T {
    x.reverse(1)
}

/// Stanford pseudocode: LSL (Logical Shift Left)
///
/// ```text
/// func LSL(x : bits(N), shift : integer) => bits(N)
/// begin
///     return x << shift;
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// LSL
#[inline]
pub fn lsl<T: BitVector>(x: T, shift: u32) -> T {
    x.shl(shift)
}

/// Stanford pseudocode: LSR (Logical Shift Right)
///
/// ```text
/// func LSR(x : bits(N), shift : integer) => bits(N)
/// begin
///     return x >> shift;
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// LSR
#[inline]
pub fn lsr<T: BitVector>(x: T, shift: u32) -> T {
    x.shr(shift)
}

/// Stanford pseudocode: ASR (Arithmetic Shift Right)
///
/// ```text
/// func ASR(x : bits(N), shift : integer) => bits(N)
/// begin
///     return x >>> shift;
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// ASR
#[inline]
pub fn asr<T: BitVector>(x: T, shift: u32) -> T {
    x.sar(shift)
}

/// Stanford pseudocode: ROR (Rotate Right)
///
/// ```text
/// func ROR(x : bits(N), shift : integer) => bits(N)
/// begin
///     shift = shift MOD N;
///     return x[shift-1:0] : x[N-1:shift];
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// ROR
#[inline]
pub fn ror<T: BitVector>(x: T, shift: u32) -> T {
    x.rotr(shift)
}

/// Stanford pseudocode: ROL (Rotate Left)
///
/// ```text
/// func ROL(x : bits(N), shift : integer) => bits(N)
/// begin
///     shift = shift MOD N;
///     return x[N-1-shift:0] : x[N-1:N-shift];
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// ROL
#[inline]
pub fn rol<T: BitVector>(x: T, shift: u32) -> T {
    x.rotl(shift)
}

/// Stanford pseudocode: Extract
///
/// Extract a bitfield from a bit vector
///
/// ```text
/// func Extract(x : bits(N), lsb : integer, width : integer) => bits(width)
/// begin
///     return x[lsb+width-1 : lsb];
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// Extract
#[inline]
pub fn extract<T: BitVector>(x: T, lsb: u32, width: u32) -> u128 {
    assert!(lsb + width <= T::width());
    x.bits(lsb + width - 1, lsb).to_uint()
}

/// Stanford pseudocode: Concatenation (:: operator)
///
/// ```text
/// a : b  // concatenation, a is high bits
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford operator:
/// ::
#[inline]
pub fn concat<T: BitVector>(high: T, low: T, low_width: u32) -> T {
    high.concat(low, low_width)
}

/// Stanford pseudocode: DecodeBitMasks
///
/// Decodes ARM64 logical immediate bitmasks from the (immN, imms, immr) encoding.
///
/// ```text
/// func DecodeBitMasks{M}(immN : bit, imms : bits(6), immr : bits(6), immediate : boolean) => (bits(M), bits(M))
/// begin
///     if (immN:NOT(imms)) == '000000' then UNDEFINED;
///     let len = HighestSetBitNZ(immN:NOT(imms));
///     assert 2 <= 2^len <= M;
///     let levels = ZeroExtend(Ones(len));
///     if immediate AND (imms AND levels == levels) then UNDEFINED;
///     let s = UInt(imms AND levels);
///     let r = UInt(immr AND levels);
///     let diff = s - r;
///     let esize = 2^len;
///     let d = UInt(diff[len-1:0]);
///     let welem = ZeroExtend(Ones(s+1));
///     let telem = ZeroExtend(Ones(d+1));
///     let wmask = Replicate(ROR(welem, r));
///     let tmask = Replicate(telem);
///     return (wmask, tmask);
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// DecodeBitMasks
pub fn decode_bit_masks<N: BitVector, M: BitVector>(
    imm_n: bool,
    imms: N,
    immr: N,
    immediate: bool,
) -> (M, M)
where
    N: BitVector,
    M: BitVector,
{
    // Width of N must be at least 6 bits (we only use lower 6 bits)
    // The actual encoding uses 6 bits, but we accept wider types for convenience
    assert!(N::width() >= 6, "N must have at least 6 bits");

    // Convert to u128 for bitwise operations
    let imms_val = imms.to_uint();
    let immr_val = immr.to_uint();

    // immN:NOT(imms) - concatenate imm_n with NOT(imms)
    // NOT(imms) = imms XOR 0x3F (for 6 bits)
    let not_imms = imms_val ^ 0x3F;
    let imm_n_not_imms = ((if imm_n { 1u128 << 5 } else { 0u128 }) | (not_imms & 0x3F)) as u8;

    // Check if immN:NOT(imms) == 0
    if imm_n_not_imms == 0 {
        panic!("DecodeBitMasks: UNDEFINED - immN:NOT(imms) == 0");
    }

    // len = HighestSetBitNZ(immN:NOT(imms))
    let len = highest_set_bit(imm_n_not_imms).expect("HighestSetBitNZ should not return None for non-zero");

    // Assert 2 <= 2^len <= M
    let two_pow_len = 1u128 << len;
    assert!(two_pow_len >= 2 && two_pow_len <= M::width() as u128, "DecodeBitMasks: 2 <= 2^len <= M violated");

    // levels = ZeroExtend(Ones(len))
    let ones_len = (1u128 << len) - 1;
    let levels = ones_len as u8;

    // If immediate AND (imms AND levels == levels) then UNDEFINED
    if immediate && (imms_val & (levels as u128)) == (levels as u128) {
        panic!("DecodeBitMasks: UNDEFINED - immediate with all bits set in levels");
    }

    // s = UInt(imms AND levels)
    let s = (imms_val & (levels as u128)) as u128;

    // r = UInt(immr AND levels)
    let r = (immr_val & (levels as u128)) as u128;

    // diff = s - r
    let diff = s.wrapping_sub(r);

    // esize = 2^len
    let esize = 1u128 << len;

    // d = UInt(diff[len-1:0])
    let d = diff & ((1u128 << len) - 1);

    // welem = ZeroExtend(Ones(s+1))
    let welem_ones = (1u128 << (s + 1)) - 1;
    let welem = M::from_uint(welem_ones);

    // telem = ZeroExtend(Ones(d+1))
    let telem_ones = (1u128 << (d + 1)) - 1;
    let telem = M::from_uint(telem_ones);

    // wmask = Replicate(ROR(welem, r)) -- rotation within esize bits
    let welem_val = welem.to_uint();
    let mask = (1u128 << esize) - 1;
    let welem_masked = welem_val & mask;
    let r_mod = (r % esize) as u32;
    let welem_rotated_val = if r == 0 {
        welem_masked
    } else {
        let right = welem_masked >> (r as u32);
        let left = (welem_masked << (esize as u32 - r as u32)) & ((1u128 << esize) - 1);
        right | left
    };
    let welem_rotated = M::from_uint(welem_rotated_val);
    let wmask = M::replicate(welem_rotated, M::width() / esize as u32);

    // tmask = Replicate(telem)
    let tmask = M::replicate(telem, M::width() / esize as u32);

    (wmask, tmask)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_bit_masks_basic() {
        // Test case: AND #0xFFFF0000FFFF0000 (immN=1, imms=0x2F, immr=0x10, immediate=false)
        // This produces: 16 ones, rotated by 16, replicated twice = 0xFFFF0000FFFF0000
        let imm_n = true;
        let imms: u8 = 0x2F; // 47
        let immr: u8 = 0x10; // 16
        let immediate = false;

        let (wmask, tmask) = decode_bit_masks::<u8, u64>(imm_n, imms, immr, immediate);
        // Pattern: 16 ones at top of each 32-bit half = 0xFFFF0000FFFF0000
        assert_eq!(wmask, 0xFFFF0000FFFF0000u64);
        // tmask differs (all ones for this encoding)
        assert_eq!(tmask, 0xFFFFFFFFFFFFFFFFu64);
    }

    #[test]
    fn test_decode_bit_masks_mov_imm() {
        // Test a known pattern: AND #0xFFFF0000FFFF0000
        // immN=1, imms=0x2F (47), immr=0x10 (16), immediate=false
        // This produces: 16 ones, rotated by 16, replicated twice = 0xFFFF0000FFFF0000
        let imm_n = true;
        let imms: u8 = 0x2F; // 47
        let immr: u8 = 0x10; // 16
        let immediate = false;

        let (wmask, tmask) = decode_bit_masks::<u8, u64>(imm_n, imms, immr, immediate);
        // Pattern: 16 ones at top of each 32-bit half = 0xFFFF0000FFFF0000
        assert_eq!(wmask, 0xFFFF0000FFFF0000u64);
        // tmask differs (all ones for this encoding)
        assert_eq!(tmask, 0xFFFFFFFFFFFFFFFFu64);
    }

    #[test]
    fn test_decode_bit_masks_undefined() {
        // immN=0, imms=0x3F -> immN:NOT(imms) = 0:000000 = 0 -> UNDEFINED
        let imm_n = false;
        let imms: u8 = 0x3F;
        let immr: u8 = 0x00;
        let immediate = false;

        let result = std::panic::catch_unwind(|| {
            decode_bit_masks::<u8, u64>(imm_n, imms, immr, false)
        });
        assert!(result.is_err());
    }
}