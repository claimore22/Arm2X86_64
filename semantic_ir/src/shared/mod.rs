//! Stanford ARM64 Shared Pseudocode - Rust Implementation
//!
//! This module provides executable Rust implementations of the shared pseudocode
//! functions from the ARM Architecture Reference Manual (Stanford edition).
//!
//! Source:
//! https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html

pub mod arithmetic;
pub mod bit_ops;
pub mod enums;
pub mod features;
pub mod predicates;
pub mod registers;
pub mod system;

// Re-export arithmetic
pub use arithmetic::{
    add_with_carry, sub_with_carry, add, sub, sq_add, uq_add, sq_sub, uq_sub,
    sq_dmulh, abs_diff,
};

// Re-export bit_ops
pub use bit_ops::{
    uint, sint, is_zero, is_ones, highest_set_bit, lowest_set_bit,
    count_leading_zeros, count_trailing_zeros, population_count,
    sign_extend, zero_extend, align, align_up, align_down_size,
    replicate_bit, reverse, bit_reverse, lsl, lsr, asr, ror, rol, extract, concat,
    decode_bit_masks,
};

// Re-export enums
pub use enums::{
    ExceptionLevel, SecurityState, Condition, ConditionFlags,
    TranslationRegime, CacheOp, CacheOpScope, Shareability, MemType, DeviceType,
    MemAttr, FaultStatus, Constraint, Unpredictable, AccessType, Endianness,
    BranchPredictorBehavior, PMUEventType, PhysMemRetStatus, MemOp, BranchType, MemAtomicOp,
};

// Re-export features
pub use features::{
    is_feature_implemented, have_el, el_using_aarch32, using_aarch32,
    have_aarch64, have_aarch32, is_secure_el2_enabled, is_in_host, el_is_in_host,
    current_security_state, ArmFeature, FeatureRegisters, set_feature_registers,
};

// Re-export predicates
pub use predicates::{
    condition_holds, condition_passed, condition_holds_nzcv,
    is_unconditional, is_never, invert_condition, bit_test, condition_syndrome,
};

// Re-export registers
pub use registers::{
    pstate_el, pstate_n, pstate_z, pstate_c, pstate_v, pstate_sp, pstate_ssbs,
    edscr, halted,
    PState, GeneralRegisters, SimdRegisters, ScalableVectorRegisters,
    SystemRegId, SystemRegisters, CpuState,
};

// Re-export system
pub use system::{
    PhysicalAddress, PASpace, MemoryAttributes, MemHint, AddressDescriptor,
    FaultRecord, AccessDescriptor, MPAMinfo, MemorySystem, ATAccess,
    no_fault, is_fault, is_external_abort, is_external_sync_abort, is_async_abort,
    is_debug_exception, create_acc_desc_i_fetch, create_acc_desc_gpr, create_acc_desc_dc,
    create_acc_desc_at, aarch64_branch_addr, addr_top, effective_tbi,
    align_address, big_endian_reverse, mem_read_with_endian, mem_write_with_endian,
    clear_tag, aarch64_translate_address, check_sp_alignment,
    mem_single_granule, aarch64_mem_single_read, aarch64_mem_single_write,
};