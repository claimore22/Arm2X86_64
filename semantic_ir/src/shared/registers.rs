//! Stanford ARM64 Shared Pseudocode - Register State
//!
//! Source:
//! https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
//!
//! Stanford concepts: PSTATE, system registers, special registers

use crate::shared::enums::{Condition, ConditionFlags, ExceptionLevel, SecurityState, Shareability};
use core::fmt;

/// PSTATE - Processor State
///
/// This represents the architectural PSTATE register which contains
/// condition flags, exception level, execution state, and other control bits.
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
#[derive(Debug, Clone, Default)]
pub struct PState {
    /// Condition flags (NZCV)
    pub flags: ConditionFlags,

    /// Current exception level
    pub el: ExceptionLevel,

    /// Stack pointer selection (0 = SP0, 1 = SP_ELx)
    pub sp: bool,

    /// Instruction set state (always AArch64 in this project)
    pub aarch64: bool,

    /// Debug exception mask (D)
    pub d: bool,

    /// SError interrupt mask (A)
    pub a: bool,

    /// IRQ interrupt mask (I)
    pub i: bool,

    /// FIQ interrupt mask (F)
    pub f: bool,

    /// Single-step bit (SS)
    pub ss: bool,

    /// Illegal execution state bit (IL)
    pub il: bool,

    /// Software step bit (for debugging)
    pub software_step: bool,

    /// Pan bit (FEAT_PAN)
    pub pan: bool,

    /// Speculative Store Bypass Safe (FEAT_SSBS)
    pub ssbs: bool,

    /// Trace prohibit (FEAT_ETE)
    pub t: bool,

    /// Endianness (always Little in this implementation)
    pub endian: crate::shared::enums::Endianness,

    /// PACM (Pointer Authentication Code Mask, FEAT_PAuth_LR)
    pub pacm: bool,

    /// UAO (User Access Override, FEAT_UAO)
    pub uao: bool,

    /// DIT (Data Independent Timing, FEAT_DIT)
    pub dit: bool,

    /// TCO (Trap Cache Operations, FEAT_TCO)
    pub tco: bool,

    /// ALLINT (All Interrupts, FEAT_HCX)
    pub allint: bool,

    /// NMI (Non-Maskable Interrupt, FEAT_NMI)
    pub nmi: bool,
}

impl PState {
    /// Create a new PState with default values for AArch64
    pub fn new_aarch64(el: ExceptionLevel) -> Self {
        Self {
            flags: ConditionFlags::default(),
            el,
            sp: false, // SP0
            aarch64: true,
            d: false,
            a: false,
            i: false,
            f: false,
            ss: false,
            il: false,
            software_step: false,
            pan: false,
            ssbs: false,
            t: false,
            endian: crate::shared::enums::Endianness::Little,
            pacm: false,
            uao: false,
            dit: false,
            tco: false,
            allint: false,
            nmi: false,
        }
    }

    /// Get the NZCV flags as a 4-bit value
    pub fn nzcv(&self) -> u8 {
        self.flags.to_nzcv()
    }

    /// Set the NZCV flags from a 4-bit value
    pub fn set_nzcv(&mut self, nzcv: u8) {
        self.flags = ConditionFlags::from_nzcv(nzcv);
    }

    /// Check if a condition holds
    pub fn condition_holds(&self, cond: Condition) -> bool {
        self.flags.condition_holds(cond)
    }

    /// Get current exception level
    pub fn current_el(&self) -> ExceptionLevel {
        self.el
    }

    /// Set current exception level
    pub fn set_el(&mut self, el: ExceptionLevel) {
        self.el = el;
    }

    /// Check if in EL0
    pub fn is_el0(&self) -> bool {
        self.el == ExceptionLevel::EL0
    }

    /// Check if in EL1
    pub fn is_el1(&self) -> bool {
        self.el == ExceptionLevel::EL1
    }

    /// Check if in EL2
    pub fn is_el2(&self) -> bool {
        self.el == ExceptionLevel::EL2
    }

    /// Check if in EL3
    pub fn is_el3(&self) -> bool {
        self.el == ExceptionLevel::EL3
    }
}

