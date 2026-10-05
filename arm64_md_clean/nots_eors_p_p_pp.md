## NOTS

Bitwise invert predicate, setting the condition flags This instruction performs a bitwise invert of each active element of the source predicate, and places the results in the corresponding elements of the destination predicate. Inactive elements in the destination predicate register are set to zero. This instruction sets the First (N), None (Z), and !Last (C) condition flags based on the predicate result, and sets the V flag to zero. This is an alias of [EORS](eors_p_p_pp.html). This means: - The encodings in this description are named to match the encodings of [EORS](eors_p_p_pp.html). - The description of [EORS](eors_p_p_pp.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [EORS](eors_p_p_pp.html) gives the operational pseudocode for this instruction.
