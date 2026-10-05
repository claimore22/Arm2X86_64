## FCMLE (vectors)

Floating-point compare less than or equal to vector This instruction compares active floating-point elements in the first source vector being less than or equal to the corresponding elements in the second source vector, and places the boolean results of the comparison in the corresponding elements of the destination predicate. Inactive elements in the destination predicate register are set to zero. This instruction does not set the condition flags. This is a pseudo-instruction of [FCM<cc> (vectors)](fcmeq_p_p_zz.html). This means: - The encodings in this description are named to match the encodings of [FCM<cc> (vectors)](fcmeq_p_p_zz.html). - The assembler syntax is used only for assembly, and is not used on disassembly. - The description of [FCM<cc> (vectors)](fcmeq_p_p_zz.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [FCM<cc> (vectors)](fcmeq_p_p_zz.html) gives the operational pseudocode for this instruction.
