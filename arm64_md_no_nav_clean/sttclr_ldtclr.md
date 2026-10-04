## STTCLR, STTCLRL

Atomic bit clear unprivileged, without return This instruction atomically loads a 32-bit word or 64-bit doubleword from memory, performs a bitwise AND with the complement of the value held in a register on it, and stores the result back to memory. - STTCLR does not have release semantics. - STTCLRL stores to memory with release semantics, as described in Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes. This is an alias of [LDTCLR, LDTCLRA, LDTCLRAL, LDTCLRL](ldtclr.html). This means: - The encodings in this description are named to match the encodings of [LDTCLR, LDTCLRA, LDTCLRAL, LDTCLRL](ldtclr.html). - The description of [LDTCLR, LDTCLRA, LDTCLRAL, LDTCLRL](ldtclr.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [LDTCLR, LDTCLRA, LDTCLRAL, LDTCLRL](ldtclr.html) gives the operational pseudocode for this instruction.
