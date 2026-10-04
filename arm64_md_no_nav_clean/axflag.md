## AXFLAG

Convert floating-point condition flags from Arm to external format This instruction converts the state of the PSTATE.{N,Z,C,V} flags from a form representing the result of an Arm floating-point scalar compare instruction to an alternative representation required by some software.

#### Decode

if !IsFeatureImplemented(FEAT_FlagM2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end;

### Operation

let z : bit = [PSTATE](shared_pseudocode.html#global_PSTATE).Z OR [PSTATE](shared_pseudocode.html#global_PSTATE).V; let c : bit = [PSTATE](shared_pseudocode.html#global_PSTATE).C AND NOT([PSTATE](shared_pseudocode.html#global_PSTATE).V); PSTATE.[N,Z,C,V] = '0' :: z :: c :: '0';
