## MVN

Bitwise NOT (vector) This instruction reads each vector element from the source SIMD&FP register, places the inverse of each value into a vector, and writes the vector to the destination SIMD&FP register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. This is an alias of [NOT](not_advsimd.html). This means: - The encodings in this description are named to match the encodings of [NOT](not_advsimd.html). - The description of [NOT](not_advsimd.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [NOT](not_advsimd.html) gives the operational pseudocode for this instruction.
