//! Stanford ARM64 Shared Pseudocode - Memory and System Functions
//!
//! Source:
//! https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
//!
//! Stanford functions: Memory access, address translation, cache operations, etc.

use crate::shared::enums::{AccessType, ExceptionLevel, FaultStatus, MemOp, PhysMemRetStatus, SecurityState, Shareability};
use crate::shared::registers::TCRConfig;
use crate::shared::bit_ops::align;

/// Physical address space identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PASpace {
    Secure,
    NonSecure,
    Realm,
    Root,
}

/// Physical address
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalAddress {
    pub address: u64,
    pub paspace: PASpace,
}

impl PhysicalAddress {
    pub fn new(address: u64, paspace: PASpace) -> Self {
        Self { address, paspace }
    }

    pub fn secure(address: u64) -> Self {
        Self::new(address, PASpace::Secure)
    }

    pub fn non_secure(address: u64) -> Self {
        Self::new(address, PASpace::NonSecure)
    }
}

/// Memory attributes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemoryAttributes {
    pub mem_type: MemType,
    pub device_type: Option<DeviceType>,
    pub inner_attrs: MemAttr,
    pub outer_attrs: MemAttr,
    pub inner_hints: MemHint,
    pub outer_hints: MemHint,
    pub shareability: Shareability,
    pub is_tagged: bool, // FEAT_MTE
}

impl Default for MemoryAttributes {
    fn default() -> Self {
        Self {
            mem_type: MemType::Normal,
            device_type: None,
            inner_attrs: MemAttr::NonCacheable,
            outer_attrs: MemAttr::NonCacheable,
            inner_hints: MemHint::None,
            outer_hints: MemHint::None,
            shareability: Shareability::NonShareable,
            is_tagged: false,
        }
    }
}

/// Memory type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemType {
    Normal,
    Device,
}

/// Device memory types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)] // memory attribute encoding names (nGnRnE, ...)
pub enum DeviceType {
    nGnRnE,
    nGnRE,
    nGRE,
    GRE,
}

/// Memory attributes (cacheability)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemAttr {
    NonCacheable,
    WriteThrough,
    WriteBack,
    WriteBackTransient, // FEAT_TWE
}

/// Memory hints (allocation policy)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemHint {
    None,
    Transient,
    WriteAllocate,
    ReadAllocate,
    ReadWriteAllocate,
}

/// Address descriptor - result of address translation
#[derive(Debug, Clone)]
pub struct AddressDescriptor {
    pub paddress: PhysicalAddress,
    pub memattrs: MemoryAttributes,
    pub fault: FaultRecord,
    pub is_external_abort: bool,
}

/// Fault record for memory faults
#[derive(Debug, Clone)]
pub struct FaultRecord {
    pub statuscode: FaultStatus,
    pub level: u8, // Translation level where fault occurred (0-3)
    pub vaddress: u64,
    pub ipaddress: Option<PhysicalAddress>,
    pub extflag: bool, // External abort flag
    pub secondstage: bool, // Stage 2 fault
    pub s2fs1walk: bool, // Stage 2 fault on stage 1 walk
    pub write: bool, // Write access
    pub accessdesc: AccessDescriptor,
}

/// Access descriptor for memory operations
#[derive(Debug, Clone)]
pub struct AccessDescriptor {
    pub acctype: AccessType,
    pub el: ExceptionLevel,
    pub ss: SecurityState,
    pub read: bool,
    pub write: bool,
    pub exclusive: bool,
    pub atomicop: bool,
    pub nontemporal: bool,
    pub pan: bool,
    pub nonfault: bool,
    pub firstfault: bool,
    pub contiguous: bool,
    pub predicated: bool,
    pub streamingsve: bool,
    pub ls64: bool,
    pub withstatus: bool,
    pub mops: bool,
    pub rcw: bool,
    pub rcws: bool,
    pub tagchecked: bool,
    pub tagaccess: bool,
    pub mpam: MPAMinfo,
}

impl Default for AccessDescriptor {
    fn default() -> Self {
        Self {
            acctype: AccessType::GPR,
            el: ExceptionLevel::EL1,
            ss: SecurityState::NonSecure,
            read: false,
            write: false,
            exclusive: false,
            atomicop: false,
            nontemporal: false,
            pan: false,
            nonfault: false,
            firstfault: false,
            contiguous: false,
            predicated: false,
            streamingsve: false,
            ls64: false,
            withstatus: false,
            mops: false,
            rcw: false,
            rcws: false,
            tagchecked: false,
            tagaccess: false,
            mpam: MPAMinfo::default(),
        }
    }
}

/// MPAM information (FEAT_MPAM)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MPAMinfo {
    pub partid: u16,
    pub pmg: u8,
}

/// Memory system model
#[derive(Debug, Clone)]
pub struct MemorySystem {
    // In a real implementation, this would contain the actual memory
    // For now, it's a placeholder
    pub ram: Vec<u8>,
    pub device_memory: Vec<u8>,
    // Exclusive monitor for LDXR/STXR
    pub exclusive_monitor: ExclusiveMonitor,
}

/// Exclusive monitor for load/store exclusive operations
#[derive(Debug, Clone, Default)]
pub struct ExclusiveMonitor {
    // Tracks the physical address and size of the last LDXR
    pub address: Option<u64>,
    pub size: Option<u64>,
    // Whether the monitor is active
    pub active: bool,
}

impl ExclusiveMonitor {
    /// Called on LDXR - marks the address as exclusively accessed
    pub fn mark_exclusive(&mut self, paddr: u64, size: u64) {
        self.address = Some(paddr);
        self.size = Some(size);
        self.active = true;
    }

    /// Called on STXR - checks if the store can succeed
    /// Returns true if the exclusive access succeeded
    pub fn check_exclusive(&mut self, paddr: u64, size: u64) -> bool {
        if self.active
            && self.address == Some(paddr)
            && self.size == Some(size) {
            self.active = false; // Clear monitor on successful store
            true
        } else {
            false
        }
    }

