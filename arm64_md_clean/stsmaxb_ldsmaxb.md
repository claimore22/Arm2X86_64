## STSMAXB, STSMAXLB

Atomic signed maximum on byte, without return This instruction atomically loads an 8-bit byte from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as signed numbers. - STSMAXB does not have release semantics. - STSMAXLB stores to memory with release semantics, as described in Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes. This is an alias of [LDSMAXB, LDSMAXAB, LDSMAXALB, LDSMAXLB](ldsmaxb.html). This means: - The encodings in this description are named to match the encodings of [LDSMAXB, LDSMAXAB, LDSMAXALB, LDSMAXLB](ldsmaxb.html). - The description of [LDSMAXB, LDSMAXAB, LDSMAXALB, LDSMAXLB](ldsmaxb.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [LDSMAXB, LDSMAXAB, LDSMAXALB, LDSMAXLB](ldsmaxb.html) gives the operational pseudocode for this instruction.
