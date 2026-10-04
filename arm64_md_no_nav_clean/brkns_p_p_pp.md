## BRKNS

Propagate break to next partition, setting the condition flags This instruction propagates break to next partition and sets the condition flags. If the Last active element of the first source predicate is false, then the destination predicate is set to all-false. Otherwise, the destination and second source predicates are left unchanged. This instruction sets the First (N), None (Z), and !Last (C) condition flags based on the predicate result, and sets the V flag to zero.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let g : integer = UInt(Pg); let n : integer = UInt(Pn); let dm : integer = UInt(Pdm); let setflags : boolean = TRUE;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let mask : bits(PL) = P{}(g); let operand1 : bits(PL) = P{}(n); let operand2 : bits(PL) = P{}(dm); var result : bits(PL); if [LastActive](shared_pseudocode.html#func_LastActive_4){PL}(mask, operand1, 8) == '1' then result = operand2; else result = Zeros{PL}; end; if setflags then PSTATE.[N,Z,C,V] = [PredTest](shared_pseudocode.html#func_PredTest_4){PL}(Ones{PL}, result, 8); end; [P](shared_pseudocode.html#accessor_P_2){PL}(dm) = result;