    /// Called on any memory write - clears the exclusive monitor
    pub fn clear(&mut self) {
        self.active = false;
        self.address = None;
        self.size = None;
    }

    /// Called on context switch or exception - clears the exclusive monitor
    pub fn clear_on_context_switch(&mut self) {
        self.active = false;
        self.address = None;
        self.size = None;
    }
}

impl MemorySystem {
    pub fn new() -> Self {
        Self {
            ram: vec![0; 1024 * 1024], // 1MB RAM for testing
            device_memory: vec![0; 64 * 1024], // 64KB device memory
            exclusive_monitor: ExclusiveMonitor::default(),
        }
    }

    /// Read memory at physical address
    pub fn read_phys(&self, paddr: u64, size: u64) -> Result<u64, FaultRecord> {
        let addr = paddr as usize;
        let size_bytes = size as usize;
        if addr + size_bytes <= self.ram.len() {
            let mut result = 0u64;
            for i in 0..size_bytes {
                result |= (self.ram[addr + i] as u64) << (i * 8);
            }
            Ok(result)
        } else {
            Err(FaultRecord {
                statuscode: FaultStatus::AddressSize,
                level: 0,
                vaddress: paddr,
                ipaddress: None,
                extflag: false,
                secondstage: false,
                s2fs1walk: false,
                write: false,
                accessdesc: AccessDescriptor::default(),
            })
        }
    }

    /// Write memory at physical address
    pub fn write_phys(&mut self, paddr: u64, size: u64, value: u64) -> Result<(), FaultRecord> {
        // Any write clears the exclusive monitor
        self.exclusive_monitor.clear();
        
        let addr = paddr as usize;
        let size_bytes = size as usize;
        if addr + size_bytes <= self.ram.len() {
            for i in 0..size_bytes {
                self.ram[addr + i] = (value >> (i * 8)) as u8;
            }
            Ok(())
        } else {
            Err(FaultRecord {
                statuscode: FaultStatus::AddressSize,
                level: 0,
                vaddress: paddr,
                ipaddress: None,
                extflag: false,
                secondstage: false,
                s2fs1walk: false,
                write: true,
                accessdesc: AccessDescriptor::default(),
            })
        }
    }

    /// Load exclusive (LDXR) - marks address for exclusive access
    pub fn load_exclusive(&mut self, paddr: u64, size: u64) -> Result<u64, FaultRecord> {
        let value = self.read_phys(paddr, size)?;
        self.exclusive_monitor.mark_exclusive(paddr, size);
        Ok(value)
    }

    /// Store exclusive (STXR) - attempts to store if exclusive access still valid
    pub fn store_exclusive(&mut self, paddr: u64, size: u64, value: u64) -> Result<bool, FaultRecord> {
        let success = self.exclusive_monitor.check_exclusive(paddr, size);
        if success {
            self.write_phys(paddr, size, value)?;
        }
        Ok(success)
    }

    /// Read memory at virtual address (with translation)
    pub fn read_virt(&self, vaddr: u64, size: u64, _accdesc: &AccessDescriptor) -> Result<u64, FaultRecord> {
        // Simplified: identity mapping for testing
        self.read_phys(vaddr, size)
    }

    /// Write memory at virtual address (with translation)
    pub fn write_virt(&mut self, vaddr: u64, size: u64, value: u64, _accdesc: &AccessDescriptor) -> Result<(), FaultRecord> {
        // Simplified: identity mapping for testing
        self.write_phys(vaddr, size, value)
    }
}

impl Default for MemorySystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Stanford pseudocode: NoFault
///
/// ```text
/// func NoFault(accdesc : AccessDescriptor, vaddress : bits(64)) => FaultRecord
/// begin
///     return FaultRecord{Fault_None, 0, vaddress, ...};
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// NoFault
pub fn no_fault(accdesc: AccessDescriptor, vaddress: u64) -> FaultRecord {
    FaultRecord {
        statuscode: FaultStatus::None,
        level: 0,
        vaddress,
        ipaddress: None,
        extflag: false,
        secondstage: false,
        s2fs1walk: false,
        write: accdesc.write,
        accessdesc: accdesc,
    }
}

/// Stanford pseudocode: IsFault
///
/// ```text
/// func IsFault(addrdesc : AddressDescriptor) => boolean
/// begin
///     return addrdesc.fault.statuscode != Fault_None;
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// IsFault
pub fn is_fault(addrdesc: &AddressDescriptor) -> bool {
    addrdesc.fault.statuscode != FaultStatus::None
}

/// Stanford pseudocode: IsExternalAbort
///
/// ```text
/// func IsExternalAbort(fault : FaultRecord) => boolean
/// begin
///     return fault.statuscode IN {Fault_SyncExternal, Fault_AsyncExternal};
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// IsExternalAbort
pub fn is_external_abort(fault: &FaultRecord) -> bool {
    matches!(
        fault.statuscode,
        FaultStatus::SyncExternal | FaultStatus::AsyncExternal
    )
}

/// Stanford pseudocode: IsExternalSyncAbort
pub fn is_external_sync_abort(fault: &FaultRecord) -> bool {
    fault.statuscode == FaultStatus::SyncExternal
}

/// Stanford pseudocode: IsAsyncAbort
pub fn is_async_abort(fault: &FaultRecord) -> bool {
    fault.statuscode == FaultStatus::AsyncExternal
}

/// Stanford pseudocode: IsDebugException
pub fn is_debug_exception(fault: &FaultRecord) -> bool {
    fault.statuscode == FaultStatus::Debug
}

/// Stanford pseudocode: CreateAccDescIFetch
pub fn create_acc_desc_i_fetch(el: ExceptionLevel, ss: SecurityState) -> AccessDescriptor {
    AccessDescriptor {
        acctype: AccessType::IFetch,
        el,
        ss,
        read: true,
        write: false,
        ..Default::default()
    }
}



/// Stanford pseudocode: CreateAccDescDC
pub fn create_acc_desc_dc(el: ExceptionLevel, ss: SecurityState) -> AccessDescriptor {
    AccessDescriptor {
        acctype: AccessType::DC,
        el,
        ss,
        read: true,
        write: true,
        ..Default::default()
    }
}

