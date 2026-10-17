//! Stanford ARM64 Shared Pseudocode - Predicate/Condition Functions
//!
//! Source:
//! https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
//!
//! Stanford functions: ConditionHolds, ConditionPassed, etc.

use crate::shared::enums::{Condition, ConditionFlags};

/// Stanford pseudocode: ConditionHolds
///
/// Determines if a condition code is satisfied given the current flags
///
/// ```text
/// func ConditionHolds(cond : Condition) => boolean
/// begin
///     case cond of
///         when EQ => return Z == '1';
///         when NE => return Z == '0';
///         when CS => return C == '1';
///         when CC => return C == '0';
///         when MI => return N == '1';
///         when PL => return N == '0';
///         when VS => return V == '1';
///         when VC => return V == '0';
///         when HI => return C == '1' && Z == '0';
///         when LS => return C == '0' || Z == '1';
///         when GE => return N == V;
///         when LT => return N != V;
///         when GT => return Z == '0' && N == V;
///         when LE => return Z == '1' || N != V;
///         when AL => return TRUE;
///         when NV => return FALSE;
///     end;
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// ConditionHolds
#[inline]
pub fn condition_holds(cond: Condition, flags: ConditionFlags) -> bool {
    flags.condition_holds(cond)
}

/// Stanford pseudocode: ConditionPassed
///
/// Alias for ConditionHolds, used in instruction pseudocode
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// ConditionPassed
#[inline]
pub fn condition_passed(cond: Condition, flags: ConditionFlags) -> bool {
    condition_holds(cond, flags)
}

/// Evaluate condition against raw NZCV bits
#[inline]
pub fn condition_holds_nzcv(cond: Condition, nzcv: u8) -> bool {
    let flags = ConditionFlags::from_nzcv(nzcv);
    condition_holds(cond, flags)
}

/// Check if condition is unconditional (AL)
#[inline]
pub fn is_unconditional(cond: Condition) -> bool {
    cond == Condition::AL
}

/// Check if condition is "never" (NV)
#[inline]
pub fn is_never(cond: Condition) -> bool {
    cond == Condition::NV
}

/// Get the inverse condition
#[inline]
pub fn invert_condition(cond: Condition) -> Condition {
    cond.invert()
}

/// Stanford pseudocode: BitTest
///
/// Extract a single bit from a bit vector
///
/// ```text
/// func BitTest(x : bits(N), bit : integer) => bit
/// begin
///     return x[bit];
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// BitTest
#[inline]
pub fn bit_test<T: crate::shared::bit_ops::BitVector>(x: T, bit: u32) -> bool {
    x.bit(bit)
}

