## MOV (tile to vector, four registers)

Move ZA four-slice operand to Z four-vector operand This instruction operates on a horizontal or vertical ZA four-slice operand within a ZA tile of the specified element size. The first slice of the four-slice operand is selected by rounding down the sum of the slice index register and the immediate offset to the nearest lower multiple of 4, modulo the number of slices in the tile. The immediate offset is a multiple of 4 in the range 0 to the number of elements in a 128-bit vector segment minus 4. This instruction is unpredicated. This is an alias of [MOVA (tile to vector, four registers)](mova_mz4_za.html). This means: - The encodings in this description are named to match the encodings of [MOVA (tile to vector, four registers)](mova_mz4_za.html). - The description of [MOVA (tile to vector, four registers)](mova_mz4_za.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction. It has encodings from 4 classes: [8-bit](#iclass_8_bit) , [16-bit](#iclass_16_bit) , [32-bit](#iclass_32_bit) and [64-bit](#iclass_64_bit)

### Operation

The description of [MOVA (tile to vector, four registers)](mova_mz4_za.html) gives the operational pseudocode for this instruction.