/// Stanford pseudocode: CreateAccDescAT
pub fn create_acc_desc_at(ss: SecurityState, el: ExceptionLevel, ataccess: ATAccess) -> AccessDescriptor {
    AccessDescriptor {
        acctype: AccessType::AT,
        el,
        ss,
        read: ataccess == ATAccess::Read,
        write: ataccess == ATAccess::Write,
        ..Default::default()
    }
}

/// AT access type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ATAccess {
    Read,
    Write,
}

/// Page table entry (PTE) for ARM64 translation tables
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_snake_case)] // is_nG mirrors the architecture's nG (not-global) bit name
pub enum PageTableEntry {
    /// Invalid/unmapped entry
    Invalid,
    /// Table descriptor (points to next level table)
    Table {
        next_level_addr: u64, // Physical address of next level table
    },
    /// Block descriptor (maps a large region)
    Block {
        output_address: u64,
        attrs: MemoryAttributes,
        permissions: PagePermissions,
        is_af: bool,         // Access Flag
        is_nG: bool,         // Not Global
        is_sh: Shareability, // Shareability
        is_ap: AccessPermissions,
        is_pxn: bool,        // Privileged Execute Never
        is_uxn: bool,        // Unprivileged Execute Never
    },
    /// Page descriptor (maps a small page)
    Page {
        output_address: u64,
        attrs: MemoryAttributes,
        permissions: PagePermissions,
        is_af: bool,
        is_nG: bool,
        is_sh: Shareability,
        is_ap: AccessPermissions,
        is_pxn: bool,
        is_uxn: bool,
        is_af_dirty: bool, // Dirty Bit Modifier (FEAT_DBM)
    },
}

/// Page permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PagePermissions {
    pub ap: AccessPermissions,  // Access Permissions (AP[2:0])
    pub pxn: bool,              // Privileged Execute Never
    pub uxn: bool,              // Unprivileged Execute Never
}

/// Access permissions (AP[2:0])
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)] // access-permission names follow the architecture (RW_EL1, ...)
pub enum AccessPermissions {
    /// No access
    None,
    /// Read/Write at EL0 and EL1
    RW_EL0_EL1,
    /// Read/Write at EL1, no access at EL0
    RW_EL1,
    /// Read-only at EL0 and EL1
    RO_EL0_EL1,
    /// Read-only at EL1, no access at EL0
    RO_EL1,
}

/// Stanford pseudocode: AArch64_BranchAddr
///
/// ```text
/// func AArch64_BranchAddr(vaddress : bits(64), el : bits(2)) => bits(64)
/// begin
///     assert !UsingAArch32();
///     let msbit : integer = AddrTop(vaddress, TRUE, el);
///     if msbit == 63 then
///         return vaddress;
///     elsif (el IN {EL0, EL1} || IsInHost()) && vaddress[msbit] == '1' then
///         return SignExtend{64}(vaddress[msbit:0]);
///     else
///         return ZeroExtend{64}(vaddress[msbit:0]);
///     end;
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// AArch64_BranchAddr
pub fn aarch64_branch_addr(vaddress: u64, _el: ExceptionLevel, tbi: bool) -> u64 {
    if !tbi {
        return vaddress;
    }

    // TBI (Top Byte Ignore) is enabled
    // Top 8 bits are ignored for address translation
    // But for branch address, we need to sign/zero extend based on bit 55
    let msbit = 55; // TBI uses 56-bit addresses (bits 55:0)

    // Extract the 56-bit address (bits 55:0)
    let addr56 = vaddress & 0x00FF_FFFF_FFFF_FFFF;

    if addr56 & (1 << msbit) != 0 {
        // Sign extend: set top 8 bits to 1
        addr56 | 0xFF00_0000_0000_0000
    } else {
        // Zero extend: top 8 bits already 0
        addr56
    }
}

/// Stanford pseudocode: AddrTop
///
/// ```text
/// func AddrTop(address : bits(64), IsInstr : boolean, el : bits(2)) => AddressSize
/// begin
///     assert HaveEL(el);
///     let regime : bits(2) = S1TranslationRegime(el);
///     if ELUsingAArch32(regime) then
///         return 31;
///     else
///         if EffectiveTBI(address, IsInstr, el) == '1' then
///             return 55;
///         else
///             return 63;
///         end;
///     end;
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// AddrTop
pub fn addr_top(_address: u64, _is_instr: bool, el: ExceptionLevel, tcr_tbi: bool) -> u8 {
    if el_using_aarch32(el) {
        31
    } else if tcr_tbi {
        55
    } else {
        63
    }
}

/// Helper for ELUsingAArch32 (from features)
fn el_using_aarch32(_el: ExceptionLevel) -> bool {
    false // AArch64 only
}

/// Stanford pseudocode: EffectiveTBI
pub fn effective_tbi(_address: u64, is_instr: bool, el: ExceptionLevel, tcr_tbi0: bool, tcr_tbi1: bool) -> bool {
    if el == ExceptionLevel::EL0 {
        // EL0 uses TCR_EL1 or TCR_EL2 depending on HCR_EL2.E2H
        // Simplified: use TBI0 for data, TBI1 for instructions
        if is_instr {
            tcr_tbi1
        } else {
            tcr_tbi0
        }
    } else {
        // EL1, EL2, EL3 use their own TCR
        // Simplified
        false
    }
}

/// Stanford pseudocode: Align
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
pub fn align_address(address: u64, alignment: u64) -> u64 {
    align(address, alignment)
}

/// Stanford pseudocode: BigEndian / BigEndianReverse
///
/// ```text
/// func BigEndian(x : bits(N)) => bits(N)
/// begin
///     return Reverse(x, 8);
/// end;
///
/// func BigEndianReverse(x : bits(N)) => bits(N)
/// begin
///     return Reverse(x, 8);
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford functions:
/// BigEndian, BigEndianReverse
pub fn big_endian_reverse<T: crate::shared::bit_ops::BitVector>(x: T) -> T {
    crate::shared::bit_ops::reverse(x, 8)
}

