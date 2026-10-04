## MOVS (unpredicated)

Move predicate (unpredicated), setting the condition flags This instruction reads all elements from the source predicate and places those elements in the destination predicate. This instruction is unpredicated. This instruction sets the First (N), None (Z), and !Last (C) condition flags based on the predicate result, and sets the V flag to zero. This is an alias of [ORRS](orrs_p_p_pp.html). This means: - The encodings in this description are named to match the encodings of [ORRS](orrs_p_p_pp.html). - The description of [ORRS](orrs_p_p_pp.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [ORRS](orrs_p_p_pp.html) gives the operational pseudocode for this instruction.
