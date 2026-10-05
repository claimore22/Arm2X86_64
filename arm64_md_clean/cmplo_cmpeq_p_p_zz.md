## CMPLO (vectors)

Compare unsigned lower than vector, setting the condition flags This instruction compares active unsigned integer elements in the first source vector being lower than the corresponding unsigned elements in the second source vector, and places the boolean results of the comparison in the corresponding elements of the destination predicate. Inactive elements in the destination predicate register are set to zero. This instruction sets the First (N), None (Z), and !Last (C) condition flags based on the predicate result, and sets the V flag to zero. This is a pseudo-instruction of [CMP<cc> (vectors)](cmpeq_p_p_zz.html). This means: - The encodings in this description are named to match the encodings of [CMP<cc> (vectors)](cmpeq_p_p_zz.html). - The assembler syntax is used only for assembly, and is not used on disassembly. - The description of [CMP<cc> (vectors)](cmpeq_p_p_zz.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [CMP<cc> (vectors)](cmpeq_p_p_zz.html) gives the operational pseudocode for this instruction.