/// Stanford pseudocode: Endianness-dependent memory access
pub fn mem_read_with_endian<T: crate::shared::bit_ops::BitVector>(
    mem: &MemorySystem,
    address: u64,
    endian: crate::shared::enums::Endianness,
) -> Result<T, FaultRecord>
where
    T: From<u128>,
{
    let size = T::width() / 8;
    let value = mem.read_phys(address, size as u64)?;
    let result = if endian == crate::shared::enums::Endianness::Big {
        // Byte reverse the value
        let mut reversed = 0u128;
        for i in 0..size {
            let byte = (value >> (i * 8)) & 0xFF;
            let shift = ((size - 1 - i) * 8) as u32;
            reversed |= (byte as u128) << shift;
        }
        T::from_uint(reversed)
    } else {
        T::from_uint(value as u128)
    };
    Ok(result)
}

pub fn mem_write_with_endian<T: crate::shared::bit_ops::BitVector>(
    mem: &mut MemorySystem,
    address: u64,
    value: T,
    endian: crate::shared::enums::Endianness,
) -> Result<(), FaultRecord>
where
    T: Into<u128>,
{
    let size = T::width() / 8;
    let val: u128 = value.into();
    let bytes = if endian == crate::shared::enums::Endianness::Big {
        let mut reversed = 0u128;
        for i in 0..size {
            let byte = (val >> (i * 8)) & 0xFF;
            let shift = ((size - 1 - i) * 8) as u32;
            reversed |= (byte as u128) << shift;
        }
        reversed
    } else {
        val
    };
    mem.write_phys(address, size as u64, bytes as u64)
}

/// Stanford pseudocode: ClearTag (FEAT_MTE)
pub fn clear_tag<T: crate::shared::bit_ops::BitVector>(address: T) -> T
where
    T: From<u128>,
{
    // Clear the top 4 bits (allocation tag)
    let width = T::width();
    let mask = (!0u128) >> (128 - width + 4) << 4;
    T::from_uint(address.to_uint() & mask)
}



/// Stanford pseudocode: AArch64_MemSingleRead
///
/// Single-copy atomic load with address translation and access checking.
///
/// ```text
/// func AArch64_MemSingleRead{size}(address : bits(64), accdesc : AccessDescriptor, aligned : boolean)
///     => (bits(size), AddressDescriptor, PhysMemRetStatus)
/// begin
///     assert IsAlignedSize(size);
///     let aligned_address = if aligned then Align(address, size/8) else address;
///     let addrdesc = AArch64_TranslateAddress(aligned_address, accdesc, aligned);
///     if IsFault(addrdesc) then
///         AArch64_Abort(addrdesc.fault);
///     if !IsAlignedSize(size) then
///         AArch64_AlignmentFault(accdesc);
///     let (status, data) = PhysMemRead(addrdesc, accdesc);
///     if status != PhysMemRetStatus_Success then
///         HandleMemoryFault(status, addrdesc, accdesc);
///     return (data, addrdesc, status);
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// AArch64_MemSingleRead
pub fn aarch64_mem_single_read(
    mem: &MemorySystem,
    address: u64,
    size: u64,
    accdesc: &AccessDescriptor,
    aligned: bool,
    tcr_config: &TCRConfig,
    ttbr0: u64,
    ttbr1: u64,
) -> Result<(u64, AddressDescriptor, PhysMemRetStatus), FaultRecord> {
    // Check alignment
    let aligned_address = if aligned {
        align_address(address, size)
    } else {
        address
    };

    // Translate address
    let addrdesc = aarch64_translate_address(
        mem,
        aligned_address,
        accdesc,
        aligned,
        size,
        tcr_config,
        ttbr0,
        ttbr1,
    )?;

    // Check for faults
    if is_fault(&addrdesc) {
        return Err(addrdesc.fault);
    }

    // Perform physical memory read
    mem.read_phys(address, size).map(|data| (data, addrdesc, PhysMemRetStatus::Success))
}

/// Stanford pseudocode: AArch64_MemSingleWrite
///
/// Single-copy atomic store with address translation and access checking.
///
/// ```text
/// func AArch64_MemSingleWrite{size}(address : bits(64), accdesc : AccessDescriptor, aligned : boolean, value : bits(size))
///     => (AddressDescriptor, PhysMemRetStatus)
/// begin
///     assert IsAlignedSize(size);
///     let aligned_address = if aligned then Align(address, size/8) else address;
///     let addrdesc = AArch64_TranslateAddress(aligned_address, accdesc, aligned);
///     if IsFault(addrdesc) then
///         AArch64_Abort(addrdesc.fault);
///     if !IsAlignedSize(size) then
///         AArch64_AlignmentFault(accdesc);
///     let status = PhysMemWrite(addrdesc, accdesc, value);
///     if status != PhysMemRetStatus_Success then
///         HandleMemoryFault(status, addrdesc, accdesc);
///     return (addrdesc, status);
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// AArch64_MemSingleWrite
pub fn aarch64_mem_single_write(
    mem: &mut MemorySystem,
    address: u64,
    size: u64,
    accdesc: &AccessDescriptor,
    aligned: bool,
    value: u64,
    tcr_config: &TCRConfig,
    ttbr0: u64,
    ttbr1: u64,
) -> Result<(AddressDescriptor, PhysMemRetStatus), FaultRecord> {
    // Check alignment
    let aligned_address = if aligned {
        align_address(address, size)
    } else {
        address
    };

    // Translate address
    let addrdesc = aarch64_translate_address(
        mem,
        aligned_address,
        accdesc,
        aligned,
        size,
        tcr_config,
        ttbr0,
        ttbr1,
    )?;

    // Check for faults
    if is_fault(&addrdesc) {
        return Err(addrdesc.fault);
    }

    // Perform physical memory write
    mem.write_phys(address, size, value).map(|_| (addrdesc, PhysMemRetStatus::Success))
}

