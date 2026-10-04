## MOV (vector to array, two registers)

Move Z two-vector operand to two ZA single-vector groups This instruction operates on two ZA single-vector groups. The single-vector group within each half of the ZA array is selected by the sum of the vector select register and offset, modulo half the number of ZA array vectors. The vector group symbol VGx2 indicates that the instruction operates on two ZA single-vector groups. The preferred disassembly syntax uses a 64-bit element size, but an assembler should accept any element size if it is used consistently for all operands. The vector group symbol is preferred for disassembly, but optional in assembler source code. This instruction is unpredicated. This is an alias of [MOVA (vector to array, two registers)](mova_za_mz2.html). This means: - The encodings in this description are named to match the encodings of [MOVA (vector to array, two registers)](mova_za_mz2.html). - The description of [MOVA (vector to array, two registers)](mova_za_mz2.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [MOVA (vector to array, two registers)](mova_za_mz2.html) gives the operational pseudocode for this instruction.
