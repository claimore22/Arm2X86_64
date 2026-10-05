## RADDHNT

Rounding add narrow high part (top) This instruction adds each vector element of the first source vector to the corresponding vector element of the second source vector, and places the most significant rounded half of the result in the odd-numbered half-width destination elements, leaving the even-numbered elements unchanged. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL) = Z{}(d); let halfesize : integer{} = esize DIV 2; for e = 0 to elements-1 do let element1 : integer = UInt(operand1[e\*:esize]); let element2 : integer = UInt(operand2[e\*:esize]); let res : integer = ((element1 + element2) + (1 << (halfesize - 1))) >> halfesize; result[(2\*e + 1)\*:halfesize] = res[halfesize-1:0]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
