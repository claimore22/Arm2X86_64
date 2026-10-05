## GCSB

Guarded Control Stack barrier This instruction generates a GCSB effect. If FEAT_GCS is not implemented, this instruction executes as a NOP.

#### Decode

if !IsFeatureImplemented(FEAT_GCS) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end;

### Operation

GCSSynchronizationBarrier();
