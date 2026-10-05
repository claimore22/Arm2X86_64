## MOV (immediate, zeroing)

Move signed integer immediate to vector elements (zeroing) This instruction moves a signed integer immediate to each Active element in the destination vector. Inactive elements in the destination vector register are set to zero. The immediate operand is a signed value in the range -128 to +127, and for element widths of 16 bits or higher it may also be a signed multiple of 256 in the range -32768 to +32512 (excluding 0). The immediate is encoded in 8 bits with an optional left shift by 8. The preferred disassembly when the shift option is specified is "#<imm>, LSL \#8". However an assembler and disassembler may also allow use of the shifted 16-bit value unless the immediate is 0 and the shift amount is 8, which must be unambiguously described as "#0, LSL \#8". This is an alias of [CPY (immediate, zeroing)](cpy_z_o_i.html). This means: - The encodings in this description are named to match the encodings of [CPY (immediate, zeroing)](cpy_z_o_i.html). - The description of [CPY (immediate, zeroing)](cpy_z_o_i.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [CPY (immediate, zeroing)](cpy_z_o_i.html) gives the operational pseudocode for this instruction.
