## MOV (element)

Move vector element to another vector element This instruction copies the vector element of the source SIMD&FP register to the specified vector element of the destination SIMD&FP register. This instruction can insert data into individual elements within a SIMD&FP register without clearing the remaining bits to zero. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. This is an alias of [INS (element)](ins_advsimd_elt.html). This means: - The encodings in this description are named to match the encodings of [INS (element)](ins_advsimd_elt.html). - The description of [INS (element)](ins_advsimd_elt.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [INS (element)](ins_advsimd_elt.html) gives the operational pseudocode for this instruction.
