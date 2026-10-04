## STEOR, STEORL

Atomic exclusive-OR on word or doubleword, without return This instruction atomically loads a 32-bit word or 64-bit doubleword from memory, performs an exclusive-OR with the value held in a register on it, and stores the result back to memory. - STEOR does not have release semantics. - STEORL stores to memory with release semantics, as described in Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes. This is an alias of [LDEOR, LDEORA, LDEORAL, LDEORL](ldeor.html). This means: - The encodings in this description are named to match the encodings of [LDEOR, LDEORA, LDEORAL, LDEORL](ldeor.html). - The description of [LDEOR, LDEORA, LDEORAL, LDEORL](ldeor.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [LDEOR, LDEORA, LDEORAL, LDEORL](ldeor.html) gives the operational pseudocode for this instruction.
