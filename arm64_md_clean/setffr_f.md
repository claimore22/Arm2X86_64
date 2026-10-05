## SETFFR

Initialize the first-fault register to all true This instruction initializes the first-fault register (FFR) to all true prior to a sequence of first-fault or non-fault loads. This instruction is unpredicated. This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end;

### Operation

[CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; [FFR](shared_pseudocode.html#accessor_FFR_1){PL}() = Ones{PL};
