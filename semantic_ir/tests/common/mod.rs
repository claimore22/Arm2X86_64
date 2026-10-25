use semantic_ir::decode::{decode_instruction, lower_to_ir};
use semantic_ir::interpreter::Interpreter;
use semantic_ir::codegen::{lower_to_x86_64, lower_to_x86_64_with_flags};
use semantic_ir::{validate_instruction, BitWidth, Reg};
use semantic_ir::primitives::ShiftKind;
use semantic_ir::types::BitWidth as BW;

/// Extract ARM NZCV flags from x86 RFLAGS
/// x86 RFLAGS: bit 0=CF, bit 6=ZF, bit 7=SF, bit 11=OF
/// ARM NZCV: N=bit3, Z=bit2, C=bit1, V=bit0
pub fn extract_arm_flags_from_x86(x86_rflags: u64, is_sub: bool) -> u64 {
    let cf = (x86_rflags >> 0) & 1;
    let zf = (x86_rflags >> 6) & 1;
    let sf = (x86_rflags >> 7) & 1;
    let of = (x86_rflags >> 11) & 1;
    
    // ARM C flag: for SUB, it's NOT borrow (i.e., NOT x86 CF)
    let c = if is_sub { cf ^ 1 } else { cf };
    
    (sf << 3) | (zf << 2) | (c << 1) | of
}

pub fn aarch64_reference_add(xn: u64, xm: u64, amount: u32) -> u64 {
    xn.wrapping_add(xm.wrapping_shl(amount))
}
pub fn aarch64_reference_sub(xn: u64, xm: u64, amount: u32) -> u64 {
    xn.wrapping_sub(xm.wrapping_shl(amount))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithmeticOp {
    Add,
    Sub,
    And,
    Or,
    Xor,
    Ands,
    Mul,
    Sdiv,
    Udiv,
}
impl ArithmeticOp {
    pub fn mnemonic(self) -> &'static str {
        match self {
            ArithmeticOp::Add => "ADD",
            ArithmeticOp::Sub => "SUB",
            ArithmeticOp::And => "AND",
            ArithmeticOp::Or => "ORR",
            ArithmeticOp::Xor => "EOR",
            ArithmeticOp::Ands => "ANDS",
            ArithmeticOp::Mul => "MUL",
            ArithmeticOp::Sdiv => "SDIV",
            ArithmeticOp::Udiv => "UDIV",
        }
    }
    pub fn instruction_word(self) -> u32 {
        match self {
            ArithmeticOp::Add => 0x92020020,
            ArithmeticOp::Sub => 0xD2020020,
            ArithmeticOp::And => 0x8A020020,
            ArithmeticOp::Or => 0xAA020020,
            ArithmeticOp::Xor => 0xCA020020,
            ArithmeticOp::Ands => 0xEA020020,
            ArithmeticOp::Mul => 0x9B027C20,
            ArithmeticOp::Sdiv => 0x9AC20C20,
            ArithmeticOp::Udiv => 0x9AC20820,
        }
    }
    pub fn reference(self, xn: u64, xm: u64, amount: u32) -> u64 {
        let shifted = xm.wrapping_shl(amount);
        match self {
            ArithmeticOp::Add => xn.wrapping_add(shifted),
            ArithmeticOp::Sub => xn.wrapping_sub(shifted),
            ArithmeticOp::And | ArithmeticOp::Ands => xn & shifted,
            ArithmeticOp::Or => xn | shifted,
            ArithmeticOp::Xor => xn ^ shifted,
            ArithmeticOp::Mul => xn.wrapping_mul(xm),
            ArithmeticOp::Sdiv => {
                if xm == 0 { 0 } else { (xn as i64 / xm as i64) as u64 }
            }
            ArithmeticOp::Udiv => {
                if xm == 0 { 0 } else { xn / xm }
            }
        }
    }
}