/// General purpose registers (X0-X30, SP, PC)
#[derive(Debug, Clone)]
pub struct GeneralRegisters {
    /// X0-X30
    pub x: [u64; 31],
    /// Stack pointer (SP_EL0/SP_ELx)
    pub sp: u64,
    /// Program counter
    pub pc: u64,
}

impl GeneralRegisters {
    pub fn new() -> Self {
        Self {
            x: [0; 31],
            sp: 0,
            pc: 0,
        }
    }

    /// Read X register (X0-X30, XZR=31)
    pub fn read_x(&self, reg: u8) -> u64 {
        if reg == 31 {
            0 // XZR/WZR
        } else if reg < 31 {
            self.x[reg as usize]
        } else {
            panic!("Invalid X register: {}", reg)
        }
    }

    /// Write X register (X0-X30)
    pub fn write_x(&mut self, reg: u8, value: u64) {
        if reg < 31 {
            self.x[reg as usize] = value;
        } else {
            panic!("Cannot write to XZR (register 31)")
        }
    }

    /// Read W register (lower 32 bits of X0-X30, WZR=31)
    pub fn read_w(&self, reg: u8) -> u32 {
        if reg == 31 {
            0
        } else if reg < 31 {
            self.x[reg as usize] as u32
        } else {
            panic!("Invalid W register: {}", reg)
        }
    }

    /// Write W register (zero-extends to upper 32 bits)
    pub fn write_w(&mut self, reg: u8, value: u32) {
        if reg < 31 {
            self.x[reg as usize] = (self.x[reg as usize] & 0xFFFF_FFFF_0000_0000) | (value as u64);
        } else {
            panic!("Cannot write to WZR (register 31)")
        }
    }

    /// Read SP
    pub fn read_sp(&self) -> u64 {
        self.sp
    }

    /// Write SP
    pub fn write_sp(&mut self, value: u64) {
        self.sp = value;
    }

    /// Read PC
    pub fn read_pc(&self) -> u64 {
        self.pc
    }

    /// Write PC
    pub fn write_pc(&mut self, value: u64) {
        self.pc = value;
    }
}

impl Default for GeneralRegisters {
    fn default() -> Self {
        Self::new()
    }
}

/// SIMD/Floating-point registers (V0-V31, 128-bit each)
#[derive(Debug, Clone)]
pub struct SimdRegisters {
    pub v: [u128; 32],
}

impl SimdRegisters {
    pub fn new() -> Self {
        Self { v: [0; 32] }
    }

    pub fn read_v(&self, reg: u8) -> u128 {
        if reg < 32 {
            self.v[reg as usize]
        } else {
            panic!("Invalid V register: {}", reg)
        }
    }

    pub fn write_v(&mut self, reg: u8, value: u128) {
        if reg < 32 {
            self.v[reg as usize] = value;
        } else {
            panic!("Invalid V register: {}", reg)
        }
    }
}

impl Default for SimdRegisters {
    fn default() -> Self {
        Self::new()
    }
}

/// SVE/SME scalable vector registers (Z0-Z31, P0-P15, FFR)
/// Vector length is configurable (VL)
#[derive(Debug, Clone)]
pub struct ScalableVectorRegisters {
    pub vl: u32, // Vector length in bits (128, 256, 512, ..., 2048)
    pub z: Vec<Vec<u8>>, // Z0-Z31, each is vl/8 bytes
    pub p: Vec<Vec<u8>>, // P0-P15, each is vl/8 bits (vl/64 bytes)
    pub ffr: Vec<u8>,    // First-fault register, vl/8 bits
}

impl ScalableVectorRegisters {
    pub fn new(vl: u32) -> Self {
        let vl_bytes = (vl / 8) as usize;
        let pl_bytes = (vl / 64) as usize;
        Self {
            vl,
            z: vec![vec![0; vl_bytes]; 32],
            p: vec![vec![0; pl_bytes]; 16],
            ffr: vec![0; pl_bytes],
        }
    }

    pub fn read_z(&self, reg: u8) -> &[u8] {
        if reg < 32 {
            &self.z[reg as usize]
        } else {
            panic!("Invalid Z register: {}", reg)
        }
    }

