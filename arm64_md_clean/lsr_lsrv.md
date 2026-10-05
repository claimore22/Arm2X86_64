## LSR (register)

Logical shift right (register) This instruction shifts a register value right by a variable number of bits, shifting in zeros, and writes the result to the destination register. The value of the second source register modulo the register size in bits gives the number of bits by which the first source register is right-shifted. This is an alias of [LSRV](lsrv.html). This means: - The encodings in this description are named to match the encodings of [LSRV](lsrv.html). - The description of [LSRV](lsrv.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [LSRV](lsrv.html) gives the operational pseudocode for this instruction.
