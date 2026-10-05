## CLRBHB

Clear branch history This instruction can be used to clear the branch history. For more information, see Branch history.

#### Decode

if !IsFeatureImplemented(FEAT_CLRBHB) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end;

### Operation

Hint_CLRBHB();
