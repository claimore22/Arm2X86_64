## STTADD, STTADDL

Atomic add unprivileged, without return This instruction atomically loads a 32-bit word or 64-bit doubleword from memory, adds the value held in a register to it, and stores the result back to memory. - STTADD does not have release semantics. - STTADDL stores to memory with release semantics, as described in Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes. This is an alias of [LDTADD, LDTADDA, LDTADDAL, LDTADDL](ldtadd.html). This means: - The encodings in this description are named to match the encodings of [LDTADD, LDTADDA, LDTADDAL, LDTADDL](ldtadd.html). - The description of [LDTADD, LDTADDA, LDTADDAL, LDTADDL](ldtadd.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [LDTADD, LDTADDA, LDTADDAL, LDTADDL](ldtadd.html) gives the operational pseudocode for this instruction.
