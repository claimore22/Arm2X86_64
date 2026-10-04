## ROR (register)

Rotate right (register) This instruction provides the value of the contents of a register rotated by a variable number of bits. The bits that are rotated off the right end are inserted into the vacated bit positions on the left. The value of the second source register modulo the register size in bits gives the number of bits by which the first source register is right-shifted. This is an alias of [RORV](rorv.html). This means: - The encodings in this description are named to match the encodings of [RORV](rorv.html). - The description of [RORV](rorv.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [RORV](rorv.html) gives the operational pseudocode for this instruction.
