## SUBPT (unpredicated)

Subtract checked pointer vectors (unpredicated) This instruction subtracts, with pointer check, all elements of the second source vector from corresponding elements of the first source vector and places the results in the corresponding elements of the destination vector. This instruction is unpredicated. This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) \|\| !IsFeatureImplemented(FEAT_CPA) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd);

### Operation

[CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 64; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL); for e = 0 to elements-1 do let element1 : bits(64) = operand1[e\*:64]; let element2 : bits(64) = operand2[e\*:64]; let res : bits(64) = element1 - element2; result[e\*:64] = [PointerAddCheck](shared_pseudocode.html#func_PointerAddCheck_2)(res, element1); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
