## NOT (predicate)

Bitwise invert predicate This instruction performs a bitwise invert of each active element of the source predicate, and places the results in the corresponding elements of the destination predicate. Inactive elements in the destination predicate register are set to zero. This instruction does not set the condition flags. This is an alias of [EOR (predicates)](eor_p_p_pp.html). This means: - The encodings in this description are named to match the encodings of [EOR (predicates)](eor_p_p_pp.html). - The description of [EOR (predicates)](eor_p_p_pp.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [EOR (predicates)](eor_p_p_pp.html) gives the operational pseudocode for this instruction.
