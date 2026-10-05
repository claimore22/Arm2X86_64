## MUL (vectors, unpredicated)

Multiply (unpredicated) This instruction multiplies elements of the first source vector by the corresponding elements of the second source vector and places the results in the corresponding elements of the destination vector. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL); for e = 0 to elements-1 do let element1 : integer = UInt(operand1[e\*:esize]); let element2 : integer = UInt(operand2[e\*:esize]); let product : integer = element1 \* element2; result[e\*:esize] = product[esize-1:0]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
