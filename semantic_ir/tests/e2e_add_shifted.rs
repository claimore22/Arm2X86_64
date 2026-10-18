use semantic_ir::{
    validate_instruction, BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    ValueType, Width,
};
use semantic_ir::codegen::lower_to_x86_64;
use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;

/// Decode ADD X0, X1, X2, LSL #0 → IR → x86-64 machine code → native execution.
/// Cross-check: IR interpreter result must equal native x86-64 execution result.
#[test]
fn add_shifted_end_to_end() {
    // ADD X0, X1, X2, LSL #0 → encoding 0x92020020
    let bits: u32 = 0x92020020;
    let decoded = decode_instruction(bits).expect("ADD X0, X1, X2, LSL #0 should decode");

    // Verify decode results
    assert_eq!(decoded.mnemonic, "ADD");
    assert_eq!(decoded.rd, Reg::X(0));
    assert_eq!(decoded.rn, Reg::X(1));
    assert_eq!(decoded.rm, Reg::X(2));
    assert_eq!(decoded.shift, semantic_ir::primitives::ShiftKind::Lsl);
    assert_eq!(decoded.shift_amount, 0);
    assert_eq!(decoded.width, BitWidth::W64);

    // Lower to IR
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).expect("ADD IR should validate");

    // --- Execute via IR interpreter ---
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, 10);
    interp.state.gpr.write_x(2, 20);
    interp.execute_instruction(&instruction).expect("ADD should execute via IR");
    let ir_result = interp.state.gpr.read_x(0) as u64;
    assert_eq!(ir_result, 30, "IR interpreter: X0 should be X1 + X2 = 10 + 20 = 30");

    // --- Lower to x86-64 machine code ---
    let bytes = lower_to_x86_64(&decoded).expect("ADD should lower to x86-64");

    // Expected bytes: 48 89 C8 (mov rax, rcx) + 48 01 D8 (add rax, rdx) + C3 (ret)
    assert_eq!(
        bytes,
        vec![0x48, 0x89, 0xC8, 0x48, 0x01, 0xD8, 0xC3],
        "x86-64 machine code must match expected encoding"
    );

    // --- Execute natively on x86-64 ---
    let native_result = unsafe {
        let mem = win32::alloc_executable(bytes.len());
        let mem = mem.expect("VirtualAlloc failed for executable memory");

        // Copy machine code into executable memory
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), mem as *mut u8, bytes.len());

        // Make memory executable
        let rc = win32::make_executable(mem, bytes.len());
        assert_eq!(rc, 0, "VirtualProtect to PAGE_EXECUTE_READ failed");

        // Cast to function pointer and call: u64 func(u64 x1, u64 x2) -> u64
        let func: extern "C" fn(u64, u64) -> u64 = unsafe { std::mem::transmute(mem) };

        let result = func(10, 20);

        // Clean up
        let _ = win32::free_mem(mem, bytes.len());

        result
    };

    // Cross-check: native execution must match IR interpreter
    assert_eq!(
        native_result, ir_result,
        "Native x86-64 execution ({}) must match IR interpreter ({})",
        native_result, ir_result
    );
    assert_eq!(
        native_result, 30,
        "Native x86-64: X0 should be X1 + X2 = 10 + 20 = 30"
    );
}

/// Windows-only: allocate executable memory via VirtualAlloc.
#[cfg(target_os = "windows")]
mod win32 {
    use std::ptr::null_mut;

    #[link(name = "kernel32")]
    unsafe extern "system" {
        pub fn VirtualAlloc(
            lpAddress: *mut std::ffi::c_void,
            dwSize: usize,
            flAllocationType: u32,
            flProtect: u32,
        ) -> *mut std::ffi::c_void;
        pub fn VirtualProtect(
            lpAddress: *mut std::ffi::c_void,
            dwSize: usize,
            flNewProtect: u32,
            lpflOldProtect: *mut u32,
        ) -> i32;
        pub fn VirtualFree(
            lpAddress: *mut std::ffi::c_void,
            dwSize: usize,
            dwFreeType: u32,
        ) -> i32;
    }

    pub const MEM_COMMIT: u32 = 0x1000;
    pub const MEM_RESERVE: u32 = 0x2000;
    pub const MEM_RELEASE: u32 = 0x8000;
    pub const PAGE_EXECUTE_READWRITE: u32 = 0x40;

    pub fn alloc_executable(size: usize) -> Option<*mut u8> {
        let mem = unsafe {
            VirtualAlloc(null_mut(), size, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE)
        };
        if mem.is_null() {
            None
        } else {
            Some(mem as *mut u8)
        }
    }

    pub fn make_executable(_mem: *mut u8, _size: usize) -> i32 {
        0 // Already executable from VirtualAlloc
    }
}

#[cfg(not(target_os = "windows"))]
mod win32 {
    pub fn alloc_executable(_size: usize) -> Option<*mut u8> {
        None
    }
    pub fn make_executable(_mem: *mut u8, _size: usize) -> i32 {
        -1
    }
    pub fn free_mem(_mem: *mut u8, _size: usize) -> i32 {
        -1
    }
}
