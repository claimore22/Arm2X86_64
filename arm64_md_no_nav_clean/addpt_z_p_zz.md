## ADDPT (predicated)

Add checked pointer vectors (predicated) This instruction adds, with pointer check, Active elements of the first source vector to the corresponding elements of the second source vector, and destructively places the results in the corresponding elements of the first source vector. Inactive elements in the destination vector register remain unmodified. This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) \|\| !IsFeatureImplemented(FEAT_CPA) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let g : integer = UInt(Pg); let dn : integer = UInt(Zdn); let m : integer = UInt(Zm);

### Operation

[CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV 64; let mask : bits(PL) = P{}(g); let operand1 : bits(VL) = Z{}(dn); let operand2 : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, 64) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(m) else Zeros{VL}; var result : bits(VL); for e = 0 to elements-1 do let element1 : bits(64) = operand1[e\*:64]; let element2 : bits(64) = operand2[e\*:64]; if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, 64) then let res : bits(64) = element1 + element2; result[e\*:64] = [PointerAddCheck](shared_pseudocode.html#func_PointerAddCheck_2)(res, element1); else result[e\*:64] = operand1[e\*:64]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result; end;
