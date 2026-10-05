## CBGE (immediate)

Compare signed greater than or equal to immediate and branch This instruction compares the signed value in a register with an immediate, and conditionally branches to a label at a PC-relative offset if the register value is greater than or equal to the immediate. This instruction provides a hint that this is not a subroutine call or return. This instruction does not affect the condition flags. This is a pseudo-instruction of [CB<cc> (immediate)](cbcc_imm.html). This means: - The encodings in this description are named to match the encodings of [CB<cc> (immediate)](cbcc_imm.html). - The assembler syntax is used only for assembly, and is not used on disassembly. - The description of [CB<cc> (immediate)](cbcc_imm.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [CB<cc> (immediate)](cbcc_imm.html) gives the operational pseudocode for this instruction.
