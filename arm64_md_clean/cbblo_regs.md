## CBBLO

Compare unsigned lower than bytes and branch This instruction compares the unsigned byte values in two registers, and conditionally branches to a label at a PC-relative offset if the second value is lower than the first. This instruction provides a hint that this is not a subroutine call or return. This instruction does not affect the condition flags. This is a pseudo-instruction of [CBB<cc>](cbbcc_regs.html). This means: - The encodings in this description are named to match the encodings of [CBB<cc>](cbbcc_regs.html). - The assembler syntax is used only for assembly, and is not used on disassembly. - The description of [CBB<cc>](cbbcc_regs.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [CBB<cc>](cbbcc_regs.html) gives the operational pseudocode for this instruction.
