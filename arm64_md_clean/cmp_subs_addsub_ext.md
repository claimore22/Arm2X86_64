## CMP (extended register)

Compare (extended register) This instruction subtracts a sign or zero-extended register value, followed by an optional left shift amount, from a register value. The argument that is extended from the <Rm> register can be a byte, halfword, word, or doubleword. It updates the condition flags based on the result, and discards the result. This is an alias of [SUBS (extended register)](subs_addsub_ext.html). This means: - The encodings in this description are named to match the encodings of [SUBS (extended register)](subs_addsub_ext.html). - The description of [SUBS (extended register)](subs_addsub_ext.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [SUBS (extended register)](subs_addsub_ext.html) gives the operational pseudocode for this instruction.
