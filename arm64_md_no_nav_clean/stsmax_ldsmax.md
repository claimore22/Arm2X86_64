## STSMAX, STSMAXL

Atomic signed maximum on word or doubleword, without return This instruction atomically loads a 32-bit word or 64-bit doubleword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as signed numbers. - STSMAX does not have release semantics. - STSMAXL stores to memory with release semantics, as described in Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes. This is an alias of [LDSMAX, LDSMAXA, LDSMAXAL, LDSMAXL](ldsmax.html). This means: - The encodings in this description are named to match the encodings of [LDSMAX, LDSMAXA, LDSMAXAL, LDSMAXL](ldsmax.html). - The description of [LDSMAX, LDSMAXA, LDSMAXAL, LDSMAXL](ldsmax.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [LDSMAX, LDSMAXA, LDSMAXAL, LDSMAXL](ldsmax.html) gives the operational pseudocode for this instruction.
