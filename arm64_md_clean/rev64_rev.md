## REV64

Reverse bytes This instruction reverses the byte order in a 64-bit general-purpose register. When assembling for Armv8.2, an assembler must support this pseudo-instruction. It is OPTIONAL whether an assembler supports this pseudo-instruction when assembling for an architecture earlier than Armv8.2. This is a pseudo-instruction of [REV](rev.html). This means: - The encodings in this description are named to match the encodings of [REV](rev.html). - The assembler syntax is used only for assembly, and is not used on disassembly. - The description of [REV](rev.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [REV](rev.html) gives the operational pseudocode for this instruction.
