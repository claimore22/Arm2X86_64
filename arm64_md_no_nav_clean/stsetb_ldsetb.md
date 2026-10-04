## STSETB, STSETLB

Atomic bit set on byte, without return This instruction atomically loads an 8-bit byte from memory, performs a bitwise OR with the value held in a register on it, and stores the result back to memory. - STSETB does not have release semantics. - STSETLB stores to memory with release semantics, as described in Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes. This is an alias of [LDSETB, LDSETAB, LDSETALB, LDSETLB](ldsetb.html). This means: - The encodings in this description are named to match the encodings of [LDSETB, LDSETAB, LDSETALB, LDSETLB](ldsetb.html). - The description of [LDSETB, LDSETAB, LDSETALB, LDSETLB](ldsetb.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [LDSETB, LDSETAB, LDSETALB, LDSETLB](ldsetb.html) gives the operational pseudocode for this instruction.
