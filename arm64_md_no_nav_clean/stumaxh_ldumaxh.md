## STUMAXH, STUMAXLH

Atomic unsigned maximum on halfword, without return This instruction atomically loads a 16-bit halfword from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as unsigned numbers. - STUMAXH does not have release semantics. - STUMAXLH stores to memory with release semantics, as described in Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes. This is an alias of [LDUMAXH, LDUMAXAH, LDUMAXALH, LDUMAXLH](ldumaxh.html). This means: - The encodings in this description are named to match the encodings of [LDUMAXH, LDUMAXAH, LDUMAXALH, LDUMAXLH](ldumaxh.html). - The description of [LDUMAXH, LDUMAXAH, LDUMAXALH, LDUMAXLH](ldumaxh.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [LDUMAXH, LDUMAXAH, LDUMAXALH, LDUMAXLH](ldumaxh.html) gives the operational pseudocode for this instruction.
