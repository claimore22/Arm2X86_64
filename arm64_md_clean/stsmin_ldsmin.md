## STSMIN, STSMINL

Atomic signed minimum on word or doubleword, without return This instruction atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the smaller value back to memory, treating the values as signed numbers. - STSMIN does not have release semantics. - STSMINL stores to memory with release semantics, as described in Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes. This is an alias of [LDSMIN, LDSMINA, LDSMINAL, LDSMINL](ldsmin.html). This means: - The encodings in this description are named to match the encodings of [LDSMIN, LDSMINA, LDSMINAL, LDSMINL](ldsmin.html). - The description of [LDSMIN, LDSMINA, LDSMINAL, LDSMINL](ldsmin.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [LDSMIN, LDSMINA, LDSMINAL, LDSMINL](ldsmin.html) gives the operational pseudocode for this instruction.
