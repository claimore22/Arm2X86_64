## EON

Bitwise exclusive-OR with inverted immediate (unpredicated) This instruction performs a bitwise exclusive-OR between an inverted immediate and each 64-bit element of the source vector, and destructively places the results in the corresponding elements of the source vector. The immediate is a 64-bit value consisting of a single run of ones or zeros repeating every 2, 4, 8, 16, 32 or 64 bits. This instruction is unpredicated. This is a pseudo-instruction of [EOR (immediate)](eor_z_zi.html). This means: - The encodings in this description are named to match the encodings of [EOR (immediate)](eor_z_zi.html). - The assembler syntax is used only for assembly, and is not used on disassembly. - The description of [EOR (immediate)](eor_z_zi.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [EOR (immediate)](eor_z_zi.html) gives the operational pseudocode for this instruction.
