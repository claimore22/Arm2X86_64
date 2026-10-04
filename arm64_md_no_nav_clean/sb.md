## SB

Speculation barrier This instruction is a barrier that controls speculation. For more information and details of the semantics, see Speculation Barrier (SB).

#### Decode

if !IsFeatureImplemented(FEAT_SB) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end;

### Operation

SpeculationBarrier();
