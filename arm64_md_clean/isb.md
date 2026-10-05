## ISB

Instruction synchronization barrier This instruction flushes the pipeline in the PE and is a context synchronization event. For more information, see Instruction Synchronization Barrier (ISB).

#### Decode

// no additional decoding required

### Operation

InstructionSynchronizationBarrier(); if IsFeatureImplemented(FEAT_BRBE) && [BRBEBranchOnISB](shared_pseudocode.html#func_BRBEBranchOnISB_0)() then [BRBEISB](shared_pseudocode.html#func_BRBEISB_0)(); end;
