## CBLO (register)

Compare unsigned lower than register and branch This instruction compares the unsigned values in two registers, and conditionally branches to a label at a PC-relative offset if the second value is lower than the first. This instruction provides a hint that this is not a subroutine call or return. This instruction does not affect the condition flags. This is a pseudo-instruction of [CB<cc> (register)](cbcc_regs.html). This means: - The encodings in this description are named to match the encodings of [CB<cc> (register)](cbcc_regs.html). - The assembler syntax is used only for assembly, and is not used on disassembly. - The description of [CB<cc> (register)](cbcc_regs.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [CB<cc> (register)](cbcc_regs.html) gives the operational pseudocode for this instruction.
