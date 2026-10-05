## MOV (scalar)

Move vector element to scalar This instruction duplicates the specified vector element in the SIMD&FP source register into a scalar, and writes the result to the SIMD&FP destination register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. This is an alias of [DUP (element)](dup_advsimd_elt.html). This means: - The encodings in this description are named to match the encodings of [DUP (element)](dup_advsimd_elt.html). - The description of [DUP (element)](dup_advsimd_elt.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [DUP (element)](dup_advsimd_elt.html) gives the operational pseudocode for this instruction.
