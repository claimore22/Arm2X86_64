## STSETH, STSETLH

Atomic bit set on halfword, without return This instruction atomically loads a 16-bit halfword from memory, performs a bitwise OR with the value held in a register on it, and stores the result back to memory. - STSETH does not have release semantics. - STSETLH stores to memory with release semantics, as described in Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes. This is an alias of [LDSETH, LDSETAH, LDSETALH, LDSETLH](ldseth.html). This means: - The encodings in this description are named to match the encodings of [LDSETH, LDSETAH, LDSETALH, LDSETLH](ldseth.html). - The description of [LDSETH, LDSETAH, LDSETALH, LDSETLH](ldseth.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [LDSETH, LDSETAH, LDSETALH, LDSETLH](ldseth.html) gives the operational pseudocode for this instruction.
