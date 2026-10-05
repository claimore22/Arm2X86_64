## STCLRB, STCLRLB

Atomic bit clear on byte, without return This instruction atomically loads an 8-bit byte from memory, performs a bitwise AND with the complement of the value held in a register on it, and stores the result back to memory. - STCLRB does not have release semantics. - STCLRLB stores to memory with release semantics, as described in Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes. This is an alias of [LDCLRB, LDCLRAB, LDCLRALB, LDCLRLB](ldclrb.html). This means: - The encodings in this description are named to match the encodings of [LDCLRB, LDCLRAB, LDCLRALB, LDCLRLB](ldclrb.html). - The description of [LDCLRB, LDCLRAB, LDCLRALB, LDCLRLB](ldclrb.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [LDCLRB, LDCLRAB, LDCLRALB, LDCLRLB](ldclrb.html) gives the operational pseudocode for this instruction.