/// Stanford pseudocode: MemSingleGranule
///
/// Returns the granule size for single-copy atomic accesses.
/// For AArch64, this is typically 64 bytes (IMPDEF).
///
/// ```text
/// func MemSingleGranule() => integer
/// begin
///     return 64; // IMPDEF, typically 64 bytes
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// MemSingleGranule
#[inline]
pub fn mem_single_granule() -> u64 {
    // IMPDEF: typically 64 bytes for AArch64
    64
}

/// Stanford pseudocode: AArch64_TranslateAddress (simplified)
///
/// ```text
/// func AArch64_TranslateAddress(vaddress : bits(64), accdesc : AccessDescriptor, aligned : boolean)
///     => AddressDescriptor
/// begin
///     // Full translation table walk
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford pseudocode: CheckSPAlignment
///
/// Checks stack pointer alignment and raises exception if misaligned.
///
/// ```text
/// func CheckSPAlignment()
/// begin
///     let sp : bits(64) = SP();
///     let stack_align_check : boolean;
///     if PSTATE.EL == EL0 then
///         stack_align_check = (sp[3:0] != '0000' && SCTLR[0] == '1') || (sp[4:0] != '00000' && SCTLR[1] == '1');
///     else
///         stack_align_check = sp[3:0] != '0000';
///     end;
///     if stack_align_check then
///         AArch64_SPAlignmentFault();
///     end;
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// CheckSPAlignment
pub fn check_sp_alignment(
    sp: u64,
    el: ExceptionLevel,
    sctlr_el: u64,
) -> Result<(), FaultRecord> {
    let stack_align_check = if el == ExceptionLevel::EL0 {
        // EL0: check 16-byte alignment if SCTLR[0] == 1, or 32-byte if SCTLR[1] == 1
        (sp & 0xF != 0 && (sctlr_el & 1) != 0) || (sp & 0x1F != 0 && (sctlr_el & 2) != 0)
    } else {
        // Other ELs: check 16-byte alignment
        sp & 0xF != 0
    };

    if stack_align_check {
        Err(FaultRecord {
            statuscode: FaultStatus::Alignment,
            level: 0,
            vaddress: sp,
            ipaddress: None,
            extflag: false,
            secondstage: false,
            s2fs1walk: false,
            write: false,
            accessdesc: AccessDescriptor::default(),
        })
    } else {
        Ok(())
    }
}

