## ASR (register)

Arithmetic shift right (register) This instruction shifts a register value right by a variable number of bits, shifting in copies of its sign bit, and writes the result to the destination register. The value of the second source register modulo the register size in bits gives the number of bits by which the first source register is right-shifted. This is an alias of [ASRV](asrv.html). This means: - The encodings in this description are named to match the encodings of [ASRV](asrv.html). - The description of [ASRV](asrv.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [ASRV](asrv.html) gives the operational pseudocode for this instruction.
