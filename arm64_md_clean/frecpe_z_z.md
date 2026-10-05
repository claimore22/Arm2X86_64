## FRECPE

Floating-point reciprocal estimate (unpredicated) This instruction finds the approximate reciprocal of each floating-point element of the source vector, and places the results in the corresponding elements of the destination vector. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn); let d : integer = UInt(Zd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand : bits(VL) = Z{}(n); var result : bits(VL); for e = 0 to elements-1 do let element : bits(esize) = operand[e\*:esize]; result[e\*:esize] = [FPRecipEstimate](shared_pseudocode.html#func_FPRecipEstimate_3){esize}(element, FPCR()); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