/// Stanford pseudocode: ConditionSyndrome
///
/// Returns the syndrome value for a conditional instruction that fails its condition check
///
/// ```text
/// func ConditionSyndrome(cond : Condition) => bits(4)
/// begin
///     return cond<3:0>;
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// ConditionSyndrome
#[inline]
pub fn condition_syndrome(cond: Condition) -> u8 {
    cond.encoding()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::enums::ConditionFlags;

    #[test]
    fn test_condition_holds_eq() {
        let flags = ConditionFlags::new(false, true, false, false); // Z=1
        assert!(condition_holds(Condition::EQ, flags));
        let flags = ConditionFlags::new(false, false, false, false); // Z=0
        assert!(!condition_holds(Condition::EQ, flags));
    }

    #[test]
    fn test_condition_holds_ne() {
        let flags = ConditionFlags::new(false, false, false, false); // Z=0
        assert!(condition_holds(Condition::NE, flags));
        let flags = ConditionFlags::new(false, true, false, false); // Z=1
        assert!(!condition_holds(Condition::NE, flags));
    }

    #[test]
    fn test_condition_holds_cs_cc() {
        let flags = ConditionFlags::new(false, false, true, false); // C=1
        assert!(condition_holds(Condition::CS, flags));
        assert!(!condition_holds(Condition::CC, flags));

        let flags = ConditionFlags::new(false, false, false, false); // C=0
        assert!(condition_holds(Condition::CC, flags));
        assert!(!condition_holds(Condition::CS, flags));
    }

    #[test]
    fn test_condition_holds_mi_pl() {
        let flags = ConditionFlags::new(true, false, false, false); // N=1
        assert!(condition_holds(Condition::MI, flags));
        assert!(!condition_holds(Condition::PL, flags));

        let flags = ConditionFlags::new(false, false, false, false); // N=0
        assert!(condition_holds(Condition::PL, flags));
        assert!(!condition_holds(Condition::MI, flags));
    }

    #[test]
    fn test_condition_holds_vs_vc() {
        let flags = ConditionFlags::new(false, false, false, true); // V=1
        assert!(condition_holds(Condition::VS, flags));
        assert!(!condition_holds(Condition::VC, flags));

        let flags = ConditionFlags::new(false, false, false, false); // V=0
        assert!(condition_holds(Condition::VC, flags));
        assert!(!condition_holds(Condition::VS, flags));
    }

    #[test]
    fn test_condition_holds_hi_ls() {
        // HI: C=1 && Z=0
        let flags = ConditionFlags::new(false, false, true, false);
        assert!(condition_holds(Condition::HI, flags));
        assert!(!condition_holds(Condition::LS, flags));

        // LS: C=0 || Z=1
        let flags = ConditionFlags::new(false, true, false, false); // Z=1
        assert!(condition_holds(Condition::LS, flags));
        assert!(!condition_holds(Condition::HI, flags));

        let flags = ConditionFlags::new(false, false, false, false); // C=0
        assert!(condition_holds(Condition::LS, flags));
        assert!(!condition_holds(Condition::HI, flags));
    }

    #[test]
    fn test_condition_holds_ge_lt() {
        // GE: N == V
        let flags = ConditionFlags::new(true, false, false, true); // N=1, V=1
        assert!(condition_holds(Condition::GE, flags));
        assert!(!condition_holds(Condition::LT, flags));

        let flags = ConditionFlags::new(false, false, false, false); // N=0, V=0
        assert!(condition_holds(Condition::GE, flags));
        assert!(!condition_holds(Condition::LT, flags));

        // LT: N != V
        let flags = ConditionFlags::new(true, false, false, false); // N=1, V=0
        assert!(condition_holds(Condition::LT, flags));
        assert!(!condition_holds(Condition::GE, flags));

        let flags = ConditionFlags::new(false, false, false, true); // N=0, V=1
        assert!(condition_holds(Condition::LT, flags));
        assert!(!condition_holds(Condition::GE, flags));
    }

    #[test]
    fn test_condition_holds_gt_le() {
        // GT: Z=0 && N==V
        let flags = ConditionFlags::new(false, false, false, false); // N=0, V=0, Z=0
        assert!(condition_holds(Condition::GT, flags));
        assert!(!condition_holds(Condition::LE, flags));

        let flags = ConditionFlags::new(true, false, false, true); // N=1, V=1, Z=0
        assert!(condition_holds(Condition::GT, flags));
        assert!(!condition_holds(Condition::LE, flags));

        // LE: Z=1 || N!=V
        let flags = ConditionFlags::new(false, true, false, false); // Z=1
        assert!(condition_holds(Condition::LE, flags));
        assert!(!condition_holds(Condition::GT, flags));

        let flags = ConditionFlags::new(true, false, false, false); // N=1, V=0
        assert!(condition_holds(Condition::LE, flags));
        assert!(!condition_holds(Condition::GT, flags));
    }

    #[test]
    fn test_condition_holds_al_nv() {
        let flags = ConditionFlags::default();
        assert!(condition_holds(Condition::AL, flags));
        assert!(!condition_holds(Condition::NV, flags));
    }

    #[test]
    fn test_invert_condition() {
        assert_eq!(invert_condition(Condition::EQ), Condition::NE);
        assert_eq!(invert_condition(Condition::NE), Condition::EQ);
        assert_eq!(invert_condition(Condition::CS), Condition::CC);
        assert_eq!(invert_condition(Condition::CC), Condition::CS);
        assert_eq!(invert_condition(Condition::MI), Condition::PL);
        assert_eq!(invert_condition(Condition::PL), Condition::MI);
        assert_eq!(invert_condition(Condition::VS), Condition::VC);
        assert_eq!(invert_condition(Condition::VC), Condition::VS);
        assert_eq!(invert_condition(Condition::HI), Condition::LS);
        assert_eq!(invert_condition(Condition::LS), Condition::HI);
        assert_eq!(invert_condition(Condition::GE), Condition::LT);
        assert_eq!(invert_condition(Condition::LT), Condition::GE);
        assert_eq!(invert_condition(Condition::GT), Condition::LE);
        assert_eq!(invert_condition(Condition::LE), Condition::GT);
        assert_eq!(invert_condition(Condition::AL), Condition::NV);
        assert_eq!(invert_condition(Condition::NV), Condition::AL);
    }
}