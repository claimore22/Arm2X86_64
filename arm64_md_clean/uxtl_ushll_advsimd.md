## UXTL, UXTL2

Unsigned extend long This instruction copies each vector element from the lower or upper half of the source SIMD&FP register into a vector, and writes the vector to the destination SIMD&FP register. The destination vector elements are twice as long as the source vector elements. The UXTL instruction extracts vector elements from the lower half of the source register. The UXTL2 instruction extracts vector elements from the upper half of the source register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. This is an alias of [USHLL, USHLL2](ushll_advsimd.html). This means: - The encodings in this description are named to match the encodings of [USHLL, USHLL2](ushll_advsimd.html). - The description of [USHLL, USHLL2](ushll_advsimd.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [USHLL, USHLL2](ushll_advsimd.html) gives the operational pseudocode for this instruction.
