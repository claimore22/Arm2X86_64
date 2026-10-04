## COSP

Clear other speculative prediction restriction by context This instruction prevents predictions, other than Cache prefetch, Control flow, and Data Value predictions, that predict execution addresses based on information gathered from earlier execution within a particular execution context. Predictions, other than Cache prefetch, Control flow, and Data Value predictions, determined by the actions of code in the target execution context or contexts appearing in program order before the instruction cannot exploitatively control any speculative access occurring after the instruction is complete and synchronized. For more information, see COSP RCTX, Clear Other Speculative Prediction Restriction by Context. This is an alias of [SYS](sys.html). This means: - The encodings in this description are named to match the encodings of [SYS](sys.html). - The description of [SYS](sys.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [SYS](sys.html) gives the operational pseudocode for this instruction.
