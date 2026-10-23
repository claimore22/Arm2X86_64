# Arm2X86_64 Audit Report

## Baseline
- Commit: bb953fea (feature/instruction-test-coverage)
- `cargo test --test e2e_add_shifted`: 2 passed (after byte-fix).
- `cargo test --test e2e_sub_shifted`: 2 passed.
- `cargo test --test e2e_bitfield`: 3 passed.
- `cargo test --test e2e_logical_shifted`: 2 passed.
- `cargo test --test e2e_mul`: 2 passed.
- `cargo test --test e2e_divide`: 1 passed.
- `cargo test --test e2e_move_wide`: 5 passed.
- `cargo test --test e2e_addsub_imm`: 1 passed.
- `cargo test --test e2e_adc_sbc`: 1 passed.
- `cargo test --test e2e_cond_select`: 2 passed.
- `cargo test --test e2e_cmp`: 1 passed.
- `cargo test --test e2e_addsub_flags`: 1 passed.
- `cargo test --test e2e_logical_not`: 1 passed.
- `cargo test --test e2e_logical_imm`: 1 passed (X64 only, bit21=1).
- `cargo test --test e2e_ccmp`: 1 passed.
- `cargo test --test differential_shifted_comprehensive`: 1 passed (162 QEMU cases).

## Implemented families (scalar)
- ADD/SUB shifted (W/X, native, QEMU)
- Logical shifted (AND/ORR/EOR/ANDS/BIC/ORN/EON/BICS) (W/X, native, QEMU for core)
- Move-wide (MOVZ/MOVN/MOVK)
- Bitfield (SBFM/UBFM/BFM)
- Add/Sub immediate
- Conditional select
- ADC/SBC
- Multiply (MUL)
- Divide (SDIV/UDIV)
- ADDS/SUBS (flags simplified)
- CMP
- Logical immediate (partial, X64 only)
- CCMP/CCMN (partial)

## Missing/unverified
- Branches, memory, SIMD/FP, system instructions.
- Logical immediate W32, extended register, branches.
- Flags for ADC/SBC/SUBS are simplified (C/V not fully correct).
- Native codegen for CCMP/conditional select not implemented.

## Safety
- Production code uses unwrap/expect in some shared code; not audited fully.
