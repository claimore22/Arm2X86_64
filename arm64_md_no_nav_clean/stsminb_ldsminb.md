## STSMINB, STSMINLB

Atomic signed minimum on byte, without return This instruction atomically loads an 8-bit byte from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as signed numbers. - STSMINB does not have release semantics. - STSMINLB stores to memory with release semantics, as described in Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes. This is an alias of [LDSMINB, LDSMINAB, LDSMINALB, LDSMINLB](ldsminb.html). This means: - The encodings in this description are named to match the encodings of [LDSMINB, LDSMINAB, LDSMINALB, LDSMINLB](ldsminb.html). - The description of [LDSMINB, LDSMINAB, LDSMINALB, LDSMINLB](ldsminb.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [LDSMINB, LDSMINAB, LDSMINALB, LDSMINLB](ldsminb.html) gives the operational pseudocode for this instruction.
