## GCSPOPCX

Guarded Control Stack pop and compare exception return record This instruction loads an exception return record from the location indicated by the current GCS pointer register, compares the loaded values with the current ELR_ELx, SPSR_ELx, and LR, and increments the pointer by the size of a GCS exception return record. For more information, see GCSPOPCX, Guarded Control Stack Pop and Compare exception return record. This is an alias of [SYS](sys.html). This means: - The encodings in this description are named to match the encodings of [SYS](sys.html). - The description of [SYS](sys.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [SYS](sys.html) gives the operational pseudocode for this instruction.