    pub fn write_z(&mut self, reg: u8, value: &[u8]) {
        if reg < 32 && value.len() == (self.vl / 8) as usize {
            self.z[reg as usize].copy_from_slice(value);
        } else {
            panic!("Invalid Z register or size")
        }
    }

    pub fn read_p(&self, reg: u8) -> &[u8] {
        if reg < 16 {
            &self.p[reg as usize]
        } else {
            panic!("Invalid P register: {}", reg)
        }
    }

    pub fn write_p(&mut self, reg: u8, value: &[u8]) {
        if reg < 16 && value.len() == (self.vl / 64) as usize {
            self.p[reg as usize].copy_from_slice(value);
        } else {
            panic!("Invalid P register or size")
        }
    }

    pub fn read_ffr(&self) -> &[u8] {
        &self.ffr
    }

    pub fn write_ffr(&mut self, value: &[u8]) {
        if value.len() == (self.vl / 64) as usize {
            self.ffr.copy_from_slice(value);
        } else {
            panic!("Invalid FFR size")
        }
    }
}

/// Translation table base register configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TCRConfig {
    pub t0sz: u8,      // Size offset for TTBR0
    pub t1sz: u8,      // Size offset for TTBR1
    pub tg0: u8,       // Granule size for TTBR0 (0=4KB, 1=64KB, 2=16KB)
    pub tg1: u8,       // Granule size for TTBR1 (0=4KB, 1=64KB, 2=16KB)
    pub sh0: Shareability, // Shareability for TTBR0
    pub sh1: Shareability, // Shareability for TTBR1
    pub orgn0: u8,     // Outer cacheability for TTBR0
    pub irgn0: u8,     // Inner cacheability for TTBR0
    pub orgn1: u8,     // Outer cacheability for TTBR1
    pub irgn1: u8,     // Inner cacheability for TTBR1
    pub e0pd: bool,    // EL0 data access disable for TTBR0
    pub e1pd: bool,    // EL0 data access disable for TTBR1
    pub tbi0: bool,    // Top Byte Ignored for TTBR0
    pub tbi1: bool,    // Top Byte Ignored for TTBR1
    pub ha: bool,      // Hardware Access Flag update
    pub hd: bool,      // Hardware Dirty Bit update
    pub tbid0: bool,   // TBI for TTBR0
    pub tbid1: bool,   // TBI for TTBR1
}

/// System Control Register (SCTLR) configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SCTLRConfig {
    pub m: bool,       // MMU enable
    pub a: bool,       // Alignment check enable
    pub c: bool,       // Cache enable
    pub sa: bool,      // SP alignment check enable (EL0)
    pub sa0: bool,     // SP alignment check enable (EL0, 32-byte)
    pub cpm: bool,     // CP15 memory barrier enable
    pub cp15ben: bool, // CP15 barrier enable
    pub itd: bool,     // IT instruction disable
    pub sed: bool,     // SETEND disable
    pub uct: bool,     // Unprivileged cache disable
    pub dze: bool,     // DC ZVA enable
    pub uci: bool,     // User cache invalidation enable
    pub eece: bool,    // Exception entry cache enable
    pub ee: bool,      // Exception endianness
    pub e0e: bool,     // EL0 endianness
    pub we: bool,      // Wide access enable
    pub tcf: bool,     // Trap cache flush
    pub twf: bool,     // Trap WFE
    pub twi: bool,     // Trap WFI
    pub dssbs: bool,   // Default SSBS
    pub at: bool,      // Address translation enable
    pub mte: bool,     // MTE enable
}

/// System register access
///
/// In ARM, system registers are accessed via MSR/MRS instructions
/// and are identified by (op0, op1, CRn, CRm, op2) tuple.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SystemRegId {
    pub op0: u8,
    pub op1: u8,
    pub crn: u8,
    pub crm: u8,
    pub op2: u8,
}

impl SystemRegId {
    pub const fn new(op0: u8, op1: u8, crn: u8, crm: u8, op2: u8) -> Self {
        Self {
            op0,
            op1,
            crn,
            crm,
            op2,
        }
    }
}

