## XAFLAG

Convert floating-point condition flags from external format to Arm format This instruction converts the state of the PSTATE.{N,Z,C,V} flags from an alternative representation required by some software to a form representing the result of an Arm floating-point scalar compare instruction.

#### Decode

if !IsFeatureImplemented(FEAT_FlagM2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end;

### Operation

let n : bit = NOT([PSTATE](shared_pseudocode.html#global_PSTATE).C) AND NOT([PSTATE](shared_pseudocode.html#global_PSTATE).Z); let z : bit = [PSTATE](shared_pseudocode.html#global_PSTATE).Z AND [PSTATE](shared_pseudocode.html#global_PSTATE).C; let c : bit = [PSTATE](shared_pseudocode.html#global_PSTATE).C OR [PSTATE](shared_pseudocode.html#global_PSTATE).Z; let v : bit = NOT([PSTATE](shared_pseudocode.html#global_PSTATE).C) AND [PSTATE](shared_pseudocode.html#global_PSTATE).Z; [PSTATE](shared_pseudocode.html#global_PSTATE).N = n; [PSTATE](shared_pseudocode.html#global_PSTATE).Z = z; [PSTATE](shared_pseudocode.html#global_PSTATE).C = c; [PSTATE](shared_pseudocode.html#global_PSTATE).V = v;