#[cfg(target_os = "windows")]
pub mod win32 {
    use std::ptr::null_mut;
    #[link(name = "kernel32")]
    unsafe extern "system" {
        pub fn VirtualAlloc(lpAddress: *mut std::ffi::c_void, dwSize: usize, flAllocationType: u32, flProtect: u32) -> *mut std::ffi::c_void;
        pub fn VirtualProtect(lpAddress: *mut std::ffi::c_void, dwSize: usize, flNewProtect: u32, lpflOldProtect: *mut u32) -> i32;
        pub fn VirtualFree(lpAddress: *mut std::ffi::c_void, dwSize: usize, dwFreeType: u32) -> i32;
    }
    pub const MEM_COMMIT: u32 = 0x1000;
    pub const MEM_RESERVE: u32 = 0x2000;
    pub const MEM_RELEASE: u32 = 0x8000;
    pub const PAGE_READWRITE: u32 = 0x04;
    pub const PAGE_EXECUTE_READ: u32 = 0x20;
    pub fn alloc_executable(size: usize) -> Option<*mut u8> {
        let mem = unsafe { VirtualAlloc(null_mut(), size, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE) };
        if mem.is_null() { None } else { Some(mem as *mut u8) }
    }
    pub fn make_executable(mem: *mut u8, size: usize) -> i32 {
        let mut old: u32 = 0;
        unsafe { VirtualProtect(mem as *mut std::ffi::c_void, size, PAGE_EXECUTE_READ, &mut old) }
    }
    pub fn free_mem(mem: *mut u8, _size: usize) -> i32 {
        unsafe { VirtualFree(mem as *mut std::ffi::c_void, 0, MEM_RELEASE) }
    }
    pub fn run_native(bytes: &[u8], x1: u64, x2: u64) -> Result<u64, String> {
        let mem = alloc_executable(bytes.len()).ok_or("VirtualAlloc failed")?;
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), mem, bytes.len()); }
        let rc = make_executable(mem, bytes.len());
        if rc == 0 { let _ = free_mem(mem, bytes.len()); return Err(format!("VirtualProtect failed: {}", rc)); }
        let result: u64;
        unsafe {
            std::arch::asm!(
                "call {func}",
                func = in(reg) mem,
                in("rcx") x1,
                in("rdx") x2,
                lateout("rax") result,
                options(nostack, preserves_flags)
            );
        }
        let _ = free_mem(mem, bytes.len());
        Ok(result)
    }
    pub fn run_native_rax(bytes: &[u8], rax_val: u64) -> Result<u64, String> {
        // prepend movabs rax, rax_val
        let mut buf = Vec::with_capacity(bytes.len() + 10);
        buf.extend_from_slice(&[0x48, 0xB8]);
        buf.extend_from_slice(&rax_val.to_le_bytes());
        buf.extend_from_slice(bytes);
        let mem = alloc_executable(buf.len()).ok_or("VirtualAlloc failed")?;
        unsafe { std::ptr::copy_nonoverlapping(buf.as_ptr(), mem, buf.len()); }
        let rc = make_executable(mem, buf.len());
        if rc == 0 { let _ = free_mem(mem, buf.len()); return Err(format!("VirtualProtect failed: {}", rc)); }
        let func: extern "C" fn() -> u64 = unsafe { std::mem::transmute(mem) };
        let result = func();
        let _ = free_mem(mem, buf.len());
        Ok(result)
    }

    /// Run native code and return both result (RAX) and x86 flags (RDX)
    /// The generated code must save result in RBX, move flags to RDX, restore result to RAX
    /// carry_in is the ARM C flag (0/1), passed in R8. Generated code sets x86 CF accordingly:
    ///   ADC: CF = carry_in
    ///   SBC: CF = NOT carry_in (borrow-in)
    #[cfg(target_os = "windows")]
    pub fn run_native_with_flags(bytes: &[u8], x1: u64, x2: u64, carry_in: bool) -> Result<(u64, u64), String> {
        let mem = alloc_executable(bytes.len()).ok_or("VirtualAlloc failed")?;
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), mem, bytes.len()); }
        let rc = make_executable(mem, bytes.len());
        if rc == 0 { let _ = free_mem(mem, bytes.len()); return Err(format!("VirtualProtect failed: {}", rc)); }

        // carry_in is prepended to generated code as `mov r8, <carry_in>`.
        // ABI sets RCX=x1, RDX=x2, R8=carry_in per Windows x64 calling convention.
        // Generated code returns RAX=result and leaves x86 flags set.
        let func: extern "C" fn(u64, u64, u64) -> u64 = unsafe { std::mem::transmute(mem) };
        let result = unsafe { func(x1, x2, carry_in as u64) };
        // Capture x86 flags set by the just-executed generated code
        let flags: u64;
        unsafe {
            std::arch::asm!("pushfq", "pop {flags}", flags = out(reg) flags);
        }
        let _ = free_mem(mem, bytes.len());
        Ok((result, flags))
    }
    
    #[cfg(not(target_os = "windows"))]
    pub fn run_native_with_flags(_bytes: &[u8], _x1: u64, _x2: u64, _carry_in: bool) -> Result<(u64, u64), String> {
        Err("Not supported on non-Windows".to_string())
    }

    /// Run native code with pre-set RAX, return flags
    pub fn run_native_rax_with_flags(bytes: &[u8], rax_val: u64) -> Result<u64, String> {
        let mut buf = Vec::with_capacity(bytes.len() + 10);
        buf.extend_from_slice(&[0x48, 0xB8]);
        buf.extend_from_slice(&rax_val.to_le_bytes());
        buf.extend_from_slice(bytes);
        let mem = alloc_executable(buf.len()).ok_or("VirtualAlloc failed")?;
        unsafe { std::ptr::copy_nonoverlapping(buf.as_ptr(), mem, buf.len()); }
        let rc = make_executable(mem, buf.len());
        if rc == 0 { let _ = free_mem(mem, buf.len()); return Err(format!("VirtualProtect failed: {}", rc)); }
        let func: extern "C" fn() -> (u64, u64) = unsafe { std::mem::transmute(mem) };
        let (_, flags) = func();
        let _ = free_mem(mem, buf.len());
        Ok(flags)
    }
}
#[cfg(not(target_os = "windows"))]
pub mod win32 {
    pub fn run_native(_bytes: &[u8], _x1: u64, _x2: u64) -> Result<u64, String> {
        Err("Not supported on non-Windows".to_string())
    }
    pub fn run_native_rax(_bytes: &[u8], _rax_val: u64) -> Result<u64, String> {
        Err("Not supported on non-Windows".to_string())
    }
}

