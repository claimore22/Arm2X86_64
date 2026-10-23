# semantic_ir

Initial typed semantic IR for translating AArch64 architectural pseudocode into a
machine-independent representation.

## Design

The IR represents **ARM architectural semantics**, not x86-64 instructions.

The intended pipeline is:

    Stanford ARM64 pseudocode
             |
             v
        DeepSeek translation
             |
             v
        semantic_ir
             |
             +--> interpreter
             |
             +--> QEMU differential testing
             |
             +--> x86-64 native codegen (JIT)

The AI must populate this IR. It must not redefine the IR.

## Rules

- No `unsafe` (except in test harness for native code execution).
- No `panic!` (except in tests).
- No `unwrap()` (except in tests).
- Explicit architectural state.
- Explicit widths.
- Explicit signed/unsigned operations.
- SP/WSP and XZR/WZR are distinct concepts.
- Memory operations carry size and ordering.
- Exceptions are represented explicitly.
- Provenance is part of an instruction.
- Do not encode x86-64 implementation details in this layer.

## Current Status

- **59 library tests passing** (decode, IR, interpreter, codegen)
- **100+ e2e tests** covering instruction decode, IR lowering, interpretation, and native x86-64 codegen
- **Native flag computation** for ADDS/SUBS/ANDS/ANDSI with differential validation against interpreter
- **Supported instructions**: ADD, SUB, ADDS, SUBS, AND, ANDS, BIC, BICS, ORR, EOR, EON, MOVZ, MOVN, MOVK, MUL, SDIV, UDIV, ANDI, ORRI, EORI, ANDSI, ADDI, SUBI, ADC, SBC, CCMP, CCMN, and many more
- **Native x86-64 codegen** for arithmetic, logical, multiply, divide, bitfield, and move-wide instructions
- **Flag capture** via x86 RFLAGS → ARM NZCV conversion

## Important

This is intentionally incomplete.

It is a **bootstrap semantic model**, not the final ARM architecture.

The next step is to translate a very small representative set of
instructions and extend the IR only when real Stanford pseudocode demonstrates
a missing architectural capability.

Initial candidates:

- ADD
- SUBS
- CSEL
- LDR
- B

Do not process the full corpus until these expose the actual requirements of
the IR.
