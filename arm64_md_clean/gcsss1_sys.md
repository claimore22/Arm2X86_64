## GCSSS1

Guarded Control Stack switch stack 1 This instruction validates that the stack being switched to contains a Valid GCS cap record, stores an In-progress GCS cap record to the stack that is being switched to, and sets the current GCS pointer register to the stack that is being switched to. If the instruction generates a synchronous Data Abort exception, Watchpoint exception, GPC exception, or GCS Data Check exception, the value of GCSPR_ELx for the current Exception level is restored to the value held in the register before the instruction was executed. For more information, see GCSSS1, Guarded Control Stack Switch Stack 1. This is an alias of [SYS](sys.html). This means: - The encodings in this description are named to match the encodings of [SYS](sys.html). - The description of [SYS](sys.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [SYS](sys.html) gives the operational pseudocode for this instruction.
