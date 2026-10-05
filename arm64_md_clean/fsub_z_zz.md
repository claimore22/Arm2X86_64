## FSUB (vectors, unpredicated)

Floating-point subtract (unpredicated) This instruction subtracts all floating-point elements of the second source vector from the corresponding elements of the first source vector, and places the results in the corresponding elements of the destination vector. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL); for e = 0 to elements-1 do let element1 : bits(esize) = operand1[e\*:esize]; let element2 : bits(esize) = operand2[e\*:esize]; result[e\*:esize] = [FPSub](shared_pseudocode.html#func_FPSub_4){esize}(element1, element2, FPCR()); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
