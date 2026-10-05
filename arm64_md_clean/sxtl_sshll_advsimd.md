## SXTL, SXTL2

Signed extend long This instruction duplicates each vector element in the lower or upper half of the source SIMD&FP register into a vector, and writes the vector to the destination SIMD&FP register. The destination vector elements are twice as long as the source vector elements. All the values in this instruction are signed integer values. The SXTL instruction extracts the source vector from the lower half of the source register. The SXTL2 instruction extracts the source vector from the upper half of the source register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. This is an alias of [SSHLL, SSHLL2](sshll_advsimd.html). This means: - The encodings in this description are named to match the encodings of [SSHLL, SSHLL2](sshll_advsimd.html). - The description of [SSHLL, SSHLL2](sshll_advsimd.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [SSHLL, SSHLL2](sshll_advsimd.html) gives the operational pseudocode for this instruction.
