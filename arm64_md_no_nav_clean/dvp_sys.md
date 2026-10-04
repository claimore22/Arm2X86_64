## DVP

Data value prediction restriction by context This instruction prevents data value predictions that predict execution addresses based on information gathered from earlier execution within a particular execution context. Data value predictions determined by the actions of code in the target execution context or contexts appearing in program order before the instruction cannot be used to exploitatively control speculative execution occurring after the instruction is complete and synchronized. For more information, see DVP RCTX, Data Value Prediction Restriction by Context. This is an alias of [SYS](sys.html). This means: - The encodings in this description are named to match the encodings of [SYS](sys.html). - The description of [SYS](sys.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [SYS](sys.html) gives the operational pseudocode for this instruction.