/// Stanford pseudocode: CreateAccDescGPR
///
/// Creates an access descriptor for general-purpose register memory operations.
///
/// ```text
/// func CreateAccDescGPR(memop : MemOp, nontemporal : boolean, privileged : boolean, tagchecked : boolean)
///     => AccessDescriptor
/// begin
///     let accdesc : AccessDescriptor = NewAccDesc();
///     accdesc.acctype = AccessType_GPR;
///     accdesc.nontemporal = nontemporal;
///     accdesc.privileged = privileged;
///     accdesc.tagchecked = tagchecked;
///     if memop == MemOp_LOAD then
///         accdesc.read = TRUE;
///     else
///         accdesc.write = TRUE;
///     end;
///     return accdesc;
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// CreateAccDescGPR
pub fn create_acc_desc_gpr(
    memop: MemOp,
    nontemporal: bool,
    privileged: bool,
    tagchecked: bool,
    el: ExceptionLevel,
    ss: SecurityState,
) -> AccessDescriptor {
    let mut accdesc = AccessDescriptor {
        acctype: AccessType::GPR,
        el,
        ss,
        nontemporal,
        pan: privileged, // PAN bit in PSTATE
        tagchecked,
        ..Default::default()
    };

    match memop {
        MemOp::LOAD => {
            accdesc.read = true;
        }
        MemOp::STORE => {
            accdesc.write = true;
        }
        MemOp::PREFETCH => {
            accdesc.read = true; // Prefetch is a read-like operation
        }
        MemOp::ATOMIC => {
            accdesc.read = true;
            accdesc.write = true;
            accdesc.exclusive = true;
            accdesc.atomicop = true;
        }
    }

    accdesc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_system_read_write() {
        let mut mem = MemorySystem::new();
        mem.write_phys(0x1000, 8, 0x1234_5678_9ABC_DEFu64).unwrap();
        let value = mem.read_phys(0x1000, 8).unwrap();
        assert_eq!(value, 0x1234_5678_9ABC_DEFu64);
    }

    #[test]
    fn test_no_fault() {
        let accdesc = AccessDescriptor::default();
        let fault = no_fault(accdesc, 0x1000);
        assert_eq!(fault.statuscode, FaultStatus::None);
        assert_eq!(fault.vaddress, 0x1000);
    }

    #[test]
    fn test_is_fault() {
        let mut fault = FaultRecord {
            statuscode: FaultStatus::None,
            level: 0,
            vaddress: 0,
            ipaddress: None,
            extflag: false,
            secondstage: false,
            s2fs1walk: false,
            write: false,
            accessdesc: AccessDescriptor::default(),
        };
        let addrdesc = AddressDescriptor {
            paddress: PhysicalAddress::non_secure(0),
            memattrs: MemoryAttributes::default(),
            fault: fault.clone(),
            is_external_abort: false,
        };
        assert!(!is_fault(&addrdesc));

        fault.statuscode = FaultStatus::Translation;
        let addrdesc = AddressDescriptor {
            paddress: PhysicalAddress::non_secure(0),
            memattrs: MemoryAttributes::default(),
            fault,
            is_external_abort: false,
        };
        assert!(is_fault(&addrdesc));
    }

    #[test]
    fn test_is_external_abort() {
        let mut fault = FaultRecord {
            statuscode: FaultStatus::SyncExternal,
            level: 0,
            vaddress: 0,
            ipaddress: None,
            extflag: false,
            secondstage: false,
            s2fs1walk: false,
            write: false,
            accessdesc: AccessDescriptor::default(),
        };
        assert!(is_external_abort(&fault));

        fault.statuscode = FaultStatus::AsyncExternal;
        assert!(is_external_abort(&fault));

        fault.statuscode = FaultStatus::Translation;
        assert!(!is_external_abort(&fault));
    }

    #[test]
    fn test_aarch64_branch_addr() {
        // Without TBI
        assert_eq!(aarch64_branch_addr(0x1234_5678_9ABC_DEF0, ExceptionLevel::EL1, false), 0x1234_5678_9ABC_DEF0);

        // With TBI, positive address (bit 55 = 0)
        assert_eq!(aarch64_branch_addr(0x007F_1234_5678_9ABC, ExceptionLevel::EL1, true), 0x007F_1234_5678_9ABC);

        // With TBI, negative address (sign extend) - bit 55 = 1
        // 56-bit address: 0x80_1234_5678_9ABC, sign-extended to 64 bits = 0xFF80_1234_5678_9ABC
        let addr = 0xFF80_1234_5678_9ABCu64;
        let result = aarch64_branch_addr(addr, ExceptionLevel::EL1, true);
        assert_eq!(result, 0xFF80_1234_5678_9ABC);
    }

    #[test]
    fn test_addr_top() {
        assert_eq!(addr_top(0, false, ExceptionLevel::EL1, false), 63);
        assert_eq!(addr_top(0, false, ExceptionLevel::EL1, true), 55);
    }

    #[test]
    fn test_align_address() {
        assert_eq!(align_address(0x1007, 8), 0x1000);
        assert_eq!(align_address(0x1000, 8), 0x1000);
        assert_eq!(align_address(0x1008, 8), 0x1008);
    }

    #[test]
    fn test_create_acc_desc() {
        let accdesc = create_acc_desc_i_fetch(ExceptionLevel::EL1, SecurityState::NonSecure);
        assert_eq!(accdesc.acctype, AccessType::IFetch);
        assert!(accdesc.read);
        assert!(!accdesc.write);

        let accdesc = create_acc_desc_gpr(MemOp::LOAD, false, false, false, ExceptionLevel::EL0, SecurityState::NonSecure);
        assert_eq!(accdesc.acctype, AccessType::GPR);
        assert!(accdesc.read);
        assert!(!accdesc.write);
    }

    #[test]
    fn test_exclusive_monitor() {
        let mut mem = MemorySystem::new();

        // Test LDXR marks address
        let val = mem.load_exclusive(0x1000, 8).unwrap();
        assert_eq!(val, 0);
        assert!(mem.exclusive_monitor.active);
        assert_eq!(mem.exclusive_monitor.address, Some(0x1000));
        assert_eq!(mem.exclusive_monitor.size, Some(8));

        // Test STXR succeeds on same address/size
        let success = mem.store_exclusive(0x1000, 8, 0x1234).unwrap();
        assert!(success);
        assert!(!mem.exclusive_monitor.active); // Monitor cleared after success

        // Verify value was stored
        let val = mem.read_phys(0x1000, 8).unwrap();
        assert_eq!(val, 0x1234);

        // Test STXR fails on different address
        mem.load_exclusive(0x1000, 8).unwrap();
        let success = mem.store_exclusive(0x2000, 8, 0x5678).unwrap();
        assert!(!success);

        // Test STXR fails on different size
        mem.load_exclusive(0x1000, 8).unwrap();
        let success = mem.store_exclusive(0x1000, 4, 0x9ABC).unwrap();
        assert!(!success);

        // Test write_phys clears monitor
        mem.load_exclusive(0x1000, 8).unwrap();
        mem.write_phys(0x1000, 8, 0xFFFF).unwrap();
        assert!(!mem.exclusive_monitor.active);

        // Test clear_on_context_switch
        mem.load_exclusive(0x1000, 8).unwrap();
        mem.exclusive_monitor.clear_on_context_switch();
        assert!(!mem.exclusive_monitor.active);
    }

    

}

// ============================================================
// Page Table Walk Implementation
// ============================================================

/// Page table walk result
#[derive(Debug, Clone)]
pub struct PageTableWalkResult {
    pub pte: PageTableEntry,
    pub level: u8,
    pub table_paddr: u64,
}

/// Granule size configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GranuleSize {
    Kb4 = 4096,
    Kb16 = 16384,
    Kb64 = 65536,
}

impl GranuleSize {
    pub fn from_tg(tg: u8) -> Self {
        match tg {
            0 => GranuleSize::Kb4,
            1 => GranuleSize::Kb64,
            2 => GranuleSize::Kb16,
            _ => GranuleSize::Kb4,
        }
    }

    pub fn size_bytes(&self) -> u64 {
        *self as u64
    }

    pub fn level_bits(&self, level: u8) -> u8 {
        match self {
            GranuleSize::Kb4 => match level {
                0 => 9, // Level 0: bits 47:39
                1 => 9, // Level 1: bits 38:30
                2 => 9, // Level 2: bits 29:21
                3 => 9, // Level 3: bits 20:12
                _ => 0,
            },
            GranuleSize::Kb16 => match level {
                0 => 11, // Level 0: bits 47:37
                1 => 11, // Level 1: bits 36:26
                2 => 11, // Level 2: bits 25:15
                3 => 11, // Level 3: bits 14:4
                _ => 0,
            },
            GranuleSize::Kb64 => match level {
                0 => 13, // Level 0: bits 47:35
                1 => 13, // Level 1: bits 34:22
                2 => 13, // Level 2: bits 21:9
                3 => 13, // Level 3: bits 8:0 (not used for 64KB)
                _ => 0,
            },
        }
    }

    pub fn max_levels(&self) -> u8 {
        match self {
            GranuleSize::Kb4 => 4,
            GranuleSize::Kb16 => 4,
            GranuleSize::Kb64 => 3,
        }
    }
}

/// Calculate the base address of the translation table from TTBR
fn ttbr_base_address(ttbr: u64, tg: u8) -> u64 {
    let granule = GranuleSize::from_tg(tg);
    let mask = !(granule.size_bytes() - 1);
    ttbr & mask
}

