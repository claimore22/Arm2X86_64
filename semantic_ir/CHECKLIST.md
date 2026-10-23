# AArch64 Instruction Implementation Checklist

Each instruction is considered **fully implemented** when all 5 pipeline stages pass:

| Stage | Meaning |
|-------|---------|
| **Decode** | Correctly parses encoding fields (registers, immediates, shifts, conditions) |
| **IR** | Produces correct typed semantic IR statements (WriteReg, WriteFlag, etc.) |
| **Interp** | Interpreter executes IR and produces correct architectural results and flags |
| **QEMU** | Differential test matches QEMU reference (≥ 90% of encoding space) |
| **Codegen** | Native x86-64 machine code produces identical results and flags |

**Status key:**
- ✅ Complete
- ⚠️ Partial / in progress
- ❌ Not started

---

## Data Processing - Register (Shifted)

| Instruction | Decode | IR | Interp | QEMU | Codegen | Status |
|-------------|--------|----|--------|------|---------|--------|
| ADD (shifted) | ✅ | ✅ | ✅ | ✅ | ✅ | **Fully implemented** |
| ADDS (shifted) | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |
| SUB (shifted) | ✅ | ✅ | ✅ | ✅ | ✅ | **Fully implemented** |
| SUBS (shifted) | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |
| AND (shifted) | ✅ | ✅ | ✅ | ✅ | ✅ | **Fully implemented** |
| ANDS (shifted) | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |
| BIC (shifted) | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |
| BICS (shifted) | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |
| ORR (shifted) | ✅ | ✅ | ✅ | ✅ | ✅ | **Fully implemented** |
| EOR (shifted) | ✅ | ✅ | ✅ | ✅ | ✅ | **Fully implemented** |
| EON (shifted) | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |
| ORN (shifted) | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |

## Data Processing - Register (Extended)

| Instruction | Decode | IR | Interp | QEMU | Codegen | Status |
|-------------|--------|----|--------|------|---------|--------|
| ADD (extended) | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |
| SUB (extended) | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |
| ADC | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |
| SBC | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |

## Data Processing - Immediate

| Instruction | Decode | IR | Interp | QEMU | Codegen | Status |
|-------------|--------|----|--------|------|---------|--------|
| ADD (imm) | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |
| SUB (imm) | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |
| ADDI | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |
| SUBI | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |
| AND (imm) | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |
| ORR (imm) | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |
| EOR (imm) | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |
| ANDS (imm) | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |
| MOVZ | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |
| MOVN | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |
| MOVK | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |

## Data Processing - Bitfield

| Instruction | Decode | IR | Interp | QEMU | Codegen | Status |
|-------------|--------|----|--------|------|---------|--------|
| SBFM | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |
| UBFM | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |
| BFM | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |

## Data Processing - Conditional

| Instruction | Decode | IR | Interp | QEMU | Codegen | Status |
|-------------|--------|----|--------|------|---------|--------|
| CSEL | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| CSINC | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| CSINV | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| CSNEG | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| CCMP | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |
| CCMN | ✅ | ✅ | ✅ | ⚠️ | ✅ | Almost |
| CSET | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |

## Data Processing - Multiply/Divide

| Instruction | Decode | IR | Interp | QEMU | Codegen | Status |
|-------------|--------|----|--------|------|---------|--------|
| MUL | ✅ | ✅ | ✅ | ✅ | ✅ | **Fully implemented** |
| MNEG | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| MADD | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| MSUB | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| SDIV | ✅ | ✅ | ✅ | ✅ | ✅ | **Fully implemented** |
| UDIV | ✅ | ✅ | ✅ | ✅ | ✅ | **Fully implemented** |

## Branches

| Instruction | Decode | IR | Interp | QEMU | Codegen | Status |
|-------------|--------|----|--------|------|---------|--------|
| B | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| BL | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| BR | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| BLR | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| RET | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| B.cond | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| CBZ | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| CBNZ | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| TBZ | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| TBNZ | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |

## Loads and Stores

| Instruction | Decode | IR | Interp | QEMU | Codegen | Status |
|-------------|--------|----|--------|------|---------|--------|
| LDR (imm) | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| LDR (reg) | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| STR (imm) | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| STR (reg) | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| LDP | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| STP | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| LDUR | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| STUR | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |

## Floating Point / SIMD

| Instruction | Decode | IR | Interp | QEMU | Codegen | Status |
|-------------|--------|----|--------|------|---------|--------|
| FADD | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| FSUB | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| FMUL | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| FDIV | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| FMOV | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| FCMP | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| FCVT | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| SIMD (ADD, SUB, MUL, etc.) | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |

## System / Exceptions

| Instruction | Decode | IR | Interp | QEMU | Codegen | Status |
|-------------|--------|----|--------|------|---------|--------|
| SVC | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| HVC | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| SMC | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| BRK | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| NOP | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| HINT | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| MSR | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| MRS | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |

## SVE / SME

| Instruction | Decode | IR | Interp | QEMU | Codegen | Status |
|-------------|--------|----|--------|------|---------|--------|
| SVE (ADD, SUB, MUL, etc.) | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |
| SME (ZA, ZT0, etc.) | ✅ | ✅ | ✅ | ⚠️ | ❌ | Partial |

---

## Summary

- **Decode**: ~90% of ARM64 instruction set covered
- **IR**: All decoded instructions lower to typed semantic IR
- **Interpreter**: All IR executes correctly
- **QEMU Differential**: ✅ Wired up for arithmetic operations (ADD, SUB, AND, ORR, EOR, MUL, SDIV, UDIV) with unique file names per test case
- **Native Codegen**: ~40% of instructions have x86-64 codegen (arithmetic, logical, bitfield, move-wide, multiply, divide, extend, adc/sbc, ccmp/ccmn)
- **Native Flags**: Flag capture working for ADDS/SUBS/ANDS/ANDSI/ADC/SBC/CCMP/CCMN

### What "fully implemented" means
An instruction is **fully implemented** when all 5 stages pass, including differential testing against QEMU and native x86-64 codegen with flag computation. Currently, arithmetic instructions (ADD, SUB, AND, ORR, EOR, MUL, SDIV, UDIV) have QEMU differential testing wired up.

### Next steps
1. ✅ QEMU differential testing wired up for arithmetic operations
2. Extend QEMU differential testing to all decoded instructions
3. Extend native codegen to cover branches, loads/stores, FP, SIMD
4. Add comprehensive test cases for each instruction
