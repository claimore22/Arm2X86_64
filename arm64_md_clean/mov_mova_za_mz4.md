## MOV (vector to array, four registers)

Move Z four-vector operand to four ZA single-vector groups This instruction operates on four ZA single-vector groups. The single-vector group within each quarter of the ZA array is selected by the sum of the vector select register and offset, modulo quarter the number of ZA array vectors. The vector group symbol VGx4 indicates that the instruction operates on four ZA single-vector groups. The preferred disassembly syntax uses a 64-bit element size, but an assembler should accept any element size if it is used consistently for all operands. The vector group symbol is preferred for disassembly, but optional in assembler source code. This instruction is unpredicated. This is an alias of [MOVA (vector to array, four registers)](mova_za_mz4.html). This means: - The encodings in this description are named to match the encodings of [MOVA (vector to array, four registers)](mova_za_mz4.html). - The description of [MOVA (vector to array, four registers)](mova_za_mz4.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [MOVA (vector to array, four registers)](mova_za_mz4.html) gives the operational pseudocode for this instruction.
