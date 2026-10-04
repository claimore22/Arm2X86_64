## MOV (vector)

Move vector This instruction copies the vector in the source SIMD&FP register into the destination SIMD&FP register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. This is an alias of [ORR (vector, register)](orr_advsimd_reg.html). This means: - The encodings in this description are named to match the encodings of [ORR (vector, register)](orr_advsimd_reg.html). - The description of [ORR (vector, register)](orr_advsimd_reg.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [ORR (vector, register)](orr_advsimd_reg.html) gives the operational pseudocode for this instruction.
