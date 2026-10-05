## CHKFEAT

Check feature status This instruction indicates the status of features. For more information, see Check Feature. If FEAT_CHK is not implemented, this instruction executes as a NOP.

#### Decode

if !IsFeatureImplemented(FEAT_CHK) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end;

### Operation

[X](shared_pseudocode.html#accessor_X_2){64}(16) = [AArch64_ChkFeat](shared_pseudocode.html#func_AArch64_ChkFeat_1)([X](shared_pseudocode.html#accessor_X_2){64}(16));
