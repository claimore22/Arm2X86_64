## MOV (to general)

Move vector element to general-purpose register This instruction reads the unsigned integer from the source SIMD&FP register, zero-extends it to form a 32-bit or 64-bit value, and writes the result to the destination general-purpose register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. This is an alias of [UMOV](umov_advsimd.html). This means: - The encodings in this description are named to match the encodings of [UMOV](umov_advsimd.html). - The description of [UMOV](umov_advsimd.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [UMOV](umov_advsimd.html) gives the operational pseudocode for this instruction.