/// System register file
#[derive(Debug, Clone, Default)]
pub struct SystemRegisters {
    // We'll store a subset of commonly used system registers
    // In a full implementation, this would be a HashMap or array
    pub sctlr_el1: u64,
    pub sctlr_el2: u64,
    pub sctlr_el3: u64,
    pub ttbr0_el1: u64,
    pub ttbr1_el1: u64,
    pub tcr_el1: u64,
    pub tcr_el2: u64,
    pub tcr_el3: u64,
    pub esr_el1: u64,
    pub esr_el2: u64,
    pub esr_el3: u64,
    pub far_el1: u64,
    pub far_el2: u64,
    pub far_el3: u64,
    pub mair_el1: u64,
    pub mair_el2: u64,
    pub mair_el3: u64,
    pub vbar_el1: u64,
    pub vbar_el2: u64,
    pub vbar_el3: u64,
    pub scr_el3: u64,
    pub hcr_el2: u64,
    pub cptr_el2: u64,
    pub cptr_el3: u64,
    pub mdscr_el1: u64,
    pub mdcr_el2: u64,
    pub mdcr_el3: u64,
    pub edscr: u64,
    pub pmcr_el0: u64,
    pub pmselr_el0: u64,
    pub pmcntenset_el0: u64,
    pub pmcntenclr_el0: u64,
    pub pmovsclr_el0: u64,
    pub pmswinc_el0: u64,
    pub pmovsset_el0: u64,
    pub pmcer_el0: u64,
    pub pmccntr_el0: u64,
    pub pmxevtyper_el0: u64,
    pub pmxevcntr_el0: u64,
    pub pmevcntr_el0: [u64; 31], // 0-30
    pub pmevtyper_el0: [u64; 31],
        pub pmccfiltr_el0: u64,
    // TCR parsed config
    pub tcr_el1_config: TCRConfig,
    pub sctlr_el1_config: SCTLRConfig,
}

impl SystemRegisters {
    pub fn new() -> Self {
        Self::default()
    }

    /// Read a system register by ID
    pub fn read(&self, id: SystemRegId) -> u64 {
        // Simplified - real implementation would decode the ID
        match (id.op0, id.op1, id.crn, id.crm, id.op2) {
            (3, 0, 1, 0, 0) => self.sctlr_el1,   // SCTLR_EL1
            (3, 4, 1, 0, 0) => self.sctlr_el2,   // SCTLR_EL2
            (3, 6, 1, 0, 0) => self.sctlr_el3,   // SCTLR_EL3
            (3, 0, 2, 0, 0) => self.ttbr0_el1,   // TTBR0_EL1
            (3, 0, 2, 0, 1) => self.ttbr1_el1,   // TTBR1_EL1
            (3, 0, 2, 0, 2) => self.tcr_el1,     // TCR_EL1
            (3, 4, 2, 0, 2) => self.tcr_el2,     // TCR_EL2
            (3, 6, 2, 0, 2) => self.tcr_el3,     // TCR_EL3
            (3, 0, 5, 2, 0) => self.esr_el1,     // ESR_EL1
            (3, 4, 5, 2, 0) => self.esr_el2,     // ESR_EL2
            (3, 6, 5, 2, 0) => self.esr_el3,     // ESR_EL3
            (3, 0, 6, 0, 0) => self.far_el1,     // FAR_EL1
            (3, 4, 6, 0, 0) => self.far_el2,     // FAR_EL2
            (3, 6, 6, 0, 0) => self.far_el3,     // FAR_EL3
            (3, 0, 10, 2, 0) => self.mair_el1,   // MAIR_EL1
            (3, 4, 10, 2, 0) => self.mair_el2,   // MAIR_EL2
            (3, 6, 10, 2, 0) => self.mair_el3,   // MAIR_EL3
            (3, 0, 12, 0, 0) => self.vbar_el1,   // VBAR_EL1
            (3, 4, 12, 0, 0) => self.vbar_el2,   // VBAR_EL2
            (3, 6, 12, 0, 0) => self.vbar_el3,   // VBAR_EL3
            (3, 6, 1, 1, 0) => self.scr_el3,     // SCR_EL3
            (3, 4, 1, 1, 0) => self.hcr_el2,     // HCR_EL2
            (2, 0, 0, 2, 0) => self.edscr,       // EDSCR
            _ => 0, // Unknown register returns 0 (or could panic)
        }
    }

