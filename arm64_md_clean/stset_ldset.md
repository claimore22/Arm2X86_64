## STSET, STSETL

Atomic bit set on word or doubleword, without return This instruction atomically loads a 32-bit word or 64-bit doubleword from memory, performs a bitwise OR with the value held in a register on it, and stores the result back to memory. - STSET does not have release semantics. - STSETL stores to memory with release semantics, as described in Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes. This is an alias of [LDSET, LDSETA, LDSETAL, LDSETL](ldset.html). This means: - The encodings in this description are named to match the encodings of [LDSET, LDSETA, LDSETAL, LDSETL](ldset.html). - The description of [LDSET, LDSETA, LDSETAL, LDSETL](ldset.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [LDSET, LDSETA, LDSETAL, LDSETL](ldset.html) gives the operational pseudocode for this instruction.
