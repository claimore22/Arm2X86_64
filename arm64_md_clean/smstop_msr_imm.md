## SMSTOP

Disable access to Streaming SVE mode and SME architectural state This instruction disables access to Streaming SVE mode and SME architectural state. SMSTOP exits Streaming SVE mode, and disables the SME ZA storage. SMSTOP SM exits Streaming SVE mode, but does not disable the SME ZA storage. SMSTOP ZA disables the SME ZA storage, but does not cause an exit from Streaming SVE mode. This is an alias of [MSR (immediate)](msr_imm.html). This means: - The encodings in this description are named to match the encodings of [MSR (immediate)](msr_imm.html). - The description of [MSR (immediate)](msr_imm.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [MSR (immediate)](msr_imm.html) gives the operational pseudocode for this instruction.