    /// Write a system register by ID
    pub fn write(&mut self, id: SystemRegId, value: u64) {
        match (id.op0, id.op1, id.crn, id.crm, id.op2) {
            (3, 0, 1, 0, 0) => self.sctlr_el1 = value,   // SCTLR_EL1
            (3, 4, 1, 0, 0) => self.sctlr_el2 = value,   // SCTLR_EL2
            (3, 6, 1, 0, 0) => self.sctlr_el3 = value,   // SCTLR_EL3
            (3, 0, 2, 0, 0) => self.ttbr0_el1 = value,   // TTBR0_EL1
            (3, 0, 2, 0, 1) => self.ttbr1_el1 = value,   // TTBR1_EL1
            (3, 0, 2, 0, 2) => self.tcr_el1 = value,     // TCR_EL1
            (3, 4, 2, 0, 2) => self.tcr_el2 = value,     // TCR_EL2
            (3, 6, 2, 0, 2) => self.tcr_el3 = value,     // TCR_EL3
            (3, 0, 5, 2, 0) => self.esr_el1 = value,     // ESR_EL1
            (3, 4, 5, 2, 0) => self.esr_el2 = value,     // ESR_EL2
            (3, 6, 5, 2, 0) => self.esr_el3 = value,     // ESR_EL3
            (3, 0, 6, 0, 0) => self.far_el1 = value,     // FAR_EL1
            (3, 4, 6, 0, 0) => self.far_el2 = value,     // FAR_EL2
            (3, 6, 6, 0, 0) => self.far_el3 = value,     // FAR_EL3
            (3, 0, 10, 2, 0) => self.mair_el1 = value,   // MAIR_EL1
            (3, 4, 10, 2, 0) => self.mair_el2 = value,   // MAIR_EL2
            (3, 6, 10, 2, 0) => self.mair_el3 = value,   // MAIR_EL3
            (3, 0, 12, 0, 0) => self.vbar_el1 = value,   // VBAR_EL1
            (3, 4, 12, 0, 0) => self.vbar_el2 = value,   // VBAR_EL2
            (3, 6, 12, 0, 0) => self.vbar_el3 = value,   // VBAR_EL3
            (3, 6, 1, 1, 0) => self.scr_el3 = value,     // SCR_EL3
            (3, 4, 1, 1, 0) => self.hcr_el2 = value,     // HCR_EL2
            (2, 0, 0, 2, 0) => self.edscr = value,       // EDSCR
            _ => {} // Ignore unknown registers
        }
    }

    /// Update TCR_EL1 parsed config from raw register value
    pub fn update_tcr_el1_config(&mut self) {
        let tcr = self.tcr_el1;
        self.tcr_el1_config = TCRConfig {
            t0sz: ((tcr >> 0) & 0x3F) as u8,
            t1sz: ((tcr >> 16) & 0x3F) as u8,
            tg0: ((tcr >> 14) & 0x3) as u8,
            tg1: ((tcr >> 30) & 0x3) as u8,
            sh0: match (tcr >> 12) & 0x3 {
                0 => Shareability::NonShareable,
                1 => Shareability::OuterShareable,
                2 => Shareability::InnerShareable,
                _ => Shareability::NonShareable,
            },
            sh1: match (tcr >> 28) & 0x3 {
                0 => Shareability::NonShareable,
                1 => Shareability::OuterShareable,
                2 => Shareability::InnerShareable,
                _ => Shareability::NonShareable,
            },
            orgn0: ((tcr >> 8) & 0x3) as u8,
            irgn0: ((tcr >> 10) & 0x3) as u8,
            orgn1: ((tcr >> 24) & 0x3) as u8,
            irgn1: ((tcr >> 26) & 0x3) as u8,
            e0pd: (tcr & (1 << 7)) != 0,
            e1pd: (tcr & (1 << 23)) != 0,
            tbi0: (tcr & (1 << 20)) != 0,
            tbi1: (tcr & (1 << 36)) != 0,
            ha: (tcr & (1 << 21)) != 0,
            hd: (tcr & (1 << 22)) != 0,
            tbid0: (tcr & (1 << 38)) != 0,
            tbid1: (tcr & (1 << 37)) != 0,
        };
    }

