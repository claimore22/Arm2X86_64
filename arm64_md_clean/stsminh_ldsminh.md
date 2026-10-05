## STSMINH, STSMINLH

Atomic signed minimum on halfword, without return This instruction atomically loads a 16-bit halfword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as signed numbers. - STSMINH does not have release semantics. - STSMINLH stores to memory with release semantics, as described in Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes. This is an alias of [LDSMINH, LDSMINAH, LDSMINALH, LDSMINLH](ldsminh.html). This means: - The encodings in this description are named to match the encodings of [LDSMINH, LDSMINAH, LDSMINALH, LDSMINLH](ldsminh.html). - The description of [LDSMINH, LDSMINAH, LDSMINALH, LDSMINLH](ldsminh.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [LDSMINH, LDSMINAH, LDSMINALH, LDSMINLH](ldsminh.html) gives the operational pseudocode for this instruction.