/// Extract the index for a given level from the virtual address
fn va_index(vaddress: u64, level: u8, granule: GranuleSize, t0sz: u8) -> u64 {
    let _va_bits = 64 - t0sz as u64;
    let level_bits = granule.level_bits(level);
    let shift = 12 + (3 - level as usize) * level_bits as usize;

    if shift >= 64 {
        return 0;
    }

    let mask = (1u64 << level_bits) - 1;
    (vaddress >> shift) & mask
}

/// Read a PTE from physical memory
fn read_pte(mem: &MemorySystem, paddr: u64) -> Result<u64, FaultRecord> {
    // PTEs are 8 bytes (64 bits)
    mem.read_phys(paddr, 8)
}

/// Decode a page table entry
fn decode_pte(pte_raw: u64, level: u8, max_level: u8) -> PageTableEntry {
    // Bit 0: Valid
    if (pte_raw & 1) == 0 {
        return PageTableEntry::Invalid;
    }

    // Bit 1: Table vs Block/Page (0 = Table, 1 = Block/Page)
    let is_table = (pte_raw & 2) == 0;

    if is_table && level < max_level - 1 {
        // Table descriptor - points to next level
        // Bits 47:12 contain the next table address
        let next_table_addr = (pte_raw & 0x0000_FFFF_FFFF_F000) << 12;
        return PageTableEntry::Table { next_level_addr: next_table_addr };
    }

    // Block or Page descriptor
    // Output address is in bits 47:12 for block, bits 47:0 for page (with offset)
    let output_addr = (pte_raw & 0x0000_FFFF_FFFF_F000) << 12;

    // Memory attributes from MAIR index (bits 5:2)
    let _attr_idx = ((pte_raw >> 2) & 0xF) as u8;

    // AP[2:1] bits 7:6 (bit 6 is AP[2], bit 7 is AP[1])
    let ap_bits = ((pte_raw >> 6) & 0x3) as u8;
    let ap = match ap_bits {
        0b00 => AccessPermissions::RW_EL0_EL1,
        0b01 => AccessPermissions::RW_EL1,
        0b10 => AccessPermissions::RO_EL0_EL1,
        _ => AccessPermissions::RO_EL1,
    };

    // NS bit (bit 5) - Non-secure
    let _is_ns = (pte_raw & (1 << 5)) != 0;

    // PXN bit (bit 53) - Privileged Execute Never
    let pxn = (pte_raw & (1 << 53)) != 0;

    // UXN bit (bit 54) - Unprivileged Execute Never
    let uxn = (pte_raw & (1 << 54)) != 0;

    // SH bits (bits 9:8) - Shareability
    let sh_bits = ((pte_raw >> 8) & 0x3) as u8;
    let shareability = match sh_bits {
        0 => Shareability::NonShareable,
        1 => Shareability::OuterShareable,
        2 => Shareability::InnerShareable,
        _ => Shareability::NonShareable,
    };

    // AF bit (bit 10) - Access Flag
    let af = (pte_raw & (1 << 10)) != 0;

    // nG bit (bit 11) - Not Global
    let ng = (pte_raw & (1 << 11)) != 0;

    // DBM bit (bit 51) - Dirty Bit Modifier (FEAT_DBM)
    let af_dirty = (pte_raw & (1 << 51)) != 0;

    let permissions = PagePermissions { ap, pxn, uxn };
    let attrs = MemoryAttributes::default(); // Would be derived from MAIR in full impl

    if is_table || level == max_level - 1 {
        // Page descriptor (level 3 for 4KB/16KB, level 2 for 64KB)
        PageTableEntry::Page {
            output_address: output_addr,
            attrs,
            permissions,
            is_af: af,
            is_nG: ng,
            is_sh: shareability,
            is_ap: ap,
            is_pxn: pxn,
            is_uxn: uxn,
            is_af_dirty: af_dirty,
        }
    } else {
        // Block descriptor (level 0, 1, 2 for 4KB; level 0, 1 for 16KB/64KB)
        PageTableEntry::Block {
            output_address: output_addr,
            attrs,
            permissions,
            is_af: af,
            is_nG: ng,
            is_sh: shareability,
            is_ap: ap,
            is_pxn: pxn,
            is_uxn: uxn,
        }
    }
}

