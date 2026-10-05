## PACM

Pointer authentication modifier This instruction is used to set the value of PSTATE.PACM to 1. If FEAT_PAuth_LR is not implemented, this instruction behaves as a NOP.

#### Decode

if !IsFeatureImplemented(FEAT_PAuth_LR) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end;

### Operation

[PSTATE](shared_pseudocode.html#global_PSTATE).PACM = if [IsPACMEnabled](shared_pseudocode.html#func_IsPACMEnabled_0)() then '1' else '0';
