## STADD, STADDL

Atomic add on word or doubleword, without return This instruction atomically loads a 32-bit word or 64-bit doubleword from memory, adds the value held in a register to it, and stores the result back to memory. - STADD does not have release semantics. - STADDL stores to memory with release semantics, as described in Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes. This is an alias of [LDADD, LDADDA, LDADDAL, LDADDL](ldadd.html). This means: - The encodings in this description are named to match the encodings of [LDADD, LDADDA, LDADDAL, LDADDL](ldadd.html). - The description of [LDADD, LDADDA, LDADDAL, LDADDL](ldadd.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [LDADD, LDADDA, LDADDAL, LDADDL](ldadd.html) gives the operational pseudocode for this instruction.
