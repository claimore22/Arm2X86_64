## STEORB, STEORLB

Atomic exclusive-OR on byte, without return This instruction atomically loads an 8-bit byte from memory, performs an exclusive-OR with the value held in a register on it, and stores the result back to memory. - STEORB does not have release semantics. - STEORLB stores to memory with release semantics, as described in Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes. This is an alias of [LDEORB, LDEORAB, LDEORALB, LDEORLB](ldeorb.html). This means: - The encodings in this description are named to match the encodings of [LDEORB, LDEORAB, LDEORALB, LDEORLB](ldeorb.html). - The description of [LDEORB, LDEORAB, LDEORALB, LDEORLB](ldeorb.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [LDEORB, LDEORAB, LDEORALB, LDEORLB](ldeorb.html) gives the operational pseudocode for this instruction.