    /// Update SCTLR_EL1 parsed config from raw register value
    pub fn update_sctlr_el1_config(&mut self) {
        let sctlr = self.sctlr_el1;
        self.sctlr_el1_config = SCTLRConfig {
            m: (sctlr & (1 << 0)) != 0,
            a: (sctlr & (1 << 1)) != 0,
            c: (sctlr & (1 << 2)) != 0,
            sa: (sctlr & (1 << 3)) != 0,
            sa0: (sctlr & (1 << 4)) != 0,
            cpm: (sctlr & (1 << 5)) != 0,
            cp15ben: (sctlr & (1 << 6)) != 0,
            itd: (sctlr & (1 << 7)) != 0,
            sed: (sctlr & (1 << 8)) != 0,
            uct: (sctlr & (1 << 9)) != 0,
            dze: (sctlr & (1 << 10)) != 0,
            uci: (sctlr & (1 << 11)) != 0,
            eece: (sctlr & (1 << 12)) != 0,
            ee: (sctlr & (1 << 13)) != 0,
            e0e: (sctlr & (1 << 14)) != 0,
            we: (sctlr & (1 << 15)) != 0,
            tcf: (sctlr & (1 << 16)) != 0,
            twf: (sctlr & (1 << 16)) != 0,
            twi: (sctlr & (1 << 17)) != 0,
            dssbs: (sctlr & (1 << 18)) != 0,
            at: (sctlr & (1 << 19)) != 0,
            mte: (sctlr & (1 << 20)) != 0,
        };
    }
}

/// Complete CPU state combining all register files
#[derive(Debug, Clone)]
pub struct CpuState {
    pub pstate: PState,
    pub gpr: GeneralRegisters,
    pub simd: SimdRegisters,
    pub sve: Option<ScalableVectorRegisters>,
    pub sys: SystemRegisters,
    pub memory: crate::shared::system::MemorySystem,
}

impl CpuState {
    pub fn new() -> Self {
        Self {
            pstate: PState::new_aarch64(ExceptionLevel::EL1),
            gpr: GeneralRegisters::new(),
            simd: SimdRegisters::new(),
            sve: None,
            sys: SystemRegisters::new(),
            memory: crate::shared::system::MemorySystem::new(),
        }
    }

    pub fn with_el(mut self, el: ExceptionLevel) -> Self {
        self.pstate.set_el(el);
        self
    }

    pub fn with_sve(mut self, vl: u32) -> Self {
        self.sve = Some(ScalableVectorRegisters::new(vl));
        self
    }
}

impl Default for CpuState {
    fn default() -> Self {
        Self::new()
    }
}

/// Stanford pseudocode: PSTATE.EL accessor
///
/// ```text
/// PSTATE.EL
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford field:
/// PSTATE.EL
#[inline]
pub fn pstate_el(state: &CpuState) -> ExceptionLevel {
    state.pstate.current_el()
}

/// Stanford pseudocode: PSTATE.N accessor
///
/// ```text
/// PSTATE.N
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford field:
/// PSTATE.N
#[inline]
pub fn pstate_n(state: &CpuState) -> bool {
    state.pstate.flags.n
}

/// Stanford pseudocode: PSTATE.Z accessor
///
/// ```text
/// PSTATE.Z
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford field:
/// PSTATE.Z
#[inline]
pub fn pstate_z(state: &CpuState) -> bool {
    state.pstate.flags.z
}

/// Stanford pseudocode: PSTATE.C accessor
///
/// ```text
/// PSTATE.C
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford field:
/// PSTATE.C
#[inline]
pub fn pstate_c(state: &CpuState) -> bool {
    state.pstate.flags.c
}

/// Stanford pseudocode: PSTATE.V accessor
///
/// ```text
/// PSTATE.V
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford field:
/// PSTATE.V
#[inline]
pub fn pstate_v(state: &CpuState) -> bool {
    state.pstate.flags.v
}