pub fn percentile(sorted: &[u64], p: f64) -> Option<u64> {
    if sorted.is_empty() { return None; }
    let rank = ((p / 100.0) * (sorted.len() as f64)).ceil() as usize;
    let idx = rank.saturating_sub(1).min(sorted.len() - 1);
    Some(sorted[idx])
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QemuStatus { Unavailable, Skipped, Passed, Mismatch, Failed }

#[derive(Debug, Clone)]
pub struct DifferentialOutcome {
    pub mnemonic: String,
    pub x1: u64,
    pub x2: u64,
    pub ir_result: u64,
    pub native_result: u64,
    pub semantic_oracle_result: u64,
    pub bytes: Vec<u8>,
    pub qemu_available: bool,
    pub qemu_invoked: bool,
    pub qemu_result: Option<u64>,
    pub qemu_status: QemuStatus,
    pub qemu_error: Option<String>,
}
#[derive(Debug, Clone)]
pub struct DifferentialMismatch {
    pub mnemonic: String,
    pub instruction_word: u32,
    pub x1: u64,
    pub x2: u64,
    pub ir_result: u64,
    pub native_result: u64,
    pub semantic_oracle_result: u64,
    pub failing_stage: &'static str,
}

pub fn run_differential_case_op(op: ArithmeticOp, x1: u64, x2: u64) -> Result<DifferentialOutcome, String> {
    let bits = op.instruction_word();
    let decoded = decode_instruction(bits).map_err(|e| format!("decode: {:?}", e))?;
    if decoded.mnemonic != op.mnemonic() {
        return Err(format!("mnemonic mismatch: expected {}, got {}", op.mnemonic(), decoded.mnemonic));
    }
    let instruction = lower_to_ir(&decoded);
    validate_instruction(&instruction).map_err(|e| format!("validate: {:?}", e))?;
    let mut interp = Interpreter::new();
    interp.state.gpr.write_x(1, x1);
    interp.state.gpr.write_x(2, x2);
    interp.execute_instruction(&instruction).map_err(|e| format!("ir exec: {:?}", e))?;
    let ir_result = interp.state.gpr.read_x(0) as u64;
    let bytes = lower_to_x86_64(&decoded).map_err(|e| format!("codegen: {:?}", e))?;
    let native_result = win32::run_native(&bytes, x1, x2)?;
    let semantic_oracle_result = op.reference(x1, x2, decoded.shift_amount);
    let qemu_available = wsl_qemu_available();
    let (qemu_invoked, qemu_result, qemu_status, qemu_error) = if qemu_available {
        match run_qemu_case(op, decoded.width, decoded.shift, decoded.shift_amount, x1, x2) {
            Ok(r) => (true, Some(r), QemuStatus::Passed, None),
            Err(e) => (true, None, QemuStatus::Failed, Some(e)),
        }
    } else {
        (false, None, QemuStatus::Unavailable, None)
    };
    Ok(DifferentialOutcome {
        mnemonic: op.mnemonic().to_string(),
        x1, x2, ir_result, native_result, semantic_oracle_result, bytes,
        qemu_available, qemu_invoked, qemu_result, qemu_status, qemu_error,
    })
}
pub fn run_differential_case(x1: u64, x2: u64) -> Result<DifferentialOutcome, String> {
    run_differential_case_op(ArithmeticOp::Add, x1, x2)
}
pub fn run_differential_suite_op(op: ArithmeticOp, cases: &[(u64,u64)]) -> Result<Vec<DifferentialMismatch>, String> {
    let mut mismatches = Vec::new();
    for &(x1,x2) in cases {
        let out = run_differential_case_op(op, x1, x2)?;
        let failing = if out.ir_result != out.semantic_oracle_result { "ir" }
            else if out.native_result != out.semantic_oracle_result { "native" }
            else if out.ir_result != out.native_result { "ir-vs-native" }
            else { "" };
        if !failing.is_empty() {
            mismatches.push(DifferentialMismatch {
                mnemonic: out.mnemonic.clone(),
                instruction_word: op.instruction_word(),
                x1, x2, ir_result: out.ir_result, native_result: out.native_result,
                semantic_oracle_result: out.semantic_oracle_result, failing_stage: failing,
            });
        }
    }
    Ok(mismatches)
}
pub fn run_differential_suite(cases: &[(u64,u64)]) -> Result<Vec<DifferentialMismatch>, String> {
    run_differential_suite_op(ArithmeticOp::Add, cases)
}

pub fn qemu_available() -> bool { wsl_qemu_available() }
pub fn wsl_qemu_available() -> bool {
    std::process::Command::new("wsl").args(["-d","Ubuntu-24.04","--","bash","-c","command -v qemu-aarch64"]).output().map(|o| o.status.success()).unwrap_or(false)
}
fn windows_to_wsl_path(path: &std::path::Path) -> String {
    let s = path.to_string_lossy();
    if s.len() >= 2 && s.as_bytes()[1] == b':' {
        let drive = s.chars().next().unwrap().to_ascii_lowercase();
        let rest = s[2..].replace('\\', "/");
        format!("/mnt/{}{}", drive, rest)
    } else {
        s.replace('\\', "/")
    }
}

pub fn generate_qemu_c(op: ArithmeticOp, width: BitWidth, shift: ShiftKind, amount: u32, x1: u64, x2: u64) -> String {
    use semantic_ir::types::BitWidth;
    let (ty, reg) = match width {
        BitWidth::W64 => ("unsigned long", "x"),
        BitWidth::W32 => ("unsigned int", "w"),
        _ => ("unsigned long", "x"),
    };
    let insn = match op {
        ArithmeticOp::Add => "add",
        ArithmeticOp::Sub => "sub",
        ArithmeticOp::And | ArithmeticOp::Ands => "and",
        ArithmeticOp::Or => "orr",
        ArithmeticOp::Xor => "eor",
        ArithmeticOp::Mul => "mul",
        ArithmeticOp::Sdiv => "sdiv",
        ArithmeticOp::Udiv => "udiv",
    };
    let (shift_str, shift_amount) = match op {
        ArithmeticOp::Mul | ArithmeticOp::Sdiv | ArithmeticOp::Udiv => ("", 0),
        _ => (match shift {
            ShiftKind::Lsl => "lsl",
            ShiftKind::Lsr => "lsr",
            ShiftKind::Asr => "asr",
            ShiftKind::Ror => "ror",
        }, amount),
    };
    let asm = if matches!(op, ArithmeticOp::Mul | ArithmeticOp::Sdiv | ArithmeticOp::Udiv) {
        format!("{} %{reg}0, %{reg}1, %{reg}2", insn, reg=reg)
    } else {
        format!("{} %{reg}0, %{reg}1, %{reg}2, {} #{}", insn, shift_str, shift_amount, reg=reg)
    };
    
    format!(
        r#"
typedef unsigned long u64;
typedef unsigned int u32;
#define X1 {x1}ULL
#define X2 {x2}ULL
static void sys_write(int fd, const void *buf, u64 n) {{
    register long x0 asm("x0") = fd;
    register const void *x1 asm("x1") = buf;
    register u64 x2 asm("x2") = n;
    register long x8 asm("x8") = 64;
    asm volatile("svc #0" : "+r"(x0) : "r"(x1), "r"(x2), "r"(x8) : "memory");
}}
static void sys_exit(int code) {{
    register long x0 asm("x0") = code;
    register long x8 asm("x8") = 93;
    asm volatile("svc #0" : : "r"(x0), "r"(x8));
    __builtin_unreachable();
}}
void _start(void) {{
    {ty} a = ({ty})X1;
    {ty} b = ({ty})X2;
    {ty} r;
    __asm__ volatile ("{asm}" : "=r"(r) : "r"(a), "r"(b));
    char tmp[32];
    int len = 0;
    u64 v = r;
    if (v == 0) tmp[len++] = '0';
    else while (v > 0) {{ tmp[len++] = '0' + (v % 10); v /= 10; }}
    char buf[32];
    for (int i = 0; i < len; i++) buf[i] = tmp[len - 1 - i];
    buf[len] = '\n';
    sys_write(1, buf, len + 1);
    sys_exit(0);
}}
"#,
        x1 = x1, x2 = x2, ty = ty, asm = asm,
    )
}

pub fn run_qemu_case(op: ArithmeticOp, width: BitWidth, shift: ShiftKind, amount: u32, x1: u64, x2: u64) -> Result<u64, String> {
    if !wsl_qemu_available() { return Err("WSL QEMU unavailable".to_string()); }
    let src = generate_qemu_c(op, width, shift, amount, x1, x2);
    let id = format!("{}_{}_{}", op.mnemonic(), x1, x2);
    let src_path = std::env::temp_dir().join(format!("arm2x86_qemu_{}.c", id));
    std::fs::write(&src_path, src).map_err(|e| format!("write src: {}", e))?;
    let wsl_src = windows_to_wsl_path(&src_path);
    let out_path = format!("/tmp/arm2x86_qemu_{}", id);
    let compile_cmd = format!("aarch64-linux-gnu-gcc -nostdlib -static -O0 -o {} {}", out_path, wsl_src);
    let comp = std::process::Command::new("wsl").args(["-d","Ubuntu-24.04","--","bash","-c",&compile_cmd]).output().map_err(|e| format!("wsl compile spawn: {}", e))?;
    if !comp.status.success() { return Err(format!("compile failed: {}", String::from_utf8_lossy(&comp.stderr))); }
    let run_cmd = format!("qemu-aarch64 {}", out_path);
    let run = std::process::Command::new("wsl").args(["-d","Ubuntu-24.04","--","bash","-c",&run_cmd]).output().map_err(|e| format!("wsl run spawn: {}", e))?;
    if !run.status.success() { return Err(format!("qemu run failed: {}", String::from_utf8_lossy(&run.stderr))); }
    let stdout = String::from_utf8_lossy(&run.stdout);
    stdout.trim().parse::<u64>().map_err(|e| format!("parse qemu output '{}': {}", stdout.trim(), e))
}

pub fn generate_qemu_move_wide_c(mnemonic: &str, width: BitWidth, imm: u32, shift: u32, old: u64) -> String {
    let (ty, reg, fmt) = match width {
        BitWidth::W64 => ("unsigned long", "x", "lu"),
        BitWidth::W32 => ("unsigned int", "w", "u"),
        _ => ("unsigned long", "x", "lu"),
    };
    let asm = match mnemonic {
        "MOVZ" => format!("movz %{reg}0, #{imm}, lsl #{shift}", reg=reg, imm=imm, shift=shift),
        "MOVN" => format!("movn %{reg}0, #{imm}, lsl #{shift}", reg=reg, imm=imm, shift=shift),
        "MOVK" => format!("movk %{reg}0, #{imm}, lsl #{shift}", reg=reg, imm=imm, shift=shift),
        _ => "movz %x0, #0".to_string(),
    };
    // For MOVK, seed r with old value via C assignment? Need real reg. Use C variable and inline asm movk on it.
    let seed = if mnemonic == "MOVK" {
        format!("{ty} r = ({ty}){old};\n    __asm__ volatile (\"{asm}\" : \"+r\"(r));", ty=ty, old=old, asm=asm)
    } else {
        format!("{ty} r;\n    __asm__ volatile (\"{asm}\" : \"=r\"(r));", ty=ty, asm=asm)
    };
    format!(
        r#"
#include <stdio.h>
int main(void) {{
    {seed}
    printf("%{fmt}\n", r);
    return 0;
}}
"#,
        seed = seed, fmt = fmt,
    )
}

pub fn run_qemu_move_wide(mnemonic: &str, width: BitWidth, imm: u32, shift: u32, old: u64) -> Result<u64, String> {
    if !wsl_qemu_available() { return Err("WSL QEMU unavailable".to_string()); }
    let src = generate_qemu_move_wide_c(mnemonic, width, imm, shift, old);
    let src_path = std::env::temp_dir().join("arm2x86_qemu_mw.c");
    std::fs::write(&src_path, src).map_err(|e| format!("write src: {}", e))?;
    let wsl_src = windows_to_wsl_path(&src_path);
    let out_path = "/tmp/arm2x86_qemu_mw";
    let compile_cmd = format!("aarch64-linux-gnu-gcc -static -O0 -o {} {}", out_path, wsl_src);
    let comp = std::process::Command::new("wsl").args(["-d","Ubuntu-24.04","--","bash","-c",&compile_cmd]).output().map_err(|e| format!("wsl compile spawn: {}", e))?;
    if !comp.status.success() { return Err(format!("compile failed: {}", String::from_utf8_lossy(&comp.stderr))); }
    let run_cmd = format!("qemu-aarch64 {}", out_path);
    let run = std::process::Command::new("wsl").args(["-d","Ubuntu-24.04","--","bash","-c",&run_cmd]).output().map_err(|e| format!("wsl run spawn: {}", e))?;
    if !run.status.success() { return Err(format!("qemu run failed: {}", String::from_utf8_lossy(&run.stderr))); }
    let stdout = String::from_utf8_lossy(&run.stdout);
    stdout.trim().parse::<u64>().map_err(|e| format!("parse qemu output '{}': {}", stdout.trim(), e))
}
