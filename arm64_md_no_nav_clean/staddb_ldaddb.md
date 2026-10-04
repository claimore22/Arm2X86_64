## STADDB, STADDLB

Atomic add on byte, without return This instruction atomically loads an 8-bit byte from memory, adds the value held in a register to it, and stores the result back to memory. - STADDB does not have release semantics. - STADDLB stores to memory with release semantics, as described in Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes. This is an alias of [LDADDB, LDADDAB, LDADDALB, LDADDLB](ldaddb.html). This means: - The encodings in this description are named to match the encodings of [LDADDB, LDADDAB, LDADDALB, LDADDLB](ldaddb.html). - The description of [LDADDB, LDADDAB, LDADDALB, LDADDLB](ldaddb.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [LDADDB, LDADDAB, LDADDALB, LDADDLB](ldaddb.html) gives the operational pseudocode for this instruction.