/// Stanford pseudocode: PSTATE.SP accessor
///
/// ```text
/// PSTATE.SP
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford field:
/// PSTATE.SP
#[inline]
pub fn pstate_sp(state: &CpuState) -> bool {
    state.pstate.sp
}

/// Stanford pseudocode: EDSCR (External Debug Status and Control Register)
///
/// ```text
/// EDSCR()
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// EDSCR
pub fn edscr(sys: &SystemRegisters) -> u64 {
    // EDSCR is at op0=2, op1=0, crn=0, crm=2, op2=0
    sys.read(SystemRegId::new(2, 0, 0, 2, 0))
}

/// Stanford pseudocode: Halted()
///
/// ```text
/// func Halted() => boolean
/// begin
///     return EDSCR.HDE == '1' && EDSCR.ERR == '1';
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// Halted
pub fn halted(sys: &SystemRegisters) -> bool {
    let edscr_val = edscr(sys);
    // HDE = bit 14, ERR = bit 6
    (edscr_val & (1 << 14)) != 0 && (edscr_val & (1 << 6)) != 0
}

/// Stanford pseudocode: PSTATE.SSBS accessor
///
/// ```text
/// PSTATE.SSBS
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford field:
/// PSTATE.SSBS
#[inline]
pub fn pstate_ssbs(state: &CpuState) -> bool {
    state.pstate.ssbs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pstate_new() {
        let pstate = PState::new_aarch64(ExceptionLevel::EL1);
        assert_eq!(pstate.el, ExceptionLevel::EL1);
        assert!(pstate.aarch64);
        assert_eq!(pstate.nzcv(), 0);
    }

    #[test]
    fn test_pstate_condition_holds() {
        let mut pstate = PState::new_aarch64(ExceptionLevel::EL1);
        pstate.set_nzcv(0b0100); // Z=1
        assert!(pstate.condition_holds(Condition::EQ));
        assert!(!pstate.condition_holds(Condition::NE));
    }

    #[test]
    fn test_gpr_read_write() {
        let mut gpr = GeneralRegisters::new();
        gpr.write_x(0, 42);
        assert_eq!(gpr.read_x(0), 42);
        assert_eq!(gpr.read_x(31), 0); // XZR

        gpr.write_w(1, 0x1234_5678);
        assert_eq!(gpr.read_w(1), 0x1234_5678);
        assert_eq!(gpr.read_x(1), 0x1234_5678);
    }

    #[test]
    fn test_gpr_sp_pc() {
        let mut gpr = GeneralRegisters::new();
        gpr.write_sp(0x1000);
        assert_eq!(gpr.read_sp(), 0x1000);
        gpr.write_pc(0x2000);
        assert_eq!(gpr.read_pc(), 0x2000);
    }

    #[test]
    fn test_simd_registers() {
        let mut simd = SimdRegisters::new();
        simd.write_v(0, 0x1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0);
        assert_eq!(simd.read_v(0), 0x1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0);
    }

    #[test]
    fn test_system_registers() {
        let mut sys = SystemRegisters::new();
        sys.write(SystemRegId::new(3, 0, 1, 0, 0), 0x1234); // SCTLR_EL1
        assert_eq!(sys.read(SystemRegId::new(3, 0, 1, 0, 0)), 0x1234);
    }

    #[test]
    fn test_cpu_state() {
        let cpu = CpuState::new();
        assert_eq!(pstate_el(&cpu), ExceptionLevel::EL1);
        assert!(!pstate_n(&cpu));
        assert!(!pstate_z(&cpu));
        assert!(!pstate_c(&cpu));
        assert!(!pstate_v(&cpu));
    }

    #[test]
    fn test_cpu_state_with_el() {
        let cpu = CpuState::new().with_el(ExceptionLevel::EL3);
        assert_eq!(pstate_el(&cpu), ExceptionLevel::EL3);
    }

    #[test]
    fn test_edscr_halted() {
        let mut sys = SystemRegisters::new();
        // Set HDE (bit 14) and ERR (bit 6)
        sys.write(SystemRegId::new(2, 0, 0, 2, 0), (1 << 14) | (1 << 6));
        assert!(halted(&sys));

        sys.write(SystemRegId::new(2, 0, 0, 2, 0), 0);
        assert!(!halted(&sys));
    }
}