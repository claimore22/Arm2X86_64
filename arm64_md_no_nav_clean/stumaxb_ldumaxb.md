## STUMAXB, STUMAXLB

Atomic unsigned maximum on byte, without return This instruction atomically loads an 8-bit byte from memory, compares it against the value held in a register, and stores the larger value back to memory, treating the values as unsigned numbers. - STUMAXB does not have release semantics. - STUMAXLB stores to memory with release semantics, as described in Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes. This is an alias of [LDUMAXB, LDUMAXAB, LDUMAXALB, LDUMAXLB](ldumaxb.html). This means: - The encodings in this description are named to match the encodings of [LDUMAXB, LDUMAXAB, LDUMAXALB, LDUMAXLB](ldumaxb.html). - The description of [LDUMAXB, LDUMAXAB, LDUMAXALB, LDUMAXLB](ldumaxb.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [LDUMAXB, LDUMAXAB, LDUMAXALB, LDUMAXLB](ldumaxb.html) gives the operational pseudocode for this instruction.
