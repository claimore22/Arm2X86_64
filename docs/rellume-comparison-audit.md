# Rellume vs Arm2X86_64: Source-Level Technical Audit

## 1. Executive Summary

Rellume (https://github.com/aengelke/rellume) is a C/C++ machine-code lifter that translates x86-64 / AArch64 / RISC-V64 binaries into LLVM IR, which can then be compiled and executed on a host. It is not a dynamic binary translator itself but a lifting library; complete DBT systems built on it include Instrew (binary instrumentation) and BinOpt (performance specialization).

Arm2X86_64 is a Rust project (`semantic_ir` crate) that decodes AArch64 machine instructions into a typed semantic IR, interprets that IR on a `CpuState` model, and lowers it to native x8-64 machine code (hand-rolled emitter). It tracks per-instruction pipeline coverage (Decode → IR → Interp → QEMU differential → Native Codegen) in `CHECKLIST.md`.

Key structural differences:
- Rellume lifts directly to **LLVM IR** and relies on LLVM for register allocation, instruction selection, and host code generation. Arm2X86_64 emits **raw x86-64 bytes** with a custom register-to-x86 mapping limited to X0–X7.
- Rellume models a full **virtual CPU state** structure (GPRs, SIMD/FP registers, PSTATE flags, PC) as LLVM values within basic blocks. Arm2X86_64 interprets typed IR on a Rust `CpuState`, then generates standalone native functions with a fixed calling convention (RCX=x1, RDX=x2, RAX=result).
- Rellume is LGPLv2.1+; Arm2X86_64 is MIT.

This audit compares actual source from both repositories (no assumptions).

---

## 2. Repository and Build Configuration Verified

### Arm2X86_64 (local)

- Workspace root: `C:\Users\claimoar\Documents\Rust\Arm2X86_64`
- Three crates:
  - `semantic_ir/Cargo.toml` — edition 2024, license MIT, dependency `thiserror = "2.0.21"`. Contains `src/` (decoder, interpreter, codegen, validate, shared) and `tests/`.
  - `semantic_ir_import/Cargo.toml` — imports Stanford pseudocode → `semantic_ir`.
  - `supervisor/Cargo.toml` — edition 2021, binary `supervisor`.
- `semantic_ir/tests/common/mod.rs` contains the QEMU differential harness (WSL → `aarch64-linux-gnu-gcc` + `qemu-aarch64`) and the `run_native_with_flags` inline-asm helper.
- Build: `cargo build --tests` succeeds (warnings only). `cargo test` has a pre-existing linker failure in `vdiv_scalar` (unrelated to this audit).
- Git remote: `https://github.com/claimore22/Arm2X86_64.git`, pushed to `main`.

### Rellume (local clone at `C:\Users\claimoar\Documents\Rust\rellume`)

- Build system: **meson** (`meson.build`, `meson_options.txt`).
- Language: C/C++, depends on **LLVM** (headers/IRBuilder/Intrinsics seen throughout).
- Source layout:
  - `src/arch.h`, `src/basicblock.{h,cc}`, `src/callconv.{h,cc}`, `src/config.h`, `src/facet.{h,cc}`, `src/function.{h,cc}`, `src/function-info.h`, `src/instr.h`, `src/lifter-base.{h,cc}`, `src/lifter.cc`, `src/lldecoder.cc`, `src/regfile.{h,cc}`, `src/rellume.cc`
  - `src/a64/` (AArch64 lifter): `main.cc` (~62KB), `simd.cc` (~78KB), `lifter.h`, `lifter-private.h`
  - `src/x86-64/` (x86-64 lifter): `lifter.cc`, `lifter-flags.cc`, `lifter-gp.cc`, `lifter-operand.cc`, `lifter-sse.cc`
  - `src/rv64/` (RISC-V64 lifter)
- License: **LGPLv2.1+** (`LICENSE`).
- Tests: `tests/a64_data_proc.txt`, `tests/a64_branches_and_cond.txt`, `tests/a64_loads_stores.txt`, `tests/a64_scalar_fp.txt`, `tests/a64_simd.txt` — assembly snippets with expected register/flag results, executed via `tests/test_driver.cc` / `tests/test_parser.py`.
- Last commit: `62a32d4 tests: Remove leftovers of old LLVM versions`; recent commits mention LLVM 21/22/23 compatibility.

---

## 3. Source-Level Architecture Comparison

### 3.1 Lifting target

| Aspect | Rellume | Arm2X86_64 |
|--------|---------|------------|
| IR target | LLVM IR (SSA) | Typed Rust semantic IR (`Expr`/`Stmt` enums) |
| Host codegen | LLVM JIT / `llc` + native compiler | Hand-rolled x86-64 byte emitter (`src/codegen.rs`) |
| Register allocation | LLVM's | Hard-coded map `X0→RAX, X1→RCX, ..., X7→RDI` (`arm_reg_to_x86`) |
| Flags | PSTATE NZCV as separate `ArchReg` i1 values in `RegFile` | `Flag::N/Z/C/V` on `PState.flags` (`ConditionFlags`) |
| Memory | `RegFile` with load/store via LLVM `load`/`store`, `memset`, `prefetch` intrinsic | `MemorySystem` in `shared/system.rs`, interpreter `memory_read`/`memory_write` |

Rellume's `Lifter` (in `src/a64/main.cc`) extends `LifterBase` (`src/lifter-base.{h,cc}`) which holds an `llvm::IRBuilder`, a `RegFile*`, and `FunctionInfo`. Each lifted instruction appends LLVM IR to basic blocks. Control flow (`B`, `BCOND`, `CBZ`, `RET`, etc.) is modeled by setting the PC value and creating conditional branches on the `RegFile` PC.

Arm2X86_64's decoder (`src/decode/mod.rs` + per-family modules) produces `Instruction { semantics: Vec<Stmt> }`. The interpreter (`src/interpreter.rs`) executes `Stmt` on `CpuState { pstate, gpr, simd, memory }`. Codegen (`src/codegen.rs`) emits raw bytes for a small subset (ADD/SUB/AND/ORR/EOR/MUL/SDIV/UDIV/bitfield/move-wide/logical-imm/ADC/SBC/CCMP) with X0/X1/X2 hard-coded to RAX/RCX/RDX.

### 3.2 Control flow

Rellume represents basic blocks via `ArchBasicBlock` (`src/basicblock.{h,cc}`). Branches set `RegFile` PC and create LLVM conditional branches (`SetIPCond` → `SetPCCond`). Indirect branches (`BR`, `BLR`, `RET`) read the target from a GPR. `BL`/`BLR` write LR (X30) and can call an external host function via `call_function` config.

Arm2X86_64 has `Stmt::Branch`, `Stmt::BranchConditional` in `src/stmt.rs`, handled in `interpreter.rs` `execute_stmt` by setting `branch_taken` and returning `InterpreterError::BranchTaken`. Native codegen does **not** emit any branch instructions — branches are unsupported in `lower_to_x86_64` (only data-processing mnemonics are matched at `codegen.rs:833`).

---

## 4. Instruction-Family Coverage Matrix

Coverage below is based on actual source inspection. "Native execution" for Arm2X86_64 means the generated x86-64 bytes are executed via `run_native`/`run_native_with_flags` in tests. "Differential" means compared against QEMU via WSL.

| Family | Rellume decoder/lifter | Rellume impl | Rellume tests | Arm2X86_64 decoder | Arm2X86_64 IR | Arm2X86_64 interp | Arm2X86_64 native | Arm2X86_64 runtime | Arm2X86_64 differential | Conclusion |
|--------|----------------------|--------------|---------------|--------------------|---------------|-------------------|-------------------|--------------------|-------------------------|------------|
| ADD/SUB shifted reg | `src/a64/main.cc`: `A64_ADD_SHIFTED`, `A64_SUB_SHIFTED` via `LiftBinOp` (SHIFT kind, `set_flags`) | Full (add/sub + flags) | `tests/a64_data_proc.txt` lines 1-4, 10-13 | `src/decode/add_sub.rs` | `BinOp::Add/Sub` + `Shift` | `eval_binop` Add/Sub | `emit_add_shifted_register` / `emit_sub_shifted_register` | `tests/e2e_add_shifted.rs` | `e2e_qemu_differential` add/sub | Partially verified (native lacks flags; QEMU wired only for add/sub/and/mul) |
| ADD/SUB extended reg | `A64_ADD_EXT`, `A64_SUB_EXT` via `LiftBinOp` (EXT kind) | Full | `tests/a64_data_proc.txt` | `src/decode/extend.rs` (`ADDX`/`SUBX`) | `ExtendReg` + Add/Sub | `eval_extend_reg` | `emit_extend` | `tests/e2e_add_shifted.rs` | Not in QEMU harness | Partially verified |
| ADD/SUB imm | `A64_ADD_IMM`, `A64_SUB_IMM` via `LiftBinOp` (IMM kind) | Full | `tests/a64_data_proc.txt` lines 5, 14 | `src/decode/addsub_imm.rs` (`ADDI`/`SUBI`/`ADDSI`/`SUBSI`) | `Binary` Add/Sub with `Const` | `eval_binop` | `emit_addsub_imm` | `tests/e2e_addsub_imm.rs` | Not in QEMU harness | Partially verified |
| ADDS/SUBS, CMP/CMN | `A64_ADDS`/`A64_SUBS` with `set_flags`; `CMP`/`CMN` as `SUBS`/`ADDS` writing flags only | Full flag calc via `FlagCalcAdd`/`FlagCalcSub` | `tests/a64_data_proc.txt` lines 7-8, 16-26 | `src/decode/addsub_flags.rs` (`ADDS`/`SUBS`) | `AddWithCarry` + `WriteFlag` N/Z/C/V | `add_with_carry` + `TupleElement` | `emit_add_shifted_register_with_flags` / `..._sub` | `tests/e2e_addsub_flags.rs` | Not in QEMU harness | Partially verified |
| Logical shifted reg | `AND_SHIFTED`, `ORR_SHIFTED`, `EOR_SHIFTED`, `BIC`, `ORN`, `EON`, `MVN` via `LiftBinOp` | Full | `tests/a64_data_proc.txt` lines 62-75 | `src/decode/logical.rs` | `BinOp::And/Or/Xor` + `Unary::Not` | `eval_binop`/`eval_unary_op` | `emit_logical_shifted_register` | `tests/e2e_logical_shifted.rs` | `e2e_qemu_differential` and | Partially verified |
| Logical imm | `AND_IMM`, `ORR_IMM`, `EOR_IMM` via `LiftBinOp` (IMM kind) | Full | `tests/a64_data_proc.txt` lines 62-64 | `src/decode/logical_imm.rs` | `Binary` with decoded mask | `eval_binop` | `emit_logical_imm` | `tests/e2e_logical_imm.rs` | Not in QEMU harness | Partially verified |
| MOVZ/MOVN/MOVK | `A64_MOVZ`, `A64_MOVN`, `A64_MOVK` | Full | `tests/a64_data_proc.txt` lines 28-45 | `src/decode/move_wide.rs` | `Const` shifts / `and`/`or` | `eval_binop` | `emit_move_wide` | `tests/e2e_move_wide.rs` | Not in QEMU harness | Partially verified |
| Bitfield (SBFM/UBFM/BFM) | Aliases only: `ASR_IMM`, `LSL_IMM`, `LSR_IMM`, `ROR_IMM`, `SBFIZ`, `SBFX`, `UBFIZ`, `UBFX`, `BFC`, `BFI`, `BFXIL` | Full via `Shift`/`Extract`/`MoveField` | `tests/a64_data_proc.txt` | `src/decode/bitfield.rs` | `Shift` ROR + `And` masks | `eval_shift`/`eval_binop` | `emit_bitfield` | `tests/e2e_bitfield.rs` | Not in QEMU harness | Partially verified |
| Cond. select (CSEL/CSINC/CSINV/CSNEG) | `A64_CSEL` etc. via `CreateSelect(IsTrue(cond), ...)` | Full | `tests/a64_branches_and_cond.txt` | `src/decode/cond_select.rs` | `Expr::Select` on `ConditionHolds` | `eval_select`/`ConditionHolds` | Not in codegen | None | Not in QEMU harness | Decode+IR+interp only |
| ADC/SBC (+flags) | `A64_ADC`/`A64_SBC` via `GetFlag(CF)` + `FlagCalcAdd/Sub` | Full | `tests/a64_data_proc.txt` (need verify) | `src/decode/adc_sbc.rs` (`ADC`/`SBC`/`ADCS`/`SBCS`) | `AddWithCarry` + flags | `add_with_carry` | `emit_adc_sbc` / `emit_adc_sbc_with_flags` (carry_in via codegen imm) | `tests/e2e_adc_sbc.rs`, `e2e_adc_sbc_flags.rs` | Not in QEMU harness | Partially verified (native flags fixed for carry_in via codegen-time CF) |
| Multiply / MADD / MSUB | `A64_MUL`, `A64_MADD`, `A64_MSUB`, `A64_MNEG`, long/widening variants | Full | `tests/a64_data_proc.txt` | `src/decode/multiply.rs` | `BinOp::Mul` + Add/Sub | `eval_binop` | `emit_mul` | `tests/e2e_mul.rs` | `e2e_qemu_differential` mul | Partially verified |
| SDIV / UDIV | `A64_SDIV`, `A64_UDIV` with div-by-zero / INT_MIN guards | Full (guards) | `tests/a64_data_proc.txt` lines 77-80 | `src/decode/divide.rs` | `BinOp::UDiv`/`Div` | `eval_binop` | `emit_divide` | `tests/e2e_divide.rs` | Not in QEMU harness | Partially verified |
| CCMP / CCMN | `A64_CCMP_REG`/`A64_CCMP_IMM` via `LiftCCmp(lhs, rhs, cond, nzcv, ccmn)` | Full (conditional flag select) | `tests/a64_data_proc.txt` (need verify) | `src/decode/ccmp.rs` | `Match` on `ConditionHolds` + `AddWithCarry` flags | `Stmt::Match` + `add_with_carry` | `emit_ccmp_ccmn_with_flags` (simplified cmp, ignores cond/nzcv) | `tests/e2e_ccmp.rs`, `e2e_ccmp_ccmn_flags.rs` | Not in QEMU harness | Decode+IR+interp done; native simplified |
| ADR / ADRP | `A64_ADR` (`PCRel`), `A64_ADRP` (page-masked PCRel) | Full | `tests/a64_data_proc.txt` lines 47-60 | `src/decode/mod.rs` (fall-through to `add_sub`) | Not clearly modeled | N/A | Not in codegen | None | Not in QEMU harness | Unsupported in Arm2X86_64 |
| Direct/indirect branches | `A64_B`, `A64_BR`, `A64_BCOND`, `A64_CBZ/CBNZ`, `A64_TBZ/TBNZ`, `A64_BL/BLR`, `A64_RET` | Full (PC manipulation + cond branches) | `tests/a64_branches_and_cond.txt` | `src/decode/mod.rs` (not dispatched; only data-processing) | `Stmt::Branch`/`BranchConditional` exist in `stmt.rs` | `execute_stmt` handles `BranchTaken` | Not emitted (no branch opcodes in codegen) | None | Not in QEMU harness | Decode not implemented; interp has stmt support; native missing |
| Load/store addressing | `A64_LDR`/`A64_STR`/`A64_LDP`/`A64_STP`/`A64_LDXR`/`A64_STXR` via `LiftLoadStore`; pre/post/index/offset/extend modes via `Addr` overloads | Full (LDXR/STXR modeled as normal load/store, STXR always success) | `tests/a64_loads_stores.txt` | `src/decode/mod.rs` (not dispatched) | `Stmt::MemRead`/`MemWrite` exist | `memory_read`/`memory_write` | Not emitted | None | Not in QEMU harness | Decode not implemented; native missing |
| Exclusive / acquire-release | LDXR/STXR/STL/LDL etc. via `LiftLoadStore` with `MemOrdering` | Partial (exclusive pairs simplified) | `tests/a64_loads_stores.txt` | Not decoded | `MemOrdering` enum exists | N/A | Not emitted | None | Not in QEMU harness | Unsupported |
| NEON/SIMD integer | `src/a64/simd.cc` `LiftSIMD` + `LiftThreeSame`/`LiftCmXX` etc. | Very large coverage | `tests/a64_simd.txt` | Not decoded | Vector `Expr` variants exist (`VectorMap`, etc.) | Returns 0 (stub) | Not emitted | None | Not in QEMU harness | Decode not implemented; native missing |
| Floating-point | `A64_FADD`/`FSUB`/`FMUL`/`FDIV`/`FMAX`/`FMIN`/`FABS`/`FNEG`/`FSQRT`/`FCVT` etc. via `LiftBinOpFP`/`LiftIntrinsicFP`/`LiftFMA` | Full scalar FP | `tests/a64_scalar_fp.txt` | Not decoded | `ScalarFPBinary` exists | `eval_expr` hits `_ => UnimplementedExpr` | Not emitted | None | Not in QEMU harness | Unsupported |
| System instructions | `MSR`/`MRS` (NZCV, TPIDR_EL0, FPCR, FPSR, DCZID, MIDR), `SVC`, `DMB`, `HINT` | Partial (userspace-relevant) | `tests/a64_data_proc.txt` | `src/decode/mod.rs` (not dispatched) | `Stmt::WriteSpecial`/`ReadSpecial` exist | `read_special`/`write_special` stubs | Not emitted | None | Not in QEMU harness | Unsupported |

### 4.1 Verification status notes

- **Arm2X86_64 native codegen** supports only `X0 = X1 op X2` (RAX/RCX/RDX). Registers X8–X30, W-registers (in many paths), SP, and memory are not supported in the native emitter.
- **QEMU differential** is wired in `tests/common/mod.rs` (`run_qemu_case`, `run_differential_suite_op`) and exercised only for ADD, SUB, AND, MUL in `tests/e2e_qemu_differential.rs`.
- **Rellume** executes its test corpus through an assembler + LLVM JIT; flag results are compared (e.g. `n=00 z=01 c=01 v=00` for `cmp`).

---

## 5. Correctness Gaps and Concrete Reproducers

### Arm2X86_64 gaps (from source inspection)

1. **Native codegen lacks branches entirely.** `lower_to_x86_64` (`src/codegen.rs:833`) matches only data-processing mnemonics. `B`, `BCOND`, `CBZ`, `RET`, etc. return `Err(UnsupportedMnemonic)`. No reproducer needed — any branch instruction fails at codegen.

2. **Native codegen register limit.** `arm_reg_to_x86` (`src/codegen.rs:24`) maps only X0–X7 to RAX–RDI. X8–X30 return `Err(UnsupportedOperand)`. Reproducer: any instruction using X8 as rd/rn/rm.

3. **ADC/SBC carry-in runtime.** The native `emit_adc_sbc` (`src/codegen.rs`) originally hardcoded `clc` (CF=0), ignoring ARM C-in. Fixed in this audit: carry-in is now encoded at codegen time via `carry_in` param (`emit_adc_sbc_with_flags(decoded, carry_in)`), and `run_native_with_flags` passes carry_in via ABI R8 (3rd integer arg). Verified by `tests/e2e_adc_sbc_flags.rs` (3 tests pass).

4. **CCMP/CCMN native is simplified.** `emit_ccmp_ccmn_with_flags` (`src/codegen.rs`) ignores the condition and NZCV immediate — it always computes `rn - rm` and saves flags. Reproducer: CCMP with condition false should copy nzcv immediate to flags, but native always does cmp.

5. **W-register writes.** Decode produces `Reg::W(n)` but codegen's `arm_reg_to_x86` errors on `Reg::W(n)` (lines 41-44). W-register native execution is unsupported.

6. **No memory operations in native codegen.** `MemRead`/`MemWrite` stmts exist but `lower_to_x86_64` has no load/store emission.

### Rellume gaps (from source inspection)

1. **LDXR/STXR exclusive pairs simplified** — STXR always returns success (Rs=0), no true exclusive monitor (`src/a64/main.cc` comment at LDXR/STXR case).

2. **Half-precision FP not implemented** — `A64_FCVT_H` and `A64_FCVT_D` with `FSZ_H` goto `unhandled`.

3. **FRINT32/FRINT64 not supported** — `A64_FRINT` with `bits != 0` or `FSZ_H` goto `unhandled`.

4. **FJCVTZS never/seldom seen** — commented out.

---

## 6. Test Results with Commands and Actual Outcomes

### Arm2X86_64 (executed)

```
cargo test --test e2e_adc_sbc_flags --test e2e_adc_sbc --test e2e_addsub_flags --test e2e_logical_flags --test e2e_ccmp_ccmn_flags
```

Results:
- `e2e_adc_sbc_flags`: 3 passed (adc_sbc_flags, adc_sbc_edge_cases, adc_sbc_32bit)
- `e2e_adc_sbc`: 1 passed
- `e2e_addsub_flags`: 1 passed
- `e2e_logical_flags`: 1 passed
- `e2e_ccmp_ccmn_flags`: 1 passed (ccmp_ccmn_flags), `ccmn_flags` test is empty/skipped

Pre-existing linker failure: `cargo test` full run fails at `vdiv_scalar` (link.exe exit code 1285) — unrelated to this audit.

QEMU differential tests (`tests/e2e_qemu_differential.rs`) pass for add/sub/and/mul when WSL + `qemu-aarch64` + `aarch64-linux-gnu-gcc` are available.

### Rellume (not executed in this environment)

No build/run attempted. Tests exist at `tests/a64_*.txt` with expected results; run via `tests/test_driver.cc` / `tests/test_parser.py` under meson.

---

## 7. Integration and Licensing Assessment

| Factor | Rellume | Arm2X86_64 |
|--------|---------|------------|
| License | LGPLv2.1+ | MIT |
| Language | C/C++ | Rust (edition 2024) |
| Build | meson + LLVM | cargo |
| Dependencies | LLVM (heavy, version-sensitive) | `thiserror` only |
| Integration | Library API: `LiftInstruction(Instr, FunctionInfo, LLConfig, ArchBasicBlock)` | `decode_instruction` → `lower_to_ir` → `lower_to_x86_64` |

Licensing: LGPLv2.1+ requires dynamic linking or source disclosure for derivative works. MIT is permissive. Integrating Rellume into Arm2X86_64 would impose LGPL obligations on the combined work.

Farmdec (the AArch64 decoder used by Rellume, referenced as `farmdec::Inst`, `fad_get_cond`, etc.) appears to be a submodule/dependency (`.gitmodules` exists). This adds another dependency surface.

---

## 8. Benchmark Methodology (Proposed)

Use the same AArch64 instruction corpus for both implementations:

**Test programs:**
1. Scalar arithmetic + condition flags: ADD/SUB/ADDS/SUBS/ADC/SBC with flag-setting variants, including carry/overflow edge cases.
2. Branch-heavy code: conditional branches, CBZ/CBNZ, TBZ/TBNZ, RET, indirect branches.
3. Loads/stores: LDR/STR with immediate, register-offset, pre-index, post-index, extend addressing; LDP/STP pairs.
4. NEON/SIMD: vector add/mul/shift from `tests/a64_simd.txt`.
5. A representative loop: e.g. a memcpy or axpy-style loop compiled to AArch64.

**Correctness:** Compare final register state, flags, and memory against QEMU (`qemu-aarch64`) or Rellume's own JIT execution where available.

**Metrics collected separately:**
- Translation/compilation time (per instruction and per function).
- Generated code size (bytes).
- Warm and cold execution throughput (cycles/iterations via `rdtsc` or `perf`).
- Memory/register pressure.

Separate translation latency from steady-state execution performance. Do not compare implementations with different semantic coverage or different optimization levels without disclosing it.

---

## 9. Prioritized Recommendations for Arm2X86_64

1. **Complete native codegen register coverage** — extend `arm_reg_to_x86` to X8–X30 and W-registers so more instructions can run natively. High benefit, low risk.

2. **Extend QEMU differential to all decoded instructions** — add ORR, EOR, MUL, SDIV, UDIV, and flag-setting variants (ADDS/SUBS/ANDS) to `ArithmeticOp` and `generate_qemu_c`. This is the biggest correctness gap per `CHECKLIST.md`.

3. **Implement native branches** — add `B`, `BCOND`, `CBZ/CBNZ`, `TBZ/TBNZ`, `RET` emission to `lower_to_x86_64`. Required for any real control flow.

4. **Fix CHECKLIST.md accuracy** — mark only instructions where all 5 stages genuinely pass; currently over-claims for some entries.

5. **Address linker failure in `vdiv_scalar`** — pre-existing test build issue blocking full `cargo test`.

**Strategy recommendation: A (keep current architecture).** Rellume's LLVM-based codegen would improve optimization and SIMD coverage but introduces LGPL licensing, a heavy LLVM dependency, and C++/Rust FFI complexity. The current Rust architecture is simpler and MIT-licensed. Rellume is best used as a **reference implementation** (Strategy B) for comparing decoding and semantics, not as a replacement codegen.

---

## 10. Unverified Claims and Evidence Needed

- Rellume's actual test pass/fail results (not executed in this environment; needs meson + LLVM build).
- Arm2X86_64 full `cargo test` results (blocked by `vdiv_scalar` linker failure).
- QEMU differential results for instructions beyond add/sub/and/mul (not yet implemented).
- Performance comparison between Arm2X86_64 native codegen and Rellume/LLVM JIT (no benchmark data collected).
- Whether farmdec decoder is a separate dependency or vendored (`.gitmodules` suggests separate; not inspected).

## 11. Final Question: What Would Rellume Give Arm2X86_64, and What Would Be Lost?

### Capabilities Rellume has that Arm2X86_64 does not

1. **LLVM-based native codegen** — Rellume lifts to LLVM IR and lets LLVM perform register allocation, instruction selection, and optimization. Arm2X86_64 emits raw x86-64 bytes with a fixed X0–X7 → RAX–RDI mapping and no register allocator. Rellume can handle any register (X0–X30, W, SP, V/SIMD registers); Arm2X86_64's codegen rejects X8+ and W-registers.

2. **SIMD/NEON and floating-point** — Rellume has ~78KB of SIMD lifting code (`src/a64/simd.cc`) plus scalar FP (`FADD`, `FMUL`, `FDIV`, `FSQRT`, `FCVT`, etc.). Arm2X86_64 has SIMD `Expr` variants but the interpreter returns 0 (stub) and codegen doesn't emit them.

3. **Branches and control flow** — Rellume lifts `B`, `BCOND`, `CBZ`, `TBZ`, `RET`, `BL/BLR` with PC manipulation and conditional branches on the RegFile PC. Arm2X86_64 codegen emits no branch opcodes at all.

4. **Loads/stores with all addressing modes** — Rellume's `LiftLoadStore` handles immediate, register-offset, pre/post-index, extend, and pair modes. Arm2X86_64 has `MemRead`/`MemWrite` stmts but no native emission.

5. **Division guards** — Rellume handles div-by-zero and INT_MIN/-1 overflow explicitly for SDIV/UDIV. Arm2X86_64's `emit_divide` does not handle division by zero (tests use nonzero divisors).

### What would be lost or complicated by integrating Rellume

1. **Licensing** — Rellume is LGPLv2.1+; Arm2X86_64 is MIT. Integrating Rellume (or linking against it statically) would impose LGPL obligations: source disclosure and the right to relink for derivative works. This changes the project's licensing posture.

2. **Dependency weight** — Rellume requires LLVM (meson build, version-sensitive, commits mention LLVM 21/22/23 compatibility) plus farmdec (`.gitmodules` submodule). Arm2X86_64 has only `thiserror`. Adding LLVM is a major build/CI burden.

3. **Language boundary** — Rellume is C++; Arm2X86_64 is Rust. Integration requires C++/Rust FFI or reimplementing the lifter in Rust. The current pure-Rust pipeline (decode → typed IR → interp → native bytes) is simpler to audit and test.

4. **Semantic model mismatch** — Rellume models CPU state as LLVM values in basic blocks (SSA). Arm2X86_64 uses a typed Rust IR (`Expr`/`Stmt`) interpreted on `CpuState`, then lowers to standalone native functions. Lowering Arm2X86_64's typed IR to LLVM IR is not trivial; the abstractions are not directly compatible.

5. **Correctness model** — Arm2X86_64's value is its per-instruction, per-stage coverage tracking (Decode → IR → Interp → QEMU → Native) in `CHECKLIST.md`. Rellume tests against its own JIT execution but doesn't have the same staged pipeline tracking.

### Most valuable next engineering step

**Extend the QEMU differential harness to cover all decoded instructions (ORR, EOR, MUL, SDIV, UDIV, ADDS/SUBS/ANDS flag variants), then fix CHECKLIST.md to mark only instructions where all 5 stages genuinely pass.**

Reason: The single biggest correctness gap identified in this audit is that `CHECKLIST.md` over-claims "Fully implemented" status for instructions (ADD, SUB, AND, ORR, EOR, MUL, SDIV, UDIV) that lack QEMU differential validation. The harness infrastructure already exists (`run_qemu_case`, `run_differential_suite_op`, `generate_qemu_c` with Mul/Sdiv/Udiv support added). Completing this gives measurable, differential evidence of correctness across the full pipeline — the core promise of the project — without the licensing, dependency, and FFI complexity of integrating Rellume. Rellume should be used as a **reference implementation** (Strategy B) for cross-checking decoding and semantics, not integrated as codegen (Strategy C).


