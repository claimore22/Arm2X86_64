## ESB

Error synchronization barrier This instruction is an error synchronization event that might also update DISR_EL1 and VDISR_EL2. This instruction can be used at all Exception levels and in Debug state. In Debug state, this instruction behaves as if SError interrupts are masked at all Exception levels. For more information, see RAS PE architecture and Arm<sup>®</sup> Reliability, Availability, and Serviceability (RAS) System Architecture, for A-profile architecture (ARM IHI 0100). If FEAT_RAS is not implemented, this instruction executes as a NOP.

#### Decode

if !IsFeatureImplemented(FEAT_RAS) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end;

### Operation

SynchronizeErrors(); [AArch64_ESBOperation](shared_pseudocode.html#func_AArch64_ESBOperation_0)(); if [PSTATE](shared_pseudocode.html#global_PSTATE).EL IN {[EL0](shared_pseudocode.html#global_EL0), [EL1](shared_pseudocode.html#global_EL1)} && [EL2Enabled](shared_pseudocode.html#func_EL2Enabled_0)() then [AArch64_vESBOperation](shared_pseudocode.html#func_AArch64_vESBOperation_0)(); elsif IsFeatureImplemented(FEAT_E3DSE) && [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL3](shared_pseudocode.html#global_EL3) then [AArch64_dESBOperation](shared_pseudocode.html#func_AArch64_dESBOperation_0)(); end; TakeUnmaskedSErrorInterrupts();
