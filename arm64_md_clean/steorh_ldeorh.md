## STEORH, STEORLH

Atomic exclusive-OR on halfword, without return This instruction atomically loads a 16-bit halfword from memory, performs an exclusive-OR with the value held in a register on it, and stores the result back to memory. - STEORH does not have release semantics. - STEORLH stores to memory with release semantics, as described in Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes. This is an alias of [LDEORH, LDEORAH, LDEORALH, LDEORLH](ldeorh.html). This means: - The encodings in this description are named to match the encodings of [LDEORH, LDEORAH, LDEORALH, LDEORLH](ldeorh.html). - The description of [LDEORH, LDEORAH, LDEORALH, LDEORLH](ldeorh.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [LDEORH, LDEORAH, LDEORALH, LDEORLH](ldeorh.html) gives the operational pseudocode for this instruction.
