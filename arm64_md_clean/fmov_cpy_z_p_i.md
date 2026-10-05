## FMOV (zero, predicated)

Move floating-point +0.0 to vector elements (predicated) This instruction moves floating-point constant +0.0 to each Active element in the destination vector. Inactive elements in the destination vector register remain unmodified. This is a pseudo-instruction of [CPY (immediate, merging)](cpy_z_p_i.html). This means: - The encodings in this description are named to match the encodings of [CPY (immediate, merging)](cpy_z_p_i.html). - The assembler syntax is used only for assembly, and is not used on disassembly. - The description of [CPY (immediate, merging)](cpy_z_p_i.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [CPY (immediate, merging)](cpy_z_p_i.html) gives the operational pseudocode for this instruction.
