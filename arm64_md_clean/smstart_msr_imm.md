## SMSTART

Enable access to Streaming SVE mode and SME architectural state This instruction enables access to Streaming SVE mode and SME architectural state. SMSTART enters Streaming SVE mode, and enables the SME ZA storage. SMSTART SM enters Streaming SVE mode, but does not enable the SME ZA storage. SMSTART ZA enables the SME ZA storage, but does not cause an entry to Streaming SVE mode. This is an alias of [MSR (immediate)](msr_imm.html). This means: - The encodings in this description are named to match the encodings of [MSR (immediate)](msr_imm.html). - The description of [MSR (immediate)](msr_imm.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [MSR (immediate)](msr_imm.html) gives the operational pseudocode for this instruction.