/// Perform a single stage 1 page table walk
pub fn aarch64_page_table_walk(
    mem: &MemorySystem,
    vaddress: u64,
    accdesc: &AccessDescriptor,
    tcr_config: &TCRConfig,
    ttbr0: u64,
    ttbr1: u64,
) -> Result<AddressDescriptor, FaultRecord> {
    // Determine which TTBR to use based on the virtual address
    // For EL0/EL1: if bit 63-t0sz is 0, use TTBR0; else use TTBR1
    let use_ttbr1 = if tcr_config.t0sz == 0 {
        false // No TTBR0 region
    } else {
        let t0sz = tcr_config.t0sz as u64;
        if t0sz >= 64 {
            false
        } else {
            let msb = 63 - t0sz;
            (vaddress >> msb) & 1 == 1
        }
    };

    let (ttbr, tg, _sh, _orgn, _irgn) = if use_ttbr1 {
        (ttbr1, tcr_config.tg1, tcr_config.sh1, tcr_config.orgn1, tcr_config.irgn1)
    } else {
        (ttbr0, tcr_config.tg0, tcr_config.sh0, tcr_config.orgn0, tcr_config.irgn0)
    };

    let granule = GranuleSize::from_tg(tg);
    let max_level = granule.max_levels();
    let table_base = ttbr_base_address(ttbr, tg);

    let mut table_paddr = table_base;

    for level in 0..max_level {
        let index = va_index(vaddress, level, granule, tcr_config.t0sz);
        let pte_offset = index * 8; // Each PTE is 8 bytes
        let pte_paddr = table_paddr + pte_offset;

        let pte_raw = read_pte(mem, pte_paddr)?;
        let pte = decode_pte(pte_raw, level, max_level);

        match pte {
            PageTableEntry::Invalid => {
                return Err(FaultRecord {
                    statuscode: FaultStatus::Translation,
                    level,
                    vaddress,
                    ipaddress: Some(PhysicalAddress::non_secure(table_paddr)),
                    extflag: false,
                    secondstage: false,
                    s2fs1walk: false,
                    write: accdesc.write,
                    accessdesc: accdesc.clone(),
                });
            }
            PageTableEntry::Table { next_level_addr } => {
                table_paddr = next_level_addr;
            }
            PageTableEntry::Block { output_address, attrs, permissions, is_af: _, is_nG: _, is_sh: _, is_ap: _, is_pxn: _, is_uxn: _ } => {
                // Check access permissions
                if !check_permissions(&permissions, accdesc) {
                    return Err(FaultRecord {
                        statuscode: FaultStatus::Permission,
                        level,
                        vaddress,
                        ipaddress: Some(PhysicalAddress::non_secure(pte_paddr)),
                        extflag: false,
                        secondstage: false,
                        s2fs1walk: false,
                        write: accdesc.write,
                        accessdesc: accdesc.clone(),
                    });
                }

                // Calculate physical address
                let level_bits = granule.level_bits(level) as u64;
                let block_size = granule.size_bytes() << (level as u64 * level_bits);
                let page_offset = vaddress & (block_size - 1);
                let paddr = output_address | page_offset;

                return Ok(AddressDescriptor {
                    paddress: PhysicalAddress::non_secure(paddr),
                    memattrs: attrs,
                    fault: no_fault(accdesc.clone(), vaddress),
                    is_external_abort: false,
                });
            }
            PageTableEntry::Page { output_address, attrs, permissions, is_af: _, is_nG: _, is_sh: _, is_ap: _, is_pxn: _, is_uxn: _, is_af_dirty: _ } => {
                // Check access permissions
                if !check_permissions(&permissions, accdesc) {
                    return Err(FaultRecord {
                        statuscode: FaultStatus::Permission,
                        level,
                        vaddress,
                        ipaddress: Some(PhysicalAddress::non_secure(pte_paddr)),
                        extflag: false,
                        secondstage: false,
                        s2fs1walk: false,
                        write: accdesc.write,
                        accessdesc: accdesc.clone(),
                    });
                }

                // Page offset is always 12 bits for 4KB pages
                let page_offset = vaddress & 0xFFF;
                let paddr = output_address | page_offset;

                return Ok(AddressDescriptor {
                    paddress: PhysicalAddress::non_secure(paddr),
                    memattrs: attrs,
                    fault: no_fault(accdesc.clone(), vaddress),
                    is_external_abort: false,
                });
            }
        }
    }

    // Should not reach here if page tables are correctly structured
    Err(FaultRecord {
        statuscode: FaultStatus::Translation,
        level: max_level - 1,
        vaddress,
        ipaddress: Some(PhysicalAddress::non_secure(table_paddr)),
        extflag: false,
        secondstage: false,
        s2fs1walk: false,
        write: accdesc.write,
        accessdesc: accdesc.clone(),
    })
}

/// Check if the access is permitted by the page permissions
fn check_permissions(perms: &PagePermissions, accdesc: &AccessDescriptor) -> bool {
    match perms.ap {
        AccessPermissions::None => false,
        AccessPermissions::RW_EL0_EL1 => true,
        AccessPermissions::RW_EL1 => accdesc.el != ExceptionLevel::EL0,
        AccessPermissions::RO_EL0_EL1 => !accdesc.write,
        AccessPermissions::RO_EL1 => !accdesc.write && accdesc.el != ExceptionLevel::EL0,
    }
}

/// Stanford pseudocode: AArch64_TranslateAddress with full page table walk
///
/// ```text
/// func AArch64_TranslateAddress(vaddress : bits(64), accdesc : AccessDescriptor, aligned : boolean)
///     => AddressDescriptor
/// begin
///     // Full translation table walk
/// end;
/// ```
///
/// Source:
/// https://www.scs.stanford.edu/~zyedidia/arm64/shared_pseudocode.html
///
/// Stanford function:
/// AArch64_TranslateAddress
pub fn aarch64_translate_address(
    mem: &MemorySystem,
    vaddress: u64,
    accdesc: &AccessDescriptor,
    aligned: bool,
    access_size: u64,
    tcr_config: &TCRConfig,
    ttbr0: u64,
    ttbr1: u64,
) -> Result<AddressDescriptor, FaultRecord> {
    // Apply TBI (Top Byte Ignore) if configured
    let vaddress = if accdesc.el == ExceptionLevel::EL0 {
        if tcr_config.tbi0 && !accdesc.read { // TBI for data
            vaddress & 0x00FF_FFFF_FFFF_FFFF
        } else if tcr_config.tbi1 && accdesc.read { // TBI for instructions
            vaddress & 0x00FF_FFFF_FFFF_FFFF
        } else {
            vaddress
        }
    } else {
        vaddress
    };

    // Check alignment if required
    if aligned && (vaddress & (access_size - 1)) != 0 {
        return Err(FaultRecord {
            statuscode: FaultStatus::Alignment,
            level: 0,
            vaddress,
            ipaddress: None,
            extflag: false,
            secondstage: false,
            s2fs1walk: false,
            write: accdesc.write,
            accessdesc: accdesc.clone(),
        });
    }

    // If MMU is disabled, use identity mapping
    if tcr_config.t0sz == 0 && tcr_config.t1sz == 0 {
        // MMU effectively disabled - identity map
        return Ok(AddressDescriptor {
            paddress: PhysicalAddress::non_secure(vaddress),
            memattrs: MemoryAttributes::default(),
            fault: no_fault(accdesc.clone(), vaddress),
            is_external_abort: false,
        });
    }

    // Perform page table walk
    aarch64_page_table_walk(mem, vaddress, accdesc, tcr_config, ttbr0, ttbr1)
}

#[test]
    fn test_big_endian_reverse() {
        let x: u32 = 0x1234_5678;
        let result = big_endian_reverse(x);
        assert_eq!(result, 0x7856_3412);
    }