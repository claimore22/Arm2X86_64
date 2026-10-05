## RDFFR (predicated)

Return predicate of successfully loaded elements This instruction reads the first-fault register (FFR) and places active elements in the corresponding elements of the destination predicate. Inactive elements in the destination predicate register are set to zero. This instruction does not set the condition flags. This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let g : integer = UInt(Pg); let d : integer = UInt(Pd); let setflags : boolean = FALSE;

### Operation

[CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let mask : bits(PL) = P{}(g); let ffr : bits(PL) = FFR{}(); let result : bits(PL) = ffr AND mask; if setflags then PSTATE.[N,Z,C,V] = [PredTest](shared_pseudocode.html#func_PredTest_4){PL}(mask, result, 8); end; [P](shared_pseudocode.html#accessor_P_2){PL}(d) = result;
