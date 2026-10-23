# Test Gaps

- Native execution tests missing for conditional select, CCMP/CCMN.
- QEMU differential not run for move-wide W32, bitfield BFM, logical immediate, ADC/SBC, etc.
- Flags C/V not verified for ADDS/SUBS/CMP.
- W32 logical immediate not implemented/tested.
- No randomized differential tests.
- No branch/memory/SIMD/FP tests (not implemented).
