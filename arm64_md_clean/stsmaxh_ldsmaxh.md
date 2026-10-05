## STSMAXH, STSMAXLH

Atomic signed maximum on halfword, without return This instruction atomically loads a 16-bit halfword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as signed numbers. - STSMAXH does not have release semantics. - STSMAXLH stores to memory with release semantics, as described in Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes. This is an alias of [LDSMAXH, LDSMAXAH, LDSMAXALH, LDSMAXLH](ldsmaxh.html). This means: - The encodings in this description are named to match the encodings of [LDSMAXH, LDSMAXAH, LDSMAXALH, LDSMAXLH](ldsmaxh.html). - The description of [LDSMAXH, LDSMAXAH, LDSMAXALH, LDSMAXLH](ldsmaxh.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [LDSMAXH, LDSMAXAH, LDSMAXALH, LDSMAXLH](ldsmaxh.html) gives the operational pseudocode for this instruction.
