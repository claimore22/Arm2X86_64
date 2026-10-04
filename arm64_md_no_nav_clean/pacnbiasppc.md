## PACNBIASPPC

Pointer Authentication Code for return address, using key A, not a branch target This instruction calculates and inserts a Pointer Authentication Code for an instruction address, using two modifiers and key A. The address is in X30. The first modifier is in SP. The second modifier is the value of PC.

#### Decode

if !IsFeatureImplemented(FEAT_PAuth_LR) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = 30;

### Operation

[X](shared_pseudocode.html#accessor_X_2){64}(d) = [AddPACIA2](shared_pseudocode.html#func_AddPACIA2_3)([X](shared_pseudocode.html#accessor_X_2){64}(d), [SP](shared_pseudocode.html#accessor_SP_1){64}(), [PC64](shared_pseudocode.html#func_PC64_0)());
