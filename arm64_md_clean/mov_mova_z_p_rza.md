## MOV (tile to vector, single)

Move ZA tile slice to Z vector This instruction operates on individual horizontal or vertical slices within a named ZA tile of the specified element size. The slice number within the tile is selected by the sum of the slice index register and immediate offset, modulo the number of such elements in a vector. The immediate offset is in the range 0 to the number of elements in a 128-bit vector segment minus 1. Inactive elements in the destination vector remain unmodified. This is an alias of [MOVA (tile to vector, single)](mova_z_p_rza.html). This means: - The encodings in this description are named to match the encodings of [MOVA (tile to vector, single)](mova_z_p_rza.html). - The description of [MOVA (tile to vector, single)](mova_z_p_rza.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction. It has encodings from 5 classes: [8-bit](#iclass_8_bit) , [16-bit](#iclass_16_bit) , [32-bit](#iclass_32_bit) , [64-bit](#iclass_64_bit) and [128-bit](#iclass_128_bit)

### Operation

The description of [MOVA (tile to vector, single)](mova_z_p_rza.html) gives the operational pseudocode for this instruction.
