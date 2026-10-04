## CFINV

Invert carry flag This instruction inverts the value of the PSTATE.C flag.

#### Decode

if !IsFeatureImplemented(FEAT_FlagM) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end;

### Operation

[PSTATE](shared_pseudocode.html#global_PSTATE).C = NOT([PSTATE](shared_pseudocode.html#global_PSTATE).C);
