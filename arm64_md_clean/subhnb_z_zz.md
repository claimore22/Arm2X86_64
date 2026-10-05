## SUBHNB

Subtract narrow high part (bottom) This instruction subtracts each vector element of the second source vector from the corresponding vector element in the first source vector, and places the most significant half of the result in the even-numbered half-width destination elements, while setting the odd-numbered elements to zero. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL); let halfesize : integer{} = esize DIV 2; for e = 0 to elements-1 do let element1 : integer = UInt(operand1[e\*:esize]); let element2 : integer = UInt(operand2[e\*:esize]); let res : integer = (element1 - element2) >> halfesize; result[(2\*e + 0)\*:halfesize] = res[halfesize-1:0]; result[(2\*e + 1)\*:halfesize] = Zeros{halfesize}; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
