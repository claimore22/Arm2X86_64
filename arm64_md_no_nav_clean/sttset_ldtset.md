## STTSET, STTSETL

Atomic bit set unprivileged, without return This instruction atomically loads a 32-bit word or 64-bit doubleword from memory, performs a bitwise OR with the value held in a register on it, and stores the result back to memory. - STTSET does not have release semantics. - STTSETL stores to memory with release semantics, as described in Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes. This is an alias of [LDTSET, LDTSETA, LDTSETAL, LDTSETL](ldtset.html). This means: - The encodings in this description are named to match the encodings of [LDTSET, LDTSETA, LDTSETAL, LDTSETL](ldtset.html). - The description of [LDTSET, LDTSETA, LDTSETAL, LDTSETL](ldtset.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [LDTSET, LDTSETA, LDTSETAL, LDTSETL](ldtset.html) gives the operational pseudocode for this instruction.
