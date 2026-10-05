## STCLRH, STCLRLH

Atomic bit clear on halfword, without return This instruction atomically loads a 16-bit halfword from memory, performs a bitwise AND with the complement of the value held in a register on it, and stores the result back to memory. - STCLRH does not have release semantics. - STCLRLH stores to memory with release semantics, as described in Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes. This is an alias of [LDCLRH, LDCLRAH, LDCLRALH, LDCLRLH](ldclrh.html). This means: - The encodings in this description are named to match the encodings of [LDCLRH, LDCLRAH, LDCLRALH, LDCLRLH](ldclrh.html). - The description of [LDCLRH, LDCLRAH, LDCLRALH, LDCLRLH](ldclrh.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [LDCLRH, LDCLRAH, LDCLRALH, LDCLRLH](ldclrh.html) gives the operational pseudocode for this instruction.
